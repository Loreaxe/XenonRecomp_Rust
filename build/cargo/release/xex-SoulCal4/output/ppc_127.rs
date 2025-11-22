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


pub fn sub_826E6AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6AF0 size=100
    let mut pc: u32 = 0x826E6AF0;
    'dispatch: loop {
        match pc {
            0x826E6AF0 => {
    //   block [0x826E6AF0..0x826E6B54)
	// 826E6AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6B04: 38AA355C  addi r5, r10, 0x355c
	ctx.r[5].s64 = ctx.r[10].s64 + 13660;
	// 826E6B08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6B10: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 826E6B14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6B24: 386A36AC  addi r3, r10, 0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13996;
	// 826E6B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6B2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6B30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E6B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6B38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E6B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6B40: 4BD802E1  bl 0x82466e20
	ctx.lr = 0x826E6B44;
	sub_82466E20(ctx, base);
	// 826E6B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6B58 size=112
    let mut pc: u32 = 0x826E6B58;
    'dispatch: loop {
        match pc {
            0x826E6B58 => {
    //   block [0x826E6B58..0x826E6BC8)
	// 826E6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6B64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6B68: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6B6C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E6B70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6B74: 390B1360  addi r8, r11, 0x1360
	ctx.r[8].s64 = ctx.r[11].s64 + 4960;
	// 826E6B78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E6B7C: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 826E6B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6B90: 386A36DC  addi r3, r10, 0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14044;
	// 826E6B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6BB4: 4BD8026D  bl 0x82466e20
	ctx.lr = 0x826E6BB8;
	sub_82466E20(ctx, base);
	// 826E6BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6BC8 size=96
    let mut pc: u32 = 0x826E6BC8;
    'dispatch: loop {
        match pc {
            0x826E6BC8 => {
    //   block [0x826E6BC8..0x826E6C28)
	// 826E6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6BD4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6BDC: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 826E6BE0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6BE8: 386A370C  addi r3, r10, 0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + 14092;
	// 826E6BEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6BF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E6BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6C08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E6C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6C10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E6C14: 4BD8020D  bl 0x82466e20
	ctx.lr = 0x826E6C18;
	sub_82466E20(ctx, base);
	// 826E6C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6C28 size=112
    let mut pc: u32 = 0x826E6C28;
    'dispatch: loop {
        match pc {
            0x826E6C28 => {
    //   block [0x826E6C28..0x826E6C98)
	// 826E6C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6C34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6C38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6C3C: 38AA370C  addi r5, r10, 0x370c
	ctx.r[5].s64 = ctx.r[10].s64 + 14092;
	// 826E6C40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6C44: 390B1390  addi r8, r11, 0x1390
	ctx.r[8].s64 = ctx.r[11].s64 + 5008;
	// 826E6C48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E6C4C: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 826E6C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6C54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6C60: 386A373C  addi r3, r10, 0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + 14140;
	// 826E6C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6C84: 4BD8019D  bl 0x82466e20
	ctx.lr = 0x826E6C88;
	sub_82466E20(ctx, base);
	// 826E6C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6C98 size=112
    let mut pc: u32 = 0x826E6C98;
    'dispatch: loop {
        match pc {
            0x826E6C98 => {
    //   block [0x826E6C98..0x826E6D08)
	// 826E6C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6CA8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6CAC: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E6CB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6CB4: 390B13A8  addi r8, r11, 0x13a8
	ctx.r[8].s64 = ctx.r[11].s64 + 5032;
	// 826E6CB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E6CBC: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 826E6CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6CC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6CD0: 386A376C  addi r3, r10, 0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + 14188;
	// 826E6CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6CE4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E6CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6CF4: 4BD8012D  bl 0x82466e20
	ctx.lr = 0x826E6CF8;
	sub_82466E20(ctx, base);
	// 826E6CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6D08 size=112
    let mut pc: u32 = 0x826E6D08;
    'dispatch: loop {
        match pc {
            0x826E6D08 => {
    //   block [0x826E6D08..0x826E6D78)
	// 826E6D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6D14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6D18: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6D1C: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 826E6D20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6D24: 390B13C0  addi r8, r11, 0x13c0
	ctx.r[8].s64 = ctx.r[11].s64 + 5056;
	// 826E6D28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E6D2C: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 826E6D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6D34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6D40: 386A379C  addi r3, r10, 0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + 14236;
	// 826E6D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6D64: 4BD800BD  bl 0x82466e20
	ctx.lr = 0x826E6D68;
	sub_82466E20(ctx, base);
	// 826E6D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E6D78 size=36
    let mut pc: u32 = 0x826E6D78;
    'dispatch: loop {
        match pc {
            0x826E6D78 => {
    //   block [0x826E6D78..0x826E6D9C)
	// 826E6D78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6D7C: 814B140C  lwz r10, 0x140c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5132 as u32) ) } as u64;
	// 826E6D80: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6D84: 396B77B8  addi r11, r11, 0x77b8
	ctx.r[11].s64 = ctx.r[11].s64 + 30648;
	// 826E6D88: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826E6D8C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E6D90: 814A12FC  lwz r10, 0x12fc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4860 as u32) ) } as u64;
	// 826E6D94: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826E6D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6DA0 size=108
    let mut pc: u32 = 0x826E6DA0;
    'dispatch: loop {
        match pc {
            0x826E6DA0 => {
    //   block [0x826E6DA0..0x826E6E0C)
	// 826E6DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6DAC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6DB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6DB4: 38EB77B8  addi r7, r11, 0x77b8
	ctx.r[7].s64 = ctx.r[11].s64 + 30648;
	// 826E6DB8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826E6DBC: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 826E6DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6DC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E6DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6DD0: 386A37CC  addi r3, r10, 0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14284;
	// 826E6DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E6DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E6DF8: 4BD80029  bl 0x82466e20
	ctx.lr = 0x826E6DFC;
	sub_82466E20(ctx, base);
	// 826E6DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E6E10 size=24
    let mut pc: u32 = 0x826E6E10;
    'dispatch: loop {
        match pc {
            0x826E6E10 => {
    //   block [0x826E6E10..0x826E6E28)
	// 826E6E10: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6E14: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E6E18: 394A7860  addi r10, r10, 0x7860
	ctx.r[10].s64 = ctx.r[10].s64 + 30816;
	// 826E6E1C: 816B12FC  lwz r11, 0x12fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4860 as u32) ) } as u64;
	// 826E6E20: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826E6E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6E28 size=116
    let mut pc: u32 = 0x826E6E28;
    'dispatch: loop {
        match pc {
            0x826E6E28 => {
    //   block [0x826E6E28..0x826E6E9C)
	// 826E6E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6E34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E6E38: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826E6E3C: 390A7860  addi r8, r10, 0x7860
	ctx.r[8].s64 = ctx.r[10].s64 + 30816;
	// 826E6E40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6E44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E6E48: 38AA37CC  addi r5, r10, 0x37cc
	ctx.r[5].s64 = ctx.r[10].s64 + 14284;
	// 826E6E4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6E50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E6E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6E5C: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 826E6E60: 396B7DBC  addi r11, r11, 0x7dbc
	ctx.r[11].s64 = ctx.r[11].s64 + 32188;
	// 826E6E64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6E68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6E6C: 386A37FC  addi r3, r10, 0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14332;
	// 826E6E70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E6E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6E78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E6E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6E88: 4BD7FF99  bl 0x82466e20
	ctx.lr = 0x826E6E8C;
	sub_82466E20(ctx, base);
	// 826E6E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6EA0 size=112
    let mut pc: u32 = 0x826E6EA0;
    'dispatch: loop {
        match pc {
            0x826E6EA0 => {
    //   block [0x826E6EA0..0x826E6F10)
	// 826E6EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6EB0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6EB4: 38AA37CC  addi r5, r10, 0x37cc
	ctx.r[5].s64 = ctx.r[10].s64 + 14284;
	// 826E6EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6EBC: 390B1410  addi r8, r11, 0x1410
	ctx.r[8].s64 = ctx.r[11].s64 + 5136;
	// 826E6EC0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E6EC4: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 826E6EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E6ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6ED8: 386A382C  addi r3, r10, 0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + 14380;
	// 826E6EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E6EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6EFC: 4BD7FF25  bl 0x82466e20
	ctx.lr = 0x826E6F00;
	sub_82466E20(ctx, base);
	// 826E6F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E6F10 size=24
    let mut pc: u32 = 0x826E6F10;
    'dispatch: loop {
        match pc {
            0x826E6F10 => {
    //   block [0x826E6F10..0x826E6F28)
	// 826E6F10: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6F14: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E6F18: 394A7950  addi r10, r10, 0x7950
	ctx.r[10].s64 = ctx.r[10].s64 + 31056;
	// 826E6F1C: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 826E6F20: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826E6F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6F28 size=116
    let mut pc: u32 = 0x826E6F28;
    'dispatch: loop {
        match pc {
            0x826E6F28 => {
    //   block [0x826E6F28..0x826E6F9C)
	// 826E6F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6F34: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E6F38: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6F3C: 392B7D80  addi r9, r11, 0x7d80
	ctx.r[9].s64 = ctx.r[11].s64 + 32128;
	// 826E6F40: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 826E6F44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6F48: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826E6F4C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826E6F50: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E6F54: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 826E6F58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6F5C: 396B7950  addi r11, r11, 0x7950
	ctx.r[11].s64 = ctx.r[11].s64 + 31056;
	// 826E6F60: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E6F64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6F68: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E6F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6F70: 386A385C  addi r3, r10, 0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + 14428;
	// 826E6F74: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826E6F78: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E6F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6F80: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E6F84: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E6F88: 4BD7FE99  bl 0x82466e20
	ctx.lr = 0x826E6F8C;
	sub_82466E20(ctx, base);
	// 826E6F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E6F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E6FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E6FA0 size=100
    let mut pc: u32 = 0x826E6FA0;
    'dispatch: loop {
        match pc {
            0x826E6FA0 => {
    //   block [0x826E6FA0..0x826E7004)
	// 826E6FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E6FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E6FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E6FAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E6FB4: 38AA397C  addi r5, r10, 0x397c
	ctx.r[5].s64 = ctx.r[10].s64 + 14716;
	// 826E6FB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E6FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E6FC0: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 826E6FC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E6FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E6FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E6FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E6FD4: 386A388C  addi r3, r10, 0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + 14476;
	// 826E6FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E6FDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E6FE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E6FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E6FE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E6FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E6FF0: 4BD7FE31  bl 0x82466e20
	ctx.lr = 0x826E6FF4;
	sub_82466E20(ctx, base);
	// 826E6FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E6FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E6FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7008 size=108
    let mut pc: u32 = 0x826E7008;
    'dispatch: loop {
        match pc {
            0x826E7008 => {
    //   block [0x826E7008..0x826E7074)
	// 826E7008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E700C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7014: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E701C: 38EB1488  addi r7, r11, 0x1488
	ctx.r[7].s64 = ctx.r[11].s64 + 5256;
	// 826E7020: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E7024: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 826E7028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E702C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E7034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7038: 386A38BC  addi r3, r10, 0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14524;
	// 826E703C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E7040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E704C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E705C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E7060: 4BD7FDC1  bl 0x82466e20
	ctx.lr = 0x826E7064;
	sub_82466E20(ctx, base);
	// 826E7064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E706C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7078 size=112
    let mut pc: u32 = 0x826E7078;
    'dispatch: loop {
        match pc {
            0x826E7078 => {
    //   block [0x826E7078..0x826E70E8)
	// 826E7078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7088: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E708C: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 826E7090: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7094: 390B14E8  addi r8, r11, 0x14e8
	ctx.r[8].s64 = ctx.r[11].s64 + 5352;
	// 826E7098: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E709C: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 826E70A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E70A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E70A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E70AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E70B0: 386A38EC  addi r3, r10, 0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14572;
	// 826E70B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E70B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E70BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E70C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E70C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E70C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E70CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E70D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E70D4: 4BD7FD4D  bl 0x82466e20
	ctx.lr = 0x826E70D8;
	sub_82466E20(ctx, base);
	// 826E70D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E70DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E70E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E70E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E70E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E70E8 size=108
    let mut pc: u32 = 0x826E70E8;
    'dispatch: loop {
        match pc {
            0x826E70E8 => {
    //   block [0x826E70E8..0x826E7154)
	// 826E70E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E70EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E70F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E70F4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E70F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E70FC: 38EB1548  addi r7, r11, 0x1548
	ctx.r[7].s64 = ctx.r[11].s64 + 5448;
	// 826E7100: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E7104: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 826E7108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E710C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E7114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7118: 386A391C  addi r3, r10, 0x391c
	ctx.r[3].s64 = ctx.r[10].s64 + 14620;
	// 826E711C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E7120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E712C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E713C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E7140: 4BD7FCE1  bl 0x82466e20
	ctx.lr = 0x826E7144;
	sub_82466E20(ctx, base);
	// 826E7144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E714C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E7158 size=28
    let mut pc: u32 = 0x826E7158;
    'dispatch: loop {
        match pc {
            0x826E7158 => {
    //   block [0x826E7158..0x826E7174)
	// 826E7158: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E715C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E7160: 394A7A58  addi r10, r10, 0x7a58
	ctx.r[10].s64 = ctx.r[10].s64 + 31320;
	// 826E7164: 816B1560  lwz r11, 0x1560(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5472 as u32) ) } as u64;
	// 826E7168: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E716C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826E7170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7178 size=112
    let mut pc: u32 = 0x826E7178;
    'dispatch: loop {
        match pc {
            0x826E7178 => {
    //   block [0x826E7178..0x826E71E8)
	// 826E7178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E717C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7184: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E7188: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826E718C: 38EA7A58  addi r7, r10, 0x7a58
	ctx.r[7].s64 = ctx.r[10].s64 + 31320;
	// 826E7190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7194: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E7198: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 826E719C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E71A0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E71A4: 396B7E80  addi r11, r11, 0x7e80
	ctx.r[11].s64 = ctx.r[11].s64 + 32384;
	// 826E71A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E71AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E71B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E71B4: 386A394C  addi r3, r10, 0x394c
	ctx.r[3].s64 = ctx.r[10].s64 + 14668;
	// 826E71B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E71BC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E71C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E71C4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E71C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E71CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E71D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E71D4: 4BD7FC4D  bl 0x82466e20
	ctx.lr = 0x826E71D8;
	sub_82466E20(ctx, base);
	// 826E71D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E71DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E71E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E71E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E71E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E71E8 size=24
    let mut pc: u32 = 0x826E71E8;
    'dispatch: loop {
        match pc {
            0x826E71E8 => {
    //   block [0x826E71E8..0x826E7200)
	// 826E71E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E71EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E71F0: 394A7BC0  addi r10, r10, 0x7bc0
	ctx.r[10].s64 = ctx.r[10].s64 + 31680;
	// 826E71F4: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 826E71F8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E71FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7200 size=116
    let mut pc: u32 = 0x826E7200;
    'dispatch: loop {
        match pc {
            0x826E7200 => {
    //   block [0x826E7200..0x826E7274)
	// 826E7200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E720C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E7210: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7214: 392B7E58  addi r9, r11, 0x7e58
	ctx.r[9].s64 = ctx.r[11].s64 + 32344;
	// 826E7218: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 826E721C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7220: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826E7224: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826E7228: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E722C: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 826E7230: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7234: 396B7BC0  addi r11, r11, 0x7bc0
	ctx.r[11].s64 = ctx.r[11].s64 + 31680;
	// 826E7238: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E723C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7240: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E7244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7248: 386A397C  addi r3, r10, 0x397c
	ctx.r[3].s64 = ctx.r[10].s64 + 14716;
	// 826E724C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E7250: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E7254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7258: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E725C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E7260: 4BD7FBC1  bl 0x82466e20
	ctx.lr = 0x826E7264;
	sub_82466E20(ctx, base);
	// 826E7264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E726C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7278 size=108
    let mut pc: u32 = 0x826E7278;
    'dispatch: loop {
        match pc {
            0x826E7278 => {
    //   block [0x826E7278..0x826E72E4)
	// 826E7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7284: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E728C: 38EB1568  addi r7, r11, 0x1568
	ctx.r[7].s64 = ctx.r[11].s64 + 5480;
	// 826E7290: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E7294: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 826E7298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E729C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E72A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E72A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E72A8: 386A39AC  addi r3, r10, 0x39ac
	ctx.r[3].s64 = ctx.r[10].s64 + 14764;
	// 826E72AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E72B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E72B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E72B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E72BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E72C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E72C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E72C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E72CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E72D0: 4BD7FB51  bl 0x82466e20
	ctx.lr = 0x826E72D4;
	sub_82466E20(ctx, base);
	// 826E72D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E72D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E72DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E72E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E72E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E72E8 size=24
    let mut pc: u32 = 0x826E72E8;
    'dispatch: loop {
        match pc {
            0x826E72E8 => {
    //   block [0x826E72E8..0x826E7300)
	// 826E72E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E72EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E72F0: 394A7C68  addi r10, r10, 0x7c68
	ctx.r[10].s64 = ctx.r[10].s64 + 31848;
	// 826E72F4: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 826E72F8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E72FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7300 size=116
    let mut pc: u32 = 0x826E7300;
    'dispatch: loop {
        match pc {
            0x826E7300 => {
    //   block [0x826E7300..0x826E7374)
	// 826E7300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E730C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E7310: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E7314: 390A7C68  addi r8, r10, 0x7c68
	ctx.r[8].s64 = ctx.r[10].s64 + 31848;
	// 826E7318: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E731C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E7320: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 826E7324: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7328: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E732C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7334: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 826E7338: 396B7EE0  addi r11, r11, 0x7ee0
	ctx.r[11].s64 = ctx.r[11].s64 + 32480;
	// 826E733C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7340: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7344: 386A39DC  addi r3, r10, 0x39dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14812;
	// 826E7348: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E734C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7350: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E7354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E735C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7360: 4BD7FAC1  bl 0x82466e20
	ctx.lr = 0x826E7364;
	sub_82466E20(ctx, base);
	// 826E7364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7378 size=112
    let mut pc: u32 = 0x826E7378;
    'dispatch: loop {
        match pc {
            0x826E7378 => {
    //   block [0x826E7378..0x826E73E8)
	// 826E7378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E737C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7388: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E738C: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E7390: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7394: 390B15C8  addi r8, r11, 0x15c8
	ctx.r[8].s64 = ctx.r[11].s64 + 5576;
	// 826E7398: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E739C: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 826E73A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E73A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E73A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E73AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E73B0: 386A3A0C  addi r3, r10, 0x3a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 14860;
	// 826E73B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E73B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E73BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E73C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E73C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E73C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E73CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E73D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E73D4: 4BD7FA4D  bl 0x82466e20
	ctx.lr = 0x826E73D8;
	sub_82466E20(ctx, base);
	// 826E73D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E73DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E73E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E73E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E73E8 size=112
    let mut pc: u32 = 0x826E73E8;
    'dispatch: loop {
        match pc {
            0x826E73E8 => {
    //   block [0x826E73E8..0x826E7458)
	// 826E73E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E73EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E73F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E73F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E73F8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E73FC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E7400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7404: 390B1658  addi r8, r11, 0x1658
	ctx.r[8].s64 = ctx.r[11].s64 + 5720;
	// 826E7408: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E740C: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 826E7410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E741C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7420: 386A3A3C  addi r3, r10, 0x3a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 14908;
	// 826E7424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E742C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E743C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7444: 4BD7F9DD  bl 0x82466e20
	ctx.lr = 0x826E7448;
	sub_82466E20(ctx, base);
	// 826E7448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E744C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7458 size=112
    let mut pc: u32 = 0x826E7458;
    'dispatch: loop {
        match pc {
            0x826E7458 => {
    //   block [0x826E7458..0x826E74C8)
	// 826E7458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7464: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7468: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E746C: 38AA385C  addi r5, r10, 0x385c
	ctx.r[5].s64 = ctx.r[10].s64 + 14428;
	// 826E7470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7474: 390B16B8  addi r8, r11, 0x16b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5816;
	// 826E7478: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E747C: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 826E7480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7490: 386A3A6C  addi r3, r10, 0x3a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 14956;
	// 826E7494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E749C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E74A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E74A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E74A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E74AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E74B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E74B4: 4BD7F96D  bl 0x82466e20
	ctx.lr = 0x826E74B8;
	sub_82466E20(ctx, base);
	// 826E74B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E74BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E74C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E74C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E74C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E74C8 size=112
    let mut pc: u32 = 0x826E74C8;
    'dispatch: loop {
        match pc {
            0x826E74C8 => {
    //   block [0x826E74C8..0x826E7538)
	// 826E74C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E74CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E74D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E74D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E74D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E74DC: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E74E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E74E4: 390B16E8  addi r8, r11, 0x16e8
	ctx.r[8].s64 = ctx.r[11].s64 + 5864;
	// 826E74E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E74EC: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 826E74F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E74F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E74F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E74FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7500: 386A3A9C  addi r3, r10, 0x3a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15004;
	// 826E7504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E750C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E751C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7524: 4BD7F8FD  bl 0x82466e20
	ctx.lr = 0x826E7528;
	sub_82466E20(ctx, base);
	// 826E7528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E752C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7538 size=112
    let mut pc: u32 = 0x826E7538;
    'dispatch: loop {
        match pc {
            0x826E7538 => {
    //   block [0x826E7538..0x826E75A8)
	// 826E7538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E753C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7548: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E754C: 38AA397C  addi r5, r10, 0x397c
	ctx.r[5].s64 = ctx.r[10].s64 + 14716;
	// 826E7550: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7554: 390B1778  addi r8, r11, 0x1778
	ctx.r[8].s64 = ctx.r[11].s64 + 6008;
	// 826E7558: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E755C: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 826E7560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E756C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7570: 386A3ACC  addi r3, r10, 0x3acc
	ctx.r[3].s64 = ctx.r[10].s64 + 15052;
	// 826E7574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E757C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E758C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7594: 4BD7F88D  bl 0x82466e20
	ctx.lr = 0x826E7598;
	sub_82466E20(ctx, base);
	// 826E7598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E759C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E75A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E75A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E75A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E75A8 size=100
    let mut pc: u32 = 0x826E75A8;
    'dispatch: loop {
        match pc {
            0x826E75A8 => {
    //   block [0x826E75A8..0x826E760C)
	// 826E75A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E75AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E75B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E75B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E75B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E75BC: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E75C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E75C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E75C8: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 826E75CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E75D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E75D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E75D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E75DC: 386A3AFC  addi r3, r10, 0x3afc
	ctx.r[3].s64 = ctx.r[10].s64 + 15100;
	// 826E75E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E75E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E75E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E75EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E75F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E75F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E75F8: 4BD7F829  bl 0x82466e20
	ctx.lr = 0x826E75FC;
	sub_82466E20(ctx, base);
	// 826E75FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7610 size=112
    let mut pc: u32 = 0x826E7610;
    'dispatch: loop {
        match pc {
            0x826E7610 => {
    //   block [0x826E7610..0x826E7680)
	// 826E7610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E761C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7620: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7624: 38AA3AFC  addi r5, r10, 0x3afc
	ctx.r[5].s64 = ctx.r[10].s64 + 15100;
	// 826E7628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E762C: 390B1790  addi r8, r11, 0x1790
	ctx.r[8].s64 = ctx.r[11].s64 + 6032;
	// 826E7630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E7634: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 826E7638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E763C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7648: 386A3B2C  addi r3, r10, 0x3b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15148;
	// 826E764C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E765C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E766C: 4BD7F7B5  bl 0x82466e20
	ctx.lr = 0x826E7670;
	sub_82466E20(ctx, base);
	// 826E7670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E767C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7680 size=112
    let mut pc: u32 = 0x826E7680;
    'dispatch: loop {
        match pc {
            0x826E7680 => {
    //   block [0x826E7680..0x826E76F0)
	// 826E7680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E768C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7690: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7694: 38AA3B2C  addi r5, r10, 0x3b2c
	ctx.r[5].s64 = ctx.r[10].s64 + 15148;
	// 826E7698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E769C: 390B17F0  addi r8, r11, 0x17f0
	ctx.r[8].s64 = ctx.r[11].s64 + 6128;
	// 826E76A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E76A4: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 826E76A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E76AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E76B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E76B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E76B8: 386A3B5C  addi r3, r10, 0x3b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15196;
	// 826E76BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E76C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E76C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E76C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E76CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E76D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E76D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E76D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E76DC: 4BD7F745  bl 0x82466e20
	ctx.lr = 0x826E76E0;
	sub_82466E20(ctx, base);
	// 826E76E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E76E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E76E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E76EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E76F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E76F0 size=24
    let mut pc: u32 = 0x826E76F0;
    'dispatch: loop {
        match pc {
            0x826E76F0 => {
    //   block [0x826E76F0..0x826E7708)
	// 826E76F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E76F4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E76F8: 394A7CE0  addi r10, r10, 0x7ce0
	ctx.r[10].s64 = ctx.r[10].s64 + 31968;
	// 826E76FC: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 826E7700: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E7704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7708 size=116
    let mut pc: u32 = 0x826E7708;
    'dispatch: loop {
        match pc {
            0x826E7708 => {
    //   block [0x826E7708..0x826E777C)
	// 826E7708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E770C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7714: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E7718: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E771C: 390A7CE0  addi r8, r10, 0x7ce0
	ctx.r[8].s64 = ctx.r[10].s64 + 31968;
	// 826E7720: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7724: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E7728: 38AA3B2C  addi r5, r10, 0x3b2c
	ctx.r[5].s64 = ctx.r[10].s64 + 15148;
	// 826E772C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7730: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E7734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E773C: 388AB78C  addi r4, r10, -0x4874
	ctx.r[4].s64 = ctx.r[10].s64 + -18548;
	// 826E7740: 396B7EF8  addi r11, r11, 0x7ef8
	ctx.r[11].s64 = ctx.r[11].s64 + 32504;
	// 826E7744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E774C: 386A3B8C  addi r3, r10, 0x3b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15244;
	// 826E7750: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E7754: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7758: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E775C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7768: 4BD7F6B9  bl 0x82466e20
	ctx.lr = 0x826E776C;
	sub_82466E20(ctx, base);
	// 826E776C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7780 size=112
    let mut pc: u32 = 0x826E7780;
    'dispatch: loop {
        match pc {
            0x826E7780 => {
    //   block [0x826E7780..0x826E77F0)
	// 826E7780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E778C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7790: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7794: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E7798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E779C: 390B1820  addi r8, r11, 0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + 6176;
	// 826E77A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E77A4: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 826E77A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E77AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E77B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E77B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E77B8: 386A3BBC  addi r3, r10, 0x3bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15292;
	// 826E77BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E77C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E77C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E77C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E77CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E77D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E77D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E77D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E77DC: 4BD7F645  bl 0x82466e20
	ctx.lr = 0x826E77E0;
	sub_82466E20(ctx, base);
	// 826E77E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E77E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E77E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E77EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E77F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E77F0 size=116
    let mut pc: u32 = 0x826E77F0;
    'dispatch: loop {
        match pc {
            0x826E77F0 => {
    //   block [0x826E77F0..0x826E7864)
	// 826E77F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E77F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E77F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E77FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7800: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E7804: 390B1854  addi r8, r11, 0x1854
	ctx.r[8].s64 = ctx.r[11].s64 + 6228;
	// 826E7808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E780C: 392A7F38  addi r9, r10, 0x7f38
	ctx.r[9].s64 = ctx.r[10].s64 + 32568;
	// 826E7810: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7814: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E7818: 38AA3E8C  addi r5, r10, 0x3e8c
	ctx.r[5].s64 = ctx.r[10].s64 + 16012;
	// 826E781C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7824: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E782C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7834: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E7838: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 826E783C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E7840: 386B3BEC  addi r3, r11, 0x3bec
	ctx.r[3].s64 = ctx.r[11].s64 + 15340;
	// 826E7844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E7848: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E784C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7850: 4BD7F5D1  bl 0x82466e20
	ctx.lr = 0x826E7854;
	sub_82466E20(ctx, base);
	// 826E7854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E785C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7868 size=112
    let mut pc: u32 = 0x826E7868;
    'dispatch: loop {
        match pc {
            0x826E7868 => {
    //   block [0x826E7868..0x826E78D8)
	// 826E7868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E786C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7878: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E787C: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7884: 390B186C  addi r8, r11, 0x186c
	ctx.r[8].s64 = ctx.r[11].s64 + 6252;
	// 826E7888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E788C: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 826E7890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7894: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E789C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E78A0: 386A3C1C  addi r3, r10, 0x3c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15388;
	// 826E78A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E78A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E78AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E78B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E78B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E78B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E78BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E78C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E78C4: 4BD7F55D  bl 0x82466e20
	ctx.lr = 0x826E78C8;
	sub_82466E20(ctx, base);
	// 826E78C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E78CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E78D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E78D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E78D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E78D8 size=116
    let mut pc: u32 = 0x826E78D8;
    'dispatch: loop {
        match pc {
            0x826E78D8 => {
    //   block [0x826E78D8..0x826E794C)
	// 826E78D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E78DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E78E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E78E4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E78E8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826E78EC: 390B1888  addi r8, r11, 0x1888
	ctx.r[8].s64 = ctx.r[11].s64 + 6280;
	// 826E78F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E78F4: 392A7F64  addi r9, r10, 0x7f64
	ctx.r[9].s64 = ctx.r[10].s64 + 32612;
	// 826E78F8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E78FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826E7900: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7904: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E790C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E791C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E7920: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 826E7924: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E7928: 386B3C4C  addi r3, r11, 0x3c4c
	ctx.r[3].s64 = ctx.r[11].s64 + 15436;
	// 826E792C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E7930: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7938: 4BD7F4E9  bl 0x82466e20
	ctx.lr = 0x826E793C;
	sub_82466E20(ctx, base);
	// 826E793C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7950 size=112
    let mut pc: u32 = 0x826E7950;
    'dispatch: loop {
        match pc {
            0x826E7950 => {
    //   block [0x826E7950..0x826E79C0)
	// 826E7950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E795C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7960: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7964: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7968: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E796C: 390B18B8  addi r8, r11, 0x18b8
	ctx.r[8].s64 = ctx.r[11].s64 + 6328;
	// 826E7970: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E7974: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 826E7978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E797C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7988: 386A3C7C  addi r3, r10, 0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15484;
	// 826E798C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E799C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E79A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E79A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E79A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E79AC: 4BD7F475  bl 0x82466e20
	ctx.lr = 0x826E79B0;
	sub_82466E20(ctx, base);
	// 826E79B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E79B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E79B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E79BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E79C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E79C0 size=112
    let mut pc: u32 = 0x826E79C0;
    'dispatch: loop {
        match pc {
            0x826E79C0 => {
    //   block [0x826E79C0..0x826E7A30)
	// 826E79C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E79C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E79C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E79CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E79D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E79D4: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E79D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E79DC: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 826E79E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E79E4: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 826E79E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E79EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E79F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E79F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E79F8: 386A3CAC  addi r3, r10, 0x3cac
	ctx.r[3].s64 = ctx.r[10].s64 + 15532;
	// 826E79FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7A1C: 4BD7F405  bl 0x82466e20
	ctx.lr = 0x826E7A20;
	sub_82466E20(ctx, base);
	// 826E7A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7A30 size=108
    let mut pc: u32 = 0x826E7A30;
    'dispatch: loop {
        match pc {
            0x826E7A30 => {
    //   block [0x826E7A30..0x826E7A9C)
	// 826E7A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7A3C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7A40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7A44: 38EB1948  addi r7, r11, 0x1948
	ctx.r[7].s64 = ctx.r[11].s64 + 6472;
	// 826E7A48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E7A4C: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 826E7A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7A54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7A58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E7A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7A60: 386A3CDC  addi r3, r10, 0x3cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 15580;
	// 826E7A64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E7A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E7A88: 4BD7F399  bl 0x82466e20
	ctx.lr = 0x826E7A8C;
	sub_82466E20(ctx, base);
	// 826E7A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7AA0 size=112
    let mut pc: u32 = 0x826E7AA0;
    'dispatch: loop {
        match pc {
            0x826E7AA0 => {
    //   block [0x826E7AA0..0x826E7B10)
	// 826E7AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7AAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7AB0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7AB4: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7ABC: 390B1990  addi r8, r11, 0x1990
	ctx.r[8].s64 = ctx.r[11].s64 + 6544;
	// 826E7AC0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E7AC4: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 826E7AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7ACC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7AD8: 386A3D0C  addi r3, r10, 0x3d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15628;
	// 826E7ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7AFC: 4BD7F325  bl 0x82466e20
	ctx.lr = 0x826E7B00;
	sub_82466E20(ctx, base);
	// 826E7B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7B10 size=116
    let mut pc: u32 = 0x826E7B10;
    'dispatch: loop {
        match pc {
            0x826E7B10 => {
    //   block [0x826E7B10..0x826E7B84)
	// 826E7B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7B1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E7B20: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7B24: 392B7FA0  addi r9, r11, 0x7fa0
	ctx.r[9].s64 = ctx.r[11].s64 + 32672;
	// 826E7B28: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7B2C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7B30: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E7B34: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826E7B38: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7B3C: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 826E7B40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7B44: 396B1A10  addi r11, r11, 0x1a10
	ctx.r[11].s64 = ctx.r[11].s64 + 6672;
	// 826E7B48: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E7B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7B50: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E7B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7B58: 386A3D3C  addi r3, r10, 0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15676;
	// 826E7B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E7B60: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E7B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7B68: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E7B6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E7B70: 4BD7F2B1  bl 0x82466e20
	ctx.lr = 0x826E7B74;
	sub_82466E20(ctx, base);
	// 826E7B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7B88 size=108
    let mut pc: u32 = 0x826E7B88;
    'dispatch: loop {
        match pc {
            0x826E7B88 => {
    //   block [0x826E7B88..0x826E7BF4)
	// 826E7B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7B94: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7B9C: 38EB1AA0  addi r7, r11, 0x1aa0
	ctx.r[7].s64 = ctx.r[11].s64 + 6816;
	// 826E7BA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E7BA4: 388AB870  addi r4, r10, -0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + -18320;
	// 826E7BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7BAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E7BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7BB8: 386A3D6C  addi r3, r10, 0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15724;
	// 826E7BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E7BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E7BE0: 4BD7F241  bl 0x82466e20
	ctx.lr = 0x826E7BE4;
	sub_82466E20(ctx, base);
	// 826E7BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7BF8 size=108
    let mut pc: u32 = 0x826E7BF8;
    'dispatch: loop {
        match pc {
            0x826E7BF8 => {
    //   block [0x826E7BF8..0x826E7C64)
	// 826E7BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7C04: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7C08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7C0C: 38EB1AE8  addi r7, r11, 0x1ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 6888;
	// 826E7C10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E7C14: 388AB898  addi r4, r10, -0x4768
	ctx.r[4].s64 = ctx.r[10].s64 + -18280;
	// 826E7C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7C1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7C20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E7C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7C28: 386A3D9C  addi r3, r10, 0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15772;
	// 826E7C2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E7C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7C4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E7C50: 4BD7F1D1  bl 0x82466e20
	ctx.lr = 0x826E7C54;
	sub_82466E20(ctx, base);
	// 826E7C54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7C68 size=112
    let mut pc: u32 = 0x826E7C68;
    'dispatch: loop {
        match pc {
            0x826E7C68 => {
    //   block [0x826E7C68..0x826E7CD8)
	// 826E7C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7C74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7C78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7C7C: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7C84: 390B1B48  addi r8, r11, 0x1b48
	ctx.r[8].s64 = ctx.r[11].s64 + 6984;
	// 826E7C88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E7C8C: 388AB8C0  addi r4, r10, -0x4740
	ctx.r[4].s64 = ctx.r[10].s64 + -18240;
	// 826E7C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7C94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7CA0: 386A3DCC  addi r3, r10, 0x3dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15820;
	// 826E7CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7CC4: 4BD7F15D  bl 0x82466e20
	ctx.lr = 0x826E7CC8;
	sub_82466E20(ctx, base);
	// 826E7CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7CD8 size=112
    let mut pc: u32 = 0x826E7CD8;
    'dispatch: loop {
        match pc {
            0x826E7CD8 => {
    //   block [0x826E7CD8..0x826E7D48)
	// 826E7CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7CE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7CE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7CEC: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7CF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7CF4: 390B1BA8  addi r8, r11, 0x1ba8
	ctx.r[8].s64 = ctx.r[11].s64 + 7080;
	// 826E7CF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E7CFC: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 826E7D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7D04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7D10: 386A3DFC  addi r3, r10, 0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15868;
	// 826E7D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7D34: 4BD7F0ED  bl 0x82466e20
	ctx.lr = 0x826E7D38;
	sub_82466E20(ctx, base);
	// 826E7D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E7D48 size=24
    let mut pc: u32 = 0x826E7D48;
    'dispatch: loop {
        match pc {
            0x826E7D48 => {
    //   block [0x826E7D48..0x826E7D60)
	// 826E7D48: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7D4C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E7D50: 394A7DB8  addi r10, r10, 0x7db8
	ctx.r[10].s64 = ctx.r[10].s64 + 32184;
	// 826E7D54: 816B1A0C  lwz r11, 0x1a0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6668 as u32) ) } as u64;
	// 826E7D58: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826E7D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7D60 size=116
    let mut pc: u32 = 0x826E7D60;
    'dispatch: loop {
        match pc {
            0x826E7D60 => {
    //   block [0x826E7D60..0x826E7DD4)
	// 826E7D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7D6C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E7D70: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E7D74: 390A7DB8  addi r8, r10, 0x7db8
	ctx.r[8].s64 = ctx.r[10].s64 + 32184;
	// 826E7D78: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7D7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826E7D80: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E7D84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7D88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E7D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7D94: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 826E7D98: 396B7FD0  addi r11, r11, 0x7fd0
	ctx.r[11].s64 = ctx.r[11].s64 + 32720;
	// 826E7D9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7DA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7DA4: 386A3E2C  addi r3, r10, 0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15916;
	// 826E7DA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E7DAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7DB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E7DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7DC0: 4BD7F061  bl 0x82466e20
	ctx.lr = 0x826E7DC4;
	sub_82466E20(ctx, base);
	// 826E7DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7DD8 size=100
    let mut pc: u32 = 0x826E7DD8;
    'dispatch: loop {
        match pc {
            0x826E7DD8 => {
    //   block [0x826E7DD8..0x826E7E3C)
	// 826E7DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7DE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7DEC: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E7DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7DF8: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 826E7DFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7E0C: 386A3E5C  addi r3, r10, 0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15964;
	// 826E7E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7E18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E7E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7E20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E7E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7E28: 4BD7EFF9  bl 0x82466e20
	ctx.lr = 0x826E7E2C;
	sub_82466E20(ctx, base);
	// 826E7E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7E40 size=100
    let mut pc: u32 = 0x826E7E40;
    'dispatch: loop {
        match pc {
            0x826E7E40 => {
    //   block [0x826E7E40..0x826E7EA4)
	// 826E7E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7E4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7E54: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E7E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7E60: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 826E7E64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7E74: 386A3E8C  addi r3, r10, 0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16012;
	// 826E7E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7E80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E7E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E7E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7E90: 4BD7EF91  bl 0x82466e20
	ctx.lr = 0x826E7E94;
	sub_82466E20(ctx, base);
	// 826E7E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7EA8 size=112
    let mut pc: u32 = 0x826E7EA8;
    'dispatch: loop {
        match pc {
            0x826E7EA8 => {
    //   block [0x826E7EA8..0x826E7F18)
	// 826E7EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7EB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7EB8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7EBC: 38AA3E5C  addi r5, r10, 0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + 15964;
	// 826E7EC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7EC4: 390B1BC0  addi r8, r11, 0x1bc0
	ctx.r[8].s64 = ctx.r[11].s64 + 7104;
	// 826E7EC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E7ECC: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 826E7ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7ED4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7EE0: 386A3EBC  addi r3, r10, 0x3ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 16060;
	// 826E7EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7F04: 4BD7EF1D  bl 0x82466e20
	ctx.lr = 0x826E7F08;
	sub_82466E20(ctx, base);
	// 826E7F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7F18 size=112
    let mut pc: u32 = 0x826E7F18;
    'dispatch: loop {
        match pc {
            0x826E7F18 => {
    //   block [0x826E7F18..0x826E7F88)
	// 826E7F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7F28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7F2C: 38AA3E5C  addi r5, r10, 0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + 15964;
	// 826E7F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7F34: 390B1C08  addi r8, r11, 0x1c08
	ctx.r[8].s64 = ctx.r[11].s64 + 7176;
	// 826E7F38: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826E7F3C: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 826E7F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7F50: 386A3EEC  addi r3, r10, 0x3eec
	ctx.r[3].s64 = ctx.r[10].s64 + 16108;
	// 826E7F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7F74: 4BD7EEAD  bl 0x82466e20
	ctx.lr = 0x826E7F78;
	sub_82466E20(ctx, base);
	// 826E7F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7F88 size=112
    let mut pc: u32 = 0x826E7F88;
    'dispatch: loop {
        match pc {
            0x826E7F88 => {
    //   block [0x826E7F88..0x826E7FF8)
	// 826E7F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E7F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E7F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7F98: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E7F9C: 38AA3EEC  addi r5, r10, 0x3eec
	ctx.r[5].s64 = ctx.r[10].s64 + 16108;
	// 826E7FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E7FA4: 390B1CC8  addi r8, r11, 0x1cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 7368;
	// 826E7FA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E7FAC: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 826E7FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E7FB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E7FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E7FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E7FC0: 386A3F1C  addi r3, r10, 0x3f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 16156;
	// 826E7FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E7FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E7FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E7FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E7FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E7FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E7FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E7FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E7FE4: 4BD7EE3D  bl 0x82466e20
	ctx.lr = 0x826E7FE8;
	sub_82466E20(ctx, base);
	// 826E7FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E7FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E7FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E7FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E7FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E7FF8 size=112
    let mut pc: u32 = 0x826E7FF8;
    'dispatch: loop {
        match pc {
            0x826E7FF8 => {
    //   block [0x826E7FF8..0x826E8068)
	// 826E7FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E7FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8008: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E800C: 38AA3AFC  addi r5, r10, 0x3afc
	ctx.r[5].s64 = ctx.r[10].s64 + 15100;
	// 826E8010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8014: 390B1CF8  addi r8, r11, 0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 7416;
	// 826E8018: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E801C: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 826E8020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E802C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8030: 386A3F4C  addi r3, r10, 0x3f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 16204;
	// 826E8034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E803C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E804C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8054: 4BD7EDCD  bl 0x82466e20
	ctx.lr = 0x826E8058;
	sub_82466E20(ctx, base);
	// 826E8058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E805C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8068 size=112
    let mut pc: u32 = 0x826E8068;
    'dispatch: loop {
        match pc {
            0x826E8068 => {
    //   block [0x826E8068..0x826E80D8)
	// 826E8068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E806C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8078: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E807C: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 826E8080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8084: 390B1D28  addi r8, r11, 0x1d28
	ctx.r[8].s64 = ctx.r[11].s64 + 7464;
	// 826E8088: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E808C: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 826E8090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E809C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E80A0: 386A3F7C  addi r3, r10, 0x3f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 16252;
	// 826E80A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E80A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E80AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E80B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E80B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E80B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E80BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E80C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E80C4: 4BD7ED5D  bl 0x82466e20
	ctx.lr = 0x826E80C8;
	sub_82466E20(ctx, base);
	// 826E80C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E80CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E80D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E80D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E80D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E80D8 size=112
    let mut pc: u32 = 0x826E80D8;
    'dispatch: loop {
        match pc {
            0x826E80D8 => {
    //   block [0x826E80D8..0x826E8148)
	// 826E80D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E80DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E80E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E80E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E80E8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E80EC: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E80F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E80F4: 390B1D58  addi r8, r11, 0x1d58
	ctx.r[8].s64 = ctx.r[11].s64 + 7512;
	// 826E80F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E80FC: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 826E8100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8104: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E810C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8110: 386A3FAC  addi r3, r10, 0x3fac
	ctx.r[3].s64 = ctx.r[10].s64 + 16300;
	// 826E8114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E811C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E812C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8134: 4BD7ECED  bl 0x82466e20
	ctx.lr = 0x826E8138;
	sub_82466E20(ctx, base);
	// 826E8138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E813C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8148 size=112
    let mut pc: u32 = 0x826E8148;
    'dispatch: loop {
        match pc {
            0x826E8148 => {
    //   block [0x826E8148..0x826E81B8)
	// 826E8148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E814C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8154: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8158: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E815C: 38AA3BEC  addi r5, r10, 0x3bec
	ctx.r[5].s64 = ctx.r[10].s64 + 15340;
	// 826E8160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8164: 390B1D88  addi r8, r11, 0x1d88
	ctx.r[8].s64 = ctx.r[11].s64 + 7560;
	// 826E8168: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E816C: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 826E8170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8174: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E817C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8180: 386A3FDC  addi r3, r10, 0x3fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 16348;
	// 826E8184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E818C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8194: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E8198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E819C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E81A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E81A4: 4BD7EC7D  bl 0x82466e20
	ctx.lr = 0x826E81A8;
	sub_82466E20(ctx, base);
	// 826E81A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E81AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E81B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E81B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E81B8 size=108
    let mut pc: u32 = 0x826E81B8;
    'dispatch: loop {
        match pc {
            0x826E81B8 => {
    //   block [0x826E81B8..0x826E8224)
	// 826E81B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E81BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E81C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E81C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E81C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E81CC: 38EB1E00  addi r7, r11, 0x1e00
	ctx.r[7].s64 = ctx.r[11].s64 + 7680;
	// 826E81D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E81D4: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 826E81D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E81DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E81E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E81E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E81E8: 386A400C  addi r3, r10, 0x400c
	ctx.r[3].s64 = ctx.r[10].s64 + 16396;
	// 826E81EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E81F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E81F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E81F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E81FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E820C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E8210: 4BD7EC11  bl 0x82466e20
	ctx.lr = 0x826E8214;
	sub_82466E20(ctx, base);
	// 826E8214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E821C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8228 size=112
    let mut pc: u32 = 0x826E8228;
    'dispatch: loop {
        match pc {
            0x826E8228 => {
    //   block [0x826E8228..0x826E8298)
	// 826E8228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8234: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8238: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E823C: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E8240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8244: 390B1E30  addi r8, r11, 0x1e30
	ctx.r[8].s64 = ctx.r[11].s64 + 7728;
	// 826E8248: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E824C: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 826E8250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E825C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8260: 386A403C  addi r3, r10, 0x403c
	ctx.r[3].s64 = ctx.r[10].s64 + 16444;
	// 826E8264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E826C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E827C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8284: 4BD7EB9D  bl 0x82466e20
	ctx.lr = 0x826E8288;
	sub_82466E20(ctx, base);
	// 826E8288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E828C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8298 size=112
    let mut pc: u32 = 0x826E8298;
    'dispatch: loop {
        match pc {
            0x826E8298 => {
    //   block [0x826E8298..0x826E8308)
	// 826E8298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E82A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E82A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E82A8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E82AC: 38AA3E8C  addi r5, r10, 0x3e8c
	ctx.r[5].s64 = ctx.r[10].s64 + 16012;
	// 826E82B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E82B4: 390B1E60  addi r8, r11, 0x1e60
	ctx.r[8].s64 = ctx.r[11].s64 + 7776;
	// 826E82B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E82BC: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 826E82C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E82C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E82C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E82CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E82D0: 386A406C  addi r3, r10, 0x406c
	ctx.r[3].s64 = ctx.r[10].s64 + 16492;
	// 826E82D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E82D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E82DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E82E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E82E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E82E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E82EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E82F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E82F4: 4BD7EB2D  bl 0x82466e20
	ctx.lr = 0x826E82F8;
	sub_82466E20(ctx, base);
	// 826E82F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E82FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8308 size=100
    let mut pc: u32 = 0x826E8308;
    'dispatch: loop {
        match pc {
            0x826E8308 => {
    //   block [0x826E8308..0x826E836C)
	// 826E8308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E830C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E831C: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E8320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8328: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 826E832C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E833C: 386A409C  addi r3, r10, 0x409c
	ctx.r[3].s64 = ctx.r[10].s64 + 16540;
	// 826E8340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8348: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E834C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8350: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E8354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8358: 4BD7EAC9  bl 0x82466e20
	ctx.lr = 0x826E835C;
	sub_82466E20(ctx, base);
	// 826E835C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8370 size=112
    let mut pc: u32 = 0x826E8370;
    'dispatch: loop {
        match pc {
            0x826E8370 => {
    //   block [0x826E8370..0x826E83E0)
	// 826E8370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E837C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8380: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8384: 38AA36DC  addi r5, r10, 0x36dc
	ctx.r[5].s64 = ctx.r[10].s64 + 14044;
	// 826E8388: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E838C: 390B1E90  addi r8, r11, 0x1e90
	ctx.r[8].s64 = ctx.r[11].s64 + 7824;
	// 826E8390: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E8394: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 826E8398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E839C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E83A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E83A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E83A8: 386A40CC  addi r3, r10, 0x40cc
	ctx.r[3].s64 = ctx.r[10].s64 + 16588;
	// 826E83AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E83B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E83B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E83B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E83BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E83C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E83C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E83C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E83CC: 4BD7EA55  bl 0x82466e20
	ctx.lr = 0x826E83D0;
	sub_82466E20(ctx, base);
	// 826E83D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E83D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E83D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E83DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E83E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E83E0 size=96
    let mut pc: u32 = 0x826E83E0;
    'dispatch: loop {
        match pc {
            0x826E83E0 => {
    //   block [0x826E83E0..0x826E8440)
	// 826E83E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E83E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E83E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E83EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E83F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E83F4: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 826E83F8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E83FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8400: 386A40FC  addi r3, r10, 0x40fc
	ctx.r[3].s64 = ctx.r[10].s64 + 16636;
	// 826E8404: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E840C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E8410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E841C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E8424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E8428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E842C: 4BD7E9F5  bl 0x82466e20
	ctx.lr = 0x826E8430;
	sub_82466E20(ctx, base);
	// 826E8430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E843C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8440 size=108
    let mut pc: u32 = 0x826E8440;
    'dispatch: loop {
        match pc {
            0x826E8440 => {
    //   block [0x826E8440..0x826E84AC)
	// 826E8440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E844C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8454: 38EB1ED8  addi r7, r11, 0x1ed8
	ctx.r[7].s64 = ctx.r[11].s64 + 7896;
	// 826E8458: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826E845C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 826E8460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8464: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E846C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8470: 386A412C  addi r3, r10, 0x412c
	ctx.r[3].s64 = ctx.r[10].s64 + 16684;
	// 826E8474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E8478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E847C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E848C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E8498: 4BD7E989  bl 0x82466e20
	ctx.lr = 0x826E849C;
	sub_82466E20(ctx, base);
	// 826E849C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E84A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E84A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E84A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E84B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E84B0 size=100
    let mut pc: u32 = 0x826E84B0;
    'dispatch: loop {
        match pc {
            0x826E84B0 => {
    //   block [0x826E84B0..0x826E8514)
	// 826E84B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E84B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E84B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E84BC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E84C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E84C4: 392A8040  addi r9, r10, -0x7fc0
	ctx.r[9].s64 = ctx.r[10].s64 + -32704;
	// 826E84C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E84CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E84D0: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 826E84D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E84D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E84DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E84E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E84E4: 386A415C  addi r3, r10, 0x415c
	ctx.r[3].s64 = ctx.r[10].s64 + 16732;
	// 826E84E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E84EC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826E84F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E84F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E84F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E84FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E8500: 4BD7E921  bl 0x82466e20
	ctx.lr = 0x826E8504;
	sub_82466E20(ctx, base);
	// 826E8504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E850C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E8518 size=24
    let mut pc: u32 = 0x826E8518;
    'dispatch: loop {
        match pc {
            0x826E8518 => {
    //   block [0x826E8518..0x826E8530)
	// 826E8518: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E851C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E8520: 394A7E60  addi r10, r10, 0x7e60
	ctx.r[10].s64 = ctx.r[10].s64 + 32352;
	// 826E8524: 816B1F40  lwz r11, 0x1f40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8000 as u32) ) } as u64;
	// 826E8528: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8530 size=112
    let mut pc: u32 = 0x826E8530;
    'dispatch: loop {
        match pc {
            0x826E8530 => {
    //   block [0x826E8530..0x826E85A0)
	// 826E8530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E853C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E8540: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8544: 392A8180  addi r9, r10, -0x7e80
	ctx.r[9].s64 = ctx.r[10].s64 + -32384;
	// 826E8548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E854C: 390B7E60  addi r8, r11, 0x7e60
	ctx.r[8].s64 = ctx.r[11].s64 + 32352;
	// 826E8550: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E8554: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 826E8558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E855C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8568: 386A418C  addi r3, r10, 0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + 16780;
	// 826E856C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E8570: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826E8574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E857C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E8588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E858C: 4BD7E895  bl 0x82466e20
	ctx.lr = 0x826E8590;
	sub_82466E20(ctx, base);
	// 826E8590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E859C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E85A0 size=112
    let mut pc: u32 = 0x826E85A0;
    'dispatch: loop {
        match pc {
            0x826E85A0 => {
    //   block [0x826E85A0..0x826E8610)
	// 826E85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E85A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E85A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E85AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E85B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E85B4: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E85B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E85BC: 390B1F48  addi r8, r11, 0x1f48
	ctx.r[8].s64 = ctx.r[11].s64 + 8008;
	// 826E85C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E85C4: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 826E85C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E85CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E85D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E85D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E85D8: 386A41BC  addi r3, r10, 0x41bc
	ctx.r[3].s64 = ctx.r[10].s64 + 16828;
	// 826E85DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E85E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E85E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E85E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E85EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E85F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E85F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E85F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E85FC: 4BD7E825  bl 0x82466e20
	ctx.lr = 0x826E8600;
	sub_82466E20(ctx, base);
	// 826E8600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8610 size=108
    let mut pc: u32 = 0x826E8610;
    'dispatch: loop {
        match pc {
            0x826E8610 => {
    //   block [0x826E8610..0x826E867C)
	// 826E8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E861C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8624: 38EB1F78  addi r7, r11, 0x1f78
	ctx.r[7].s64 = ctx.r[11].s64 + 8056;
	// 826E8628: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E862C: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 826E8630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8634: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E863C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8640: 386A41EC  addi r3, r10, 0x41ec
	ctx.r[3].s64 = ctx.r[10].s64 + 16876;
	// 826E8644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E8648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E864C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E865C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E8668: 4BD7E7B9  bl 0x82466e20
	ctx.lr = 0x826E866C;
	sub_82466E20(ctx, base);
	// 826E866C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8680 size=112
    let mut pc: u32 = 0x826E8680;
    'dispatch: loop {
        match pc {
            0x826E8680 => {
    //   block [0x826E8680..0x826E86F0)
	// 826E8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E868C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8690: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8694: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E869C: 390B1F90  addi r8, r11, 0x1f90
	ctx.r[8].s64 = ctx.r[11].s64 + 8080;
	// 826E86A0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826E86A4: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 826E86A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E86AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E86B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E86B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E86B8: 386A421C  addi r3, r10, 0x421c
	ctx.r[3].s64 = ctx.r[10].s64 + 16924;
	// 826E86BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E86C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E86C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E86C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E86CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E86D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E86D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E86D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E86DC: 4BD7E745  bl 0x82466e20
	ctx.lr = 0x826E86E0;
	sub_82466E20(ctx, base);
	// 826E86E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E86E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E86E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E86EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E86F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E86F0 size=100
    let mut pc: u32 = 0x826E86F0;
    'dispatch: loop {
        match pc {
            0x826E86F0 => {
    //   block [0x826E86F0..0x826E8754)
	// 826E86F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E86F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E86F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E86FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8704: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E870C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8710: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 826E8714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E871C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8724: 386A424C  addi r3, r10, 0x424c
	ctx.r[3].s64 = ctx.r[10].s64 + 16972;
	// 826E8728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E872C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8730: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E8734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E873C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8740: 4BD7E6E1  bl 0x82466e20
	ctx.lr = 0x826E8744;
	sub_82466E20(ctx, base);
	// 826E8744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E874C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8758 size=112
    let mut pc: u32 = 0x826E8758;
    'dispatch: loop {
        match pc {
            0x826E8758 => {
    //   block [0x826E8758..0x826E87C8)
	// 826E8758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8768: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E876C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8774: 390B2050  addi r8, r11, 0x2050
	ctx.r[8].s64 = ctx.r[11].s64 + 8272;
	// 826E8778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E877C: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 826E8780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E878C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8790: 386A427C  addi r3, r10, 0x427c
	ctx.r[3].s64 = ctx.r[10].s64 + 17020;
	// 826E8794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E879C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E87A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E87A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E87A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E87AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E87B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E87B4: 4BD7E66D  bl 0x82466e20
	ctx.lr = 0x826E87B8;
	sub_82466E20(ctx, base);
	// 826E87B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E87BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E87C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E87C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E87C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E87C8 size=112
    let mut pc: u32 = 0x826E87C8;
    'dispatch: loop {
        match pc {
            0x826E87C8 => {
    //   block [0x826E87C8..0x826E8838)
	// 826E87C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E87CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E87D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E87D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E87D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E87DC: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E87E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E87E4: 390B2068  addi r8, r11, 0x2068
	ctx.r[8].s64 = ctx.r[11].s64 + 8296;
	// 826E87E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E87EC: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 826E87F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E87F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E87F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E87FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8800: 386A42AC  addi r3, r10, 0x42ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17068;
	// 826E8804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E880C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E881C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8824: 4BD7E5FD  bl 0x82466e20
	ctx.lr = 0x826E8828;
	sub_82466E20(ctx, base);
	// 826E8828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E882C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8838 size=112
    let mut pc: u32 = 0x826E8838;
    'dispatch: loop {
        match pc {
            0x826E8838 => {
    //   block [0x826E8838..0x826E88A8)
	// 826E8838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E883C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8844: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8848: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E884C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8854: 390B2098  addi r8, r11, 0x2098
	ctx.r[8].s64 = ctx.r[11].s64 + 8344;
	// 826E8858: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E885C: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826E8860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E886C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8870: 386A42DC  addi r3, r10, 0x42dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17116;
	// 826E8874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E887C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E888C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8894: 4BD7E58D  bl 0x82466e20
	ctx.lr = 0x826E8898;
	sub_82466E20(ctx, base);
	// 826E8898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E889C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E88A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E88A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E88A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E88A8 size=112
    let mut pc: u32 = 0x826E88A8;
    'dispatch: loop {
        match pc {
            0x826E88A8 => {
    //   block [0x826E88A8..0x826E8918)
	// 826E88A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E88AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E88B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E88B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E88B8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E88BC: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E88C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E88C4: 390B20C8  addi r8, r11, 0x20c8
	ctx.r[8].s64 = ctx.r[11].s64 + 8392;
	// 826E88C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E88CC: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 826E88D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E88D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E88D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E88DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E88E0: 386A430C  addi r3, r10, 0x430c
	ctx.r[3].s64 = ctx.r[10].s64 + 17164;
	// 826E88E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E88E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E88EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E88F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E88F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E88F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E88FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8904: 4BD7E51D  bl 0x82466e20
	ctx.lr = 0x826E8908;
	sub_82466E20(ctx, base);
	// 826E8908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E890C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8918 size=112
    let mut pc: u32 = 0x826E8918;
    'dispatch: loop {
        match pc {
            0x826E8918 => {
    //   block [0x826E8918..0x826E8988)
	// 826E8918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E891C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8924: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8928: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E892C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8930: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8934: 390B20F8  addi r8, r11, 0x20f8
	ctx.r[8].s64 = ctx.r[11].s64 + 8440;
	// 826E8938: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E893C: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826E8940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8944: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E894C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8950: 386A433C  addi r3, r10, 0x433c
	ctx.r[3].s64 = ctx.r[10].s64 + 17212;
	// 826E8954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E895C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E896C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8974: 4BD7E4AD  bl 0x82466e20
	ctx.lr = 0x826E8978;
	sub_82466E20(ctx, base);
	// 826E8978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E897C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8988 size=112
    let mut pc: u32 = 0x826E8988;
    'dispatch: loop {
        match pc {
            0x826E8988 => {
    //   block [0x826E8988..0x826E89F8)
	// 826E8988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E898C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8998: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E899C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E89A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E89A4: 390B2110  addi r8, r11, 0x2110
	ctx.r[8].s64 = ctx.r[11].s64 + 8464;
	// 826E89A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E89AC: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 826E89B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E89B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E89B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E89BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E89C0: 386A436C  addi r3, r10, 0x436c
	ctx.r[3].s64 = ctx.r[10].s64 + 17260;
	// 826E89C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E89C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E89CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E89D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E89D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E89D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E89DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E89E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E89E4: 4BD7E43D  bl 0x82466e20
	ctx.lr = 0x826E89E8;
	sub_82466E20(ctx, base);
	// 826E89E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E89EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E89F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E89F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E89F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E89F8 size=112
    let mut pc: u32 = 0x826E89F8;
    'dispatch: loop {
        match pc {
            0x826E89F8 => {
    //   block [0x826E89F8..0x826E8A68)
	// 826E89F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E89FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8A04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8A08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8A0C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8A10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8A14: 390B2128  addi r8, r11, 0x2128
	ctx.r[8].s64 = ctx.r[11].s64 + 8488;
	// 826E8A18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E8A1C: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 826E8A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8A30: 386A439C  addi r3, r10, 0x439c
	ctx.r[3].s64 = ctx.r[10].s64 + 17308;
	// 826E8A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8A54: 4BD7E3CD  bl 0x82466e20
	ctx.lr = 0x826E8A58;
	sub_82466E20(ctx, base);
	// 826E8A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8A68 size=112
    let mut pc: u32 = 0x826E8A68;
    'dispatch: loop {
        match pc {
            0x826E8A68 => {
    //   block [0x826E8A68..0x826E8AD8)
	// 826E8A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8A78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8A7C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8A80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8A84: 390B2170  addi r8, r11, 0x2170
	ctx.r[8].s64 = ctx.r[11].s64 + 8560;
	// 826E8A88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E8A8C: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 826E8A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8A94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8AA0: 386A43CC  addi r3, r10, 0x43cc
	ctx.r[3].s64 = ctx.r[10].s64 + 17356;
	// 826E8AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8AC4: 4BD7E35D  bl 0x82466e20
	ctx.lr = 0x826E8AC8;
	sub_82466E20(ctx, base);
	// 826E8AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8AD8 size=112
    let mut pc: u32 = 0x826E8AD8;
    'dispatch: loop {
        match pc {
            0x826E8AD8 => {
    //   block [0x826E8AD8..0x826E8B48)
	// 826E8AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8AE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8AE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8AEC: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8AF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8AF4: 390B21B8  addi r8, r11, 0x21b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8632;
	// 826E8AF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E8AFC: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 826E8B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8B04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8B10: 386A43FC  addi r3, r10, 0x43fc
	ctx.r[3].s64 = ctx.r[10].s64 + 17404;
	// 826E8B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8B34: 4BD7E2ED  bl 0x82466e20
	ctx.lr = 0x826E8B38;
	sub_82466E20(ctx, base);
	// 826E8B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8B48 size=112
    let mut pc: u32 = 0x826E8B48;
    'dispatch: loop {
        match pc {
            0x826E8B48 => {
    //   block [0x826E8B48..0x826E8BB8)
	// 826E8B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8B54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8B58: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8B5C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8B60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8B64: 390B21D0  addi r8, r11, 0x21d0
	ctx.r[8].s64 = ctx.r[11].s64 + 8656;
	// 826E8B68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E8B6C: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826E8B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8B74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8B80: 386A442C  addi r3, r10, 0x442c
	ctx.r[3].s64 = ctx.r[10].s64 + 17452;
	// 826E8B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8BA4: 4BD7E27D  bl 0x82466e20
	ctx.lr = 0x826E8BA8;
	sub_82466E20(ctx, base);
	// 826E8BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8BB8 size=116
    let mut pc: u32 = 0x826E8BB8;
    'dispatch: loop {
        match pc {
            0x826E8BB8 => {
    //   block [0x826E8BB8..0x826E8C2C)
	// 826E8BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8BC4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E8BC8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E8BCC: 390A2200  addi r8, r10, 0x2200
	ctx.r[8].s64 = ctx.r[10].s64 + 8704;
	// 826E8BD0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8BD4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826E8BD8: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8BDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8BE0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E8BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8BEC: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 826E8BF0: 396B81A8  addi r11, r11, -0x7e58
	ctx.r[11].s64 = ctx.r[11].s64 + -32344;
	// 826E8BF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8BF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8BFC: 386A445C  addi r3, r10, 0x445c
	ctx.r[3].s64 = ctx.r[10].s64 + 17500;
	// 826E8C00: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E8C04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8C08: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E8C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8C18: 4BD7E209  bl 0x82466e20
	ctx.lr = 0x826E8C1C;
	sub_82466E20(ctx, base);
	// 826E8C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8C30 size=116
    let mut pc: u32 = 0x826E8C30;
    'dispatch: loop {
        match pc {
            0x826E8C30 => {
    //   block [0x826E8C30..0x826E8CA4)
	// 826E8C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8C3C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E8C40: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826E8C44: 390A2278  addi r8, r10, 0x2278
	ctx.r[8].s64 = ctx.r[10].s64 + 8824;
	// 826E8C48: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8C4C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826E8C50: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8C54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8C58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E8C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8C60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8C64: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826E8C68: 396B81C0  addi r11, r11, -0x7e40
	ctx.r[11].s64 = ctx.r[11].s64 + -32320;
	// 826E8C6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8C70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8C74: 386A448C  addi r3, r10, 0x448c
	ctx.r[3].s64 = ctx.r[10].s64 + 17548;
	// 826E8C78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826E8C7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8C80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826E8C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8C90: 4BD7E191  bl 0x82466e20
	ctx.lr = 0x826E8C94;
	sub_82466E20(ctx, base);
	// 826E8C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E8CA8 size=24
    let mut pc: u32 = 0x826E8CA8;
    'dispatch: loop {
        match pc {
            0x826E8CA8 => {
    //   block [0x826E8CA8..0x826E8CC0)
	// 826E8CA8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8CAC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E8CB0: 394A7E78  addi r10, r10, 0x7e78
	ctx.r[10].s64 = ctx.r[10].s64 + 32376;
	// 826E8CB4: 816B2308  lwz r11, 0x2308(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8968 as u32) ) } as u64;
	// 826E8CB8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E8CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8CC0 size=116
    let mut pc: u32 = 0x826E8CC0;
    'dispatch: loop {
        match pc {
            0x826E8CC0 => {
    //   block [0x826E8CC0..0x826E8D34)
	// 826E8CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8CCC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826E8CD0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8CD4: 392B81EC  addi r9, r11, -0x7e14
	ctx.r[9].s64 = ctx.r[11].s64 + -32276;
	// 826E8CD8: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8CDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8CE0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826E8CE4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826E8CE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8CEC: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 826E8CF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8CF4: 396B7E78  addi r11, r11, 0x7e78
	ctx.r[11].s64 = ctx.r[11].s64 + 32376;
	// 826E8CF8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826E8CFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8D00: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826E8D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8D08: 386A44BC  addi r3, r10, 0x44bc
	ctx.r[3].s64 = ctx.r[10].s64 + 17596;
	// 826E8D0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E8D10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826E8D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8D18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826E8D1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E8D20: 4BD7E101  bl 0x82466e20
	ctx.lr = 0x826E8D24;
	sub_82466E20(ctx, base);
	// 826E8D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8D38 size=112
    let mut pc: u32 = 0x826E8D38;
    'dispatch: loop {
        match pc {
            0x826E8D38 => {
    //   block [0x826E8D38..0x826E8DA8)
	// 826E8D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8D44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8D48: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8D4C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8D54: 390B2310  addi r8, r11, 0x2310
	ctx.r[8].s64 = ctx.r[11].s64 + 8976;
	// 826E8D58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E8D5C: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826E8D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8D70: 386A44EC  addi r3, r10, 0x44ec
	ctx.r[3].s64 = ctx.r[10].s64 + 17644;
	// 826E8D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8D94: 4BD7E08D  bl 0x82466e20
	ctx.lr = 0x826E8D98;
	sub_82466E20(ctx, base);
	// 826E8D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8DA8 size=112
    let mut pc: u32 = 0x826E8DA8;
    'dispatch: loop {
        match pc {
            0x826E8DA8 => {
    //   block [0x826E8DA8..0x826E8E18)
	// 826E8DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8DB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8DB8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8DBC: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8DC4: 390B2370  addi r8, r11, 0x2370
	ctx.r[8].s64 = ctx.r[11].s64 + 9072;
	// 826E8DC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826E8DCC: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 826E8DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8DD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8DE0: 386A451C  addi r3, r10, 0x451c
	ctx.r[3].s64 = ctx.r[10].s64 + 17692;
	// 826E8DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8E04: 4BD7E01D  bl 0x82466e20
	ctx.lr = 0x826E8E08;
	sub_82466E20(ctx, base);
	// 826E8E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8E18 size=112
    let mut pc: u32 = 0x826E8E18;
    'dispatch: loop {
        match pc {
            0x826E8E18 => {
    //   block [0x826E8E18..0x826E8E88)
	// 826E8E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8E24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8E28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8E2C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8E34: 390B2418  addi r8, r11, 0x2418
	ctx.r[8].s64 = ctx.r[11].s64 + 9240;
	// 826E8E38: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E8E3C: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826E8E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8E50: 386A454C  addi r3, r10, 0x454c
	ctx.r[3].s64 = ctx.r[10].s64 + 17740;
	// 826E8E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8E74: 4BD7DFAD  bl 0x82466e20
	ctx.lr = 0x826E8E78;
	sub_82466E20(ctx, base);
	// 826E8E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8E88 size=112
    let mut pc: u32 = 0x826E8E88;
    'dispatch: loop {
        match pc {
            0x826E8E88 => {
    //   block [0x826E8E88..0x826E8EF8)
	// 826E8E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8E94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8E98: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8E9C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8EA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8EA4: 390B2490  addi r8, r11, 0x2490
	ctx.r[8].s64 = ctx.r[11].s64 + 9360;
	// 826E8EA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E8EAC: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 826E8EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8EB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8EC0: 386A457C  addi r3, r10, 0x457c
	ctx.r[3].s64 = ctx.r[10].s64 + 17788;
	// 826E8EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8EE4: 4BD7DF3D  bl 0x82466e20
	ctx.lr = 0x826E8EE8;
	sub_82466E20(ctx, base);
	// 826E8EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8EF8 size=112
    let mut pc: u32 = 0x826E8EF8;
    'dispatch: loop {
        match pc {
            0x826E8EF8 => {
    //   block [0x826E8EF8..0x826E8F68)
	// 826E8EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8F04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8F08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8F0C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8F10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8F14: 390B24D8  addi r8, r11, 0x24d8
	ctx.r[8].s64 = ctx.r[11].s64 + 9432;
	// 826E8F18: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826E8F1C: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 826E8F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8F30: 386A45AC  addi r3, r10, 0x45ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17836;
	// 826E8F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8F54: 4BD7DECD  bl 0x82466e20
	ctx.lr = 0x826E8F58;
	sub_82466E20(ctx, base);
	// 826E8F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8F68 size=112
    let mut pc: u32 = 0x826E8F68;
    'dispatch: loop {
        match pc {
            0x826E8F68 => {
    //   block [0x826E8F68..0x826E8FD8)
	// 826E8F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8F74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8F78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8F7C: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8F80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8F84: 390B2568  addi r8, r11, 0x2568
	ctx.r[8].s64 = ctx.r[11].s64 + 9576;
	// 826E8F88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E8F8C: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 826E8F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E8F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E8F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E8FA0: 386A45DC  addi r3, r10, 0x45dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17884;
	// 826E8FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E8FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E8FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E8FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E8FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E8FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E8FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E8FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E8FC4: 4BD7DE5D  bl 0x82466e20
	ctx.lr = 0x826E8FC8;
	sub_82466E20(ctx, base);
	// 826E8FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E8FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E8FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E8FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E8FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E8FD8 size=112
    let mut pc: u32 = 0x826E8FD8;
    'dispatch: loop {
        match pc {
            0x826E8FD8 => {
    //   block [0x826E8FD8..0x826E9048)
	// 826E8FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E8FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E8FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E8FE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E8FE8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E8FEC: 38AA418C  addi r5, r10, 0x418c
	ctx.r[5].s64 = ctx.r[10].s64 + 16780;
	// 826E8FF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E8FF4: 390B25C8  addi r8, r11, 0x25c8
	ctx.r[8].s64 = ctx.r[11].s64 + 9672;
	// 826E8FF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E8FFC: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 826E9000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E900C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9010: 386A460C  addi r3, r10, 0x460c
	ctx.r[3].s64 = ctx.r[10].s64 + 17932;
	// 826E9014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E901C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E902C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9034: 4BD7DDED  bl 0x82466e20
	ctx.lr = 0x826E9038;
	sub_82466E20(ctx, base);
	// 826E9038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E903C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9048 size=112
    let mut pc: u32 = 0x826E9048;
    'dispatch: loop {
        match pc {
            0x826E9048 => {
    //   block [0x826E9048..0x826E90B8)
	// 826E9048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E904C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9054: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9058: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E905C: 38AA460C  addi r5, r10, 0x460c
	ctx.r[5].s64 = ctx.r[10].s64 + 17932;
	// 826E9060: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9064: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 826E9068: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E906C: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826E9070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E907C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9080: 386A463C  addi r3, r10, 0x463c
	ctx.r[3].s64 = ctx.r[10].s64 + 17980;
	// 826E9084: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E908C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E909C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E90A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E90A4: 4BD7DD7D  bl 0x82466e20
	ctx.lr = 0x826E90A8;
	sub_82466E20(ctx, base);
	// 826E90A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E90AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E90B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E90B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E90B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E90B8 size=112
    let mut pc: u32 = 0x826E90B8;
    'dispatch: loop {
        match pc {
            0x826E90B8 => {
    //   block [0x826E90B8..0x826E9128)
	// 826E90B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E90BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E90C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E90C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E90C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E90CC: 38AA460C  addi r5, r10, 0x460c
	ctx.r[5].s64 = ctx.r[10].s64 + 17932;
	// 826E90D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E90D4: 390B2658  addi r8, r11, 0x2658
	ctx.r[8].s64 = ctx.r[11].s64 + 9816;
	// 826E90D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E90DC: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 826E90E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E90E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E90E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E90EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E90F0: 386A466C  addi r3, r10, 0x466c
	ctx.r[3].s64 = ctx.r[10].s64 + 18028;
	// 826E90F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E90F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E90FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E910C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9114: 4BD7DD0D  bl 0x82466e20
	ctx.lr = 0x826E9118;
	sub_82466E20(ctx, base);
	// 826E9118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E911C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9128 size=100
    let mut pc: u32 = 0x826E9128;
    'dispatch: loop {
        match pc {
            0x826E9128 => {
    //   block [0x826E9128..0x826E918C)
	// 826E9128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9134: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E913C: 38AA460C  addi r5, r10, 0x460c
	ctx.r[5].s64 = ctx.r[10].s64 + 17932;
	// 826E9140: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9148: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826E914C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E915C: 386A469C  addi r3, r10, 0x469c
	ctx.r[3].s64 = ctx.r[10].s64 + 18076;
	// 826E9160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9164: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9168: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E916C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9170: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E9174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9178: 4BD7DCA9  bl 0x82466e20
	ctx.lr = 0x826E917C;
	sub_82466E20(ctx, base);
	// 826E917C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9190 size=112
    let mut pc: u32 = 0x826E9190;
    'dispatch: loop {
        match pc {
            0x826E9190 => {
    //   block [0x826E9190..0x826E9200)
	// 826E9190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E919C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E91A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E91A4: 38AA460C  addi r5, r10, 0x460c
	ctx.r[5].s64 = ctx.r[10].s64 + 17932;
	// 826E91A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E91AC: 390B26A0  addi r8, r11, 0x26a0
	ctx.r[8].s64 = ctx.r[11].s64 + 9888;
	// 826E91B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E91B4: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 826E91B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E91BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E91C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E91C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E91C8: 386A46CC  addi r3, r10, 0x46cc
	ctx.r[3].s64 = ctx.r[10].s64 + 18124;
	// 826E91CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E91D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E91D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E91D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E91DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E91E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E91E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E91E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E91EC: 4BD7DC35  bl 0x82466e20
	ctx.lr = 0x826E91F0;
	sub_82466E20(ctx, base);
	// 826E91F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E91F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E91F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E91FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9200 size=108
    let mut pc: u32 = 0x826E9200;
    'dispatch: loop {
        match pc {
            0x826E9200 => {
    //   block [0x826E9200..0x826E926C)
	// 826E9200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E920C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9210: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9214: 38EB26B8  addi r7, r11, 0x26b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9912;
	// 826E9218: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E921C: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 826E9220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9228: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E922C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9230: 386A46FC  addi r3, r10, 0x46fc
	ctx.r[3].s64 = ctx.r[10].s64 + 18172;
	// 826E9234: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E9238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E923C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E924C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9258: 4BD7DBC9  bl 0x82466e20
	ctx.lr = 0x826E925C;
	sub_82466E20(ctx, base);
	// 826E925C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9270 size=112
    let mut pc: u32 = 0x826E9270;
    'dispatch: loop {
        match pc {
            0x826E9270 => {
    //   block [0x826E9270..0x826E92E0)
	// 826E9270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E927C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9280: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9284: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E9288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E928C: 390B2700  addi r8, r11, 0x2700
	ctx.r[8].s64 = ctx.r[11].s64 + 9984;
	// 826E9290: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826E9294: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 826E9298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E929C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E92A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E92A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E92A8: 386A472C  addi r3, r10, 0x472c
	ctx.r[3].s64 = ctx.r[10].s64 + 18220;
	// 826E92AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E92B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E92B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E92B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E92BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E92C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E92C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E92C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E92CC: 4BD7DB55  bl 0x82466e20
	ctx.lr = 0x826E92D0;
	sub_82466E20(ctx, base);
	// 826E92D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E92D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E92D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E92DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E92E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E92E0 size=112
    let mut pc: u32 = 0x826E92E0;
    'dispatch: loop {
        match pc {
            0x826E92E0 => {
    //   block [0x826E92E0..0x826E9350)
	// 826E92E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E92E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E92E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E92EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E92F0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E92F4: 38AA472C  addi r5, r10, 0x472c
	ctx.r[5].s64 = ctx.r[10].s64 + 18220;
	// 826E92F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E92FC: 390B2760  addi r8, r11, 0x2760
	ctx.r[8].s64 = ctx.r[11].s64 + 10080;
	// 826E9300: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9304: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 826E9308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E930C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9318: 386A475C  addi r3, r10, 0x475c
	ctx.r[3].s64 = ctx.r[10].s64 + 18268;
	// 826E931C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E932C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E933C: 4BD7DAE5  bl 0x82466e20
	ctx.lr = 0x826E9340;
	sub_82466E20(ctx, base);
	// 826E9340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E934C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9350 size=112
    let mut pc: u32 = 0x826E9350;
    'dispatch: loop {
        match pc {
            0x826E9350 => {
    //   block [0x826E9350..0x826E93C0)
	// 826E9350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E935C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9360: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9364: 38AA472C  addi r5, r10, 0x472c
	ctx.r[5].s64 = ctx.r[10].s64 + 18220;
	// 826E9368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E936C: 390B2778  addi r8, r11, 0x2778
	ctx.r[8].s64 = ctx.r[11].s64 + 10104;
	// 826E9370: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826E9374: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 826E9378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E937C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9388: 386A478C  addi r3, r10, 0x478c
	ctx.r[3].s64 = ctx.r[10].s64 + 18316;
	// 826E938C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E939C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E93A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E93A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E93A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E93AC: 4BD7DA75  bl 0x82466e20
	ctx.lr = 0x826E93B0;
	sub_82466E20(ctx, base);
	// 826E93B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E93B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E93B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E93BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E93C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E93C0 size=112
    let mut pc: u32 = 0x826E93C0;
    'dispatch: loop {
        match pc {
            0x826E93C0 => {
    //   block [0x826E93C0..0x826E9430)
	// 826E93C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E93C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E93C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E93CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E93D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E93D4: 38AA472C  addi r5, r10, 0x472c
	ctx.r[5].s64 = ctx.r[10].s64 + 18220;
	// 826E93D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E93DC: 390B27A8  addi r8, r11, 0x27a8
	ctx.r[8].s64 = ctx.r[11].s64 + 10152;
	// 826E93E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E93E4: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 826E93E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E93EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E93F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E93F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E93F8: 386A47BC  addi r3, r10, 0x47bc
	ctx.r[3].s64 = ctx.r[10].s64 + 18364;
	// 826E93FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E940C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E941C: 4BD7DA05  bl 0x82466e20
	ctx.lr = 0x826E9420;
	sub_82466E20(ctx, base);
	// 826E9420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E942C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E9430 size=24
    let mut pc: u32 = 0x826E9430;
    'dispatch: loop {
        match pc {
            0x826E9430 => {
    //   block [0x826E9430..0x826E9448)
	// 826E9430: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9434: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E9438: 394A7F38  addi r10, r10, 0x7f38
	ctx.r[10].s64 = ctx.r[10].s64 + 32568;
	// 826E943C: 816B230C  lwz r11, 0x230c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8972 as u32) ) } as u64;
	// 826E9440: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826E9444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9448 size=112
    let mut pc: u32 = 0x826E9448;
    'dispatch: loop {
        match pc {
            0x826E9448 => {
    //   block [0x826E9448..0x826E94B8)
	// 826E9448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9454: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9458: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E945C: 392A8250  addi r9, r10, -0x7db0
	ctx.r[9].s64 = ctx.r[10].s64 + -32176;
	// 826E9460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9464: 390B7F38  addi r8, r11, 0x7f38
	ctx.r[8].s64 = ctx.r[11].s64 + 32568;
	// 826E9468: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E946C: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 826E9470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E947C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9480: 386A47EC  addi r3, r10, 0x47ec
	ctx.r[3].s64 = ctx.r[10].s64 + 18412;
	// 826E9484: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9488: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E948C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E949C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E94A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E94A4: 4BD7D97D  bl 0x82466e20
	ctx.lr = 0x826E94A8;
	sub_82466E20(ctx, base);
	// 826E94A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E94AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E94B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E94B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E94B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E94B8 size=108
    let mut pc: u32 = 0x826E94B8;
    'dispatch: loop {
        match pc {
            0x826E94B8 => {
    //   block [0x826E94B8..0x826E9524)
	// 826E94B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E94BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E94C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E94C4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E94C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E94CC: 38EB27C0  addi r7, r11, 0x27c0
	ctx.r[7].s64 = ctx.r[11].s64 + 10176;
	// 826E94D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826E94D4: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 826E94D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E94DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E94E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E94E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E94E8: 386A481C  addi r3, r10, 0x481c
	ctx.r[3].s64 = ctx.r[10].s64 + 18460;
	// 826E94EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E94F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E94F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E94F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E94FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E950C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9510: 4BD7D911  bl 0x82466e20
	ctx.lr = 0x826E9514;
	sub_82466E20(ctx, base);
	// 826E9514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E951C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9528 size=108
    let mut pc: u32 = 0x826E9528;
    'dispatch: loop {
        match pc {
            0x826E9528 => {
    //   block [0x826E9528..0x826E9594)
	// 826E9528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9534: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E953C: 38EB27D8  addi r7, r11, 0x27d8
	ctx.r[7].s64 = ctx.r[11].s64 + 10200;
	// 826E9540: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826E9544: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 826E9548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E954C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E9554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9558: 386A484C  addi r3, r10, 0x484c
	ctx.r[3].s64 = ctx.r[10].s64 + 18508;
	// 826E955C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E9560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E956C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E957C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9580: 4BD7D8A1  bl 0x82466e20
	ctx.lr = 0x826E9584;
	sub_82466E20(ctx, base);
	// 826E9584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E958C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9598 size=116
    let mut pc: u32 = 0x826E9598;
    'dispatch: loop {
        match pc {
            0x826E9598 => {
    //   block [0x826E9598..0x826E960C)
	// 826E9598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E959C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E95A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E95A4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E95A8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E95AC: 390B2824  addi r8, r11, 0x2824
	ctx.r[8].s64 = ctx.r[11].s64 + 10276;
	// 826E95B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E95B4: 392A8310  addi r9, r10, -0x7cf0
	ctx.r[9].s64 = ctx.r[10].s64 + -31984;
	// 826E95B8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E95BC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E95C0: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E95C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E95C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E95CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E95D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E95D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E95D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E95DC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E95E0: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 826E95E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E95E8: 386B487C  addi r3, r11, 0x487c
	ctx.r[3].s64 = ctx.r[11].s64 + 18556;
	// 826E95EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E95F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E95F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E95F8: 4BD7D829  bl 0x82466e20
	ctx.lr = 0x826E95FC;
	sub_82466E20(ctx, base);
	// 826E95FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E9610 size=24
    let mut pc: u32 = 0x826E9610;
    'dispatch: loop {
        match pc {
            0x826E9610 => {
    //   block [0x826E9610..0x826E9628)
	// 826E9610: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9614: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826E9618: 394A7F80  addi r10, r10, 0x7f80
	ctx.r[10].s64 = ctx.r[10].s64 + 32640;
	// 826E961C: 816B283C  lwz r11, 0x283c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10300 as u32) ) } as u64;
	// 826E9620: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826E9624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9628 size=116
    let mut pc: u32 = 0x826E9628;
    'dispatch: loop {
        match pc {
            0x826E9628 => {
    //   block [0x826E9628..0x826E969C)
	// 826E9628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9634: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9638: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E963C: 390B7F80  addi r8, r11, 0x7f80
	ctx.r[8].s64 = ctx.r[11].s64 + 32640;
	// 826E9640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9644: 392A8380  addi r9, r10, -0x7c80
	ctx.r[9].s64 = ctx.r[10].s64 + -31872;
	// 826E9648: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E964C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826E9650: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E9654: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E965C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E966C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E9670: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 826E9674: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9678: 386B48AC  addi r3, r11, 0x48ac
	ctx.r[3].s64 = ctx.r[11].s64 + 18604;
	// 826E967C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826E9680: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9688: 4BD7D799  bl 0x82466e20
	ctx.lr = 0x826E968C;
	sub_82466E20(ctx, base);
	// 826E968C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E96A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E96A0 size=108
    let mut pc: u32 = 0x826E96A0;
    'dispatch: loop {
        match pc {
            0x826E96A0 => {
    //   block [0x826E96A0..0x826E970C)
	// 826E96A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E96A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E96A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E96AC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E96B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E96B4: 38EB284C  addi r7, r11, 0x284c
	ctx.r[7].s64 = ctx.r[11].s64 + 10316;
	// 826E96B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E96BC: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 826E96C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E96C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E96C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E96CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E96D0: 386A48DC  addi r3, r10, 0x48dc
	ctx.r[3].s64 = ctx.r[10].s64 + 18652;
	// 826E96D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E96D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E96DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E96E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E96E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E96E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E96EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E96F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E96F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E96F8: 4BD7D729  bl 0x82466e20
	ctx.lr = 0x826E96FC;
	sub_82466E20(ctx, base);
	// 826E96FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9710 size=112
    let mut pc: u32 = 0x826E9710;
    'dispatch: loop {
        match pc {
            0x826E9710 => {
    //   block [0x826E9710..0x826E9780)
	// 826E9710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E971C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9720: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9724: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E972C: 390B287C  addi r8, r11, 0x287c
	ctx.r[8].s64 = ctx.r[11].s64 + 10364;
	// 826E9730: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9734: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 826E9738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E973C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9748: 386A490C  addi r3, r10, 0x490c
	ctx.r[3].s64 = ctx.r[10].s64 + 18700;
	// 826E974C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E975C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E976C: 4BD7D6B5  bl 0x82466e20
	ctx.lr = 0x826E9770;
	sub_82466E20(ctx, base);
	// 826E9770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E977C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9780 size=112
    let mut pc: u32 = 0x826E9780;
    'dispatch: loop {
        match pc {
            0x826E9780 => {
    //   block [0x826E9780..0x826E97F0)
	// 826E9780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E978C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9790: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9794: 392A83D8  addi r9, r10, -0x7c28
	ctx.r[9].s64 = ctx.r[10].s64 + -31784;
	// 826E9798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E979C: 390B2898  addi r8, r11, 0x2898
	ctx.r[8].s64 = ctx.r[11].s64 + 10392;
	// 826E97A0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826E97A4: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 826E97A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E97AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E97B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E97B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E97B8: 386A493C  addi r3, r10, 0x493c
	ctx.r[3].s64 = ctx.r[10].s64 + 18748;
	// 826E97BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E97C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E97C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E97C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E97CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E97D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E97D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E97D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E97DC: 4BD7D645  bl 0x82466e20
	ctx.lr = 0x826E97E0;
	sub_82466E20(ctx, base);
	// 826E97E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E97E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E97E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E97EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E97F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E97F0 size=112
    let mut pc: u32 = 0x826E97F0;
    'dispatch: loop {
        match pc {
            0x826E97F0 => {
    //   block [0x826E97F0..0x826E9860)
	// 826E97F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E97F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E97F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E97FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9800: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9804: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E980C: 390B28E0  addi r8, r11, 0x28e0
	ctx.r[8].s64 = ctx.r[11].s64 + 10464;
	// 826E9810: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9814: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 826E9818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E981C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9828: 386A496C  addi r3, r10, 0x496c
	ctx.r[3].s64 = ctx.r[10].s64 + 18796;
	// 826E982C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E983C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E984C: 4BD7D5D5  bl 0x82466e20
	ctx.lr = 0x826E9850;
	sub_82466E20(ctx, base);
	// 826E9850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E985C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9860 size=112
    let mut pc: u32 = 0x826E9860;
    'dispatch: loop {
        match pc {
            0x826E9860 => {
    //   block [0x826E9860..0x826E98D0)
	// 826E9860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E986C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9870: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9874: 392A8404  addi r9, r10, -0x7bfc
	ctx.r[9].s64 = ctx.r[10].s64 + -31740;
	// 826E9878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E987C: 390B2900  addi r8, r11, 0x2900
	ctx.r[8].s64 = ctx.r[11].s64 + 10496;
	// 826E9880: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826E9884: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 826E9888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E988C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9898: 386A499C  addi r3, r10, 0x499c
	ctx.r[3].s64 = ctx.r[10].s64 + 18844;
	// 826E989C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E98A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E98A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E98A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E98AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E98B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E98B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E98B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E98BC: 4BD7D565  bl 0x82466e20
	ctx.lr = 0x826E98C0;
	sub_82466E20(ctx, base);
	// 826E98C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E98C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E98C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E98CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E98D0 size=112
    let mut pc: u32 = 0x826E98D0;
    'dispatch: loop {
        match pc {
            0x826E98D0 => {
    //   block [0x826E98D0..0x826E9940)
	// 826E98D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E98D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E98D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E98DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E98E0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E98E4: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E98E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E98EC: 390B2990  addi r8, r11, 0x2990
	ctx.r[8].s64 = ctx.r[11].s64 + 10640;
	// 826E98F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E98F4: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 826E98F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E98FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9908: 386A49CC  addi r3, r10, 0x49cc
	ctx.r[3].s64 = ctx.r[10].s64 + 18892;
	// 826E990C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E991C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E992C: 4BD7D4F5  bl 0x82466e20
	ctx.lr = 0x826E9930;
	sub_82466E20(ctx, base);
	// 826E9930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E993C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9940 size=112
    let mut pc: u32 = 0x826E9940;
    'dispatch: loop {
        match pc {
            0x826E9940 => {
    //   block [0x826E9940..0x826E99B0)
	// 826E9940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E994C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9950: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9954: 38AA4A2C  addi r5, r10, 0x4a2c
	ctx.r[5].s64 = ctx.r[10].s64 + 18988;
	// 826E9958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E995C: 390B29A8  addi r8, r11, 0x29a8
	ctx.r[8].s64 = ctx.r[11].s64 + 10664;
	// 826E9960: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826E9964: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826E9968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E996C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9978: 386A49FC  addi r3, r10, 0x49fc
	ctx.r[3].s64 = ctx.r[10].s64 + 18940;
	// 826E997C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E999C: 4BD7D485  bl 0x82466e20
	ctx.lr = 0x826E99A0;
	sub_82466E20(ctx, base);
	// 826E99A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E99A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E99A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E99AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E99B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E99B0 size=100
    let mut pc: u32 = 0x826E99B0;
    'dispatch: loop {
        match pc {
            0x826E99B0 => {
    //   block [0x826E99B0..0x826E9A14)
	// 826E99B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E99B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E99B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E99BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E99C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E99C4: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826E99C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E99CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E99D0: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 826E99D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E99D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E99DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E99E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E99E4: 386A4A2C  addi r3, r10, 0x4a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 18988;
	// 826E99E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E99EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E99F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826E99F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E99F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826E99FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9A00: 4BD7D421  bl 0x82466e20
	ctx.lr = 0x826E9A04;
	sub_82466E20(ctx, base);
	// 826E9A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826E9A18 size=24
    let mut pc: u32 = 0x826E9A18;
    'dispatch: loop {
        match pc {
            0x826E9A18 => {
    //   block [0x826E9A18..0x826E9A30)
	// 826E9A18: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9A1C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826E9A20: 394A8058  addi r10, r10, -0x7fa8
	ctx.r[10].s64 = ctx.r[10].s64 + -32680;
	// 826E9A24: 816B28FC  lwz r11, 0x28fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10492 as u32) ) } as u64;
	// 826E9A28: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826E9A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9A30 size=116
    let mut pc: u32 = 0x826E9A30;
    'dispatch: loop {
        match pc {
            0x826E9A30 => {
    //   block [0x826E9A30..0x826E9AA4)
	// 826E9A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9A3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826E9A40: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9A44: 390B8058  addi r8, r11, -0x7fa8
	ctx.r[8].s64 = ctx.r[11].s64 + -32680;
	// 826E9A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9A4C: 392A8440  addi r9, r10, -0x7bc0
	ctx.r[9].s64 = ctx.r[10].s64 + -31680;
	// 826E9A50: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9A54: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826E9A58: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9A5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9A64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9A74: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E9A78: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826E9A7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9A80: 386B4A5C  addi r3, r11, 0x4a5c
	ctx.r[3].s64 = ctx.r[11].s64 + 19036;
	// 826E9A84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E9A88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9A90: 4BD7D391  bl 0x82466e20
	ctx.lr = 0x826E9A94;
	sub_82466E20(ctx, base);
	// 826E9A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9AA8 size=108
    let mut pc: u32 = 0x826E9AA8;
    'dispatch: loop {
        match pc {
            0x826E9AA8 => {
    //   block [0x826E9AA8..0x826E9B14)
	// 826E9AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9AB4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9ABC: 38EB2A20  addi r7, r11, 0x2a20
	ctx.r[7].s64 = ctx.r[11].s64 + 10784;
	// 826E9AC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E9AC4: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 826E9AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9ACC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E9AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9AD8: 386A4A8C  addi r3, r10, 0x4a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 19084;
	// 826E9ADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E9AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9B00: 4BD7D321  bl 0x82466e20
	ctx.lr = 0x826E9B04;
	sub_82466E20(ctx, base);
	// 826E9B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9B18 size=112
    let mut pc: u32 = 0x826E9B18;
    'dispatch: loop {
        match pc {
            0x826E9B18 => {
    //   block [0x826E9B18..0x826E9B88)
	// 826E9B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9B24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9B28: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9B2C: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9B30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9B34: 390B2A50  addi r8, r11, 0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + 10832;
	// 826E9B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9B3C: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826E9B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9B50: 386A4ABC  addi r3, r10, 0x4abc
	ctx.r[3].s64 = ctx.r[10].s64 + 19132;
	// 826E9B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9B74: 4BD7D2AD  bl 0x82466e20
	ctx.lr = 0x826E9B78;
	sub_82466E20(ctx, base);
	// 826E9B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9B88 size=112
    let mut pc: u32 = 0x826E9B88;
    'dispatch: loop {
        match pc {
            0x826E9B88 => {
    //   block [0x826E9B88..0x826E9BF8)
	// 826E9B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9B94: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9B98: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9B9C: 392A8464  addi r9, r10, -0x7b9c
	ctx.r[9].s64 = ctx.r[10].s64 + -31644;
	// 826E9BA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9BA4: 390B2A70  addi r8, r11, 0x2a70
	ctx.r[8].s64 = ctx.r[11].s64 + 10864;
	// 826E9BA8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E9BAC: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 826E9BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9BC0: 386A4AEC  addi r3, r10, 0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + 19180;
	// 826E9BC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9BC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E9BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9BE4: 4BD7D23D  bl 0x82466e20
	ctx.lr = 0x826E9BE8;
	sub_82466E20(ctx, base);
	// 826E9BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9BF8 size=112
    let mut pc: u32 = 0x826E9BF8;
    'dispatch: loop {
        match pc {
            0x826E9BF8 => {
    //   block [0x826E9BF8..0x826E9C68)
	// 826E9BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9C04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9C08: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9C0C: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9C14: 390B2B18  addi r8, r11, 0x2b18
	ctx.r[8].s64 = ctx.r[11].s64 + 11032;
	// 826E9C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9C1C: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 826E9C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9C24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9C30: 386A4B1C  addi r3, r10, 0x4b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 19228;
	// 826E9C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9C54: 4BD7D1CD  bl 0x82466e20
	ctx.lr = 0x826E9C58;
	sub_82466E20(ctx, base);
	// 826E9C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9C68 size=112
    let mut pc: u32 = 0x826E9C68;
    'dispatch: loop {
        match pc {
            0x826E9C68 => {
    //   block [0x826E9C68..0x826E9CD8)
	// 826E9C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9C74: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9C78: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9C7C: 392A84BC  addi r9, r10, -0x7b44
	ctx.r[9].s64 = ctx.r[10].s64 + -31556;
	// 826E9C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9C84: 390B2B38  addi r8, r11, 0x2b38
	ctx.r[8].s64 = ctx.r[11].s64 + 11064;
	// 826E9C88: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E9C8C: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 826E9C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9C94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9CA0: 386A4B4C  addi r3, r10, 0x4b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 19276;
	// 826E9CA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9CA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E9CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9CC4: 4BD7D15D  bl 0x82466e20
	ctx.lr = 0x826E9CC8;
	sub_82466E20(ctx, base);
	// 826E9CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9CD8 size=116
    let mut pc: u32 = 0x826E9CD8;
    'dispatch: loop {
        match pc {
            0x826E9CD8 => {
    //   block [0x826E9CD8..0x826E9D4C)
	// 826E9CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9CE4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9CE8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9CEC: 390B2BE0  addi r8, r11, 0x2be0
	ctx.r[8].s64 = ctx.r[11].s64 + 11232;
	// 826E9CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9CF4: 392A8490  addi r9, r10, -0x7b70
	ctx.r[9].s64 = ctx.r[10].s64 + -31600;
	// 826E9CF8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9CFC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826E9D00: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9D04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9D0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9D1C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826E9D20: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 826E9D24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9D28: 386B4B7C  addi r3, r11, 0x4b7c
	ctx.r[3].s64 = ctx.r[11].s64 + 19324;
	// 826E9D2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E9D30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9D38: 4BD7D0E9  bl 0x82466e20
	ctx.lr = 0x826E9D3C;
	sub_82466E20(ctx, base);
	// 826E9D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9D50 size=108
    let mut pc: u32 = 0x826E9D50;
    'dispatch: loop {
        match pc {
            0x826E9D50 => {
    //   block [0x826E9D50..0x826E9DBC)
	// 826E9D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9D5C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9D60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9D64: 38EB2BF8  addi r7, r11, 0x2bf8
	ctx.r[7].s64 = ctx.r[11].s64 + 11256;
	// 826E9D68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E9D6C: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826E9D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9D74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E9D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9D80: 386A4BAC  addi r3, r10, 0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + 19372;
	// 826E9D84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E9D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9DA8: 4BD7D079  bl 0x82466e20
	ctx.lr = 0x826E9DAC;
	sub_82466E20(ctx, base);
	// 826E9DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9DC0 size=112
    let mut pc: u32 = 0x826E9DC0;
    'dispatch: loop {
        match pc {
            0x826E9DC0 => {
    //   block [0x826E9DC0..0x826E9E30)
	// 826E9DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9DD0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9DD4: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9DD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9DDC: 390B2C28  addi r8, r11, 0x2c28
	ctx.r[8].s64 = ctx.r[11].s64 + 11304;
	// 826E9DE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9DE4: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 826E9DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9DEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9DF8: 386A4BDC  addi r3, r10, 0x4bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 19420;
	// 826E9DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9E1C: 4BD7D005  bl 0x82466e20
	ctx.lr = 0x826E9E20;
	sub_82466E20(ctx, base);
	// 826E9E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9E30 size=112
    let mut pc: u32 = 0x826E9E30;
    'dispatch: loop {
        match pc {
            0x826E9E30 => {
    //   block [0x826E9E30..0x826E9EA0)
	// 826E9E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9E3C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826E9E40: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9E44: 392A84F0  addi r9, r10, -0x7b10
	ctx.r[9].s64 = ctx.r[10].s64 + -31504;
	// 826E9E48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9E4C: 390B2C40  addi r8, r11, 0x2c40
	ctx.r[8].s64 = ctx.r[11].s64 + 11328;
	// 826E9E50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826E9E54: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826E9E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9E5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9E68: 386A4C0C  addi r3, r10, 0x4c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 19468;
	// 826E9E6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826E9E70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826E9E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9E8C: 4BD7CF95  bl 0x82466e20
	ctx.lr = 0x826E9E90;
	sub_82466E20(ctx, base);
	// 826E9E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9EA0 size=112
    let mut pc: u32 = 0x826E9EA0;
    'dispatch: loop {
        match pc {
            0x826E9EA0 => {
    //   block [0x826E9EA0..0x826E9F10)
	// 826E9EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9EB0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9EB4: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9EBC: 390B2CE8  addi r8, r11, 0x2ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 11496;
	// 826E9EC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826E9EC4: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 826E9EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9ED8: 386A4C3C  addi r3, r10, 0x4c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 19516;
	// 826E9EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9EFC: 4BD7CF25  bl 0x82466e20
	ctx.lr = 0x826E9F00;
	sub_82466E20(ctx, base);
	// 826E9F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9F10 size=108
    let mut pc: u32 = 0x826E9F10;
    'dispatch: loop {
        match pc {
            0x826E9F10 => {
    //   block [0x826E9F10..0x826E9F7C)
	// 826E9F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9F1C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9F20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9F24: 38EB2D30  addi r7, r11, 0x2d30
	ctx.r[7].s64 = ctx.r[11].s64 + 11568;
	// 826E9F28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826E9F2C: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826E9F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9F34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826E9F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9F40: 386A4C6C  addi r3, r10, 0x4c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 19564;
	// 826E9F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826E9F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826E9F68: 4BD7CEB9  bl 0x82466e20
	ctx.lr = 0x826E9F6C;
	sub_82466E20(ctx, base);
	// 826E9F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9F80 size=112
    let mut pc: u32 = 0x826E9F80;
    'dispatch: loop {
        match pc {
            0x826E9F80 => {
    //   block [0x826E9F80..0x826E9FF0)
	// 826E9F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9F8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9F90: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826E9F94: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826E9F98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826E9F9C: 390B2D60  addi r8, r11, 0x2d60
	ctx.r[8].s64 = ctx.r[11].s64 + 11616;
	// 826E9FA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826E9FA4: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 826E9FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826E9FAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826E9FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826E9FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826E9FB8: 386A4C9C  addi r3, r10, 0x4c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 19612;
	// 826E9FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826E9FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826E9FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826E9FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826E9FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826E9FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826E9FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826E9FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826E9FDC: 4BD7CE45  bl 0x82466e20
	ctx.lr = 0x826E9FE0;
	sub_82466E20(ctx, base);
	// 826E9FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826E9FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826E9FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826E9FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826E9FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826E9FF0 size=112
    let mut pc: u32 = 0x826E9FF0;
    'dispatch: loop {
        match pc {
            0x826E9FF0 => {
    //   block [0x826E9FF0..0x826EA060)
	// 826E9FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826E9FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826E9FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826E9FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA000: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA004: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EA008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA00C: 390B2D78  addi r8, r11, 0x2d78
	ctx.r[8].s64 = ctx.r[11].s64 + 11640;
	// 826EA010: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826EA014: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 826EA018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA01C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA028: 386A4CCC  addi r3, r10, 0x4ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 19660;
	// 826EA02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA04C: 4BD7CDD5  bl 0x82466e20
	ctx.lr = 0x826EA050;
	sub_82466E20(ctx, base);
	// 826EA050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA060 size=100
    let mut pc: u32 = 0x826EA060;
    'dispatch: loop {
        match pc {
            0x826EA060 => {
    //   block [0x826EA060..0x826EA0C4)
	// 826EA060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA06C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA074: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EA078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA080: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826EA084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA094: 386A4CFC  addi r3, r10, 0x4cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 19708;
	// 826EA098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826EA0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826EA0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA0B0: 4BD7CD71  bl 0x82466e20
	ctx.lr = 0x826EA0B4;
	sub_82466E20(ctx, base);
	// 826EA0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA0C8 size=112
    let mut pc: u32 = 0x826EA0C8;
    'dispatch: loop {
        match pc {
            0x826EA0C8 => {
    //   block [0x826EA0C8..0x826EA138)
	// 826EA0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA0D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA0D8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA0DC: 38AA48AC  addi r5, r10, 0x48ac
	ctx.r[5].s64 = ctx.r[10].s64 + 18604;
	// 826EA0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA0E4: 390B2E38  addi r8, r11, 0x2e38
	ctx.r[8].s64 = ctx.r[11].s64 + 11832;
	// 826EA0E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EA0EC: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 826EA0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA100: 386A4D2C  addi r3, r10, 0x4d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 19756;
	// 826EA104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA124: 4BD7CCFD  bl 0x82466e20
	ctx.lr = 0x826EA128;
	sub_82466E20(ctx, base);
	// 826EA128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA138 size=112
    let mut pc: u32 = 0x826EA138;
    'dispatch: loop {
        match pc {
            0x826EA138 => {
    //   block [0x826EA138..0x826EA1A8)
	// 826EA138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA148: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA14C: 38AA472C  addi r5, r10, 0x472c
	ctx.r[5].s64 = ctx.r[10].s64 + 18220;
	// 826EA150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA154: 390B2E68  addi r8, r11, 0x2e68
	ctx.r[8].s64 = ctx.r[11].s64 + 11880;
	// 826EA158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EA15C: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826EA160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA170: 386A4D5C  addi r3, r10, 0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 19804;
	// 826EA174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA194: 4BD7CC8D  bl 0x82466e20
	ctx.lr = 0x826EA198;
	sub_82466E20(ctx, base);
	// 826EA198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA1A8 size=108
    let mut pc: u32 = 0x826EA1A8;
    'dispatch: loop {
        match pc {
            0x826EA1A8 => {
    //   block [0x826EA1A8..0x826EA214)
	// 826EA1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA1B4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA1B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA1BC: 38EB2E80  addi r7, r11, 0x2e80
	ctx.r[7].s64 = ctx.r[11].s64 + 11904;
	// 826EA1C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826EA1C4: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 826EA1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA1CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA1D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EA1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA1D8: 386A4D8C  addi r3, r10, 0x4d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 19852;
	// 826EA1DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EA1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA200: 4BD7CC21  bl 0x82466e20
	ctx.lr = 0x826EA204;
	sub_82466E20(ctx, base);
	// 826EA204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA218 size=112
    let mut pc: u32 = 0x826EA218;
    'dispatch: loop {
        match pc {
            0x826EA218 => {
    //   block [0x826EA218..0x826EA288)
	// 826EA218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA228: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA22C: 38AA4CFC  addi r5, r10, 0x4cfc
	ctx.r[5].s64 = ctx.r[10].s64 + 19708;
	// 826EA230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA234: 390B2EB0  addi r8, r11, 0x2eb0
	ctx.r[8].s64 = ctx.r[11].s64 + 11952;
	// 826EA238: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826EA23C: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 826EA240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA250: 386A4DBC  addi r3, r10, 0x4dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 19900;
	// 826EA254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA274: 4BD7CBAD  bl 0x82466e20
	ctx.lr = 0x826EA278;
	sub_82466E20(ctx, base);
	// 826EA278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA288 size=112
    let mut pc: u32 = 0x826EA288;
    'dispatch: loop {
        match pc {
            0x826EA288 => {
    //   block [0x826EA288..0x826EA2F8)
	// 826EA288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA294: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EA298: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA29C: 392A851C  addi r9, r10, -0x7ae4
	ctx.r[9].s64 = ctx.r[10].s64 + -31460;
	// 826EA2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA2A4: 390B2F48  addi r8, r11, 0x2f48
	ctx.r[8].s64 = ctx.r[11].s64 + 12104;
	// 826EA2A8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826EA2AC: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 826EA2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA2C0: 386A4DEC  addi r3, r10, 0x4dec
	ctx.r[3].s64 = ctx.r[10].s64 + 19948;
	// 826EA2C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EA2C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EA2CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA2DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA2E4: 4BD7CB3D  bl 0x82466e20
	ctx.lr = 0x826EA2E8;
	sub_82466E20(ctx, base);
	// 826EA2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA2F8 size=112
    let mut pc: u32 = 0x826EA2F8;
    'dispatch: loop {
        match pc {
            0x826EA2F8 => {
    //   block [0x826EA2F8..0x826EA368)
	// 826EA2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA304: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA308: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA30C: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EA310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA314: 390B2F90  addi r8, r11, 0x2f90
	ctx.r[8].s64 = ctx.r[11].s64 + 12176;
	// 826EA318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EA31C: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 826EA320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA324: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA330: 386A4E1C  addi r3, r10, 0x4e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 19996;
	// 826EA334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA354: 4BD7CACD  bl 0x82466e20
	ctx.lr = 0x826EA358;
	sub_82466E20(ctx, base);
	// 826EA358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA368 size=108
    let mut pc: u32 = 0x826EA368;
    'dispatch: loop {
        match pc {
            0x826EA368 => {
    //   block [0x826EA368..0x826EA3D4)
	// 826EA368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA374: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA37C: 38EB2FA8  addi r7, r11, 0x2fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 12200;
	// 826EA380: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826EA384: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 826EA388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA38C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EA394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA398: 386A4E4C  addi r3, r10, 0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 20044;
	// 826EA39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EA3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA3C0: 4BD7CA61  bl 0x82466e20
	ctx.lr = 0x826EA3C4;
	sub_82466E20(ctx, base);
	// 826EA3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA3D8 size=116
    let mut pc: u32 = 0x826EA3D8;
    'dispatch: loop {
        match pc {
            0x826EA3D8 => {
    //   block [0x826EA3D8..0x826EA44C)
	// 826EA3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA3E4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826EA3E8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826EA3EC: 390A3038  addi r8, r10, 0x3038
	ctx.r[8].s64 = ctx.r[10].s64 + 12344;
	// 826EA3F0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA3F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826EA3F8: 38AA4CFC  addi r5, r10, 0x4cfc
	ctx.r[5].s64 = ctx.r[10].s64 + 19708;
	// 826EA3FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA400: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EA404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA40C: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 826EA410: 396B8530  addi r11, r11, -0x7ad0
	ctx.r[11].s64 = ctx.r[11].s64 + -31440;
	// 826EA414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA418: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA41C: 386A4E7C  addi r3, r10, 0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 20092;
	// 826EA420: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826EA424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA428: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826EA42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA438: 4BD7C9E9  bl 0x82466e20
	ctx.lr = 0x826EA43C;
	sub_82466E20(ctx, base);
	// 826EA43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA450 size=112
    let mut pc: u32 = 0x826EA450;
    'dispatch: loop {
        match pc {
            0x826EA450 => {
    //   block [0x826EA450..0x826EA4C0)
	// 826EA450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA45C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EA460: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA464: 392A857C  addi r9, r10, -0x7a84
	ctx.r[9].s64 = ctx.r[10].s64 + -31364;
	// 826EA468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA46C: 390B3110  addi r8, r11, 0x3110
	ctx.r[8].s64 = ctx.r[11].s64 + 12560;
	// 826EA470: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826EA474: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 826EA478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA47C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA488: 386A4EAC  addi r3, r10, 0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + 20140;
	// 826EA48C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EA490: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EA494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA4A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA4AC: 4BD7C975  bl 0x82466e20
	ctx.lr = 0x826EA4B0;
	sub_82466E20(ctx, base);
	// 826EA4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA4C0 size=112
    let mut pc: u32 = 0x826EA4C0;
    'dispatch: loop {
        match pc {
            0x826EA4C0 => {
    //   block [0x826EA4C0..0x826EA530)
	// 826EA4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA4CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA4D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA4D4: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EA4D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA4DC: 390B3170  addi r8, r11, 0x3170
	ctx.r[8].s64 = ctx.r[11].s64 + 12656;
	// 826EA4E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826EA4E4: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 826EA4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA4EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA4F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA4F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA4F8: 386A4EDC  addi r3, r10, 0x4edc
	ctx.r[3].s64 = ctx.r[10].s64 + 20188;
	// 826EA4FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA51C: 4BD7C905  bl 0x82466e20
	ctx.lr = 0x826EA520;
	sub_82466E20(ctx, base);
	// 826EA520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA530 size=108
    let mut pc: u32 = 0x826EA530;
    'dispatch: loop {
        match pc {
            0x826EA530 => {
    //   block [0x826EA530..0x826EA59C)
	// 826EA530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA53C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA544: 38EB3188  addi r7, r11, 0x3188
	ctx.r[7].s64 = ctx.r[11].s64 + 12680;
	// 826EA548: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826EA54C: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 826EA550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA554: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EA55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA560: 386A4F0C  addi r3, r10, 0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 20236;
	// 826EA564: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EA568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA588: 4BD7C899  bl 0x82466e20
	ctx.lr = 0x826EA58C;
	sub_82466E20(ctx, base);
	// 826EA58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA5A0 size=112
    let mut pc: u32 = 0x826EA5A0;
    'dispatch: loop {
        match pc {
            0x826EA5A0 => {
    //   block [0x826EA5A0..0x826EA610)
	// 826EA5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA5AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA5B0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA5B4: 38AA4CFC  addi r5, r10, 0x4cfc
	ctx.r[5].s64 = ctx.r[10].s64 + 19708;
	// 826EA5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA5BC: 390B31D0  addi r8, r11, 0x31d0
	ctx.r[8].s64 = ctx.r[11].s64 + 12752;
	// 826EA5C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826EA5C4: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 826EA5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA5CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA5D8: 386A4F3C  addi r3, r10, 0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 20284;
	// 826EA5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA5FC: 4BD7C825  bl 0x82466e20
	ctx.lr = 0x826EA600;
	sub_82466E20(ctx, base);
	// 826EA600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA610 size=112
    let mut pc: u32 = 0x826EA610;
    'dispatch: loop {
        match pc {
            0x826EA610 => {
    //   block [0x826EA610..0x826EA680)
	// 826EA610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA61C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA620: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA624: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EA628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA62C: 390B3248  addi r8, r11, 0x3248
	ctx.r[8].s64 = ctx.r[11].s64 + 12872;
	// 826EA630: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826EA634: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 826EA638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA63C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA648: 386A4F6C  addi r3, r10, 0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 20332;
	// 826EA64C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA66C: 4BD7C7B5  bl 0x82466e20
	ctx.lr = 0x826EA670;
	sub_82466E20(ctx, base);
	// 826EA670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA680 size=108
    let mut pc: u32 = 0x826EA680;
    'dispatch: loop {
        match pc {
            0x826EA680 => {
    //   block [0x826EA680..0x826EA6EC)
	// 826EA680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA68C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA694: 38EB3278  addi r7, r11, 0x3278
	ctx.r[7].s64 = ctx.r[11].s64 + 12920;
	// 826EA698: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826EA69C: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 826EA6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA6A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EA6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA6B0: 386A4F9C  addi r3, r10, 0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 20380;
	// 826EA6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EA6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA6D8: 4BD7C749  bl 0x82466e20
	ctx.lr = 0x826EA6DC;
	sub_82466E20(ctx, base);
	// 826EA6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA6F0 size=108
    let mut pc: u32 = 0x826EA6F0;
    'dispatch: loop {
        match pc {
            0x826EA6F0 => {
    //   block [0x826EA6F0..0x826EA75C)
	// 826EA6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA6FC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA704: 38EB32D8  addi r7, r11, 0x32d8
	ctx.r[7].s64 = ctx.r[11].s64 + 13016;
	// 826EA708: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826EA70C: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 826EA710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826EA71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA720: 386A4FCC  addi r3, r10, 0x4fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 20428;
	// 826EA724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826EA728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826EA748: 4BD7C6D9  bl 0x82466e20
	ctx.lr = 0x826EA74C;
	sub_82466E20(ctx, base);
	// 826EA74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA760 size=112
    let mut pc: u32 = 0x826EA760;
    'dispatch: loop {
        match pc {
            0x826EA760 => {
    //   block [0x826EA760..0x826EA7D0)
	// 826EA760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA76C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA770: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA774: 38AA487C  addi r5, r10, 0x487c
	ctx.r[5].s64 = ctx.r[10].s64 + 18556;
	// 826EA778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA77C: 390B3350  addi r8, r11, 0x3350
	ctx.r[8].s64 = ctx.r[11].s64 + 13136;
	// 826EA780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826EA784: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 826EA788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA78C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA798: 386A4FFC  addi r3, r10, 0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 20476;
	// 826EA79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826EA7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826EA7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826EA7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA7BC: 4BD7C665  bl 0x82466e20
	ctx.lr = 0x826EA7C0;
	sub_82466E20(ctx, base);
	// 826EA7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826EA7D0 size=24
    let mut pc: u32 = 0x826EA7D0;
    'dispatch: loop {
        match pc {
            0x826EA7D0 => {
    //   block [0x826EA7D0..0x826EA7E8)
	// 826EA7D0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826EA7D4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826EA7D8: 394A80D0  addi r10, r10, -0x7f30
	ctx.r[10].s64 = ctx.r[10].s64 + -32560;
	// 826EA7DC: 816B3398  lwz r11, 0x3398(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13208 as u32) ) } as u64;
	// 826EA7E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826EA7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826EA7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826EA7E8 size=116
    let mut pc: u32 = 0x826EA7E8;
    'dispatch: loop {
        match pc {
            0x826EA7E8 => {
    //   block [0x826EA7E8..0x826EA85C)
	// 826EA7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826EA7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826EA7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826EA7F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826EA7F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826EA7FC: 390B80D0  addi r8, r11, -0x7f30
	ctx.r[8].s64 = ctx.r[11].s64 + -32560;
	// 826EA800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826EA804: 392A85C0  addi r9, r10, -0x7a40
	ctx.r[9].s64 = ctx.r[10].s64 + -31296;
	// 826EA808: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826EA80C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826EA810: 38AA292C  addi r5, r10, 0x292c
	ctx.r[5].s64 = ctx.r[10].s64 + 10540;
	// 826EA814: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826EA818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826EA81C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826EA820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826EA824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826EA828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826EA82C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826EA830: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826EA834: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826EA838: 386B502C  addi r3, r11, 0x502c
	ctx.r[3].s64 = ctx.r[11].s64 + 20524;
	// 826EA83C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826EA840: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826EA844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826EA848: 4BD7C5D9  bl 0x82466e20
	ctx.lr = 0x826EA84C;
	sub_82466E20(ctx, base);
	// 826EA84C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826EA850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826EA854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826EA858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


