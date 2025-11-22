pub fn sub_8312C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C260 size=132
    let mut pc: u32 = 0x8312C260;
    'dispatch: loop {
        match pc {
            0x8312C260 => {
    //   block [0x8312C260..0x8312C2E4)
	// 8312C260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C264: 4807BF09  bl 0x831a816c
	ctx.lr = 0x8312C268;
	sub_831A8130(ctx, base);
	// 8312C268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C26C: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 8312C270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C274: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312C278: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C27C: 48002D85  bl 0x8312f000
	ctx.lr = 0x8312C280;
	sub_8312F000(ctx, base);
	// 8312C280: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C284: 409A0018  bne cr6, 0x8312c29c
	if !ctx.cr[6].eq {
	pc = 0x8312C29C; continue 'dispatch;
	}
	// 8312C288: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C28C: 386B0720  addi r3, r11, 0x720
	ctx.r[3].s64 = ctx.r[11].s64 + 1824;
	// 8312C290: 4BFF9FB1  bl 0x83126240
	ctx.lr = 0x8312C294;
	sub_83126240(ctx, base);
	// 8312C294: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312C298: 48000038  b 0x8312c2d0
	pc = 0x8312C2D0; continue 'dispatch;
	// 8312C29C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C2A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C2A4: 409A0010  bne cr6, 0x8312c2b4
	if !ctx.cr[6].eq {
	pc = 0x8312C2B4; continue 'dispatch;
	}
	// 8312C2A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C2AC: 386B06F8  addi r3, r11, 0x6f8
	ctx.r[3].s64 = ctx.r[11].s64 + 1784;
	// 8312C2B0: 4BFFFFE0  b 0x8312c290
	pc = 0x8312C290; continue 'dispatch;
	// 8312C2B4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312C2B8: 419A0014  beq cr6, 0x8312c2cc
	if ctx.cr[6].eq {
	pc = 0x8312C2CC; continue 'dispatch;
	}
	// 8312C2BC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C2C0: 616B000C  ori r11, r11, 0xc
	ctx.r[11].u64 = ctx.r[11].u64 | 12;
	// 8312C2C4: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312C2C8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312C2CC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312C2D0: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C2D4: 48002DC5  bl 0x8312f098
	ctx.lr = 0x8312C2D8;
	sub_8312F098(ctx, base);
	// 8312C2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C2DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C2E0: 4807BEDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C2E8 size=68
    let mut pc: u32 = 0x8312C2E8;
    'dispatch: loop {
        match pc {
            0x8312C2E8 => {
    //   block [0x8312C2E8..0x8312C32C)
	// 8312C2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C2EC: 4807BE81  bl 0x831a816c
	ctx.lr = 0x8312C2F0;
	sub_831A8130(ctx, base);
	// 8312C2F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C2F4: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8312C2F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C2FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8312C300: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C304: 48002CFD  bl 0x8312f000
	ctx.lr = 0x8312C308;
	sub_8312F000(ctx, base);
	// 8312C308: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312C30C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C310: 4BFFF369  bl 0x8312b678
	ctx.lr = 0x8312C314;
	sub_8312B678(ctx, base);
	// 8312C314: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C318: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C31C: 48002D7D  bl 0x8312f098
	ctx.lr = 0x8312C320;
	sub_8312F098(ctx, base);
	// 8312C320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C324: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C328: 4807BE94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C330 size=148
    let mut pc: u32 = 0x8312C330;
    'dispatch: loop {
        match pc {
            0x8312C330 => {
    //   block [0x8312C330..0x8312C3C4)
	// 8312C330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C334: 4807BE39  bl 0x831a816c
	ctx.lr = 0x8312C338;
	sub_831A8130(ctx, base);
	// 8312C338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C33C: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 8312C340: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C344: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312C348: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C34C: 48002CB5  bl 0x8312f000
	ctx.lr = 0x8312C350;
	sub_8312F000(ctx, base);
	// 8312C350: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C354: 409A0018  bne cr6, 0x8312c36c
	if !ctx.cr[6].eq {
	pc = 0x8312C36C; continue 'dispatch;
	}
	// 8312C358: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C35C: 386B07C0  addi r3, r11, 0x7c0
	ctx.r[3].s64 = ctx.r[11].s64 + 1984;
	// 8312C360: 4BFF9EE1  bl 0x83126240
	ctx.lr = 0x8312C364;
	sub_83126240(ctx, base);
	// 8312C364: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312C368: 48000048  b 0x8312c3b0
	pc = 0x8312C3B0; continue 'dispatch;
	// 8312C36C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C370: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C374: 409A0010  bne cr6, 0x8312c384
	if !ctx.cr[6].eq {
	pc = 0x8312C384; continue 'dispatch;
	}
	// 8312C378: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C37C: 386B0798  addi r3, r11, 0x798
	ctx.r[3].s64 = ctx.r[11].s64 + 1944;
	// 8312C380: 4BFFFFE0  b 0x8312c360
	pc = 0x8312C360; continue 'dispatch;
	// 8312C384: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312C388: 419A0024  beq cr6, 0x8312c3ac
	if ctx.cr[6].eq {
	pc = 0x8312C3AC; continue 'dispatch;
	}
	// 8312C38C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C390: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312C394: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 8312C398: 614A0018  ori r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u64 | 24;
	// 8312C39C: 7D5F502A  ldx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	// 8312C3A0: 7D7F5AAA  lwax r11, r31, r11
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as i32) as i64;
	// 8312C3A4: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8312C3A8: F97E0000  std r11, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 8312C3AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312C3B0: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C3B4: 48002CE5  bl 0x8312f098
	ctx.lr = 0x8312C3B8;
	sub_8312F098(ctx, base);
	// 8312C3B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C3BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C3C0: 4807BDFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312C3C8 size=132
    let mut pc: u32 = 0x8312C3C8;
    'dispatch: loop {
        match pc {
            0x8312C3C8 => {
    //   block [0x8312C3C8..0x8312C44C)
	// 8312C3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C3CC: 4807BDA1  bl 0x831a816c
	ctx.lr = 0x8312C3D0;
	sub_831A8130(ctx, base);
	// 8312C3D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C3D4: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 8312C3D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C3DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312C3E0: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C3E4: 48002C1D  bl 0x8312f000
	ctx.lr = 0x8312C3E8;
	sub_8312F000(ctx, base);
	// 8312C3E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C3EC: 409A0018  bne cr6, 0x8312c404
	if !ctx.cr[6].eq {
	pc = 0x8312C404; continue 'dispatch;
	}
	// 8312C3F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C3F4: 386B0810  addi r3, r11, 0x810
	ctx.r[3].s64 = ctx.r[11].s64 + 2064;
	// 8312C3F8: 4BFF9E49  bl 0x83126240
	ctx.lr = 0x8312C3FC;
	sub_83126240(ctx, base);
	// 8312C3FC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312C400: 48000038  b 0x8312c438
	pc = 0x8312C438; continue 'dispatch;
	// 8312C404: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C408: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C40C: 409A0010  bne cr6, 0x8312c41c
	if !ctx.cr[6].eq {
	pc = 0x8312C41C; continue 'dispatch;
	}
	// 8312C410: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C414: 386B07E8  addi r3, r11, 0x7e8
	ctx.r[3].s64 = ctx.r[11].s64 + 2024;
	// 8312C418: 4BFFFFE0  b 0x8312c3f8
	pc = 0x8312C3F8; continue 'dispatch;
	// 8312C41C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312C420: 419A0014  beq cr6, 0x8312c434
	if ctx.cr[6].eq {
	pc = 0x8312C434; continue 'dispatch;
	}
	// 8312C424: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C428: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 8312C42C: 7C1F5C2E  lfsx f0, r31, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312C430: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8312C434: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312C438: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C43C: 48002C5D  bl 0x8312f098
	ctx.lr = 0x8312C440;
	sub_8312F098(ctx, base);
	// 8312C440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C448: 4807BD74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C450 size=132
    let mut pc: u32 = 0x8312C450;
    'dispatch: loop {
        match pc {
            0x8312C450 => {
    //   block [0x8312C450..0x8312C4D4)
	// 8312C450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C454: 4807BD19  bl 0x831a816c
	ctx.lr = 0x8312C458;
	sub_831A8130(ctx, base);
	// 8312C458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C45C: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 8312C460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C464: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312C468: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C46C: 48002B95  bl 0x8312f000
	ctx.lr = 0x8312C470;
	sub_8312F000(ctx, base);
	// 8312C470: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C474: 409A0018  bne cr6, 0x8312c48c
	if !ctx.cr[6].eq {
	pc = 0x8312C48C; continue 'dispatch;
	}
	// 8312C478: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C47C: 386B0860  addi r3, r11, 0x860
	ctx.r[3].s64 = ctx.r[11].s64 + 2144;
	// 8312C480: 4BFF9DC1  bl 0x83126240
	ctx.lr = 0x8312C484;
	sub_83126240(ctx, base);
	// 8312C484: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312C488: 48000038  b 0x8312c4c0
	pc = 0x8312C4C0; continue 'dispatch;
	// 8312C48C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C490: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C494: 409A0010  bne cr6, 0x8312c4a4
	if !ctx.cr[6].eq {
	pc = 0x8312C4A4; continue 'dispatch;
	}
	// 8312C498: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C49C: 386B0838  addi r3, r11, 0x838
	ctx.r[3].s64 = ctx.r[11].s64 + 2104;
	// 8312C4A0: 4BFFFFE0  b 0x8312c480
	pc = 0x8312C480; continue 'dispatch;
	// 8312C4A4: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 8312C4A8: 396B004C  addi r11, r11, 0x4c
	ctx.r[11].s64 = ctx.r[11].s64 + 76;
	// 8312C4AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C4B0: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8312C4B4: 419A0008  beq cr6, 0x8312c4bc
	if ctx.cr[6].eq {
	pc = 0x8312C4BC; continue 'dispatch;
	}
	// 8312C4B8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8312C4BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312C4C0: 807D76A4  lwz r3, 0x76a4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C4C4: 48002BD5  bl 0x8312f098
	ctx.lr = 0x8312C4C8;
	sub_8312F098(ctx, base);
	// 8312C4C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C4CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C4D0: 4807BCEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C4D8 size=144
    let mut pc: u32 = 0x8312C4D8;
    'dispatch: loop {
        match pc {
            0x8312C4D8 => {
    //   block [0x8312C4D8..0x8312C568)
	// 8312C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C4DC: 4807BC8D  bl 0x831a8168
	ctx.lr = 0x8312C4E0;
	sub_831A8130(ctx, base);
	// 8312C4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C4E4: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 8312C4E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C4EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8312C4F0: 807C76A4  lwz r3, 0x76a4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C4F4: 48002B0D  bl 0x8312f000
	ctx.lr = 0x8312C4F8;
	sub_8312F000(ctx, base);
	// 8312C4F8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312C4FC: 409A0010  bne cr6, 0x8312c50c
	if !ctx.cr[6].eq {
	pc = 0x8312C50C; continue 'dispatch;
	}
	// 8312C500: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C504: 386B08DC  addi r3, r11, 0x8dc
	ctx.r[3].s64 = ctx.r[11].s64 + 2268;
	// 8312C508: 48000044  b 0x8312c54c
	pc = 0x8312C54C; continue 'dispatch;
	// 8312C50C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C510: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C514: 409A0010  bne cr6, 0x8312c524
	if !ctx.cr[6].eq {
	pc = 0x8312C524; continue 'dispatch;
	}
	// 8312C518: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C51C: 386B08B4  addi r3, r11, 0x8b4
	ctx.r[3].s64 = ctx.r[11].s64 + 2228;
	// 8312C520: 4800002C  b 0x8312c54c
	pc = 0x8312C54C; continue 'dispatch;
	// 8312C524: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 8312C528: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8312C52C: 41990018  bgt cr6, 0x8312c544
	if ctx.cr[6].gt {
	pc = 0x8312C544; continue 'dispatch;
	}
	// 8312C530: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C534: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312C538: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 8312C53C: 7FBE592E  stwx r29, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8312C540: 48000014  b 0x8312c554
	pc = 0x8312C554; continue 'dispatch;
	// 8312C544: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C548: 386B0888  addi r3, r11, 0x888
	ctx.r[3].s64 = ctx.r[11].s64 + 2184;
	// 8312C54C: 4BFF9CF5  bl 0x83126240
	ctx.lr = 0x8312C550;
	sub_83126240(ctx, base);
	// 8312C550: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312C554: 807C76A4  lwz r3, 0x76a4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C558: 48002B41  bl 0x8312f098
	ctx.lr = 0x8312C55C;
	sub_8312F098(ctx, base);
	// 8312C55C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312C564: 4807BC54  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C568 size=136
    let mut pc: u32 = 0x8312C568;
    'dispatch: loop {
        match pc {
            0x8312C568 => {
    //   block [0x8312C568..0x8312C5F0)
	// 8312C568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C56C: 4807BBFD  bl 0x831a8168
	ctx.lr = 0x8312C570;
	sub_831A8130(ctx, base);
	// 8312C570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C574: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 8312C578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C57C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8312C580: 807C76A4  lwz r3, 0x76a4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C584: 48002A7D  bl 0x8312f000
	ctx.lr = 0x8312C588;
	sub_8312F000(ctx, base);
	// 8312C588: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C58C: 409A0018  bne cr6, 0x8312c5a4
	if !ctx.cr[6].eq {
	pc = 0x8312C5A4; continue 'dispatch;
	}
	// 8312C590: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C594: 386B092C  addi r3, r11, 0x92c
	ctx.r[3].s64 = ctx.r[11].s64 + 2348;
	// 8312C598: 4BFF9CA9  bl 0x83126240
	ctx.lr = 0x8312C59C;
	sub_83126240(ctx, base);
	// 8312C59C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8312C5A0: 4800003C  b 0x8312c5dc
	pc = 0x8312C5DC; continue 'dispatch;
	// 8312C5A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C5A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C5AC: 409A0010  bne cr6, 0x8312c5bc
	if !ctx.cr[6].eq {
	pc = 0x8312C5BC; continue 'dispatch;
	}
	// 8312C5B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C5B4: 386B0904  addi r3, r11, 0x904
	ctx.r[3].s64 = ctx.r[11].s64 + 2308;
	// 8312C5B8: 4BFFFFE0  b 0x8312c598
	pc = 0x8312C598; continue 'dispatch;
	// 8312C5BC: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C5C0: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312C5C4: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 8312C5C8: 614A0030  ori r10, r10, 0x30
	ctx.r[10].u64 = ctx.r[10].u64 | 48;
	// 8312C5CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312C5D0: 7D7F5AAA  lwax r11, r31, r11
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as i32) as i64;
	// 8312C5D4: 7D7D5BD2  divd r11, r29, r11
	ctx.r[11].s64 = ctx.r[29].s64 / ctx.r[11].s64;
	// 8312C5D8: 7D7F512A  stdx r11, r31, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u64) };
	// 8312C5DC: 807C76A4  lwz r3, 0x76a4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C5E0: 48002AB9  bl 0x8312f098
	ctx.lr = 0x8312C5E4;
	sub_8312F098(ctx, base);
	// 8312C5E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312C5EC: 4807BBCC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312C5F0 size=164
    let mut pc: u32 = 0x8312C5F0;
    'dispatch: loop {
        match pc {
            0x8312C5F0 => {
    //   block [0x8312C5F0..0x8312C694)
	// 8312C5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312C5F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312C5FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312C600: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8312C604: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C608: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8312C60C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8312C610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C614: 807E76A4  lwz r3, 0x76a4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C618: 480029E9  bl 0x8312f000
	ctx.lr = 0x8312C61C;
	sub_8312F000(ctx, base);
	// 8312C61C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C620: 409A0018  bne cr6, 0x8312c638
	if !ctx.cr[6].eq {
	pc = 0x8312C638; continue 'dispatch;
	}
	// 8312C624: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C628: 386B092C  addi r3, r11, 0x92c
	ctx.r[3].s64 = ctx.r[11].s64 + 2348;
	// 8312C62C: 4BFF9C15  bl 0x83126240
	ctx.lr = 0x8312C630;
	sub_83126240(ctx, base);
	// 8312C630: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312C634: 48000038  b 0x8312c66c
	pc = 0x8312C66C; continue 'dispatch;
	// 8312C638: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C63C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C640: 409A0010  bne cr6, 0x8312c650
	if !ctx.cr[6].eq {
	pc = 0x8312C650; continue 'dispatch;
	}
	// 8312C644: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C648: 386B0904  addi r3, r11, 0x904
	ctx.r[3].s64 = ctx.r[11].s64 + 2308;
	// 8312C64C: 4BFFFFE0  b 0x8312c62c
	pc = 0x8312C62C; continue 'dispatch;
	// 8312C650: 3D7F0002  addis r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 131072;
	// 8312C654: 396B0050  addi r11, r11, 0x50
	ctx.r[11].s64 = ctx.r[11].s64 + 80;
	// 8312C658: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312C65C: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8312C660: 419A0008  beq cr6, 0x8312c668
	if ctx.cr[6].eq {
	pc = 0x8312C668; continue 'dispatch;
	}
	// 8312C664: D3EB0000  stfs f31, 0(r11)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8312C668: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312C66C: 807E76A4  lwz r3, 0x76a4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C670: 48002A29  bl 0x8312f098
	ctx.lr = 0x8312C674;
	sub_8312F098(ctx, base);
	// 8312C674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C678: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312C680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312C684: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8312C688: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312C68C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312C690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C698 size=92
    let mut pc: u32 = 0x8312C698;
    'dispatch: loop {
        match pc {
            0x8312C698 => {
    //   block [0x8312C698..0x8312C6F4)
	// 8312C698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C69C: 4807BACD  bl 0x831a8168
	ctx.lr = 0x8312C6A0;
	sub_831A8130(ctx, base);
	// 8312C6A0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8312C6A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C6A8: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8312C6AC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8312C6B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C6B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8312C6B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8312C6BC: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C6C0: 48002941  bl 0x8312f000
	ctx.lr = 0x8312C6C4;
	sub_8312F000(ctx, base);
	// 8312C6C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8312C6C8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312C6CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312C6D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C6D4: 4BFFF055  bl 0x8312b728
	ctx.lr = 0x8312C6D8;
	sub_8312B728(ctx, base);
	// 8312C6D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C6DC: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C6E0: 480029B9  bl 0x8312f098
	ctx.lr = 0x8312C6E4;
	sub_8312F098(ctx, base);
	// 8312C6E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312C6EC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8312C6F0: 4807BAC8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C6F8 size=288
    let mut pc: u32 = 0x8312C6F8;
    'dispatch: loop {
        match pc {
            0x8312C6F8 => {
    //   block [0x8312C6F8..0x8312C818)
	// 8312C6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C6FC: 4807BA69  bl 0x831a8164
	ctx.lr = 0x8312C700;
	sub_831A8130(ctx, base);
	// 8312C700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C704: 3F608339  lis r27, -0x7cc7
	ctx.r[27].s64 = -2093416448;
	// 8312C708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C70C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312C710: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312C714: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8312C718: 807B76A4  lwz r3, 0x76a4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C71C: 480028E5  bl 0x8312f000
	ctx.lr = 0x8312C720;
	sub_8312F000(ctx, base);
	// 8312C720: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312C724: 419A00D0  beq cr6, 0x8312c7f4
	if ctx.cr[6].eq {
	pc = 0x8312C7F4; continue 'dispatch;
	}
	// 8312C728: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312C72C: 419A00C8  beq cr6, 0x8312c7f4
	if ctx.cr[6].eq {
	pc = 0x8312C7F4; continue 'dispatch;
	}
	// 8312C730: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C734: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C738: 409A0010  bne cr6, 0x8312c748
	if !ctx.cr[6].eq {
	pc = 0x8312C748; continue 'dispatch;
	}
	// 8312C73C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C740: 386B0A30  addi r3, r11, 0xa30
	ctx.r[3].s64 = ctx.r[11].s64 + 2608;
	// 8312C744: 480000B8  b 0x8312c7fc
	pc = 0x8312C7FC; continue 'dispatch;
	// 8312C748: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8312C74C: 419A009C  beq cr6, 0x8312c7e8
	if ctx.cr[6].eq {
	pc = 0x8312C7E8; continue 'dispatch;
	}
	// 8312C750: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8312C754: 41990094  bgt cr6, 0x8312c7e8
	if ctx.cr[6].gt {
	pc = 0x8312C7E8; continue 'dispatch;
	}
	// 8312C758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312C75C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8312C760: 40990038  ble cr6, 0x8312c798
	if !ctx.cr[6].gt {
	pc = 0x8312C798; continue 'dispatch;
	}
	// 8312C764: 3D1F0002  addis r8, r31, 2
	ctx.r[8].s64 = ctx.r[31].s64 + 131072;
	// 8312C768: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 8312C76C: 39080038  addi r8, r8, 0x38
	ctx.r[8].s64 = ctx.r[8].s64 + 56;
	// 8312C770: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8312C774: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8312C778: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C77C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8312C780: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8312C784: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8312C788: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8312C78C: 4082FFEC  bne 0x8312c778
	if !ctx.cr[0].eq {
	pc = 0x8312C778; continue 'dispatch;
	}
	// 8312C790: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 8312C794: 40980034  bge cr6, 0x8312c7c8
	if !ctx.cr[6].lt {
	pc = 0x8312C7C8; continue 'dispatch;
	}
	// 8312C798: 214B0002  subfic r10, r11, 2
	ctx.xer.ca = ctx.r[11].u32 <= 2 as u32;
	ctx.r[10].s64 = (2 as i64) - ctx.r[11].s64;
	// 8312C79C: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 8312C7A0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8312C7A4: 396B800E  addi r11, r11, -0x7ff2
	ctx.r[11].s64 = ctx.r[11].s64 + -32754;
	// 8312C7A8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312C7AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312C7B0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8312C7B4: 41820014  beq 0x8312c7c8
	if ctx.cr[0].eq {
	pc = 0x8312C7C8; continue 'dispatch;
	}
	// 8312C7B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8312C7BC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8312C7C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8312C7C4: 4200FFF8  bdnz 0x8312c7bc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8312C7BC; continue 'dispatch;
	}
	// 8312C7C8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C7CC: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312C7D0: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 8312C7D4: 614A0044  ori r10, r10, 0x44
	ctx.r[10].u64 = ctx.r[10].u64 | 68;
	// 8312C7D8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312C7DC: 7FBF592E  stwx r29, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8312C7E0: 7F9F512E  stwx r28, r31, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u32) };
	// 8312C7E4: 48000020  b 0x8312c804
	pc = 0x8312C804; continue 'dispatch;
	// 8312C7E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C7EC: 386B09F8  addi r3, r11, 0x9f8
	ctx.r[3].s64 = ctx.r[11].s64 + 2552;
	// 8312C7F0: 4800000C  b 0x8312c7fc
	pc = 0x8312C7FC; continue 'dispatch;
	// 8312C7F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312C7F8: 386B09D0  addi r3, r11, 0x9d0
	ctx.r[3].s64 = ctx.r[11].s64 + 2512;
	// 8312C7FC: 4BFF9A45  bl 0x83126240
	ctx.lr = 0x8312C800;
	sub_83126240(ctx, base);
	// 8312C800: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8312C804: 807B76A4  lwz r3, 0x76a4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C808: 48002891  bl 0x8312f098
	ctx.lr = 0x8312C80C;
	sub_8312F098(ctx, base);
	// 8312C80C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312C814: 4807B9A0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C818 size=84
    let mut pc: u32 = 0x8312C818;
    'dispatch: loop {
        match pc {
            0x8312C818 => {
    //   block [0x8312C818..0x8312C86C)
	// 8312C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312C820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312C824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312C828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C82C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8312C830: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C834: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C838: 480027C9  bl 0x8312f000
	ctx.lr = 0x8312C83C;
	sub_8312F000(ctx, base);
	// 8312C83C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C840: 4BFFF119  bl 0x8312b958
	ctx.lr = 0x8312C844;
	sub_8312B958(ctx, base);
	// 8312C844: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C848: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C84C: 4800284D  bl 0x8312f098
	ctx.lr = 0x8312C850;
	sub_8312F098(ctx, base);
	// 8312C850: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C854: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312C85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312C860: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312C864: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312C868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312C870 size=84
    let mut pc: u32 = 0x8312C870;
    'dispatch: loop {
        match pc {
            0x8312C870 => {
    //   block [0x8312C870..0x8312C8C4)
	// 8312C870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312C878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312C87C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312C880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C884: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8312C888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C88C: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C890: 48002771  bl 0x8312f000
	ctx.lr = 0x8312C894;
	sub_8312F000(ctx, base);
	// 8312C894: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C898: 4BFFF319  bl 0x8312bbb0
	ctx.lr = 0x8312C89C;
	sub_8312BBB0(ctx, base);
	// 8312C89C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312C8A0: 807F76A4  lwz r3, 0x76a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312C8A4: 480027F5  bl 0x8312f098
	ctx.lr = 0x8312C8A8;
	sub_8312F098(ctx, base);
	// 8312C8A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312C8AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312C8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312C8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312C8B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312C8BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312C8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312C8C8 size=308
    let mut pc: u32 = 0x8312C8C8;
    'dispatch: loop {
        match pc {
            0x8312C8C8 => {
    //   block [0x8312C8C8..0x8312C9FC)
	// 8312C8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312C8CC: 4807B89D  bl 0x831a8168
	ctx.lr = 0x8312C8D0;
	sub_831A8130(ctx, base);
	// 8312C8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312C8D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312C8D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312C8DC: 3FBF0002  addis r29, r31, 2
	ctx.r[29].s64 = ctx.r[31].s64 + 131072;
	// 8312C8E0: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 8312C8E4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C8E8: 480AB5D9  bl 0x831d7ec0
	ctx.lr = 0x8312C8EC;
	sub_831D7EC0(ctx, base);
	// 8312C8EC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C8F0: 480AB531  bl 0x831d7e20
	ctx.lr = 0x8312C8F4;
	sub_831D7E20(ctx, base);
	// 8312C8F4: 3F9F0002  addis r28, r31, 2
	ctx.r[28].s64 = ctx.r[31].s64 + 131072;
	// 8312C8F8: 3B9C00B4  addi r28, r28, 0xb4
	ctx.r[28].s64 = ctx.r[28].s64 + 180;
	// 8312C8FC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C900: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C904: 409A00EC  bne cr6, 0x8312c9f0
	if !ctx.cr[6].eq {
	pc = 0x8312C9F0; continue 'dispatch;
	}
	// 8312C908: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C90C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312C910: 616B004C  ori r11, r11, 0x4c
	ctx.r[11].u64 = ctx.r[11].u64 | 76;
	// 8312C914: 614A000C  ori r10, r10, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 | 12;
	// 8312C918: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312C91C: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8312C920: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8312C924: 409A0024  bne cr6, 0x8312c948
	if !ctx.cr[6].eq {
	pc = 0x8312C948; continue 'dispatch;
	}
	// 8312C928: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C92C: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312C930: 616B0050  ori r11, r11, 0x50
	ctx.r[11].u64 = ctx.r[11].u64 | 80;
	// 8312C934: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 8312C938: 7C1F5C2E  lfsx f0, r31, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312C93C: 7DBF542E  lfsx f13, r31, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312C940: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8312C944: 419A000C  beq cr6, 0x8312c950
	if ctx.cr[6].eq {
	pc = 0x8312C950; continue 'dispatch;
	}
	// 8312C948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C94C: 4BFFE875  bl 0x8312b1c0
	ctx.lr = 0x8312C950;
	sub_8312B1C0(ctx, base);
	// 8312C950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312C954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C958: 4BFFE931  bl 0x8312b288
	ctx.lr = 0x8312C95C;
	sub_8312B288(ctx, base);
	// 8312C95C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C960: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312C964: 616B00B8  ori r11, r11, 0xb8
	ctx.r[11].u64 = ctx.r[11].u64 | 184;
	// 8312C968: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312C96C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C970: 409A0024  bne cr6, 0x8312c994
	if !ctx.cr[6].eq {
	pc = 0x8312C994; continue 'dispatch;
	}
	// 8312C974: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C978: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8312C97C: 616B0044  ori r11, r11, 0x44
	ctx.r[11].u64 = ctx.r[11].u64 | 68;
	// 8312C980: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312C984: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312C988: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8312C98C: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312C990: 41980060  blt cr6, 0x8312c9f0
	if ctx.cr[6].lt {
	pc = 0x8312C9F0; continue 'dispatch;
	}
	// 8312C994: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8312C998: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312C99C: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 8312C9A0: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8312C9A4: 2F2B0080  cmpdi cr6, r11, 0x80
	ctx.cr[6].compare_i64(ctx.r[11].s64, 128, &mut ctx.xer);
	// 8312C9A8: 4098000C  bge cr6, 0x8312c9b4
	if !ctx.cr[6].lt {
	pc = 0x8312C9B4; continue 'dispatch;
	}
	// 8312C9AC: 4BFFF535  bl 0x8312bee0
	ctx.lr = 0x8312C9B0;
	sub_8312BEE0(ctx, base);
	// 8312C9B0: 48000008  b 0x8312c9b8
	pc = 0x8312C9B8; continue 'dispatch;
	// 8312C9B4: 4BFFF655  bl 0x8312c008
	ctx.lr = 0x8312C9B8;
	sub_8312C008(ctx, base);
	// 8312C9B8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C9BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312C9C0: 409A0028  bne cr6, 0x8312c9e8
	if !ctx.cr[6].eq {
	pc = 0x8312C9E8; continue 'dispatch;
	}
	// 8312C9C4: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312C9C8: E95E0000  ld r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8312C9CC: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8312C9D0: 7D7F582A  ldx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 8312C9D4: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8312C9D8: 40980010  bge cr6, 0x8312c9e8
	if !ctx.cr[6].lt {
	pc = 0x8312C9E8; continue 'dispatch;
	}
	// 8312C9DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312C9E0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312C9E4: 480AB495  bl 0x831d7e78
	ctx.lr = 0x8312C9E8;
	sub_831D7E78(ctx, base);
	// 8312C9E8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8312C9EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8312C9F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312C9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312C9F8: 4807B7C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312CA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312CA00 size=356
    let mut pc: u32 = 0x8312CA00;
    'dispatch: loop {
        match pc {
            0x8312CA00 => {
    //   block [0x8312CA00..0x8312CB64)
	// 8312CA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312CA04: 4807B765  bl 0x831a8168
	ctx.lr = 0x8312CA08;
	sub_831A8130(ctx, base);
	// 8312CA08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312CA0C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8312CA10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312CA14: 4BFFE875  bl 0x8312b288
	ctx.lr = 0x8312CA18;
	sub_8312B288(ctx, base);
	// 8312CA18: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8312CA1C: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312CA20: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8312CA24: 617D0018  ori r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u64 | 24;
	// 8312CA28: 40990024  ble cr6, 0x8312ca4c
	if !ctx.cr[6].gt {
	pc = 0x8312CA4C; continue 'dispatch;
	}
	// 8312CA2C: 7D7FE82A  ldx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	// 8312CA30: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8312CA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CA38: 2F2B0080  cmpdi cr6, r11, 0x80
	ctx.cr[6].compare_i64(ctx.r[11].s64, 128, &mut ctx.xer);
	// 8312CA3C: 4098000C  bge cr6, 0x8312ca48
	if !ctx.cr[6].lt {
	pc = 0x8312CA48; continue 'dispatch;
	}
	// 8312CA40: 4BFFF4A1  bl 0x8312bee0
	ctx.lr = 0x8312CA44;
	sub_8312BEE0(ctx, base);
	// 8312CA44: 48000008  b 0x8312ca4c
	pc = 0x8312CA4C; continue 'dispatch;
	// 8312CA48: 4BFFF5C1  bl 0x8312c008
	ctx.lr = 0x8312CA4C;
	sub_8312C008(ctx, base);
	// 8312CA4C: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8312CA50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312CA54: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8312CA58: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CA5C: 480AB30D  bl 0x831d7d68
	ctx.lr = 0x8312CA60;
	sub_831D7D68(ctx, base);
	// 8312CA60: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312CA64: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312CA68: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312CA6C: 616B00B4  ori r11, r11, 0xb4
	ctx.r[11].u64 = ctx.r[11].u64 | 180;
	// 8312CA70: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312CA74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312CA78: 40820030  bne 0x8312caa8
	if !ctx.cr[0].eq {
	pc = 0x8312CAA8; continue 'dispatch;
	}
	// 8312CA7C: 409A003C  bne cr6, 0x8312cab8
	if !ctx.cr[6].eq {
	pc = 0x8312CAB8; continue 'dispatch;
	}
	// 8312CA80: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312CA84: 7D5FE82A  ldx r10, r31, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	// 8312CA88: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 8312CA8C: 7D7F582A  ldx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 8312CA90: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8312CA94: 40980024  bge cr6, 0x8312cab8
	if !ctx.cr[6].lt {
	pc = 0x8312CAB8; continue 'dispatch;
	}
	// 8312CA98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312CA9C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CAA0: 480AB3D9  bl 0x831d7e78
	ctx.lr = 0x8312CAA4;
	sub_831D7E78(ctx, base);
	// 8312CAA4: 48000014  b 0x8312cab8
	pc = 0x8312CAB8; continue 'dispatch;
	// 8312CAA8: 419A0010  beq cr6, 0x8312cab8
	if ctx.cr[6].eq {
	pc = 0x8312CAB8; continue 'dispatch;
	}
	// 8312CAAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312CAB0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CAB4: 480AB40D  bl 0x831d7ec0
	ctx.lr = 0x8312CAB8;
	sub_831D7EC0(ctx, base);
	// 8312CAB8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312CABC: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312CAC0: 616B004C  ori r11, r11, 0x4c
	ctx.r[11].u64 = ctx.r[11].u64 | 76;
	// 8312CAC4: 614A000C  ori r10, r10, 0xc
	ctx.r[10].u64 = ctx.r[10].u64 | 12;
	// 8312CAC8: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312CACC: 7D5F502E  lwzx r10, r31, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8312CAD0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8312CAD4: 409A0024  bne cr6, 0x8312caf8
	if !ctx.cr[6].eq {
	pc = 0x8312CAF8; continue 'dispatch;
	}
	// 8312CAD8: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312CADC: 3D400002  lis r10, 2
	ctx.r[10].s64 = 131072;
	// 8312CAE0: 616B0050  ori r11, r11, 0x50
	ctx.r[11].u64 = ctx.r[11].u64 | 80;
	// 8312CAE4: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 8312CAE8: 7C1F5C2E  lfsx f0, r31, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312CAEC: 7DBF542E  lfsx f13, r31, r10
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312CAF0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8312CAF4: 419A000C  beq cr6, 0x8312cb00
	if ctx.cr[6].eq {
	pc = 0x8312CB00; continue 'dispatch;
	}
	// 8312CAF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CAFC: 4BFFE6C5  bl 0x8312b1c0
	ctx.lr = 0x8312CB00;
	sub_8312B1C0(ctx, base);
	// 8312CB00: 3FDF0002  addis r30, r31, 2
	ctx.r[30].s64 = ctx.r[31].s64 + 131072;
	// 8312CB04: 3BDE00B8  addi r30, r30, 0xb8
	ctx.r[30].s64 = ctx.r[30].s64 + 184;
	// 8312CB08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CB0C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312CB10: 419A0048  beq cr6, 0x8312cb58
	if ctx.cr[6].eq {
	pc = 0x8312CB58; continue 'dispatch;
	}
	// 8312CB14: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8312CB18: 41990040  bgt cr6, 0x8312cb58
	if ctx.cr[6].gt {
	pc = 0x8312CB58; continue 'dispatch;
	}
	// 8312CB1C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8312CB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CB24: 4BFFEB55  bl 0x8312b678
	ctx.lr = 0x8312CB28;
	sub_8312B678(ctx, base);
	// 8312CB28: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8312CB2C: 7D5FE82A  ldx r10, r31, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) };
	// 8312CB30: 616B0048  ori r11, r11, 0x48
	ctx.r[11].u64 = ctx.r[11].u64 | 72;
	// 8312CB34: 7D7F5AAA  lwax r11, r31, r11
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as i32) as i64;
	// 8312CB38: 7D6B51D2  mulld r11, r11, r10
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[10].s64;
	// 8312CB3C: E9410058  ld r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8312CB40: 7F2A5800  cmpd cr6, r10, r11
	ctx.cr[6].compare_i64(ctx.r[10].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8312CB44: 41980014  blt cr6, 0x8312cb58
	if ctx.cr[6].lt {
	pc = 0x8312CB58; continue 'dispatch;
	}
	// 8312CB48: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8312CB4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8312CB50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8312CB54: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8312CB58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312CB5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8312CB60: 4807B658  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312CB68 size=168
    let mut pc: u32 = 0x8312CB68;
    'dispatch: loop {
        match pc {
            0x8312CB68 => {
    //   block [0x8312CB68..0x8312CC10)
	// 8312CB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312CB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312CB70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312CB74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312CB78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312CB7C: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8312CB80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312CB84: 807E76A4  lwz r3, 0x76a4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312CB88: 48002479  bl 0x8312f000
	ctx.lr = 0x8312CB8C;
	sub_8312F000(ctx, base);
	// 8312CB8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312CB90: 409A0018  bne cr6, 0x8312cba8
	if !ctx.cr[6].eq {
	pc = 0x8312CBA8; continue 'dispatch;
	}
	// 8312CB94: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312CB98: 386B0B60  addi r3, r11, 0xb60
	ctx.r[3].s64 = ctx.r[11].s64 + 2912;
	// 8312CB9C: 4BFF96A5  bl 0x83126240
	ctx.lr = 0x8312CBA0;
	sub_83126240(ctx, base);
	// 8312CBA0: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312CBA4: 48000048  b 0x8312cbec
	pc = 0x8312CBEC; continue 'dispatch;
	// 8312CBA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CBAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312CBB0: 409A0010  bne cr6, 0x8312cbc0
	if !ctx.cr[6].eq {
	pc = 0x8312CBC0; continue 'dispatch;
	}
	// 8312CBB4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312CBB8: 386B0B38  addi r3, r11, 0xb38
	ctx.r[3].s64 = ctx.r[11].s64 + 2872;
	// 8312CBBC: 4BFFFFE0  b 0x8312cb9c
	pc = 0x8312CB9C; continue 'dispatch;
	// 8312CBC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CBC4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312CBC8: 419A0018  beq cr6, 0x8312cbe0
	if ctx.cr[6].eq {
	pc = 0x8312CBE0; continue 'dispatch;
	}
	// 8312CBCC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8312CBD0: 409A0018  bne cr6, 0x8312cbe8
	if !ctx.cr[6].eq {
	pc = 0x8312CBE8; continue 'dispatch;
	}
	// 8312CBD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CBD8: 4BFFFE29  bl 0x8312ca00
	ctx.lr = 0x8312CBDC;
	sub_8312CA00(ctx, base);
	// 8312CBDC: 4800000C  b 0x8312cbe8
	pc = 0x8312CBE8; continue 'dispatch;
	// 8312CBE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CBE4: 4BFFFCE5  bl 0x8312c8c8
	ctx.lr = 0x8312CBE8;
	sub_8312C8C8(ctx, base);
	// 8312CBE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312CBEC: 807E76A4  lwz r3, 0x76a4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(30372 as u32) ) } as u64;
	// 8312CBF0: 480024A9  bl 0x8312f098
	ctx.lr = 0x8312CBF4;
	sub_8312F098(ctx, base);
	// 8312CBF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CBF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312CC04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312CC08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312CC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312CC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312CC10 size=172
    let mut pc: u32 = 0x8312CC10;
    'dispatch: loop {
        match pc {
            0x8312CC10 => {
    //   block [0x8312CC10..0x8312CCBC)
	// 8312CC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312CC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312CC18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312CC1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312CC20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312CC24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312CC28: 4BF9AEB9  bl 0x830c7ae0
	ctx.lr = 0x8312CC2C;
	sub_830C7AE0(ctx, base);
	// 8312CC2C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312CC30: 48001AE1  bl 0x8312e710
	ctx.lr = 0x8312CC34;
	sub_8312E710(ctx, base);
	// 8312CC34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312CC38: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312CC3C: 48001AED  bl 0x8312e728
	ctx.lr = 0x8312CC40;
	sub_8312E728(ctx, base);
	// 8312CC40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312CC44: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312CC48: 48001B01  bl 0x8312e748
	ctx.lr = 0x8312CC4C;
	sub_8312E748(ctx, base);
	// 8312CC4C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CC50: 48004691  bl 0x831312e0
	ctx.lr = 0x8312CC54;
	sub_831312E0(ctx, base);
	// 8312CC54: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8312CC58: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312CC5C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8312CC60: 409A0024  bne cr6, 0x8312cc84
	if !ctx.cr[6].eq {
	pc = 0x8312CC84; continue 'dispatch;
	}
	// 8312CC64: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8312CC68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312CC6C: 41820018  beq 0x8312cc84
	if ctx.cr[0].eq {
	pc = 0x8312CC84; continue 'dispatch;
	}
	// 8312CC70: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8312CC74: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CC78: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312CC7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312CC80: 4E800421  bctrl
	ctx.lr = 0x8312CC84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312CC84: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8312CC88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312CC8C: 41820008  beq 0x8312cc94
	if ctx.cr[0].eq {
	pc = 0x8312CC94; continue 'dispatch;
	}
	// 8312CC90: 480D7919  bl 0x832045a8
	ctx.lr = 0x8312CC94;
	sub_832045A8(ctx, base);
	// 8312CC94: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8312CC98: 9BDF0001  stb r30, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[30].u8 ) };
	// 8312CC9C: 9BDF00A8  stb r30, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u8 ) };
	// 8312CCA0: 4BF9AE41  bl 0x830c7ae0
	ctx.lr = 0x8312CCA4;
	sub_830C7AE0(ctx, base);
	// 8312CCA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312CCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312CCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312CCB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312CCB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312CCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312CCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312CCC0 size=140
    let mut pc: u32 = 0x8312CCC0;
    'dispatch: loop {
        match pc {
            0x8312CCC0 => {
    //   block [0x8312CCC0..0x8312CD4C)
	// 8312CCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312CCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312CCC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312CCCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312CCD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312CCD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312CCD8: 409A0014  bne cr6, 0x8312ccec
	if !ctx.cr[6].eq {
	pc = 0x8312CCEC; continue 'dispatch;
	}
	// 8312CCDC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312CCE0: 386B0B88  addi r3, r11, 0xb88
	ctx.r[3].s64 = ctx.r[11].s64 + 2952;
	// 8312CCE4: 48003CF5  bl 0x831309d8
	ctx.lr = 0x8312CCE8;
	sub_831309D8(ctx, base);
	// 8312CCE8: 48000050  b 0x8312cd38
	pc = 0x8312CD38; continue 'dispatch;
	// 8312CCEC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312CCF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312CCF4: 41820008  beq 0x8312ccfc
	if ctx.cr[0].eq {
	pc = 0x8312CCFC; continue 'dispatch;
	}
	// 8312CCF8: 48006469  bl 0x83133160
	ctx.lr = 0x8312CCFC;
	sub_83133160(ctx, base);
	// 8312CCFC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8312CD00: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8312CD04: 409A002C  bne cr6, 0x8312cd30
	if !ctx.cr[6].eq {
	pc = 0x8312CD30; continue 'dispatch;
	}
	// 8312CD08: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8312CD0C: 48008EB5  bl 0x83135bc0
	ctx.lr = 0x8312CD10;
	sub_83135BC0(ctx, base);
	// 8312CD10: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8312CD14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312CD18: 419A0018  beq cr6, 0x8312cd30
	if ctx.cr[6].eq {
	pc = 0x8312CD30; continue 'dispatch;
	}
	// 8312CD1C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8312CD20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CD24: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8312CD28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312CD2C: 4E800421  bctrl
	ctx.lr = 0x8312CD30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312CD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CD34: 4BFFFEDD  bl 0x8312cc10
	ctx.lr = 0x8312CD38;
	sub_8312CC10(ctx, base);
	// 8312CD38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312CD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312CD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312CD44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312CD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312CD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312CD50 size=172
    let mut pc: u32 = 0x8312CD50;
    'dispatch: loop {
        match pc {
            0x8312CD50 => {
    //   block [0x8312CD50..0x8312CDFC)
	// 8312CD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312CD54: 4807B419  bl 0x831a816c
	ctx.lr = 0x8312CD58;
	sub_831A8130(ctx, base);
	// 8312CD58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312CD5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312CD60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8312CD64: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312CD68: 897E0001  lbz r11, 1(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1 as u32) ) } as u64;
	// 8312CD6C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312CD70: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8312CD74: 419A0060  beq cr6, 0x8312cdd4
	if ctx.cr[6].eq {
	pc = 0x8312CDD4; continue 'dispatch;
	}
	// 8312CD78: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8312CD7C: 419A0058  beq cr6, 0x8312cdd4
	if ctx.cr[6].eq {
	pc = 0x8312CDD4; continue 'dispatch;
	}
	// 8312CD80: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8312CD84: 409A003C  bne cr6, 0x8312cdc0
	if !ctx.cr[6].eq {
	pc = 0x8312CDC0; continue 'dispatch;
	}
	// 8312CD88: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CD8C: 48004BF5  bl 0x83131980
	ctx.lr = 0x8312CD90;
	sub_83131980(ctx, base);
	// 8312CD90: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8312CD94: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CD98: 48004BC9  bl 0x83131960
	ctx.lr = 0x8312CD9C;
	sub_83131960(ctx, base);
	// 8312CD9C: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8312CDA0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CDA4: 48004BCD  bl 0x83131970
	ctx.lr = 0x8312CDA8;
	sub_83131970(ctx, base);
	// 8312CDA8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8312CDAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CDB0: 7D4A1BD6  divw r10, r10, r3
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 8312CDB4: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8312CDB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312CDBC: 48000028  b 0x8312cde4
	pc = 0x8312CDE4; continue 'dispatch;
	// 8312CDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312CDC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8312CDC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312CDCC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8312CDD0: 48000014  b 0x8312cde4
	pc = 0x8312CDE4; continue 'dispatch;
	// 8312CDD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312CDD8: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312CDDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312CDE0: 48001989  bl 0x8312e768
	ctx.lr = 0x8312CDE4;
	sub_8312E768(ctx, base);
	// 8312CDE4: 817E0088  lwz r11, 0x88(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 8312CDE8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312CDEC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8312CDF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312CDF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312CDF8: 4807B3C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312CE00 size=648
    let mut pc: u32 = 0x8312CE00;
    'dispatch: loop {
        match pc {
            0x8312CE00 => {
    //   block [0x8312CE00..0x8312D088)
	// 8312CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312CE04: 4807B35D  bl 0x831a8160
	ctx.lr = 0x8312CE08;
	sub_831A8130(ctx, base);
	// 8312CE08: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312CE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312CE10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312CE14: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8312CE18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312CE1C: 419A0258  beq cr6, 0x8312d074
	if ctx.cr[6].eq {
	pc = 0x8312D074; continue 'dispatch;
	}
	// 8312CE20: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312CE24: 419A0250  beq cr6, 0x8312d074
	if ctx.cr[6].eq {
	pc = 0x8312D074; continue 'dispatch;
	}
	// 8312CE28: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8312CE2C: 419A0248  beq cr6, 0x8312d074
	if ctx.cr[6].eq {
	pc = 0x8312D074; continue 'dispatch;
	}
	// 8312CE30: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312CE34: 816B76F8  lwz r11, 0x76f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30456 as u32) ) } as u64;
	// 8312CE38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312CE3C: 409A000C  bne cr6, 0x8312ce48
	if !ctx.cr[6].eq {
	pc = 0x8312CE48; continue 'dispatch;
	}
	// 8312CE40: 4BFFFF11  bl 0x8312cd50
	ctx.lr = 0x8312CE44;
	sub_8312CD50(ctx, base);
	// 8312CE44: 4800023C  b 0x8312d080
	pc = 0x8312D080; continue 'dispatch;
	// 8312CE48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8312CE4C: 3F808344  lis r28, -0x7cbc
	ctx.r[28].s64 = -2092695552;
	// 8312CE50: 3FA08344  lis r29, -0x7cbc
	ctx.r[29].s64 = -2092695552;
	// 8312CE54: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312CE58: D01C6B40  stfs f0, 0x6b40(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(27456 as u32), tmp.u32 ) };
	// 8312CE5C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8312CE60: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312CE64: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8312CE68: 419A00B8  beq cr6, 0x8312cf20
	if ctx.cr[6].eq {
	pc = 0x8312CF20; continue 'dispatch;
	}
	// 8312CE6C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8312CE70: 419A00B0  beq cr6, 0x8312cf20
	if ctx.cr[6].eq {
	pc = 0x8312CF20; continue 'dispatch;
	}
	// 8312CE74: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8312CE78: 409A009C  bne cr6, 0x8312cf14
	if !ctx.cr[6].eq {
	pc = 0x8312CF14; continue 'dispatch;
	}
	// 8312CE7C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CE80: 48004B01  bl 0x83131980
	ctx.lr = 0x8312CE84;
	sub_83131980(ctx, base);
	// 8312CE84: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8312CE88: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CE8C: 48004AD5  bl 0x83131960
	ctx.lr = 0x8312CE90;
	sub_83131960(ctx, base);
	// 8312CE90: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 8312CE94: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312CE98: 48004AD9  bl 0x83131970
	ctx.lr = 0x8312CE9C;
	sub_83131970(ctx, base);
	// 8312CE9C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 8312CEA0: 7D2B1BD6  divw r9, r11, r3
	ctx.r[9].s32 = ctx.r[11].s32 / ctx.r[3].s32;
	// 8312CEA4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312CEA8: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8312CEAC: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8312CEB0: E941005A  lwa r10, 0x58(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8312CEB4: E91D6B46  lwa r8, 0x6b44(r29)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27460 as u32) ) } as i32) as i64;
	// 8312CEB8: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8312CEBC: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8312CEC0: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8312CEC4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312CEC8: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8312CECC: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312CED0: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 8312CED4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8312CED8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312CEDC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8312CEE0: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8312CEE4: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8312CEE8: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8312CEEC: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8312CEF0: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8312CEF4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8312CEF8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8312CEFC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8312CF00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312CF04: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8312CF08: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8312CF0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312CF10: 48000008  b 0x8312cf18
	pc = 0x8312CF18; continue 'dispatch;
	// 8312CF14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312CF18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312CF1C: 4800013C  b 0x8312d058
	pc = 0x8312D058; continue 'dispatch;
	// 8312CF20: 897F0072  lbz r11, 0x72(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(114 as u32) ) } as u64;
	// 8312CF24: 3F608339  lis r27, -0x7cc7
	ctx.r[27].s64 = -2093416448;
	// 8312CF28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312CF2C: 409A0020  bne cr6, 0x8312cf4c
	if !ctx.cr[6].eq {
	pc = 0x8312CF4C; continue 'dispatch;
	}
	// 8312CF30: 813F00A0  lwz r9, 0xa0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8312CF34: 817B757C  lwz r11, 0x757c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(30076 as u32) ) } as u64;
	// 8312CF38: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8312CF3C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8312CF40: 1D6B0064  mulli r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 * 100;
	// 8312CF44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8312CF48: 48000008  b 0x8312cf50
	pc = 0x8312CF50; continue 'dispatch;
	// 8312CF4C: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8312CF50: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8312CF54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312CF58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312CF5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312CF60: 4BFFFDF1  bl 0x8312cd50
	ctx.lr = 0x8312CF64;
	sub_8312CD50(ctx, base);
	// 8312CF64: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 8312CF68: E921005A  lwa r9, 0x58(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8312CF6C: E91E0002  lwa r8, 0(r30)
	ctx.r[8].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i32) as i64;
	// 8312CF70: E97D6B46  lwa r11, 0x6b44(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27460 as u32) ) } as i32) as i64;
	// 8312CF74: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 8312CF78: F9210068  std r9, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 8312CF7C: F9010070  std r8, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u64 ) };
	// 8312CF80: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 8312CF84: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8312CF88: C8010060  lfd f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8312CF8C: C9A10068  lfd f13, 0x68(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8312CF90: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312CF94: C9810070  lfd f12, 0x70(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 8312CF98: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8312CF9C: C9610078  lfd f11, 0x78(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 8312CFA0: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8312CFA4: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8312CFA8: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312CFAC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8312CFB0: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8312CFB4: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8312CFB8: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8312CFBC: EDAC5824  fdivs f13, f12, f11
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 8312CFC0: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8312CFC4: C00BDD6C  lfs f0, -0x2294(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312CFC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8312CFCC: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8312CFD0: C1AB9584  lfs f13, -0x6a7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312CFD4: D01C6B40  stfs f0, 0x6b40(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(27456 as u32), tmp.u32 ) };
	// 8312CFD8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8312CFDC: 41990014  bgt cr6, 0x8312cff0
	if ctx.cr[6].gt {
	pc = 0x8312CFF0; continue 'dispatch;
	}
	// 8312CFE0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8312CFE4: C1AB78A8  lfs f13, 0x78a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30888 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312CFE8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8312CFEC: 4098006C  bge cr6, 0x8312d058
	if !ctx.cr[6].lt {
	pc = 0x8312D058; continue 'dispatch;
	}
	// 8312CFF0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8312CFF4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312CFF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312CFFC: 4800176D  bl 0x8312e768
	ctx.lr = 0x8312D000;
	sub_8312E768(ctx, base);
	// 8312D000: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 8312D004: E921005A  lwa r9, 0x58(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8312D008: E97D6B46  lwa r11, 0x6b44(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27460 as u32) ) } as i32) as i64;
	// 8312D00C: F9410078  std r10, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u64 ) };
	// 8312D010: F9210070  std r9, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u64 ) };
	// 8312D014: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 8312D018: C8010078  lfd f0, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 8312D01C: C9A10070  lfd f13, 0x70(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 8312D020: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312D024: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8312D028: C9810068  lfd f12, 0x68(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 8312D02C: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8312D030: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312D034: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8312D038: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8312D03C: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8312D040: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8312D044: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8312D048: 3980009C  li r12, 0x9c
	ctx.r[12].s64 = 156;
	// 8312D04C: 7C1F67AE  stfiwx f0, r31, r12
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[12].u32), tmp.u32) };
	// 8312D050: 817B757C  lwz r11, 0x757c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(30076 as u32) ) } as u64;
	// 8312D054: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8312D058: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8312D05C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D060: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8312D064: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312D068: 817D6B44  lwz r11, 0x6b44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(27460 as u32) ) } as u64;
	// 8312D06C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312D070: 48000010  b 0x8312d080
	pc = 0x8312D080; continue 'dispatch;
	// 8312D074: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D078: 386B0BD8  addi r3, r11, 0xbd8
	ctx.r[3].s64 = ctx.r[11].s64 + 3032;
	// 8312D07C: 4800395D  bl 0x831309d8
	ctx.lr = 0x8312D080;
	sub_831309D8(ctx, base);
	// 8312D080: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8312D084: 4807B12C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312D088 size=296
    let mut pc: u32 = 0x8312D088;
    'dispatch: loop {
        match pc {
            0x8312D088 => {
    //   block [0x8312D088..0x8312D1B0)
	// 8312D088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D08C: 4807B0DD  bl 0x831a8168
	ctx.lr = 0x8312D090;
	sub_831A8130(ctx, base);
	// 8312D090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D094: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8312D098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312D09C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8312D0A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8312D0A4: 409A0010  bne cr6, 0x8312d0b4
	if !ctx.cr[6].eq {
	pc = 0x8312D0B4; continue 'dispatch;
	}
	// 8312D0A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D0AC: 386B0CDC  addi r3, r11, 0xcdc
	ctx.r[3].s64 = ctx.r[11].s64 + 3292;
	// 8312D0B0: 480000F4  b 0x8312d1a4
	pc = 0x8312D1A4; continue 'dispatch;
	// 8312D0B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8312D0B8: 419A0018  beq cr6, 0x8312d0d0
	if ctx.cr[6].eq {
	pc = 0x8312D0D0; continue 'dispatch;
	}
	// 8312D0BC: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8312D0C0: 419A0010  beq cr6, 0x8312d0d0
	if ctx.cr[6].eq {
	pc = 0x8312D0D0; continue 'dispatch;
	}
	// 8312D0C4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D0C8: 386B0CB0  addi r3, r11, 0xcb0
	ctx.r[3].s64 = ctx.r[11].s64 + 3248;
	// 8312D0CC: 480000D8  b 0x8312d1a4
	pc = 0x8312D1A4; continue 'dispatch;
	// 8312D0D0: 2F1CFF80  cmpwi cr6, r28, -0x80
	ctx.cr[6].compare_i32(ctx.r[28].s32, -128, &mut ctx.xer);
	// 8312D0D4: 419A0020  beq cr6, 0x8312d0f4
	if ctx.cr[6].eq {
	pc = 0x8312D0F4; continue 'dispatch;
	}
	// 8312D0D8: 2F1CFFF1  cmpwi cr6, r28, -0xf
	ctx.cr[6].compare_i32(ctx.r[28].s32, -15, &mut ctx.xer);
	// 8312D0DC: 4098000C  bge cr6, 0x8312d0e8
	if !ctx.cr[6].lt {
	pc = 0x8312D0E8; continue 'dispatch;
	}
	// 8312D0E0: 3B80FFF1  li r28, -0xf
	ctx.r[28].s64 = -15;
	// 8312D0E4: 48000010  b 0x8312d0f4
	pc = 0x8312D0F4; continue 'dispatch;
	// 8312D0E8: 2F1C000F  cmpwi cr6, r28, 0xf
	ctx.cr[6].compare_i32(ctx.r[28].s32, 15, &mut ctx.xer);
	// 8312D0EC: 40990008  ble cr6, 0x8312d0f4
	if !ctx.cr[6].gt {
	pc = 0x8312D0F4; continue 'dispatch;
	}
	// 8312D0F0: 3B80000F  li r28, 0xf
	ctx.r[28].s64 = 15;
	// 8312D0F4: 897D00A9  lbz r11, 0xa9(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(169 as u32) ) } as u64;
	// 8312D0F8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8312D0FC: 409A001C  bne cr6, 0x8312d118
	if !ctx.cr[6].eq {
	pc = 0x8312D118; continue 'dispatch;
	}
	// 8312D100: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312D104: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D108: 48004919  bl 0x83131a20
	ctx.lr = 0x8312D10C;
	sub_83131A20(ctx, base);
	// 8312D10C: 7C7F0734  extsh r31, r3
	ctx.r[31].s64 = ctx.r[3].s16 as i64;
	// 8312D110: 2F1FFF80  cmpwi cr6, r31, -0x80
	ctx.cr[6].compare_i32(ctx.r[31].s32, -128, &mut ctx.xer);
	// 8312D114: 409A0008  bne cr6, 0x8312d11c
	if !ctx.cr[6].eq {
	pc = 0x8312D11C; continue 'dispatch;
	}
	// 8312D118: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312D11C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312D120: 816B7574  lwz r11, 0x7574(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30068 as u32) ) } as u64;
	// 8312D124: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312D128: 409A0044  bne cr6, 0x8312d16c
	if !ctx.cr[6].eq {
	pc = 0x8312D16C; continue 'dispatch;
	}
	// 8312D12C: 2F1CFF80  cmpwi cr6, r28, -0x80
	ctx.cr[6].compare_i32(ctx.r[28].s32, -128, &mut ctx.xer);
	// 8312D130: 409A0034  bne cr6, 0x8312d164
	if !ctx.cr[6].eq {
	pc = 0x8312D164; continue 'dispatch;
	}
	// 8312D134: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D138: 48004831  bl 0x83131968
	ctx.lr = 0x8312D13C;
	sub_83131968(ctx, base);
	// 8312D13C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8312D140: 409A0018  bne cr6, 0x8312d158
	if !ctx.cr[6].eq {
	pc = 0x8312D158; continue 'dispatch;
	}
	// 8312D144: 217E0000  subfic r11, r30, 0
	ctx.xer.ca = ctx.r[30].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[30].s64;
	// 8312D148: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8312D14C: 556B06FC  rlwinm r11, r11, 0, 0x1b, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8312D150: 396BFFF1  addi r11, r11, -0xf
	ctx.r[11].s64 = ctx.r[11].s64 + -15;
	// 8312D154: 48000008  b 0x8312d15c
	pc = 0x8312D15C; continue 'dispatch;
	// 8312D158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312D15C: 7CABFA14  add r5, r11, r31
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8312D160: 48000010  b 0x8312d170
	pc = 0x8312D170; continue 'dispatch;
	// 8312D164: 7CBFE214  add r5, r31, r28
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[28].u64;
	// 8312D168: 48000008  b 0x8312d170
	pc = 0x8312D170; continue 'dispatch;
	// 8312D16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8312D170: 397E0021  addi r11, r30, 0x21
	ctx.r[11].s64 = ctx.r[30].s64 + 33;
	// 8312D174: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312D178: 7F8BEB2E  sthx r28, r11, r29
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u16) };
	// 8312D17C: 897D0003  lbz r11, 3(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(3 as u32) ) } as u64;
	// 8312D180: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312D184: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312D188: 40980014  bge cr6, 0x8312d19c
	if !ctx.cr[6].lt {
	pc = 0x8312D19C; continue 'dispatch;
	}
	// 8312D18C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312D190: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D194: 480018D5  bl 0x8312ea68
	ctx.lr = 0x8312D198;
	sub_8312EA68(ctx, base);
	// 8312D198: 48000010  b 0x8312d1a8
	pc = 0x8312D1A8; continue 'dispatch;
	// 8312D19C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D1A0: 386B0C84  addi r3, r11, 0xc84
	ctx.r[3].s64 = ctx.r[11].s64 + 3204;
	// 8312D1A4: 48003835  bl 0x831309d8
	ctx.lr = 0x8312D1A8;
	sub_831309D8(ctx, base);
	// 8312D1A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312D1AC: 4807B00C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D1B0 size=276
    let mut pc: u32 = 0x8312D1B0;
    'dispatch: loop {
        match pc {
            0x8312D1B0 => {
    //   block [0x8312D1B0..0x8312D2C4)
	// 8312D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D1B4: 4807AFB5  bl 0x831a8168
	ctx.lr = 0x8312D1B8;
	sub_831A8130(ctx, base);
	// 8312D1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D1BC: 4BF9A925  bl 0x830c7ae0
	ctx.lr = 0x8312D1C0;
	sub_830C7AE0(ctx, base);
	// 8312D1C0: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 8312D1C4: 817C76FC  lwz r11, 0x76fc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(30460 as u32) ) } as u64;
	// 8312D1C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312D1CC: 419A000C  beq cr6, 0x8312d1d8
	if ctx.cr[6].eq {
	pc = 0x8312D1D8; continue 'dispatch;
	}
	// 8312D1D0: 4BF9A911  bl 0x830c7ae0
	ctx.lr = 0x8312D1D4;
	sub_830C7AE0(ctx, base);
	// 8312D1D4: 480000E8  b 0x8312d2bc
	pc = 0x8312D2BC; continue 'dispatch;
	// 8312D1D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8312D1DC: 917C76FC  stw r11, 0x76fc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8312D1E0: 4BF9A901  bl 0x830c7ae0
	ctx.lr = 0x8312D1E4;
	sub_830C7AE0(ctx, base);
	// 8312D1E4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312D1E8: 3BEB76E4  addi r31, r11, 0x76e4
	ctx.r[31].s64 = ctx.r[11].s64 + 30436;
	// 8312D1EC: 817FFFF4  lwz r11, -0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8312D1F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312D1F4: 419A0018  beq cr6, 0x8312d20c
	if ctx.cr[6].eq {
	pc = 0x8312D20C; continue 'dispatch;
	}
	// 8312D1F8: 397FFFF4  addi r11, r31, -0xc
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	// 8312D1FC: 807FFFF8  lwz r3, -8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D200: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D208: 4E800421  bctrl
	ctx.lr = 0x8312D20C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D20C: 480050BD  bl 0x831322c8
	ctx.lr = 0x8312D210;
	sub_831322C8(ctx, base);
	// 8312D210: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8312D214: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8312D218: 3BAB6B60  addi r29, r11, 0x6b60
	ctx.r[29].s64 = ctx.r[11].s64 + 27488;
	// 8312D21C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 8312D220: 915C76FC  stw r10, 0x76fc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(30460 as u32), ctx.r[10].u32 ) };
	// 8312D224: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D228: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8312D22C: 409A000C  bne cr6, 0x8312d238
	if !ctx.cr[6].eq {
	pc = 0x8312D238; continue 'dispatch;
	}
	// 8312D230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312D234: 48009DA5  bl 0x83136fd8
	ctx.lr = 0x8312D238;
	sub_83136FD8(ctx, base);
	// 8312D238: 3BDE00C4  addi r30, r30, 0xc4
	ctx.r[30].s64 = ctx.r[30].s64 + 196;
	// 8312D23C: 397D1880  addi r11, r29, 0x1880
	ctx.r[11].s64 = ctx.r[29].s64 + 6272;
	// 8312D240: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312D244: 4198FFE0  blt cr6, 0x8312d224
	if ctx.cr[6].lt {
	pc = 0x8312D224; continue 'dispatch;
	}
	// 8312D248: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8312D24C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312D254: 915C76FC  stw r10, 0x76fc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(30460 as u32), ctx.r[10].u32 ) };
	// 8312D258: 419A0018  beq cr6, 0x8312d270
	if ctx.cr[6].eq {
	pc = 0x8312D270; continue 'dispatch;
	}
	// 8312D25C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 8312D260: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312D264: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D268: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D26C: 4E800421  bctrl
	ctx.lr = 0x8312D270;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D270: 480015E1  bl 0x8312e850
	ctx.lr = 0x8312D274;
	sub_8312E850(ctx, base);
	// 8312D274: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D278: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312D27C: 419A0018  beq cr6, 0x8312d294
	if ctx.cr[6].eq {
	pc = 0x8312D294; continue 'dispatch;
	}
	// 8312D280: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 8312D284: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312D288: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D28C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D290: 4E800421  bctrl
	ctx.lr = 0x8312D294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8312D298: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8312D29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312D2A0: 915C76FC  stw r10, 0x76fc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(30460 as u32), ctx.r[10].u32 ) };
	// 8312D2A4: 419A0018  beq cr6, 0x8312d2bc
	if ctx.cr[6].eq {
	pc = 0x8312D2BC; continue 'dispatch;
	}
	// 8312D2A8: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 8312D2AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D2B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D2B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D2B8: 4E800421  bctrl
	ctx.lr = 0x8312D2BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312D2C0: 4807AEF8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D2C8 size=132
    let mut pc: u32 = 0x8312D2C8;
    'dispatch: loop {
        match pc {
            0x8312D2C8 => {
    //   block [0x8312D2C8..0x8312D34C)
	// 8312D2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D2D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D2D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D2D8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8312D2DC: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8312D2E0: 4098000C  bge cr6, 0x8312d2ec
	if !ctx.cr[6].lt {
	pc = 0x8312D2EC; continue 'dispatch;
	}
	// 8312D2E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312D2E8: 48000050  b 0x8312d338
	pc = 0x8312D338; continue 'dispatch;
	// 8312D2EC: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D2F0: 2B0B8000  cmplwi cr6, r11, 0x8000
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32768 as u32, &mut ctx.xer);
	// 8312D2F4: 409AFFF0  bne cr6, 0x8312d2e4
	if !ctx.cr[6].eq {
	pc = 0x8312D2E4; continue 'dispatch;
	}
	// 8312D2F8: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 8312D2FC: 39610068  addi r11, r1, 0x68
	ctx.r[11].s64 = ctx.r[1].s64 + 104;
	// 8312D300: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8312D304: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8312D308: 39010061  addi r8, r1, 0x61
	ctx.r[8].s64 = ctx.r[1].s64 + 97;
	// 8312D30C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 8312D310: 38E10062  addi r7, r1, 0x62
	ctx.r[7].s64 = ctx.r[1].s64 + 98;
	// 8312D314: 38C10063  addi r6, r1, 0x63
	ctx.r[6].s64 = ctx.r[1].s64 + 99;
	// 8312D318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8312D31C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 8312D320: 48008B49  bl 0x83135e68
	ctx.lr = 0x8312D324;
	sub_83135E68(ctx, base);
	// 8312D324: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312D328: 4180FFBC  blt 0x8312d2e4
	if ctx.cr[0].lt {
	pc = 0x8312D2E4; continue 'dispatch;
	}
	// 8312D32C: A9610064  lha r11, 0x64(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as i16) as i64;
	// 8312D330: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312D334: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312D338: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8312D33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312D350 size=16
    let mut pc: u32 = 0x8312D350;
    'dispatch: loop {
        match pc {
            0x8312D350 => {
    //   block [0x8312D350..0x8312D360)
	// 8312D350: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8312D354: 4098000C  bge cr6, 0x8312d360
	if !ctx.cr[6].lt {
		sub_8312D360(ctx, base);
		return;
	}
	// 8312D358: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312D35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312D360 size=24
    let mut pc: u32 = 0x8312D360;
    'dispatch: loop {
        match pc {
            0x8312D360 => {
    //   block [0x8312D360..0x8312D378)
	// 8312D360: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D364: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 8312D368: 409AFFF0  bne cr6, 0x8312d358
	if !ctx.cr[6].eq {
		sub_8312D350(ctx, base);
		return;
	}
	// 8312D36C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312D370: 90850000  stw r4, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8312D374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312D378 size=344
    let mut pc: u32 = 0x8312D378;
    'dispatch: loop {
        match pc {
            0x8312D378 => {
    //   block [0x8312D378..0x8312D4D0)
	// 8312D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D37C: 4807ADE9  bl 0x831a8164
	ctx.lr = 0x8312D380;
	sub_831A8130(ctx, base);
	// 8312D380: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D384: 83E30014  lwz r31, 0x14(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8312D388: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D38C: 4082000C  bne 0x8312d398
	if !ctx.cr[0].eq {
	pc = 0x8312D398; continue 'dispatch;
	}
	// 8312D390: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312D394: 48000134  b 0x8312d4c8
	pc = 0x8312D4C8; continue 'dispatch;
	// 8312D398: 7CAA2E70  srawi r10, r5, 5
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[5].s32 >> 5) as i64;
	// 8312D39C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D3A0: 1FC40012  mulli r30, r4, 0x12
	ctx.r[30].s64 = ctx.r[4].s64 * 18;
	// 8312D3A4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8312D3A8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8312D3AC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8312D3B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D3B4: 7F8AF1D6  mullw r28, r10, r30
	ctx.r[28].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8312D3B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D3BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8312D3C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D3C4: 4E800421  bctrl
	ctx.lr = 0x8312D3C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D3C8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8312D3CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D3D0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312D3D4: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8312D3D8: 7FABF1D6  mullw r29, r11, r30
	ctx.r[29].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8312D3DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312D3E0: 4807AE01  bl 0x831a81e0
	ctx.lr = 0x8312D3E4;
	sub_831A81E0(ctx, base);
	// 8312D3E4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8312D3E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8312D3EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312D3F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8312D3F4: 48007D9D  bl 0x83135190
	ctx.lr = 0x8312D3F8;
	sub_83135190(ctx, base);
	// 8312D3F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D3FC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8312D400: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312D404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D408: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312D40C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D410: 4E800421  bctrl
	ctx.lr = 0x8312D414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D414: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D418: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8312D41C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D424: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8312D428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D42C: 4E800421  bctrl
	ctx.lr = 0x8312D430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D430: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D434: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 8312D438: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8312D43C: 7CBDE050  subf r5, r29, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 8312D440: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D444: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8312D448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D44C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D450: 4E800421  bctrl
	ctx.lr = 0x8312D454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D454: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8312D458: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D45C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312D460: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8312D464: 7FABF1D6  mullw r29, r11, r30
	ctx.r[29].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 8312D468: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312D46C: 4807AD75  bl 0x831a81e0
	ctx.lr = 0x8312D470;
	sub_831A81E0(ctx, base);
	// 8312D470: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8312D474: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8312D478: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312D47C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8312D480: 48007D11  bl 0x83135190
	ctx.lr = 0x8312D484;
	sub_83135190(ctx, base);
	// 8312D484: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D488: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8312D48C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312D490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D494: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312D498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D49C: 4E800421  bctrl
	ctx.lr = 0x8312D4A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D4A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D4A4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8312D4A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D4AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D4B0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8312D4B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D4B8: 4E800421  bctrl
	ctx.lr = 0x8312D4BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D4BC: 7D7DDA14  add r11, r29, r27
	ctx.r[11].u64 = ctx.r[29].u64 + ctx.r[27].u64;
	// 8312D4C0: 7D6BF3D6  divw r11, r11, r30
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[30].s32;
	// 8312D4C4: 55632834  slwi r3, r11, 5
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8312D4C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8312D4CC: 4807ACE8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D4D0 size=424
    let mut pc: u32 = 0x8312D4D0;
    'dispatch: loop {
        match pc {
            0x8312D4D0 => {
    //   block [0x8312D4D0..0x8312D678)
	// 8312D4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D4D4: 4807AC95  bl 0x831a8168
	ctx.lr = 0x8312D4D8;
	sub_831A8130(ctx, base);
	// 8312D4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D4DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D4E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D4E4: 409A0014  bne cr6, 0x8312d4f8
	if !ctx.cr[6].eq {
	pc = 0x8312D4F8; continue 'dispatch;
	}
	// 8312D4E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D4EC: 386B0E68  addi r3, r11, 0xe68
	ctx.r[3].s64 = ctx.r[11].s64 + 3688;
	// 8312D4F0: 480034E9  bl 0x831309d8
	ctx.lr = 0x8312D4F4;
	sub_831309D8(ctx, base);
	// 8312D4F4: 4800017C  b 0x8312d670
	pc = 0x8312D670; continue 'dispatch;
	// 8312D4F8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312D4FC: 814B7708  lwz r10, 0x7708(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30472 as u32) ) } as u64;
	// 8312D500: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8312D504: 419A0014  beq cr6, 0x8312d518
	if ctx.cr[6].eq {
	pc = 0x8312D518; continue 'dispatch;
	}
	// 8312D508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D50C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312D510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D514: 4E800421  bctrl
	ctx.lr = 0x8312D518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D518: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D51C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8312D520: 409A000C  bne cr6, 0x8312d52c
	if !ctx.cr[6].eq {
	pc = 0x8312D52C; continue 'dispatch;
	}
	// 8312D524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D528: 4BFFF799  bl 0x8312ccc0
	ctx.lr = 0x8312D52C;
	sub_8312CCC0(ctx, base);
	// 8312D52C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D530: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8312D534: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D538: 4182000C  beq 0x8312d544
	if ctx.cr[0].eq {
	pc = 0x8312D544; continue 'dispatch;
	}
	// 8312D53C: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8312D540: 48001849  bl 0x8312ed88
	ctx.lr = 0x8312D544;
	sub_8312ED88(ctx, base);
	// 8312D544: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D548: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D54C: 4182000C  beq 0x8312d558
	if ctx.cr[0].eq {
	pc = 0x8312D558; continue 'dispatch;
	}
	// 8312D550: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8312D554: 48003CBD  bl 0x83131210
	ctx.lr = 0x8312D558;
	sub_83131210(ctx, base);
	// 8312D558: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312D55C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D560: 41820020  beq 0x8312d580
	if ctx.cr[0].eq {
	pc = 0x8312D580; continue 'dispatch;
	}
	// 8312D564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8312D568: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8312D56C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312D574: 480051FD  bl 0x83132770
	ctx.lr = 0x8312D578;
	sub_83132770(ctx, base);
	// 8312D578: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312D57C: 48005CB5  bl 0x83133230
	ctx.lr = 0x8312D580;
	sub_83133230(ctx, base);
	// 8312D580: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8312D584: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D588: 4182000C  beq 0x8312d594
	if ctx.cr[0].eq {
	pc = 0x8312D594; continue 'dispatch;
	}
	// 8312D58C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 8312D590: 480086C9  bl 0x83135c58
	ctx.lr = 0x8312D594;
	sub_83135C58(ctx, base);
	// 8312D594: 4BF9A54D  bl 0x830c7ae0
	ctx.lr = 0x8312D598;
	sub_830C7AE0(ctx, base);
	// 8312D598: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312D59C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D5A0: 41820018  beq 0x8312d5b8
	if ctx.cr[0].eq {
	pc = 0x8312D5B8; continue 'dispatch;
	}
	// 8312D5A4: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8312D5A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D5AC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D5B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D5B4: 4E800421  bctrl
	ctx.lr = 0x8312D5B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D5B8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8312D5BC: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 8312D5C0: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312D5C4: 40810080  ble 0x8312d644
	if !ctx.cr[0].gt {
	pc = 0x8312D644; continue 'dispatch;
	}
	// 8312D5C8: 3BDF0078  addi r30, r31, 0x78
	ctx.r[30].s64 = ctx.r[31].s64 + 120;
	// 8312D5CC: 807EFFA0  lwz r3, -0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-96 as u32) ) } as u64;
	// 8312D5D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D5D4: 41820018  beq 0x8312d5ec
	if ctx.cr[0].eq {
	pc = 0x8312D5EC; continue 'dispatch;
	}
	// 8312D5D8: 93BEFFA0  stw r29, -0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-96 as u32), ctx.r[29].u32 ) };
	// 8312D5DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D5E0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D5E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D5E8: 4E800421  bctrl
	ctx.lr = 0x8312D5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D5EC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D5F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D5F4: 41820018  beq 0x8312d60c
	if ctx.cr[0].eq {
	pc = 0x8312D60C; continue 'dispatch;
	}
	// 8312D5F8: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8312D5FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D600: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D608: 4E800421  bctrl
	ctx.lr = 0x8312D60C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D60C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312D610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D614: 41820018  beq 0x8312d62c
	if ctx.cr[0].eq {
	pc = 0x8312D62C; continue 'dispatch;
	}
	// 8312D618: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8312D61C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312D620: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312D628: 4E800421  bctrl
	ctx.lr = 0x8312D62C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312D62C: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8312D630: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8312D634: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8312D638: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312D63C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312D640: 4198FF8C  blt cr6, 0x8312d5cc
	if ctx.cr[6].lt {
	pc = 0x8312D5CC; continue 'dispatch;
	}
	// 8312D644: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8312D648: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312D64C: 4182000C  beq 0x8312d658
	if ctx.cr[0].eq {
	pc = 0x8312D658; continue 'dispatch;
	}
	// 8312D650: 93BF0074  stw r29, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 8312D654: 48007CFD  bl 0x83135350
	ctx.lr = 0x8312D658;
	sub_83135350(ctx, base);
	// 8312D658: 38A000C4  li r5, 0xc4
	ctx.r[5].s64 = 196;
	// 8312D65C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312D660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D664: 4807AB7D  bl 0x831a81e0
	ctx.lr = 0x8312D668;
	sub_831A81E0(ctx, base);
	// 8312D668: 9BBF0000  stb r29, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8312D66C: 4BF9A475  bl 0x830c7ae0
	ctx.lr = 0x8312D670;
	sub_830C7AE0(ctx, base);
	// 8312D670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312D674: 4807AB44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D678 size=56
    let mut pc: u32 = 0x8312D678;
    'dispatch: loop {
        match pc {
            0x8312D678 => {
    //   block [0x8312D678..0x8312D6B0)
	// 8312D678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D68C: 4BFF971D  bl 0x83126da8
	ctx.lr = 0x8312D690;
	sub_83126DA8(ctx, base);
	// 8312D690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D694: 4BFFF62D  bl 0x8312ccc0
	ctx.lr = 0x8312D698;
	sub_8312CCC0(ctx, base);
	// 8312D698: 4BFF9751  bl 0x83126de8
	ctx.lr = 0x8312D69C;
	sub_83126DE8(ctx, base);
	// 8312D69C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312D6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D6A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D6B0 size=88
    let mut pc: u32 = 0x8312D6B0;
    'dispatch: loop {
        match pc {
            0x8312D6B0 => {
    //   block [0x8312D6B0..0x8312D708)
	// 8312D6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D6B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D6BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D6C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D6C4: 4BFF96E5  bl 0x83126da8
	ctx.lr = 0x8312D6C8;
	sub_83126DA8(ctx, base);
	// 8312D6C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D6CC: 409A0018  bne cr6, 0x8312d6e4
	if !ctx.cr[6].eq {
	pc = 0x8312D6E4; continue 'dispatch;
	}
	// 8312D6D0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D6D4: 386B0BB0  addi r3, r11, 0xbb0
	ctx.r[3].s64 = ctx.r[11].s64 + 2992;
	// 8312D6D8: 48003301  bl 0x831309d8
	ctx.lr = 0x8312D6DC;
	sub_831309D8(ctx, base);
	// 8312D6DC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312D6E0: 4800000C  b 0x8312d6ec
	pc = 0x8312D6EC; continue 'dispatch;
	// 8312D6E4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8312D6E8: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 8312D6EC: 4BFF96FD  bl 0x83126de8
	ctx.lr = 0x8312D6F0;
	sub_83126DE8(ctx, base);
	// 8312D6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D6F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312D6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D708 size=56
    let mut pc: u32 = 0x8312D708;
    'dispatch: loop {
        match pc {
            0x8312D708 => {
    //   block [0x8312D708..0x8312D740)
	// 8312D708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D70C: 4807AA61  bl 0x831a816c
	ctx.lr = 0x8312D710;
	sub_831A8130(ctx, base);
	// 8312D710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D718: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312D71C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312D720: 4BFF9689  bl 0x83126da8
	ctx.lr = 0x8312D724;
	sub_83126DA8(ctx, base);
	// 8312D724: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312D728: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312D72C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D730: 4BFFF6D1  bl 0x8312ce00
	ctx.lr = 0x8312D734;
	sub_8312CE00(ctx, base);
	// 8312D734: 4BFF96B5  bl 0x83126de8
	ctx.lr = 0x8312D738;
	sub_83126DE8(ctx, base);
	// 8312D738: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312D73C: 4807AA80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D740 size=116
    let mut pc: u32 = 0x8312D740;
    'dispatch: loop {
        match pc {
            0x8312D740 => {
    //   block [0x8312D740..0x8312D7B4)
	// 8312D740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D74C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D754: 4BFF9655  bl 0x83126da8
	ctx.lr = 0x8312D758;
	sub_83126DA8(ctx, base);
	// 8312D758: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D75C: 409A0018  bne cr6, 0x8312d774
	if !ctx.cr[6].eq {
	pc = 0x8312D774; continue 'dispatch;
	}
	// 8312D760: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D764: 386B0C00  addi r3, r11, 0xc00
	ctx.r[3].s64 = ctx.r[11].s64 + 3072;
	// 8312D768: 48003271  bl 0x831309d8
	ctx.lr = 0x8312D76C;
	sub_831309D8(ctx, base);
	// 8312D76C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312D770: 48000028  b 0x8312d798
	pc = 0x8312D798; continue 'dispatch;
	// 8312D774: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8312D778: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312D77C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8312D780: 41980014  blt cr6, 0x8312d794
	if ctx.cr[6].lt {
	pc = 0x8312D794; continue 'dispatch;
	}
	// 8312D784: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D788: 480041F9  bl 0x83131980
	ctx.lr = 0x8312D78C;
	sub_83131980(ctx, base);
	// 8312D78C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D790: 48000008  b 0x8312d798
	pc = 0x8312D798; continue 'dispatch;
	// 8312D794: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312D798: 4BFF9651  bl 0x83126de8
	ctx.lr = 0x8312D79C;
	sub_83126DE8(ctx, base);
	// 8312D79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D7A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312D7A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D7A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D7AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D7B8 size=116
    let mut pc: u32 = 0x8312D7B8;
    'dispatch: loop {
        match pc {
            0x8312D7B8 => {
    //   block [0x8312D7B8..0x8312D82C)
	// 8312D7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D7C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D7C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D7C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D7CC: 4BFF95DD  bl 0x83126da8
	ctx.lr = 0x8312D7D0;
	sub_83126DA8(ctx, base);
	// 8312D7D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D7D4: 409A0018  bne cr6, 0x8312d7ec
	if !ctx.cr[6].eq {
	pc = 0x8312D7EC; continue 'dispatch;
	}
	// 8312D7D8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D7DC: 386B0C2C  addi r3, r11, 0xc2c
	ctx.r[3].s64 = ctx.r[11].s64 + 3116;
	// 8312D7E0: 480031F9  bl 0x831309d8
	ctx.lr = 0x8312D7E4;
	sub_831309D8(ctx, base);
	// 8312D7E4: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312D7E8: 48000028  b 0x8312d810
	pc = 0x8312D810; continue 'dispatch;
	// 8312D7EC: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8312D7F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312D7F4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8312D7F8: 41980014  blt cr6, 0x8312d80c
	if ctx.cr[6].lt {
	pc = 0x8312D80C; continue 'dispatch;
	}
	// 8312D7FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D800: 48004161  bl 0x83131960
	ctx.lr = 0x8312D804;
	sub_83131960(ctx, base);
	// 8312D804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D808: 48000008  b 0x8312d810
	pc = 0x8312D810; continue 'dispatch;
	// 8312D80C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312D810: 4BFF95D9  bl 0x83126de8
	ctx.lr = 0x8312D814;
	sub_83126DE8(ctx, base);
	// 8312D814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312D81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D830 size=116
    let mut pc: u32 = 0x8312D830;
    'dispatch: loop {
        match pc {
            0x8312D830 => {
    //   block [0x8312D830..0x8312D8A4)
	// 8312D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D83C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D844: 4BFF9565  bl 0x83126da8
	ctx.lr = 0x8312D848;
	sub_83126DA8(ctx, base);
	// 8312D848: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D84C: 409A0018  bne cr6, 0x8312d864
	if !ctx.cr[6].eq {
	pc = 0x8312D864; continue 'dispatch;
	}
	// 8312D850: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D854: 386B0C58  addi r3, r11, 0xc58
	ctx.r[3].s64 = ctx.r[11].s64 + 3160;
	// 8312D858: 48003181  bl 0x831309d8
	ctx.lr = 0x8312D85C;
	sub_831309D8(ctx, base);
	// 8312D85C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312D860: 48000028  b 0x8312d888
	pc = 0x8312D888; continue 'dispatch;
	// 8312D864: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8312D868: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312D86C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8312D870: 41980014  blt cr6, 0x8312d884
	if ctx.cr[6].lt {
	pc = 0x8312D884; continue 'dispatch;
	}
	// 8312D874: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D878: 480040F1  bl 0x83131968
	ctx.lr = 0x8312D87C;
	sub_83131968(ctx, base);
	// 8312D87C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D880: 48000008  b 0x8312d888
	pc = 0x8312D888; continue 'dispatch;
	// 8312D884: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312D888: 4BFF9561  bl 0x83126de8
	ctx.lr = 0x8312D88C;
	sub_83126DE8(ctx, base);
	// 8312D88C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D89C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D8A8 size=56
    let mut pc: u32 = 0x8312D8A8;
    'dispatch: loop {
        match pc {
            0x8312D8A8 => {
    //   block [0x8312D8A8..0x8312D8E0)
	// 8312D8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D8AC: 4807A8C1  bl 0x831a816c
	ctx.lr = 0x8312D8B0;
	sub_831A8130(ctx, base);
	// 8312D8B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D8B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312D8BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312D8C0: 4BFF94E9  bl 0x83126da8
	ctx.lr = 0x8312D8C4;
	sub_83126DA8(ctx, base);
	// 8312D8C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312D8C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312D8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D8D0: 4BFFF7B9  bl 0x8312d088
	ctx.lr = 0x8312D8D4;
	sub_8312D088(ctx, base);
	// 8312D8D4: 4BFF9515  bl 0x83126de8
	ctx.lr = 0x8312D8D8;
	sub_83126DE8(ctx, base);
	// 8312D8D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312D8DC: 4807A8E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D8E0 size=100
    let mut pc: u32 = 0x8312D8E0;
    'dispatch: loop {
        match pc {
            0x8312D8E0 => {
    //   block [0x8312D8E0..0x8312D944)
	// 8312D8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D8E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312D8EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D8F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D8F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312D8FC: 4BFF94AD  bl 0x83126da8
	ctx.lr = 0x8312D900;
	sub_83126DA8(ctx, base);
	// 8312D900: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D904: 409A0014  bne cr6, 0x8312d918
	if !ctx.cr[6].eq {
	pc = 0x8312D918; continue 'dispatch;
	}
	// 8312D908: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D90C: 386B0D10  addi r3, r11, 0xd10
	ctx.r[3].s64 = ctx.r[11].s64 + 3344;
	// 8312D910: 480030C9  bl 0x831309d8
	ctx.lr = 0x8312D914;
	sub_831309D8(ctx, base);
	// 8312D914: 48000010  b 0x8312d924
	pc = 0x8312D924; continue 'dispatch;
	// 8312D918: 397E0021  addi r11, r30, 0x21
	ctx.r[11].s64 = ctx.r[30].s64 + 33;
	// 8312D91C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312D920: 7FEBFAAE  lhax r31, r11, r31
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as i16) as i64;
	// 8312D924: 4BFF94C5  bl 0x83126de8
	ctx.lr = 0x8312D928;
	sub_83126DE8(ctx, base);
	// 8312D928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312D92C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312D930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D938: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312D93C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D948 size=136
    let mut pc: u32 = 0x8312D948;
    'dispatch: loop {
        match pc {
            0x8312D948 => {
    //   block [0x8312D948..0x8312D9D0)
	// 8312D948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312D954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D960: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312D964: 4BFF9445  bl 0x83126da8
	ctx.lr = 0x8312D968;
	sub_83126DA8(ctx, base);
	// 8312D968: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D96C: 409A0014  bne cr6, 0x8312d980
	if !ctx.cr[6].eq {
	pc = 0x8312D980; continue 'dispatch;
	}
	// 8312D970: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D974: 386B0D3C  addi r3, r11, 0xd3c
	ctx.r[3].s64 = ctx.r[11].s64 + 3388;
	// 8312D978: 48003061  bl 0x831309d8
	ctx.lr = 0x8312D97C;
	sub_831309D8(ctx, base);
	// 8312D97C: 48000038  b 0x8312d9b4
	pc = 0x8312D9B4; continue 'dispatch;
	// 8312D980: 895F00A9  lbz r10, 0xa9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(169 as u32) ) } as u64;
	// 8312D984: B3DF0040  sth r30, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u16 ) };
	// 8312D988: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8312D98C: 409A0014  bne cr6, 0x8312d9a0
	if !ctx.cr[6].eq {
	pc = 0x8312D9A0; continue 'dispatch;
	}
	// 8312D990: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312D994: 4800402D  bl 0x831319c0
	ctx.lr = 0x8312D998;
	sub_831319C0(ctx, base);
	// 8312D998: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8312D99C: 48000008  b 0x8312d9a4
	pc = 0x8312D9A4; continue 'dispatch;
	// 8312D9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312D9A4: A95F0040  lha r10, 0x40(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as i16) as i64;
	// 8312D9A8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312D9AC: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8312D9B0: 48001051  bl 0x8312ea00
	ctx.lr = 0x8312D9B4;
	sub_8312EA00(ctx, base);
	// 8312D9B4: 4BFF9435  bl 0x83126de8
	ctx.lr = 0x8312D9B8;
	sub_83126DE8(ctx, base);
	// 8312D9B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312D9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312D9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312D9C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312D9C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312D9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312D9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312D9D0 size=80
    let mut pc: u32 = 0x8312D9D0;
    'dispatch: loop {
        match pc {
            0x8312D9D0 => {
    //   block [0x8312D9D0..0x8312DA20)
	// 8312D9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312D9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312D9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312D9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312D9E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312D9E4: 4BFF93C5  bl 0x83126da8
	ctx.lr = 0x8312D9E8;
	sub_83126DA8(ctx, base);
	// 8312D9E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312D9EC: 409A0014  bne cr6, 0x8312da00
	if !ctx.cr[6].eq {
	pc = 0x8312DA00; continue 'dispatch;
	}
	// 8312D9F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312D9F4: 386B0D68  addi r3, r11, 0xd68
	ctx.r[3].s64 = ctx.r[11].s64 + 3432;
	// 8312D9F8: 48002FE1  bl 0x831309d8
	ctx.lr = 0x8312D9FC;
	sub_831309D8(ctx, base);
	// 8312D9FC: 48000008  b 0x8312da04
	pc = 0x8312DA04; continue 'dispatch;
	// 8312DA00: ABFF0040  lha r31, 0x40(r31)
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as i16) as i64;
	// 8312DA04: 4BFF93E5  bl 0x83126de8
	ctx.lr = 0x8312DA08;
	sub_83126DE8(ctx, base);
	// 8312DA08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312DA0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312DA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DA18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DA20 size=64
    let mut pc: u32 = 0x8312DA20;
    'dispatch: loop {
        match pc {
            0x8312DA20 => {
    //   block [0x8312DA20..0x8312DA60)
	// 8312DA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DA28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DA2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DA30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DA34: 4BFF9375  bl 0x83126da8
	ctx.lr = 0x8312DA38;
	sub_83126DA8(ctx, base);
	// 8312DA38: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312DA3C: 93EB7700  stw r31, 0x7700(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(30464 as u32), ctx.r[31].u32 ) };
	// 8312DA40: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312DA44: 93EB7704  stw r31, 0x7704(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(30468 as u32), ctx.r[31].u32 ) };
	// 8312DA48: 4BFF93A1  bl 0x83126de8
	ctx.lr = 0x8312DA4C;
	sub_83126DE8(ctx, base);
	// 8312DA4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312DA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DA58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DA60 size=96
    let mut pc: u32 = 0x8312DA60;
    'dispatch: loop {
        match pc {
            0x8312DA60 => {
    //   block [0x8312DA60..0x8312DAC0)
	// 8312DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DA68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312DA6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DA70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DA74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312DA78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8312DA7C: 4BFF932D  bl 0x83126da8
	ctx.lr = 0x8312DA80;
	sub_83126DA8(ctx, base);
	// 8312DA80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312DA84: 409A0014  bne cr6, 0x8312da98
	if !ctx.cr[6].eq {
	pc = 0x8312DA98; continue 'dispatch;
	}
	// 8312DA88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DA8C: 386B0D94  addi r3, r11, 0xd94
	ctx.r[3].s64 = ctx.r[11].s64 + 3476;
	// 8312DA90: 48002F49  bl 0x831309d8
	ctx.lr = 0x8312DA94;
	sub_831309D8(ctx, base);
	// 8312DA94: 48000010  b 0x8312daa4
	pc = 0x8312DAA4; continue 'dispatch;
	// 8312DA98: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312DA9C: 93FE0038  stw r31, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 8312DAA0: 93EB7704  stw r31, 0x7704(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(30468 as u32), ctx.r[31].u32 ) };
	// 8312DAA4: 4BFF9345  bl 0x83126de8
	ctx.lr = 0x8312DAA8;
	sub_83126DE8(ctx, base);
	// 8312DAA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DAB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312DAB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DAC0 size=64
    let mut pc: u32 = 0x8312DAC0;
    'dispatch: loop {
        match pc {
            0x8312DAC0 => {
    //   block [0x8312DAC0..0x8312DB00)
	// 8312DAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DAC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312DACC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DAD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DAD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DAD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DADC: 4BFF92CD  bl 0x83126da8
	ctx.lr = 0x8312DAE0;
	sub_83126DA8(ctx, base);
	// 8312DAE0: 9BDF006D  stb r30, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[30].u8 ) };
	// 8312DAE4: 4BFF9305  bl 0x83126de8
	ctx.lr = 0x8312DAE8;
	sub_83126DE8(ctx, base);
	// 8312DAE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DAF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312DAF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DB00 size=40
    let mut pc: u32 = 0x8312DB00;
    'dispatch: loop {
        match pc {
            0x8312DB00 => {
    //   block [0x8312DB00..0x8312DB28)
	// 8312DB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DB0C: 4BFF929D  bl 0x83126da8
	ctx.lr = 0x8312DB10;
	sub_83126DA8(ctx, base);
	// 8312DB10: 4BFFF6A1  bl 0x8312d1b0
	ctx.lr = 0x8312DB14;
	sub_8312D1B0(ctx, base);
	// 8312DB14: 4BFF92D5  bl 0x83126de8
	ctx.lr = 0x8312DB18;
	sub_83126DE8(ctx, base);
	// 8312DB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312DB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DB28 size=84
    let mut pc: u32 = 0x8312DB28;
    'dispatch: loop {
        match pc {
            0x8312DB28 => {
    //   block [0x8312DB28..0x8312DB7C)
	// 8312DB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DB30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DB34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DB38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DB3C: 4BFF926D  bl 0x83126da8
	ctx.lr = 0x8312DB40;
	sub_83126DA8(ctx, base);
	// 8312DB40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312DB44: 409A0018  bne cr6, 0x8312db5c
	if !ctx.cr[6].eq {
	pc = 0x8312DB5C; continue 'dispatch;
	}
	// 8312DB48: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DB4C: 386B0DC0  addi r3, r11, 0xdc0
	ctx.r[3].s64 = ctx.r[11].s64 + 3520;
	// 8312DB50: 48002E89  bl 0x831309d8
	ctx.lr = 0x8312DB54;
	sub_831309D8(ctx, base);
	// 8312DB54: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312DB58: 48000008  b 0x8312db60
	pc = 0x8312DB60; continue 'dispatch;
	// 8312DB5C: ABFF0060  lha r31, 0x60(r31)
	ctx.r[31].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as i16) as i64;
	// 8312DB60: 4BFF9289  bl 0x83126de8
	ctx.lr = 0x8312DB64;
	sub_83126DE8(ctx, base);
	// 8312DB64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312DB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DB74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312DB80 size=244
    let mut pc: u32 = 0x8312DB80;
    'dispatch: loop {
        match pc {
            0x8312DB80 => {
    //   block [0x8312DB80..0x8312DC74)
	// 8312DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DB84: 4807A5E5  bl 0x831a8168
	ctx.lr = 0x8312DB88;
	sub_831A8130(ctx, base);
	// 8312DB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DB8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DB90: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8312DB94: 4BFF9215  bl 0x83126da8
	ctx.lr = 0x8312DB98;
	sub_83126DA8(ctx, base);
	// 8312DB98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312DB9C: 409A0014  bne cr6, 0x8312dbb0
	if !ctx.cr[6].eq {
	pc = 0x8312DBB0; continue 'dispatch;
	}
	// 8312DBA0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DBA4: 386B0DEC  addi r3, r11, 0xdec
	ctx.r[3].s64 = ctx.r[11].s64 + 3564;
	// 8312DBA8: 48002E31  bl 0x831309d8
	ctx.lr = 0x8312DBAC;
	sub_831309D8(ctx, base);
	// 8312DBAC: 480000BC  b 0x8312dc68
	pc = 0x8312DC68; continue 'dispatch;
	// 8312DBB0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8312DBB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312DBB8: 4082000C  bne 0x8312dbc4
	if !ctx.cr[0].eq {
	pc = 0x8312DBC4; continue 'dispatch;
	}
	// 8312DBBC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312DBC0: 4800001C  b 0x8312dbdc
	pc = 0x8312DBDC; continue 'dispatch;
	// 8312DBC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312DBC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312DBCC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8312DBD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312DBD4: 4E800421  bctrl
	ctx.lr = 0x8312DBD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312DBD8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312DBDC: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8312DBE0: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8312DBE4: 419A0080  beq cr6, 0x8312dc64
	if ctx.cr[6].eq {
	pc = 0x8312DC64; continue 'dispatch;
	}
	// 8312DBE8: 897F006C  lbz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8312DBEC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8312DBF0: 409A0074  bne cr6, 0x8312dc64
	if !ctx.cr[6].eq {
	pc = 0x8312DC64; continue 'dispatch;
	}
	// 8312DBF4: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8312DBF8: 409A006C  bne cr6, 0x8312dc64
	if !ctx.cr[6].eq {
	pc = 0x8312DC64; continue 'dispatch;
	}
	// 8312DBFC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312DC00: 4BEA6799  bl 0x82fd4398
	ctx.lr = 0x8312DC04;
	sub_82FD4398(ctx, base);
	// 8312DC04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8312DC08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312DC0C: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8312DC10: 48003D89  bl 0x83131998
	ctx.lr = 0x8312DC14;
	sub_83131998(ctx, base);
	// 8312DC14: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8312DC18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312DC1C: 396B07FF  addi r11, r11, 0x7ff
	ctx.r[11].s64 = ctx.r[11].s64 + 2047;
	// 8312DC20: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8312DC24: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8312DC28: 557E5828  slwi r30, r11, 0xb
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8312DC2C: 48003D8D  bl 0x831319b8
	ctx.lr = 0x8312DC30;
	sub_831319B8(ctx, base);
	// 8312DC30: 396307FF  addi r11, r3, 0x7ff
	ctx.r[11].s64 = ctx.r[3].s64 + 2047;
	// 8312DC34: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8312DC38: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8312DC3C: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312DC40: 7D7E5851  subf. r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312DC44: 4181000C  bgt 0x8312dc50
	if ctx.cr[0].gt {
	pc = 0x8312DC50; continue 'dispatch;
	}
	// 8312DC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312DC4C: 48000014  b 0x8312dc60
	pc = 0x8312DC60; continue 'dispatch;
	// 8312DC50: 7D5EE850  subf r10, r30, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8312DC54: 7D4A5BD6  divw r10, r10, r11
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[11].s32;
	// 8312DC58: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 8312DC5C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8312DC60: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8312DC64: 9B9F006C  stb r28, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[28].u8 ) };
	// 8312DC68: 4BFF9181  bl 0x83126de8
	ctx.lr = 0x8312DC6C;
	sub_83126DE8(ctx, base);
	// 8312DC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312DC70: 4807A548  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DC78 size=72
    let mut pc: u32 = 0x8312DC78;
    'dispatch: loop {
        match pc {
            0x8312DC78 => {
    //   block [0x8312DC78..0x8312DCC0)
	// 8312DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312DC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DC8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DC90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DC94: 4BFF9115  bl 0x83126da8
	ctx.lr = 0x8312DC98;
	sub_83126DA8(ctx, base);
	// 8312DC98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312DC9C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312DCA0: 48000F29  bl 0x8312ebc8
	ctx.lr = 0x8312DCA4;
	sub_8312EBC8(ctx, base);
	// 8312DCA4: 4BFF9145  bl 0x83126de8
	ctx.lr = 0x8312DCA8;
	sub_83126DE8(ctx, base);
	// 8312DCA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DCB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312DCB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DCC0 size=80
    let mut pc: u32 = 0x8312DCC0;
    'dispatch: loop {
        match pc {
            0x8312DCC0 => {
    //   block [0x8312DCC0..0x8312DD10)
	// 8312DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DCC4: 4807A4A9  bl 0x831a816c
	ctx.lr = 0x8312DCC8;
	sub_831A8130(ctx, base);
	// 8312DCC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DCCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DCD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DCD4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312DCD8: 4BFF90D1  bl 0x83126da8
	ctx.lr = 0x8312DCDC;
	sub_83126DA8(ctx, base);
	// 8312DCDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312DCE0: 409A0014  bne cr6, 0x8312dcf4
	if !ctx.cr[6].eq {
	pc = 0x8312DCF4; continue 'dispatch;
	}
	// 8312DCE4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DCE8: 386B0E18  addi r3, r11, 0xe18
	ctx.r[3].s64 = ctx.r[11].s64 + 3608;
	// 8312DCEC: 48002CED  bl 0x831309d8
	ctx.lr = 0x8312DCF0;
	sub_831309D8(ctx, base);
	// 8312DCF0: 48000014  b 0x8312dd04
	pc = 0x8312DD04; continue 'dispatch;
	// 8312DCF4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312DCF8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312DCFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312DD00: 48000BE1  bl 0x8312e8e0
	ctx.lr = 0x8312DD04;
	sub_8312E8E0(ctx, base);
	// 8312DD04: 4BFF90E5  bl 0x83126de8
	ctx.lr = 0x8312DD08;
	sub_83126DE8(ctx, base);
	// 8312DD08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DD0C: 4807A4B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DD10 size=80
    let mut pc: u32 = 0x8312DD10;
    'dispatch: loop {
        match pc {
            0x8312DD10 => {
    //   block [0x8312DD10..0x8312DD60)
	// 8312DD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DD14: 4807A459  bl 0x831a816c
	ctx.lr = 0x8312DD18;
	sub_831A8130(ctx, base);
	// 8312DD18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DD1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DD20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DD24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312DD28: 4BFF9081  bl 0x83126da8
	ctx.lr = 0x8312DD2C;
	sub_83126DA8(ctx, base);
	// 8312DD2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312DD30: 409A0014  bne cr6, 0x8312dd44
	if !ctx.cr[6].eq {
	pc = 0x8312DD44; continue 'dispatch;
	}
	// 8312DD34: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DD38: 386B0E40  addi r3, r11, 0xe40
	ctx.r[3].s64 = ctx.r[11].s64 + 3648;
	// 8312DD3C: 48002C9D  bl 0x831309d8
	ctx.lr = 0x8312DD40;
	sub_831309D8(ctx, base);
	// 8312DD40: 48000014  b 0x8312dd54
	pc = 0x8312DD54; continue 'dispatch;
	// 8312DD44: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312DD48: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312DD4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312DD50: 48000C01  bl 0x8312e950
	ctx.lr = 0x8312DD54;
	sub_8312E950(ctx, base);
	// 8312DD54: 4BFF9095  bl 0x83126de8
	ctx.lr = 0x8312DD58;
	sub_83126DE8(ctx, base);
	// 8312DD58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DD5C: 4807A460  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DD60 size=56
    let mut pc: u32 = 0x8312DD60;
    'dispatch: loop {
        match pc {
            0x8312DD60 => {
    //   block [0x8312DD60..0x8312DD98)
	// 8312DD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DD68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DD6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DD70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DD74: 4BFF9035  bl 0x83126da8
	ctx.lr = 0x8312DD78;
	sub_83126DA8(ctx, base);
	// 8312DD78: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312DD7C: 48003515  bl 0x83131290
	ctx.lr = 0x8312DD80;
	sub_83131290(ctx, base);
	// 8312DD80: 4BFF9069  bl 0x83126de8
	ctx.lr = 0x8312DD84;
	sub_83126DE8(ctx, base);
	// 8312DD84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312DD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DD90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DD98 size=64
    let mut pc: u32 = 0x8312DD98;
    'dispatch: loop {
        match pc {
            0x8312DD98 => {
    //   block [0x8312DD98..0x8312DDD8)
	// 8312DD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DDA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312DDA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DDAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DDB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DDB4: 4BFF8FF5  bl 0x83126da8
	ctx.lr = 0x8312DDB8;
	sub_83126DA8(ctx, base);
	// 8312DDB8: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8312DDBC: 4BFF902D  bl 0x83126de8
	ctx.lr = 0x8312DDC0;
	sub_83126DE8(ctx, base);
	// 8312DDC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DDCC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312DDD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DDD8 size=84
    let mut pc: u32 = 0x8312DDD8;
    'dispatch: loop {
        match pc {
            0x8312DDD8 => {
    //   block [0x8312DDD8..0x8312DE2C)
	// 8312DDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312DDE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312DDE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312DDE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DDEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DDF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DDF4: 4BFF8FB5  bl 0x83126da8
	ctx.lr = 0x8312DDF8;
	sub_83126DA8(ctx, base);
	// 8312DDF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312DDFC: 9BDF0098  stb r30, 0x98(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u8 ) };
	// 8312DE00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312DE04: 4182000C  beq 0x8312de10
	if ctx.cr[0].eq {
	pc = 0x8312DE10; continue 'dispatch;
	}
	// 8312DE08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312DE0C: 48003B15  bl 0x83131920
	ctx.lr = 0x8312DE10;
	sub_83131920(ctx, base);
	// 8312DE10: 4BFF8FD9  bl 0x83126de8
	ctx.lr = 0x8312DE14;
	sub_83126DE8(ctx, base);
	// 8312DE14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312DE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312DE20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312DE24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312DE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DE30 size=56
    let mut pc: u32 = 0x8312DE30;
    'dispatch: loop {
        match pc {
            0x8312DE30 => {
    //   block [0x8312DE30..0x8312DE68)
	// 8312DE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DE34: 4807A339  bl 0x831a816c
	ctx.lr = 0x8312DE38;
	sub_831A8130(ctx, base);
	// 8312DE38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DE3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DE40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DE44: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312DE48: 4BFF8F61  bl 0x83126da8
	ctx.lr = 0x8312DE4C;
	sub_83126DA8(ctx, base);
	// 8312DE4C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312DE50: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312DE54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312DE58: 48003AD1  bl 0x83131928
	ctx.lr = 0x8312DE5C;
	sub_83131928(ctx, base);
	// 8312DE5C: 4BFF8F8D  bl 0x83126de8
	ctx.lr = 0x8312DE60;
	sub_83126DE8(ctx, base);
	// 8312DE60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DE64: 4807A358  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312DE68 size=64
    let mut pc: u32 = 0x8312DE68;
    'dispatch: loop {
        match pc {
            0x8312DE68 => {
    //   block [0x8312DE68..0x8312DEA8)
	// 8312DE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DE6C: 4807A301  bl 0x831a816c
	ctx.lr = 0x8312DE70;
	sub_831A8130(ctx, base);
	// 8312DE70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DE74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DE78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312DE7C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312DE80: 4BFF8F29  bl 0x83126da8
	ctx.lr = 0x8312DE84;
	sub_83126DA8(ctx, base);
	// 8312DE84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312DE88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312DE8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312DE90: 4BFFF4E9  bl 0x8312d378
	ctx.lr = 0x8312DE94;
	sub_8312D378(ctx, base);
	// 8312DE94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312DE98: 4BFF8F51  bl 0x83126de8
	ctx.lr = 0x8312DE9C;
	sub_83126DE8(ctx, base);
	// 8312DE9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312DEA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312DEA4: 4807A318  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312DEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312DEA8 size=720
    let mut pc: u32 = 0x8312DEA8;
    'dispatch: loop {
        match pc {
            0x8312DEA8 => {
    //   block [0x8312DEA8..0x8312E178)
	// 8312DEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312DEAC: 4807A2B9  bl 0x831a8164
	ctx.lr = 0x8312DEB0;
	sub_831A8130(ctx, base);
	// 8312DEB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312DEB4: 3964003F  addi r11, r4, 0x3f
	ctx.r[11].s64 = ctx.r[4].s64 + 63;
	// 8312DEB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312DEBC: 557D0032  rlwinm r29, r11, 0, 0, 0x19
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8312DEC0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8312DEC4: 7D7D2050  subf r11, r29, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[29].s64;
	// 8312DEC8: 7F8B2A14  add r28, r11, r5
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 8312DECC: 41980294  blt cr6, 0x8312e160
	if ctx.cr[6].lt {
	pc = 0x8312E160; continue 'dispatch;
	}
	// 8312DED0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8312DED4: 419A028C  beq cr6, 0x8312e160
	if ctx.cr[6].eq {
	pc = 0x8312E160; continue 'dispatch;
	}
	// 8312DED8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8312DEDC: 41980284  blt cr6, 0x8312e160
	if ctx.cr[6].lt {
	pc = 0x8312E160; continue 'dispatch;
	}
	// 8312DEE0: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8312DEE4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8312DEE8: 394B6B60  addi r10, r11, 0x6b60
	ctx.r[10].s64 = ctx.r[11].s64 + 27488;
	// 8312DEEC: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8312DEF0: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8312DEF4: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312DEF8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8312DEFC: 419A0018  beq cr6, 0x8312df14
	if ctx.cr[6].eq {
	pc = 0x8312DF14; continue 'dispatch;
	}
	// 8312DF00: 392900C4  addi r9, r9, 0xc4
	ctx.r[9].s64 = ctx.r[9].s64 + 196;
	// 8312DF04: 390A1880  addi r8, r10, 0x1880
	ctx.r[8].s64 = ctx.r[10].s64 + 6272;
	// 8312DF08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312DF0C: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8312DF10: 4198FFE4  blt cr6, 0x8312def4
	if ctx.cr[6].lt {
	pc = 0x8312DEF4; continue 'dispatch;
	}
	// 8312DF14: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8312DF18: 409A0010  bne cr6, 0x8312df28
	if !ctx.cr[6].eq {
	pc = 0x8312DF28; continue 'dispatch;
	}
	// 8312DF1C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DF20: 386B0EE8  addi r3, r11, 0xee8
	ctx.r[3].s64 = ctx.r[11].s64 + 3816;
	// 8312DF24: 48000244  b 0x8312e168
	pc = 0x8312E168; continue 'dispatch;
	// 8312DF28: 1D6B00C4  mulli r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 * 196;
	// 8312DF2C: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8312DF30: 38A000C4  li r5, 0xc4
	ctx.r[5].s64 = 196;
	// 8312DF34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312DF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312DF3C: 4807A2A5  bl 0x831a81e0
	ctx.lr = 0x8312DF40;
	sub_831A81E0(ctx, base);
	// 8312DF40: 1D7E40C0  mulli r11, r30, 0x40c0
	ctx.r[11].s64 = ctx.r[30].s64 * 16576;
	// 8312DF44: 9BDF0003  stb r30, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[30].u8 ) };
	// 8312DF48: 7D4BE050  subf r10, r11, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 8312DF4C: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8312DF50: 396AFEDC  addi r11, r10, -0x124
	ctx.r[11].s64 = ctx.r[10].s64 + -292;
	// 8312DF54: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 8312DF58: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8312DF5C: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 8312DF60: 55645829  rlwinm. r4, r11, 0xb, 0, 0x14
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x001FFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8312DF64: 909F0024  stw r4, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 8312DF68: 40800010  bge 0x8312df78
	if !ctx.cr[0].lt {
	pc = 0x8312DF78; continue 'dispatch;
	}
	// 8312DF6C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312DF70: 386B0EB8  addi r3, r11, 0xeb8
	ctx.r[3].s64 = ctx.r[11].s64 + 3768;
	// 8312DF74: 480001F4  b 0x8312e168
	pc = 0x8312E168; continue 'dispatch;
	// 8312DF78: 7D641A14  add r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 8312DF7C: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8312DF80: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 8312DF84: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8312DF88: 396B0024  addi r11, r11, 0x24
	ctx.r[11].s64 = ctx.r[11].s64 + 36;
	// 8312DF8C: 39202000  li r9, 0x2000
	ctx.r[9].s64 = 8192;
	// 8312DF90: 39002060  li r8, 0x2060
	ctx.r[8].s64 = 8288;
	// 8312DF94: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 8312DF98: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8312DF9C: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 8312DFA0: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 8312DFA4: 911F0034  stw r8, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 8312DFA8: 48006829  bl 0x831347d0
	ctx.lr = 0x8312DFAC;
	sub_831347D0(ctx, base);
	// 8312DFAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312DFB0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8312DFB4: 40820010  bne 0x8312dfc4
	if !ctx.cr[0].eq {
	pc = 0x8312DFC4; continue 'dispatch;
	}
	// 8312DFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312DFBC: 4BFFF515  bl 0x8312d4d0
	ctx.lr = 0x8312DFC0;
	sub_8312D4D0(ctx, base);
	// 8312DFC0: 480001AC  b 0x8312e16c
	pc = 0x8312E16C; continue 'dispatch;
	// 8312DFC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312DFC8: 48004F11  bl 0x83132ed8
	ctx.lr = 0x8312DFCC;
	sub_83132ED8(ctx, base);
	// 8312DFCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312DFD0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8312DFD4: 4182FFE4  beq 0x8312dfb8
	if ctx.cr[0].eq {
	pc = 0x8312DFB8; continue 'dispatch;
	}
	// 8312DFD8: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 8312DFDC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8312DFE0: 4099004C  ble cr6, 0x8312e02c
	if !ctx.cr[6].gt {
	pc = 0x8312E02C; continue 'dispatch;
	}
	// 8312DFE4: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8312DFE8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8312DFEC: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8312DFF0: 7D2BE9D6  mullw r9, r11, r29
	ctx.r[9].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8312DFF4: 811F002C  lwz r8, 0x2c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8312DFF8: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8312DFFC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312E000: 5544083C  slwi r4, r10, 1
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8312E004: 7C694214  add r3, r9, r8
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8312E008: 5565083C  slwi r5, r11, 1
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8312E00C: 480067C5  bl 0x831347d0
	ctx.lr = 0x8312E010;
	sub_831347D0(ctx, base);
	// 8312E010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E014: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8312E018: 4182FFA0  beq 0x8312dfb8
	if ctx.cr[0].eq {
	pc = 0x8312DFB8; continue 'dispatch;
	}
	// 8312E01C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8312E020: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8312E024: 7F1DF000  cmpw cr6, r29, r30
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8312E028: 4198FFC0  blt cr6, 0x8312dfe8
	if ctx.cr[6].lt {
	pc = 0x8312DFE8; continue 'dispatch;
	}
	// 8312E02C: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 8312E030: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312E034: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312E038: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312E03C: 48003A65  bl 0x83131aa0
	ctx.lr = 0x8312E040;
	sub_83131AA0(ctx, base);
	// 8312E040: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E044: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8312E048: 4182FF70  beq 0x8312dfb8
	if ctx.cr[0].eq {
	pc = 0x8312DFB8; continue 'dispatch;
	}
	// 8312E04C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312E050: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8312E054: 480005DD  bl 0x8312e630
	ctx.lr = 0x8312E058;
	sub_8312E630(ctx, base);
	// 8312E058: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E05C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8312E060: 4182FF58  beq 0x8312dfb8
	if ctx.cr[0].eq {
	pc = 0x8312DFB8; continue 'dispatch;
	}
	// 8312E064: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312E068: 48007771  bl 0x831357d8
	ctx.lr = 0x8312E06C;
	sub_831357D8(ctx, base);
	// 8312E06C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E070: 907F0094  stw r3, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[3].u32 ) };
	// 8312E074: 4182FF44  beq 0x8312dfb8
	if ctx.cr[0].eq {
	pc = 0x8312DFB8; continue 'dispatch;
	}
	// 8312E078: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E07C: 4800779D  bl 0x83135818
	ctx.lr = 0x8312E080;
	sub_83135818(ctx, base);
	// 8312E080: 4BF99A61  bl 0x830c7ae0
	ctx.lr = 0x8312E084;
	sub_830C7AE0(ctx, base);
	// 8312E084: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312E088: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8312E08C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8312E090: B37F0040  sth r27, 0x40(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[27].u16 ) };
	// 8312E094: 814B7700  lwz r10, 0x7700(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30464 as u32) ) } as u64;
	// 8312E098: 7D2B5E70  srawi r11, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 8312E09C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8312E0A0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8312E0A4: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8312E0A8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312E0AC: B17F003C  sth r11, 0x3c(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u16 ) };
	// 8312E0B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8312E0B4: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8312E0B8: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312E0BC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312E0C0: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312E0C4: C00B31D8  lfs f0, 0x31d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312E0C8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8312E0CC: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8312E0D0: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 8312E0D4: A1610056  lhz r11, 0x56(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8312E0D8: B17F003E  sth r11, 0x3e(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[11].u16 ) };
	// 8312E0DC: 40990024  ble cr6, 0x8312e100
	if !ctx.cr[6].gt {
	pc = 0x8312E100; continue 'dispatch;
	}
	// 8312E0E0: 397F0042  addi r11, r31, 0x42
	ctx.r[11].s64 = ctx.r[31].s64 + 66;
	// 8312E0E4: 3940FF80  li r10, -0x80
	ctx.r[10].s64 = -128;
	// 8312E0E8: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E0EC: 41820014  beq 0x8312e100
	if ctx.cr[0].eq {
	pc = 0x8312E100; continue 'dispatch;
	}
	// 8312E0F0: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 8312E0F4: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 8312E0F8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8312E0FC: 4200FFF8  bdnz 0x8312e0f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8312E0F4; continue 'dispatch;
	}
	// 8312E100: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8312E104: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312E108: B37F0046  sth r27, 0x46(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(70 as u32), ctx.r[27].u16 ) };
	// 8312E10C: 937F0054  stw r27, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 8312E110: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E114: 937F0058  stw r27, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[27].u32 ) };
	// 8312E118: 937F005C  stw r27, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 8312E11C: 9BDF006C  stb r30, 0x6c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u8 ) };
	// 8312E120: B37F0060  sth r27, 0x60(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[27].u16 ) };
	// 8312E124: 937F0064  stw r27, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 8312E128: B37F0068  sth r27, 0x68(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[27].u16 ) };
	// 8312E12C: B37F006A  sth r27, 0x6a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(106 as u32), ctx.r[27].u16 ) };
	// 8312E130: 9BDF006D  stb r30, 0x6d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(109 as u32), ctx.r[30].u8 ) };
	// 8312E134: 9B7F0072  stb r27, 0x72(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(114 as u32), ctx.r[27].u8 ) };
	// 8312E138: 937F0088  stw r27, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[27].u32 ) };
	// 8312E13C: 9B7F0098  stb r27, 0x98(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[27].u8 ) };
	// 8312E140: 4182000C  beq 0x8312e14c
	if ctx.cr[0].eq {
	pc = 0x8312E14C; continue 'dispatch;
	}
	// 8312E144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312E148: 480037D9  bl 0x83131920
	ctx.lr = 0x8312E14C;
	sub_83131920(ctx, base);
	// 8312E14C: 9BDF00A9  stb r30, 0xa9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(169 as u32), ctx.r[30].u8 ) };
	// 8312E150: 9BDF0000  stb r30, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 8312E154: 4BF9998D  bl 0x830c7ae0
	ctx.lr = 0x8312E158;
	sub_830C7AE0(ctx, base);
	// 8312E158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E15C: 48000014  b 0x8312e170
	pc = 0x8312E170; continue 'dispatch;
	// 8312E160: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312E164: 386B0E90  addi r3, r11, 0xe90
	ctx.r[3].s64 = ctx.r[11].s64 + 3728;
	// 8312E168: 48002871  bl 0x831309d8
	ctx.lr = 0x8312E16C;
	sub_831309D8(ctx, base);
	// 8312E16C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312E170: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8312E174: 4807A040  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E178 size=56
    let mut pc: u32 = 0x8312E178;
    'dispatch: loop {
        match pc {
            0x8312E178 => {
    //   block [0x8312E178..0x8312E1B0)
	// 8312E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E180: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E184: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E18C: 4BFF8C1D  bl 0x83126da8
	ctx.lr = 0x8312E190;
	sub_83126DA8(ctx, base);
	// 8312E190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E194: 4BFFF33D  bl 0x8312d4d0
	ctx.lr = 0x8312E198;
	sub_8312D4D0(ctx, base);
	// 8312E198: 4BFF8C51  bl 0x83126de8
	ctx.lr = 0x8312E19C;
	sub_83126DE8(ctx, base);
	// 8312E19C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312E1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E1A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E1B0 size=100
    let mut pc: u32 = 0x8312E1B0;
    'dispatch: loop {
        match pc {
            0x8312E1B0 => {
    //   block [0x8312E1B0..0x8312E214)
	// 8312E1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E1B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312E1BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E1C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E1C4: 4BFF8BE5  bl 0x83126da8
	ctx.lr = 0x8312E1C8;
	sub_83126DA8(ctx, base);
	// 8312E1C8: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8312E1CC: 3BCB6B60  addi r30, r11, 0x6b60
	ctx.r[30].s64 = ctx.r[11].s64 + 27488;
	// 8312E1D0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8312E1D4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E1D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8312E1DC: 409A000C  bne cr6, 0x8312e1e8
	if !ctx.cr[6].eq {
	pc = 0x8312E1E8; continue 'dispatch;
	}
	// 8312E1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E1E4: 4BFFF2ED  bl 0x8312d4d0
	ctx.lr = 0x8312E1E8;
	sub_8312D4D0(ctx, base);
	// 8312E1E8: 3BFF00C4  addi r31, r31, 0xc4
	ctx.r[31].s64 = ctx.r[31].s64 + 196;
	// 8312E1EC: 397E1880  addi r11, r30, 0x1880
	ctx.r[11].s64 = ctx.r[30].s64 + 6272;
	// 8312E1F0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312E1F4: 4198FFE0  blt cr6, 0x8312e1d4
	if ctx.cr[6].lt {
	pc = 0x8312E1D4; continue 'dispatch;
	}
	// 8312E1F8: 4BFF8BF1  bl 0x83126de8
	ctx.lr = 0x8312E1FC;
	sub_83126DE8(ctx, base);
	// 8312E1FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E208: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312E20C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E218 size=220
    let mut pc: u32 = 0x8312E218;
    'dispatch: loop {
        match pc {
            0x8312E218 => {
    //   block [0x8312E218..0x8312E2F4)
	// 8312E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E21C: 48079F49  bl 0x831a8164
	ctx.lr = 0x8312E220;
	sub_831A8130(ctx, base);
	// 8312E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E228: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312E22C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8312E230: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8312E234: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8312E238: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312E23C: 40810034  ble 0x8312e270
	if !ctx.cr[0].gt {
	pc = 0x8312E270; continue 'dispatch;
	}
	// 8312E240: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 8312E244: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E24C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8312E250: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312E254: 4E800421  bctrl
	ctx.lr = 0x8312E258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312E258: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8312E25C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8312E260: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8312E264: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8312E268: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312E26C: 4198FFD8  blt cr6, 0x8312e244
	if ctx.cr[6].lt {
	pc = 0x8312E244; continue 'dispatch;
	}
	// 8312E270: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8312E274: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312E278: 48002FF9  bl 0x83131270
	ctx.lr = 0x8312E27C;
	sub_83131270(ctx, base);
	// 8312E27C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312E280: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8312E284: 48003015  bl 0x83131298
	ctx.lr = 0x8312E288;
	sub_83131298(ctx, base);
	// 8312E288: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8312E28C: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 8312E290: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8312E294: 9BDF0071  stb r30, 0x71(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(113 as u32), ctx.r[30].u8 ) };
	// 8312E298: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8312E29C: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 8312E2A0: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 8312E2A4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8312E2A8: 895F0002  lbz r10, 2(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8312E2AC: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 8312E2B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312E2B4: 2B0A0004  cmplwi cr6, r10, 4
	ctx.cr[6].compare_u32(ctx.r[10].u32, 4 as u32, &mut ctx.xer);
	// 8312E2B8: 911F0090  stw r8, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 8312E2BC: 913F008C  stw r9, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 8312E2C0: 816B757C  lwz r11, 0x757c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30076 as u32) ) } as u64;
	// 8312E2C4: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 8312E2C8: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8312E2CC: 409A0010  bne cr6, 0x8312e2dc
	if !ctx.cr[6].eq {
	pc = 0x8312E2DC; continue 'dispatch;
	}
	// 8312E2D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312E2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E2D8: 4BFFF8A9  bl 0x8312db80
	ctx.lr = 0x8312E2DC;
	sub_8312DB80(ctx, base);
	// 8312E2DC: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8312E2E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E2E4: 41820008  beq 0x8312e2ec
	if ctx.cr[0].eq {
	pc = 0x8312E2EC; continue 'dispatch;
	}
	// 8312E2E8: 480070B1  bl 0x83135398
	ctx.lr = 0x8312E2EC;
	sub_83135398(ctx, base);
	// 8312E2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312E2F0: 48079EC4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E2F8 size=172
    let mut pc: u32 = 0x8312E2F8;
    'dispatch: loop {
        match pc {
            0x8312E2F8 => {
    //   block [0x8312E2F8..0x8312E3A4)
	// 8312E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E2FC: 48079E69  bl 0x831a8164
	ctx.lr = 0x8312E300;
	sub_831A8130(ctx, base);
	// 8312E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E308: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312E30C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312E310: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8312E314: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 8312E318: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E31C: A97F003C  lha r11, 0x3c(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as i16) as i64;
	// 8312E320: A95F003E  lha r10, 0x3e(r31)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(62 as u32) ) } as i16) as i64;
	// 8312E324: 55655828  slwi r5, r11, 0xb
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8312E328: 55445828  slwi r4, r10, 0xb
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8312E32C: 48004B3D  bl 0x83132e68
	ctx.lr = 0x8312E330;
	sub_83132E68(ctx, base);
	// 8312E330: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8312E334: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E338: 808B5A40  lwz r4, 0x5a40(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23104 as u32) ) } as u64;
	// 8312E33C: 48004465  bl 0x831327a0
	ctx.lr = 0x8312E340;
	sub_831327A0(ctx, base);
	// 8312E340: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8312E344: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E348: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312E34C: 48004425  bl 0x83132770
	ctx.lr = 0x8312E350;
	sub_83132770(ctx, base);
	// 8312E350: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312E354: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E358: 48004371  bl 0x831326c8
	ctx.lr = 0x8312E35C;
	sub_831326C8(ctx, base);
	// 8312E35C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E360: 48004CB1  bl 0x83133010
	ctx.lr = 0x8312E364;
	sub_83133010(ctx, base);
	// 8312E364: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E368: 48004DF9  bl 0x83133160
	ctx.lr = 0x8312E36C;
	sub_83133160(ctx, base);
	// 8312E36C: 7F6B07B4  extsw r11, r27
	ctx.r[11].s64 = ctx.r[27].s32 as i64;
	// 8312E370: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8312E374: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E378: 79675D24  sldi r7, r11, 0xb
	ctx.r[7].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[7].u32 = ctx.r[7].u64 as u32;
	// 8312E37C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312E380: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312E384: 48004BA5  bl 0x83132f28
	ctx.lr = 0x8312E388;
	sub_83132F28(ctx, base);
	// 8312E388: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312E38C: 48004C05  bl 0x83132f90
	ctx.lr = 0x8312E390;
	sub_83132F90(ctx, base);
	// 8312E390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E394: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312E398: 4BFFFE81  bl 0x8312e218
	ctx.lr = 0x8312E39C;
	sub_8312E218(ctx, base);
	// 8312E39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312E3A0: 48079E14  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312E3A8 size=220
    let mut pc: u32 = 0x8312E3A8;
    'dispatch: loop {
        match pc {
            0x8312E3A8 => {
    //   block [0x8312E3A8..0x8312E484)
	// 8312E3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E3AC: 48079DBD  bl 0x831a8168
	ctx.lr = 0x8312E3B0;
	sub_831A8130(ctx, base);
	// 8312E3B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E3B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E3B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312E3BC: 4BFF89ED  bl 0x83126da8
	ctx.lr = 0x8312E3C0;
	sub_83126DA8(ctx, base);
	// 8312E3C0: 897F0072  lbz r11, 0x72(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(114 as u32) ) } as u64;
	// 8312E3C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312E3C8: 409A000C  bne cr6, 0x8312e3d4
	if !ctx.cr[6].eq {
	pc = 0x8312E3D4; continue 'dispatch;
	}
	// 8312E3CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8312E3D0: 480000A4  b 0x8312e474
	pc = 0x8312E474; continue 'dispatch;
	// 8312E3D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312E3D8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312E3DC: 480500AD  bl 0x8317e488
	ctx.lr = 0x8312E3E0;
	sub_8317E488(ctx, base);
	// 8312E3E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8312E3E4: 4BFF89C5  bl 0x83126da8
	ctx.lr = 0x8312E3E8;
	sub_83126DA8(ctx, base);
	// 8312E3E8: 4BFFEDC9  bl 0x8312d1b0
	ctx.lr = 0x8312E3EC;
	sub_8312D1B0(ctx, base);
	// 8312E3EC: 4BFF89FD  bl 0x83126de8
	ctx.lr = 0x8312E3F0;
	sub_83126DE8(ctx, base);
	// 8312E3F0: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8312E3F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312E3F8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8312E3FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312E400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E404: 839E76F8  lwz r28, 0x76f8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(30456 as u32) ) } as u64;
	// 8312E408: 917E76F8  stw r11, 0x76f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(30456 as u32), ctx.r[11].u32 ) };
	// 8312E40C: 4BFFE9F5  bl 0x8312ce00
	ctx.lr = 0x8312E410;
	sub_8312CE00(ctx, base);
	// 8312E410: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8312E414: 939E76F8  stw r28, 0x76f8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(30456 as u32), ctx.r[28].u32 ) };
	// 8312E418: 391F009C  addi r8, r31, 0x9c
	ctx.r[8].s64 = ctx.r[31].s64 + 156;
	// 8312E41C: E9410052  lwa r10, 0x50(r1)
	ctx.r[10].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 8312E420: E921005A  lwa r9, 0x58(r1)
	ctx.r[9].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as i32) as i64;
	// 8312E424: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8312E428: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8312E42C: E96B6B46  lwa r11, 0x6b44(r11)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27460 as u32) ) } as i32) as i64;
	// 8312E430: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312E434: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8312E438: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312E43C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8312E440: F9610060  std r11, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 8312E444: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312E448: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312E44C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8312E450: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8312E454: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8312E458: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8312E45C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8312E460: EC000332  fmuls f0, f0, f12
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8312E464: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 8312E468: 7C0047AE  stfiwx f0, 0, r8
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32, tmp.u32) };
	// 8312E46C: 816B757C  lwz r11, 0x757c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30076 as u32) ) } as u64;
	// 8312E470: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 8312E474: 4BFF8975  bl 0x83126de8
	ctx.lr = 0x8312E478;
	sub_83126DE8(ctx, base);
	// 8312E478: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8312E47C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8312E480: 48079D38  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E488 size=64
    let mut pc: u32 = 0x8312E488;
    'dispatch: loop {
        match pc {
            0x8312E488 => {
    //   block [0x8312E488..0x8312E4C8)
	// 8312E488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E48C: 48079CE1  bl 0x831a816c
	ctx.lr = 0x8312E490;
	sub_831A8130(ctx, base);
	// 8312E490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E498: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312E49C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312E4A0: 4BFF8909  bl 0x83126da8
	ctx.lr = 0x8312E4A4;
	sub_83126DA8(ctx, base);
	// 8312E4A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312E4A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312E4AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E4B0: 4BFFF9F9  bl 0x8312dea8
	ctx.lr = 0x8312E4B4;
	sub_8312DEA8(ctx, base);
	// 8312E4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E4B8: 4BFF8931  bl 0x83126de8
	ctx.lr = 0x8312E4BC;
	sub_83126DE8(ctx, base);
	// 8312E4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E4C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E4C4: 48079CF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E4C8 size=84
    let mut pc: u32 = 0x8312E4C8;
    'dispatch: loop {
        match pc {
            0x8312E4C8 => {
    //   block [0x8312E4C8..0x8312E51C)
	// 8312E4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E4D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312E4D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E4D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E4DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E4E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312E4E4: 4BFF88C5  bl 0x83126da8
	ctx.lr = 0x8312E4E8;
	sub_83126DA8(ctx, base);
	// 8312E4E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8312E4EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312E4F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312E4F4: 4BFFF9B5  bl 0x8312dea8
	ctx.lr = 0x8312E4F8;
	sub_8312DEA8(ctx, base);
	// 8312E4F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E4FC: 4BFF88ED  bl 0x83126de8
	ctx.lr = 0x8312E500;
	sub_83126DE8(ctx, base);
	// 8312E500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312E514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E520 size=156
    let mut pc: u32 = 0x8312E520;
    'dispatch: loop {
        match pc {
            0x8312E520 => {
    //   block [0x8312E520..0x8312E5BC)
	// 8312E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312E52C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E538: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312E53C: 4BFF886D  bl 0x83126da8
	ctx.lr = 0x8312E540;
	sub_83126DA8(ctx, base);
	// 8312E540: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312E544: 419A0050  beq cr6, 0x8312e594
	if ctx.cr[6].eq {
	pc = 0x8312E594; continue 'dispatch;
	}
	// 8312E548: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312E54C: 419A0048  beq cr6, 0x8312e594
	if ctx.cr[6].eq {
	pc = 0x8312E594; continue 'dispatch;
	}
	// 8312E550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E554: 4BFFE76D  bl 0x8312ccc0
	ctx.lr = 0x8312E558;
	sub_8312CCC0(ctx, base);
	// 8312E558: 4BF99589  bl 0x830c7ae0
	ctx.lr = 0x8312E55C;
	sub_830C7AE0(ctx, base);
	// 8312E55C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8312E560: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312E564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E568: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 8312E56C: 4BFFFCAD  bl 0x8312e218
	ctx.lr = 0x8312E570;
	sub_8312E218(ctx, base);
	// 8312E570: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8312E574: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312E578: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E57C: 997F0098  stb r11, 0x98(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 8312E580: 4182000C  beq 0x8312e58c
	if ctx.cr[0].eq {
	pc = 0x8312E58C; continue 'dispatch;
	}
	// 8312E584: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312E588: 48003399  bl 0x83131920
	ctx.lr = 0x8312E58C;
	sub_83131920(ctx, base);
	// 8312E58C: 4BF99555  bl 0x830c7ae0
	ctx.lr = 0x8312E590;
	sub_830C7AE0(ctx, base);
	// 8312E590: 48000010  b 0x8312e5a0
	pc = 0x8312E5A0; continue 'dispatch;
	// 8312E594: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312E598: 386B0F14  addi r3, r11, 0xf14
	ctx.r[3].s64 = ctx.r[11].s64 + 3860;
	// 8312E59C: 4800243D  bl 0x831309d8
	ctx.lr = 0x8312E5A0;
	sub_831309D8(ctx, base);
	// 8312E5A0: 4BFF8849  bl 0x83126de8
	ctx.lr = 0x8312E5A4;
	sub_83126DE8(ctx, base);
	// 8312E5A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E5B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312E5B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E5C0 size=108
    let mut pc: u32 = 0x8312E5C0;
    'dispatch: loop {
        match pc {
            0x8312E5C0 => {
    //   block [0x8312E5C0..0x8312E62C)
	// 8312E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E5C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E5CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E5D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312E5D4: 3BEB7728  addi r31, r11, 0x7728
	ctx.r[31].s64 = ctx.r[11].s64 + 30504;
	// 8312E5D8: 391F0500  addi r8, r31, 0x500
	ctx.r[8].s64 = ctx.r[31].s64 + 1280;
	// 8312E5DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8312E5E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312E5E4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8312E5E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8312E5EC: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8312E5F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312E5F4: 4082FFE8  bne 0x8312e5dc
	if !ctx.cr[0].eq {
	pc = 0x8312E5DC; continue 'dispatch;
	}
	// 8312E5F8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8312E5FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312E600: 409A0018  bne cr6, 0x8312e618
	if !ctx.cr[6].eq {
	pc = 0x8312E618; continue 'dispatch;
	}
	// 8312E604: 4BFFD2CD  bl 0x8312b8d0
	ctx.lr = 0x8312E608;
	sub_8312B8D0(ctx, base);
	// 8312E608: 38A00500  li r5, 0x500
	ctx.r[5].s64 = 1280;
	// 8312E60C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312E610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E614: 48079BCD  bl 0x831a81e0
	ctx.lr = 0x8312E618;
	sub_831A81E0(ctx, base);
	// 8312E618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312E61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E630 size=220
    let mut pc: u32 = 0x8312E630;
    'dispatch: loop {
        match pc {
            0x8312E630 => {
    //   block [0x8312E630..0x8312E70C)
	// 8312E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E634: 48079B35  bl 0x831a8168
	ctx.lr = 0x8312E638;
	sub_831A8130(ctx, base);
	// 8312E638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E63C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312E640: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8312E644: 394B7728  addi r10, r11, 0x7728
	ctx.r[10].s64 = ctx.r[11].s64 + 30504;
	// 8312E648: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8312E64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8312E650: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8312E654: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E658: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8312E65C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8312E660: 419A0018  beq cr6, 0x8312e678
	if ctx.cr[6].eq {
	pc = 0x8312E678; continue 'dispatch;
	}
	// 8312E664: 396B0028  addi r11, r11, 0x28
	ctx.r[11].s64 = ctx.r[11].s64 + 40;
	// 8312E668: 390A0500  addi r8, r10, 0x500
	ctx.r[8].s64 = ctx.r[10].s64 + 1280;
	// 8312E66C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8312E670: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8312E674: 4198FFE0  blt cr6, 0x8312e654
	if ctx.cr[6].lt {
	pc = 0x8312E654; continue 'dispatch;
	}
	// 8312E678: 2F090020  cmpwi cr6, r9, 0x20
	ctx.cr[6].compare_i32(ctx.r[9].s32, 32, &mut ctx.xer);
	// 8312E67C: 419A0084  beq cr6, 0x8312e700
	if ctx.cr[6].eq {
	pc = 0x8312E700; continue 'dispatch;
	}
	// 8312E680: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 8312E684: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312E688: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8312E68C: 48079B55  bl 0x831a81e0
	ctx.lr = 0x8312E690;
	sub_831A81E0(ctx, base);
	// 8312E690: 3BDD0020  addi r30, r29, 0x20
	ctx.r[30].s64 = ctx.r[29].s64 + 32;
	// 8312E694: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312E698: 4BFFE181  bl 0x8312c818
	ctx.lr = 0x8312E69C;
	sub_8312C818(ctx, base);
	// 8312E69C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312E6A0: 41800060  blt 0x8312e700
	if ctx.cr[0].lt {
	pc = 0x8312E700; continue 'dispatch;
	}
	// 8312E6A4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E6A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E6AC: 41820054  beq 0x8312e700
	if ctx.cr[0].eq {
	pc = 0x8312E700; continue 'dispatch;
	}
	// 8312E6B0: 38C02020  li r6, 0x2020
	ctx.r[6].s64 = 8224;
	// 8312E6B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8312E6B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8312E6BC: 4BFFE03D  bl 0x8312c6f8
	ctx.lr = 0x8312E6C0;
	sub_8312C6F8(ctx, base);
	// 8312E6C0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8312E6C4: 40990028  ble cr6, 0x8312e6ec
	if !ctx.cr[6].gt {
	pc = 0x8312E6EC; continue 'dispatch;
	}
	// 8312E6C8: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8312E6CC: 393D0018  addi r9, r29, 0x18
	ctx.r[9].s64 = ctx.r[29].s64 + 24;
	// 8312E6D0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8312E6D4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E6D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312E6DC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8312E6E0: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8312E6E4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8312E6E8: 4082FFEC  bne 0x8312e6d4
	if !ctx.cr[0].eq {
	pc = 0x8312E6D4; continue 'dispatch;
	}
	// 8312E6EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8312E6F0: 93FD0008  stw r31, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8312E6F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8312E6F8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312E6FC: 48000008  b 0x8312e704
	pc = 0x8312E704; continue 'dispatch;
	// 8312E700: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312E704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312E708: 48079AB0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E710 size=12
    let mut pc: u32 = 0x8312E710;
    'dispatch: loop {
        match pc {
            0x8312E710 => {
    //   block [0x8312E710..0x8312E71C)
	// 8312E710: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E714: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E718: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E71C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E71C size=8
    let mut pc: u32 = 0x8312E71C;
    'dispatch: loop {
        match pc {
            0x8312E71C => {
    //   block [0x8312E71C..0x8312E724)
	// 8312E71C: 4BFFD63C  b 0x8312bd58
	sub_8312BD58(ctx, base);
	return;
	// 8312E720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E728 size=12
    let mut pc: u32 = 0x8312E728;
    'dispatch: loop {
        match pc {
            0x8312E728 => {
    //   block [0x8312E728..0x8312E734)
	// 8312E728: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E72C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E730: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E734 size=8
    let mut pc: u32 = 0x8312E734;
    'dispatch: loop {
        match pc {
            0x8312E734 => {
    //   block [0x8312E734..0x8312E73C)
	// 8312E734: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8312E738: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E73C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E73C size=8
    let mut pc: u32 = 0x8312E73C;
    'dispatch: loop {
        match pc {
            0x8312E73C => {
    //   block [0x8312E73C..0x8312E744)
	// 8312E73C: 4BFFD61C  b 0x8312bd58
	sub_8312BD58(ctx, base);
	return;
	// 8312E740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E748 size=12
    let mut pc: u32 = 0x8312E748;
    'dispatch: loop {
        match pc {
            0x8312E748 => {
    //   block [0x8312E748..0x8312E754)
	// 8312E748: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E74C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E750: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E754(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E754 size=12
    let mut pc: u32 = 0x8312E754;
    'dispatch: loop {
        match pc {
            0x8312E754 => {
    //   block [0x8312E754..0x8312E760)
	// 8312E754: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8312E758: 409A0008  bne cr6, 0x8312e760
	if !ctx.cr[6].eq {
		sub_8312E760(ctx, base);
		return;
	}
	// 8312E75C: 4BFFD55C  b 0x8312bcb8
	sub_8312BCB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E760 size=8
    let mut pc: u32 = 0x8312E760;
    'dispatch: loop {
        match pc {
            0x8312E760 => {
    //   block [0x8312E760..0x8312E768)
	// 8312E760: 4BFFD5F8  b 0x8312bd58
	sub_8312BD58(ctx, base);
	return;
	// 8312E764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E768 size=80
    let mut pc: u32 = 0x8312E768;
    'dispatch: loop {
        match pc {
            0x8312E768 => {
    //   block [0x8312E768..0x8312E7B8)
	// 8312E768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E76C: 48079A01  bl 0x831a816c
	ctx.lr = 0x8312E770;
	sub_831A8130(ctx, base);
	// 8312E770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312E778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312E77C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312E780: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E784: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E788: 41820028  beq 0x8312e7b0
	if ctx.cr[0].eq {
	pc = 0x8312E7B0; continue 'dispatch;
	}
	// 8312E78C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 8312E790: 4BFFDB59  bl 0x8312c2e8
	ctx.lr = 0x8312E794;
	sub_8312C2E8(ctx, base);
	// 8312E794: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312E798: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E79C: 4BFFDAC5  bl 0x8312c260
	ctx.lr = 0x8312E7A0;
	sub_8312C260(ctx, base);
	// 8312E7A0: E9610058  ld r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8312E7A4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312E7A8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312E7AC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8312E7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312E7B4: 48079A08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E7B8 size=56
    let mut pc: u32 = 0x8312E7B8;
    'dispatch: loop {
        match pc {
            0x8312E7B8 => {
    //   block [0x8312E7B8..0x8312E7F0)
	// 8312E7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E7C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E7C4: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8312E7C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312E7CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E7D0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8312E7D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312E7D8: 4E800421  bctrl
	ctx.lr = 0x8312E7DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312E7DC: 5463F87E  srwi r3, r3, 1
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8312E7E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312E7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E7F0 size=12
    let mut pc: u32 = 0x8312E7F0;
    'dispatch: loop {
        match pc {
            0x8312E7F0 => {
    //   block [0x8312E7F0..0x8312E7FC)
	// 8312E7F0: 3C607FFF  lis r3, 0x7fff
	ctx.r[3].s64 = 2147418112;
	// 8312E7F4: 6063FFFF  ori r3, r3, 0xffff
	ctx.r[3].u64 = ctx.r[3].u64 | 65535;
	// 8312E7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E800 size=80
    let mut pc: u32 = 0x8312E800;
    'dispatch: loop {
        match pc {
            0x8312E800 => {
    //   block [0x8312E800..0x8312E850)
	// 8312E800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E808: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E80C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E810: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E814: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312E818: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E81C: 4182001C  beq 0x8312e838
	if ctx.cr[0].eq {
	pc = 0x8312E838; continue 'dispatch;
	}
	// 8312E820: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312E824: 4BFFD9BD  bl 0x8312c1e0
	ctx.lr = 0x8312E828;
	sub_8312C1E0(ctx, base);
	// 8312E828: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312E82C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8312E830: 409A0008  bne cr6, 0x8312e838
	if !ctx.cr[6].eq {
	pc = 0x8312E838; continue 'dispatch;
	}
	// 8312E834: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8312E838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312E83C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E848: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312E850 size=96
    let mut pc: u32 = 0x8312E850;
    'dispatch: loop {
        match pc {
            0x8312E850 => {
    //   block [0x8312E850..0x8312E8B0)
	// 8312E850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E854: 48079919  bl 0x831a816c
	ctx.lr = 0x8312E858;
	sub_831A8130(ctx, base);
	// 8312E858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E85C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312E860: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8312E864: 3BCB7728  addi r30, r11, 0x7728
	ctx.r[30].s64 = ctx.r[11].s64 + 30504;
	// 8312E868: 3BFE0020  addi r31, r30, 0x20
	ctx.r[31].s64 = ctx.r[30].s64 + 32;
	// 8312E86C: 817FFFE0  lwz r11, -0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-32 as u32) ) } as u64;
	// 8312E870: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312E874: 419A0018  beq cr6, 0x8312e88c
	if ctx.cr[6].eq {
	pc = 0x8312E88C; continue 'dispatch;
	}
	// 8312E878: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312E87C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E880: 41820008  beq 0x8312e888
	if ctx.cr[0].eq {
	pc = 0x8312E888; continue 'dispatch;
	}
	// 8312E884: 4BFFE2E5  bl 0x8312cb68
	ctx.lr = 0x8312E888;
	sub_8312CB68(ctx, base);
	// 8312E888: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8312E88C: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8312E890: 397E0520  addi r11, r30, 0x520
	ctx.r[11].s64 = ctx.r[30].s64 + 1312;
	// 8312E894: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312E898: 4198FFD4  blt cr6, 0x8312e86c
	if ctx.cr[6].lt {
	pc = 0x8312E86C; continue 'dispatch;
	}
	// 8312E89C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8312E8A0: 40990008  ble cr6, 0x8312e8a8
	if !ctx.cr[6].gt {
	pc = 0x8312E8A8; continue 'dispatch;
	}
	// 8312E8A4: 4BFFCD95  bl 0x8312b638
	ctx.lr = 0x8312E8A8;
	sub_8312B638(ctx, base);
	// 8312E8A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E8AC: 48079910  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E8B0 size=12
    let mut pc: u32 = 0x8312E8B0;
    'dispatch: loop {
        match pc {
            0x8312E8B0 => {
    //   block [0x8312E8B0..0x8312E8BC)
	// 8312E8B0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E8B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E8B8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E8BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E8BC size=8
    let mut pc: u32 = 0x8312E8BC;
    'dispatch: loop {
        match pc {
            0x8312E8BC => {
    //   block [0x8312E8BC..0x8312E8C4)
	// 8312E8BC: 4BFFDC1C  b 0x8312c4d8
	sub_8312C4D8(ctx, base);
	return;
	// 8312E8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E8C8 size=12
    let mut pc: u32 = 0x8312E8C8;
    'dispatch: loop {
        match pc {
            0x8312E8C8 => {
    //   block [0x8312E8C8..0x8312E8D4)
	// 8312E8C8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E8CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E8D0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E8D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312E8D4 size=8
    let mut pc: u32 = 0x8312E8D4;
    'dispatch: loop {
        match pc {
            0x8312E8D4 => {
    //   block [0x8312E8D4..0x8312E8DC)
	// 8312E8D4: 4BFFDB7C  b 0x8312c450
	sub_8312C450(ctx, base);
	return;
	// 8312E8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312E8E0 size=108
    let mut pc: u32 = 0x8312E8E0;
    'dispatch: loop {
        match pc {
            0x8312E8E0 => {
    //   block [0x8312E8E0..0x8312E94C)
	// 8312E8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E8E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E8EC: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E8F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E8F4: 41820048  beq 0x8312e93c
	if ctx.cr[0].eq {
	pc = 0x8312E93C; continue 'dispatch;
	}
	// 8312E8F8: 7C8A07B4  extsw r10, r4
	ctx.r[10].s64 = ctx.r[4].s32 as i64;
	// 8312E8FC: 7CA907B4  extsw r9, r5
	ctx.r[9].s64 = ctx.r[5].s32 as i64;
	// 8312E900: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312E904: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8312E908: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 8312E90C: C00B0F98  lfs f0, 0xf98(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3992 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312E910: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8312E914: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312E918: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8312E91C: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8312E920: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8312E924: FD606818  frsp f11, f13
	ctx.f[11].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8312E928: C1AB9520  lfs f13, -0x6ae0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27360 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312E92C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8312E930: EDAB0372  fmuls f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 8312E934: EC2C683A  fmadds f1, f12, f0, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8312E938: 4BFFDCB9  bl 0x8312c5f0
	ctx.lr = 0x8312E93C;
	sub_8312C5F0(ctx, base);
	// 8312E93C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312E950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312E950 size=172
    let mut pc: u32 = 0x8312E950;
    'dispatch: loop {
        match pc {
            0x8312E950 => {
    //   block [0x8312E950..0x8312E9FC)
	// 8312E950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312E954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312E958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312E95C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312E960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312E964: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312E968: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8312E96C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8312E970: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312E974: 41820014  beq 0x8312e988
	if ctx.cr[0].eq {
	pc = 0x8312E988; continue 'dispatch;
	}
	// 8312E978: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312E97C: 4BFFDA4D  bl 0x8312c3c8
	ctx.lr = 0x8312E980;
	sub_8312C3C8(ctx, base);
	// 8312E980: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312E984: 4800000C  b 0x8312e990
	pc = 0x8312E990; continue 'dispatch;
	// 8312E988: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8312E98C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312E990: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312E994: 419A0018  beq cr6, 0x8312e9ac
	if ctx.cr[6].eq {
	pc = 0x8312E9AC; continue 'dispatch;
	}
	// 8312E998: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8312E99C: C1AB9590  lfs f13, -0x6a70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27248 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312E9A0: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8312E9A4: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8312E9A8: 7DA0FFAE  stfiwx f13, 0, r31
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32, tmp.u32) };
	// 8312E9AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312E9B0: 419A0034  beq cr6, 0x8312e9e4
	if ctx.cr[6].eq {
	pc = 0x8312E9E4; continue 'dispatch;
	}
	// 8312E9B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312E9B8: 394004B0  li r10, 0x4b0
	ctx.r[10].s64 = 1200;
	// 8312E9BC: C1AB0F9C  lfs f13, 0xf9c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3996 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8312E9C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8312E9C4: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8312E9C8: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8312E9CC: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 8312E9D0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312E9D4: 7D4B53D6  divw r10, r11, r10
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8312E9D8: 1D4A04B0  mulli r10, r10, 0x4b0
	ctx.r[10].s64 = ctx.r[10].s64 * 1200;
	// 8312E9DC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312E9E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312E9E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312E9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312E9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312E9F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312E9F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312E9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312EA00 size=104
    let mut pc: u32 = 0x8312EA00;
    'dispatch: loop {
        match pc {
            0x8312EA00 => {
    //   block [0x8312EA00..0x8312EA68)
	// 8312EA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EA08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312EA0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EA10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EA14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312EA18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8312EA1C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EA20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312EA24: 41820028  beq 0x8312ea4c
	if ctx.cr[0].eq {
	pc = 0x8312EA4C; continue 'dispatch;
	}
	// 8312EA28: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 8312EA2C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8312EA30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8312EA34: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312EA38: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312EA3C: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312EA40: C00B964C  lfs f0, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312EA44: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8312EA48: 4BFFCDF1  bl 0x8312b838
	ctx.lr = 0x8312EA4C;
	sub_8312B838(ctx, base);
	// 8312EA4C: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8312EA50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EA5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312EA60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312EA68 size=328
    let mut pc: u32 = 0x8312EA68;
    'dispatch: loop {
        match pc {
            0x8312EA68 => {
    //   block [0x8312EA68..0x8312EBB0)
	// 8312EA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EA70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312EA74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EA78: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8312EA7C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8312EA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EA84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312EA88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312EA8C: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 8312EA90: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312EA94: 7CABF92E  stwx r5, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[5].u32) };
	// 8312EA98: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8312EA9C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312EAA0: 409A00F0  bne cr6, 0x8312eb90
	if !ctx.cr[6].eq {
	pc = 0x8312EB90; continue 'dispatch;
	}
	// 8312EAA4: 3565000F  addic. r11, r5, 0xf
	ctx.xer.ca = (ctx.r[5].u32 > (!(15 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + 15;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312EAA8: 2125FFF1  subfic r9, r5, -0xf
	ctx.xer.ca = ctx.r[5].u32 <= -15 as u32;
	ctx.r[9].s64 = (-15 as i64) - ctx.r[5].s64;
	// 8312EAAC: 41800008  blt 0x8312eab4
	if ctx.cr[0].lt {
	pc = 0x8312EAB4; continue 'dispatch;
	}
	// 8312EAB0: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8312EAB4: 3565FFF1  addic. r11, r5, -0xf
	ctx.xer.ca = (ctx.r[5].u32 > (!(-15 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -15;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312EAB8: 2145000F  subfic r10, r5, 0xf
	ctx.xer.ca = ctx.r[5].u32 <= 15 as u32;
	ctx.r[10].s64 = (15 as i64) - ctx.r[5].s64;
	// 8312EABC: 41800008  blt 0x8312eac4
	if ctx.cr[0].lt {
	pc = 0x8312EAC4; continue 'dispatch;
	}
	// 8312EAC0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312EAC4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8312EAC8: 40980008  bge cr6, 0x8312ead0
	if !ctx.cr[6].lt {
	pc = 0x8312EAD0; continue 'dispatch;
	}
	// 8312EACC: 7CA500D0  neg r5, r5
	ctx.r[5].s64 = -ctx.r[5].s64;
	// 8312EAD0: 2F09000F  cmpwi cr6, r9, 0xf
	ctx.cr[6].compare_i32(ctx.r[9].s32, 15, &mut ctx.xer);
	// 8312EAD4: 41980008  blt cr6, 0x8312eadc
	if ctx.cr[6].lt {
	pc = 0x8312EADC; continue 'dispatch;
	}
	// 8312EAD8: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 8312EADC: 2F0A000F  cmpwi cr6, r10, 0xf
	ctx.cr[6].compare_i32(ctx.r[10].s32, 15, &mut ctx.xer);
	// 8312EAE0: 41980008  blt cr6, 0x8312eae8
	if ctx.cr[6].lt {
	pc = 0x8312EAE8; continue 'dispatch;
	}
	// 8312EAE4: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 8312EAE8: 2F05000F  cmpwi cr6, r5, 0xf
	ctx.cr[6].compare_i32(ctx.r[5].s32, 15, &mut ctx.xer);
	// 8312EAEC: 41980008  blt cr6, 0x8312eaf4
	if ctx.cr[6].lt {
	pc = 0x8312EAF4; continue 'dispatch;
	}
	// 8312EAF0: 38A0000F  li r5, 0xf
	ctx.r[5].s64 = 15;
	// 8312EAF4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EAF8: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EAFC: 54A8103A  slwi r8, r5, 2
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8312EB00: 396B0F40  addi r11, r11, 0xf40
	ctx.r[11].s64 = ctx.r[11].s64 + 3904;
	// 8312EB04: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8312EB08: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8312EB0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8312EB10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312EB14: 7FC85C2E  lfsx f30, r8, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8312EB18: 7FEA5C2E  lfsx f31, r10, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8312EB1C: 7C295C2E  lfsx f1, r9, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8312EB20: 4BFFDB79  bl 0x8312c698
	ctx.lr = 0x8312EB24;
	sub_8312C698(ctx, base);
	// 8312EB24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8312EB28: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EB2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312EB30: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312EB34: 4BFFDB65  bl 0x8312c698
	ctx.lr = 0x8312EB38;
	sub_8312C698(ctx, base);
	// 8312EB38: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8312EB3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312EB40: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EB44: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8312EB48: 4BFFDB51  bl 0x8312c698
	ctx.lr = 0x8312EB4C;
	sub_8312C698(ctx, base);
	// 8312EB4C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EB50: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8312EB54: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EB58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312EB5C: C3EB0498  lfs f31, 0x498(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1176 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8312EB60: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312EB64: 4BFFDB35  bl 0x8312c698
	ctx.lr = 0x8312EB68;
	sub_8312C698(ctx, base);
	// 8312EB68: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8312EB6C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EB70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312EB74: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312EB78: 4BFFDB21  bl 0x8312c698
	ctx.lr = 0x8312EB7C;
	sub_8312C698(ctx, base);
	// 8312EB7C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8312EB80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312EB84: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EB88: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312EB8C: 4BFFDB0D  bl 0x8312c698
	ctx.lr = 0x8312EB90;
	sub_8312C698(ctx, base);
	// 8312EB90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312EB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EB9C: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8312EBA0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8312EBA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312EBA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EBB0 size=12
    let mut pc: u32 = 0x8312EBB0;
    'dispatch: loop {
        match pc {
            0x8312EBB0 => {
    //   block [0x8312EBB0..0x8312EBBC)
	// 8312EBB0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EBB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312EBB8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EBBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EBBC size=8
    let mut pc: u32 = 0x8312EBBC;
    'dispatch: loop {
        match pc {
            0x8312EBBC => {
    //   block [0x8312EBBC..0x8312EBC4)
	// 8312EBBC: 4BFFD5AC  b 0x8312c168
	sub_8312C168(ctx, base);
	return;
	// 8312EBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EBC8 size=16
    let mut pc: u32 = 0x8312EBC8;
    'dispatch: loop {
        match pc {
            0x8312EBC8 => {
    //   block [0x8312EBC8..0x8312EBD8)
	// 8312EBC8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EBCC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8312EBD0: 419A0008  beq cr6, 0x8312ebd8
	if ctx.cr[6].eq {
		sub_8312EBD8(ctx, base);
		return;
	}
	// 8312EBD4: 4BFFD21C  b 0x8312bdf0
	sub_8312BDF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EBD8 size=4
    let mut pc: u32 = 0x8312EBD8;
    'dispatch: loop {
        match pc {
            0x8312EBD8 => {
    //   block [0x8312EBD8..0x8312EBDC)
	// 8312EBD8: 4BFFD290  b 0x8312be68
	sub_8312BE68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312EBE0 size=112
    let mut pc: u32 = 0x8312EBE0;
    'dispatch: loop {
        match pc {
            0x8312EBE0 => {
    //   block [0x8312EBE0..0x8312EC50)
	// 8312EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EBE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EBEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EBF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312EBF4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EBF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312EBFC: 41820038  beq 0x8312ec34
	if ctx.cr[0].eq {
	pc = 0x8312EC34; continue 'dispatch;
	}
	// 8312EC00: 7CCA07B4  extsw r10, r6
	ctx.r[10].s64 = ctx.r[6].s32 as i64;
	// 8312EC04: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EC08: 54A9103A  slwi r9, r5, 2
	ctx.r[9].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8312EC0C: 396B0F80  addi r11, r11, 0xf80
	ctx.r[11].s64 = ctx.r[11].s64 + 3968;
	// 8312EC10: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8312EC14: 7CA9582E  lwzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8312EC18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8312EC1C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312EC20: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8312EC24: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8312EC28: C00B964C  lfs f0, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8312EC2C: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8312EC30: 4BFFDA69  bl 0x8312c698
	ctx.lr = 0x8312EC34;
	sub_8312C698(ctx, base);
	// 8312EC34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8312EC38: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8312EC3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312EC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EC48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8312EC50 size=216
    let mut pc: u32 = 0x8312EC50;
    'dispatch: loop {
        match pc {
            0x8312EC50 => {
    //   block [0x8312EC50..0x8312ED28)
	// 8312EC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EC58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312EC5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EC60: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8312EC64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EC68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8312EC6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8312EC70: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EC74: 93FE0024  stw r31, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8312EC78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312EC7C: 419A0090  beq cr6, 0x8312ed0c
	if ctx.cr[6].eq {
	pc = 0x8312ED0C; continue 'dispatch;
	}
	// 8312EC80: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EC84: C3EB0498  lfs f31, 0x498(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1176 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8312EC88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8312EC8C: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EC90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312EC94: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312EC98: 4BFFDA01  bl 0x8312c698
	ctx.lr = 0x8312EC9C;
	sub_8312C698(ctx, base);
	// 8312EC9C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8312ECA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312ECA4: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ECA8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312ECAC: 4BFFD9ED  bl 0x8312c698
	ctx.lr = 0x8312ECB0;
	sub_8312C698(ctx, base);
	// 8312ECB0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8312ECB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312ECB8: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ECBC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312ECC0: 4BFFD9D9  bl 0x8312c698
	ctx.lr = 0x8312ECC4;
	sub_8312C698(ctx, base);
	// 8312ECC4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8312ECC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312ECCC: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ECD0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312ECD4: 4BFFD9C5  bl 0x8312c698
	ctx.lr = 0x8312ECD8;
	sub_8312C698(ctx, base);
	// 8312ECD8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8312ECDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312ECE0: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ECE4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312ECE8: 4BFFD9B1  bl 0x8312c698
	ctx.lr = 0x8312ECEC;
	sub_8312C698(ctx, base);
	// 8312ECEC: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8312ECF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312ECF4: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ECF8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8312ECFC: 4BFFD99D  bl 0x8312c698
	ctx.lr = 0x8312ED00;
	sub_8312C698(ctx, base);
	// 8312ED00: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8312ED04: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 8312ED08: 4198FF80  blt cr6, 0x8312ec88
	if ctx.cr[6].lt {
	pc = 0x8312EC88; continue 'dispatch;
	}
	// 8312ED0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312ED10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312ED14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312ED18: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8312ED1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312ED20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312ED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312ED28 size=88
    let mut pc: u32 = 0x8312ED28;
    'dispatch: loop {
        match pc {
            0x8312ED28 => {
    //   block [0x8312ED28..0x8312ED80)
	// 8312ED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312ED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312ED30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312ED34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312ED38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312ED3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312ED40: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ED44: 4BFFD5ED  bl 0x8312c330
	ctx.lr = 0x8312ED48;
	sub_8312C330(ctx, base);
	// 8312ED48: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8312ED4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312ED50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312ED54: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8312ED58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312ED5C: 4E800421  bctrl
	ctx.lr = 0x8312ED60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312ED60: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8312ED64: 546BF87E  srwi r11, r3, 1
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312ED68: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8312ED6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312ED70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312ED74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312ED78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312ED7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312ED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312ED80 size=8
    let mut pc: u32 = 0x8312ED80;
    'dispatch: loop {
        match pc {
            0x8312ED80 => {
    //   block [0x8312ED80..0x8312ED88)
	// 8312ED80: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312ED84: 4BFFD7E4  b 0x8312c568
	sub_8312C568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312ED88 size=96
    let mut pc: u32 = 0x8312ED88;
    'dispatch: loop {
        match pc {
            0x8312ED88 => {
    //   block [0x8312ED88..0x8312EDE8)
	// 8312ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312ED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312ED90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312ED94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312ED98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312ED9C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EDA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312EDA4: 41820008  beq 0x8312edac
	if ctx.cr[0].eq {
	pc = 0x8312EDAC; continue 'dispatch;
	}
	// 8312EDA8: 4BFFCFB1  bl 0x8312bd58
	ctx.lr = 0x8312EDAC;
	sub_8312BD58(ctx, base);
	// 8312EDAC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8312EDB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312EDB4: 41820010  beq 0x8312edc4
	if ctx.cr[0].eq {
	pc = 0x8312EDC4; continue 'dispatch;
	}
	// 8312EDB8: 4BFFDAB9  bl 0x8312c870
	ctx.lr = 0x8312EDBC;
	sub_8312C870(ctx, base);
	// 8312EDBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312EDC0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8312EDC4: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 8312EDC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312EDCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312EDD0: 48079411  bl 0x831a81e0
	ctx.lr = 0x8312EDD4;
	sub_831A81E0(ctx, base);
	// 8312EDD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312EDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EDE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312EDE8 size=156
    let mut pc: u32 = 0x8312EDE8;
    'dispatch: loop {
        match pc {
            0x8312EDE8 => {
    //   block [0x8312EDE8..0x8312EE84)
	// 8312EDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EDF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312EDF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EDFC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312EE00: 3BCB7728  addi r30, r11, 0x7728
	ctx.r[30].s64 = ctx.r[11].s64 + 30504;
	// 8312EE04: 391E0500  addi r8, r30, 0x500
	ctx.r[8].s64 = ctx.r[30].s64 + 1280;
	// 8312EE08: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8312EE0C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312EE10: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8312EE14: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8312EE18: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8312EE1C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312EE20: 4082FFE8  bne 0x8312ee08
	if !ctx.cr[0].eq {
	pc = 0x8312EE08; continue 'dispatch;
	}
	// 8312EE24: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8312EE28: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312EE2C: 409A0040  bne cr6, 0x8312ee6c
	if !ctx.cr[6].eq {
	pc = 0x8312EE6C; continue 'dispatch;
	}
	// 8312EE30: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8312EE34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312EE38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312EE3C: 419A000C  beq cr6, 0x8312ee48
	if ctx.cr[6].eq {
	pc = 0x8312EE48; continue 'dispatch;
	}
	// 8312EE40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312EE44: 4BFFFF45  bl 0x8312ed88
	ctx.lr = 0x8312EE48;
	sub_8312ED88(ctx, base);
	// 8312EE48: 3BFF0028  addi r31, r31, 0x28
	ctx.r[31].s64 = ctx.r[31].s64 + 40;
	// 8312EE4C: 397E0500  addi r11, r30, 0x500
	ctx.r[11].s64 = ctx.r[30].s64 + 1280;
	// 8312EE50: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8312EE54: 4198FFE0  blt cr6, 0x8312ee34
	if ctx.cr[6].lt {
	pc = 0x8312EE34; continue 'dispatch;
	}
	// 8312EE58: 4BFFC4C9  bl 0x8312b320
	ctx.lr = 0x8312EE5C;
	sub_8312B320(ctx, base);
	// 8312EE5C: 38A00500  li r5, 0x500
	ctx.r[5].s64 = 1280;
	// 8312EE60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312EE64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312EE68: 48079379  bl 0x831a81e0
	ctx.lr = 0x8312EE6C;
	sub_831A81E0(ctx, base);
	// 8312EE6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312EE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EE78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312EE7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EE88 size=8
    let mut pc: u32 = 0x8312EE88;
    'dispatch: loop {
        match pc {
            0x8312EE88 => {
    //   block [0x8312EE88..0x8312EE90)
	// 8312EE88: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 8312EE8C: 8224BE50  lwz r17, -0x41b0(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16816 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312EE90 size=200
    let mut pc: u32 = 0x8312EE90;
    'dispatch: loop {
        match pc {
            0x8312EE90 => {
    //   block [0x8312EE90..0x8312EF58)
	// 8312EE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EE98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312EE9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EEA0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8312EEA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EEA8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EEAC: 396B0FA0  addi r11, r11, 0xfa0
	ctx.r[11].s64 = ctx.r[11].s64 + 4000;
	// 8312EEB0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8312EEB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312EEB8: 916A7C2C  stw r11, 0x7c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31788 as u32), ctx.r[11].u32 ) };
	// 8312EEBC: 409A0018  bne cr6, 0x8312eed4
	if !ctx.cr[6].eq {
	pc = 0x8312EED4; continue 'dispatch;
	}
	// 8312EEC0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EEC4: 386B1054  addi r3, r11, 0x1054
	ctx.r[3].s64 = ctx.r[11].s64 + 4180;
	// 8312EEC8: 4BFF7379  bl 0x83126240
	ctx.lr = 0x8312EECC;
	sub_83126240(ctx, base);
	// 8312EECC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312EED0: 48000070  b 0x8312ef40
	pc = 0x8312EF40; continue 'dispatch;
	// 8312EED4: 2B040020  cmplwi cr6, r4, 0x20
	ctx.cr[6].compare_u32(ctx.r[4].u32, 32 as u32, &mut ctx.xer);
	// 8312EED8: 40980010  bge cr6, 0x8312eee8
	if !ctx.cr[6].lt {
	pc = 0x8312EEE8; continue 'dispatch;
	}
	// 8312EEDC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EEE0: 386B1024  addi r3, r11, 0x1024
	ctx.r[3].s64 = ctx.r[11].s64 + 4132;
	// 8312EEE4: 4BFFFFE4  b 0x8312eec8
	pc = 0x8312EEC8; continue 'dispatch;
	// 8312EEE8: 39630003  addi r11, r3, 3
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	// 8312EEEC: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8312EEF0: 557E003A  rlwinm r30, r11, 0, 0, 0x1d
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8312EEF4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8312EEF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312EEFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312EF00: 480792E1  bl 0x831a81e0
	ctx.lr = 0x8312EF04;
	sub_831A81E0(ctx, base);
	// 8312EF04: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EF08: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EF0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312EF10: 48113ACD  bl 0x832429dc
	ctx.lr = 0x8312EF14;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8312EF14: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EF18: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EF1C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EF20: 4800001C  b 0x8312ef3c
	pc = 0x8312EF3C; continue 'dispatch;
	// 8312EF24: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EF28: 386B0FF0  addi r3, r11, 0xff0
	ctx.r[3].s64 = ctx.r[11].s64 + 4080;
	// 8312EF2C: 4BFF7315  bl 0x83126240
	ctx.lr = 0x8312EF30;
	sub_83126240(ctx, base);
	// 8312EF30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312EF34: 83DF0050  lwz r30, 0x50(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312EF38: 48000008  b 0x8312ef40
	pc = 0x8312EF40; continue 'dispatch;
	// 8312EF3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312EF40: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8312EF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EF4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312EF50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EF58 size=12
    let mut pc: u32 = 0x8312EF58;
    'dispatch: loop {
        match pc {
            0x8312EF58 => {
    //   block [0x8312EF58..0x8312EF64)
	// 8312EF58: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312EF5C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EF68 size=8
    let mut pc: u32 = 0x8312EF68;
    'dispatch: loop {
        match pc {
            0x8312EF68 => {
    //   block [0x8312EF68..0x8312EF70)
	// 8312EF68: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 8312EF6C: 8224BE68  lwz r17, -0x4198(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312EF70 size=124
    let mut pc: u32 = 0x8312EF70;
    'dispatch: loop {
        match pc {
            0x8312EF70 => {
    //   block [0x8312EF70..0x8312EFEC)
	// 8312EF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312EF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312EF78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312EF7C: 3BE1FFA0  addi r31, r1, -0x60
	ctx.r[31].s64 = ctx.r[1].s64 + -96;
	// 8312EF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312EF84: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 8312EF88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312EF8C: 409A0014  bne cr6, 0x8312efa0
	if !ctx.cr[6].eq {
	pc = 0x8312EFA0; continue 'dispatch;
	}
	// 8312EF90: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EF94: 386B10B4  addi r3, r11, 0x10b4
	ctx.r[3].s64 = ctx.r[11].s64 + 4276;
	// 8312EF98: 4BFF72A9  bl 0x83126240
	ctx.lr = 0x8312EF9C;
	sub_83126240(ctx, base);
	// 8312EF9C: 4800003C  b 0x8312efd8
	pc = 0x8312EFD8; continue 'dispatch;
	// 8312EFA0: 60000000  nop
	// 8312EFA4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EFA8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EFAC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EFB0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EFB4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EFB8: 48000014  b 0x8312efcc
	pc = 0x8312EFCC; continue 'dispatch;
	// 8312EFBC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312EFC0: 386B1084  addi r3, r11, 0x1084
	ctx.r[3].s64 = ctx.r[11].s64 + 4228;
	// 8312EFC4: 4BFF727D  bl 0x83126240
	ctx.lr = 0x8312EFC8;
	sub_83126240(ctx, base);
	// 8312EFC8: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8312EFCC: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8312EFD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312EFD4: 4807920D  bl 0x831a81e0
	ctx.lr = 0x8312EFD8;
	sub_831A81E0(ctx, base);
	// 8312EFD8: 383F0060  addi r1, r31, 0x60
	ctx.r[1].s64 = ctx.r[31].s64 + 96;
	// 8312EFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312EFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312EFE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312EFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EFEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EFEC size=12
    let mut pc: u32 = 0x8312EFEC;
    'dispatch: loop {
        match pc {
            0x8312EFEC => {
    //   block [0x8312EFEC..0x8312EFF8)
	// 8312EFEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312EFF0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312EFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312EFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312EFF8 size=8
    let mut pc: u32 = 0x8312EFF8;
    'dispatch: loop {
        match pc {
            0x8312EFF8 => {
    //   block [0x8312EFF8..0x8312F000)
	// 8312EFF8: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 8312EFFC: 8224BE80  lwz r17, -0x4180(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16768 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F000 size=132
    let mut pc: u32 = 0x8312F000;
    'dispatch: loop {
        match pc {
            0x8312F000 => {
    //   block [0x8312F000..0x8312F084)
	// 8312F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F008: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312F00C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F010: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8312F014: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F018: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312F01C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312F020: 409A0018  bne cr6, 0x8312f038
	if !ctx.cr[6].eq {
	pc = 0x8312F038; continue 'dispatch;
	}
	// 8312F024: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F028: 386B110C  addi r3, r11, 0x110c
	ctx.r[3].s64 = ctx.r[11].s64 + 4364;
	// 8312F02C: 4BFF7215  bl 0x83126240
	ctx.lr = 0x8312F030;
	sub_83126240(ctx, base);
	// 8312F030: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F034: 48000038  b 0x8312f06c
	pc = 0x8312F06C; continue 'dispatch;
	// 8312F038: 60000000  nop
	// 8312F03C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F040: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F044: 48113929  bl 0x8324296c
	ctx.lr = 0x8312F048;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8312F048: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F04C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F050: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F054: 48000014  b 0x8312f068
	pc = 0x8312F068; continue 'dispatch;
	// 8312F058: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F05C: 386B10E0  addi r3, r11, 0x10e0
	ctx.r[3].s64 = ctx.r[11].s64 + 4320;
	// 8312F060: 4BFF71E1  bl 0x83126240
	ctx.lr = 0x8312F064;
	sub_83126240(ctx, base);
	// 8312F064: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8312F068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F06C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8312F070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F078: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312F07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F084(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F084 size=12
    let mut pc: u32 = 0x8312F084;
    'dispatch: loop {
        match pc {
            0x8312F084 => {
    //   block [0x8312F084..0x8312F090)
	// 8312F084: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312F088: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F090 size=8
    let mut pc: u32 = 0x8312F090;
    'dispatch: loop {
        match pc {
            0x8312F090 => {
    //   block [0x8312F090..0x8312F098)
	// 8312F090: 8324280C  lwz r25, 0x280c(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10252 as u32) ) } as u64;
	// 8312F094: 8224BE98  lwz r17, -0x4168(r4)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16744 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F098 size=132
    let mut pc: u32 = 0x8312F098;
    'dispatch: loop {
        match pc {
            0x8312F098 => {
    //   block [0x8312F098..0x8312F11C)
	// 8312F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312F0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F0A8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8312F0AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F0B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8312F0B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312F0B8: 409A0018  bne cr6, 0x8312f0d0
	if !ctx.cr[6].eq {
	pc = 0x8312F0D0; continue 'dispatch;
	}
	// 8312F0BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F0C0: 386B1160  addi r3, r11, 0x1160
	ctx.r[3].s64 = ctx.r[11].s64 + 4448;
	// 8312F0C4: 4BFF717D  bl 0x83126240
	ctx.lr = 0x8312F0C8;
	sub_83126240(ctx, base);
	// 8312F0C8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F0CC: 48000038  b 0x8312f104
	pc = 0x8312F104; continue 'dispatch;
	// 8312F0D0: 60000000  nop
	// 8312F0D4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F0D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F0DC: 48113881  bl 0x8324295c
	ctx.lr = 0x8312F0E0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8312F0E0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F0E4: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F0E8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F0EC: 48000014  b 0x8312f100
	pc = 0x8312F100; continue 'dispatch;
	// 8312F0F0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F0F4: 386B1134  addi r3, r11, 0x1134
	ctx.r[3].s64 = ctx.r[11].s64 + 4404;
	// 8312F0F8: 4BFF7149  bl 0x83126240
	ctx.lr = 0x8312F0FC;
	sub_83126240(ctx, base);
	// 8312F0FC: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8312F100: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F104: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8312F108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F110: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312F114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F11C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F11C size=12
    let mut pc: u32 = 0x8312F11C;
    'dispatch: loop {
        match pc {
            0x8312F11C => {
    //   block [0x8312F11C..0x8312F128)
	// 8312F11C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312F120: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8312F124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F128 size=156
    let mut pc: u32 = 0x8312F128;
    'dispatch: loop {
        match pc {
            0x8312F128 => {
    //   block [0x8312F128..0x8312F1C4)
	// 8312F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312F134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F13C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8312F140: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312F144: 419A0058  beq cr6, 0x8312f19c
	if ctx.cr[6].eq {
	pc = 0x8312F19C; continue 'dispatch;
	}
	// 8312F148: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312F14C: 419A0050  beq cr6, 0x8312f19c
	if ctx.cr[6].eq {
	pc = 0x8312F19C; continue 'dispatch;
	}
	// 8312F150: 2B040118  cmplwi cr6, r4, 0x118
	ctx.cr[6].compare_u32(ctx.r[4].u32, 280 as u32, &mut ctx.xer);
	// 8312F154: 41980030  blt cr6, 0x8312f184
	if ctx.cr[6].lt {
	pc = 0x8312F184; continue 'dispatch;
	}
	// 8312F158: 39630003  addi r11, r3, 3
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	// 8312F15C: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 8312F160: 557F003A  rlwinm r31, r11, 0, 0, 0x1d
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8312F164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312F168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F16C: 48079075  bl 0x831a81e0
	ctx.lr = 0x8312F170;
	sub_831A81E0(ctx, base);
	// 8312F170: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8312F174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312F178: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312F17C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8312F180: 4800002C  b 0x8312f1ac
	pc = 0x8312F1AC; continue 'dispatch;
	// 8312F184: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F188: 386B11B0  addi r3, r11, 0x11b0
	ctx.r[3].s64 = ctx.r[11].s64 + 4528;
	// 8312F18C: 4BFF70B5  bl 0x83126240
	ctx.lr = 0x8312F190;
	sub_83126240(ctx, base);
	// 8312F190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312F194: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312F198: 48000010  b 0x8312f1a8
	pc = 0x8312F1A8; continue 'dispatch;
	// 8312F19C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F1A0: 386B1188  addi r3, r11, 0x1188
	ctx.r[3].s64 = ctx.r[11].s64 + 4488;
	// 8312F1A4: 4BFF709D  bl 0x83126240
	ctx.lr = 0x8312F1A8;
	sub_83126240(ctx, base);
	// 8312F1A8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F1AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312F1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F1B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312F1BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F1C8 size=44
    let mut pc: u32 = 0x8312F1C8;
    'dispatch: loop {
        match pc {
            0x8312F1C8 => {
    //   block [0x8312F1C8..0x8312F1F4)
	// 8312F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F1D4: 38A00118  li r5, 0x118
	ctx.r[5].s64 = 280;
	// 8312F1D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312F1DC: 48079005  bl 0x831a81e0
	ctx.lr = 0x8312F1E0;
	sub_831A81E0(ctx, base);
	// 8312F1E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312F1E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F1F8 size=68
    let mut pc: u32 = 0x8312F1F8;
    'dispatch: loop {
        match pc {
            0x8312F1F8 => {
    //   block [0x8312F1F8..0x8312F23C)
	// 8312F1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F204: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8312F208: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312F20C: 409A0018  bne cr6, 0x8312f224
	if !ctx.cr[6].eq {
	pc = 0x8312F224; continue 'dispatch;
	}
	// 8312F210: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F214: 386B11D8  addi r3, r11, 0x11d8
	ctx.r[3].s64 = ctx.r[11].s64 + 4568;
	// 8312F218: 4BFF7029  bl 0x83126240
	ctx.lr = 0x8312F21C;
	sub_83126240(ctx, base);
	// 8312F21C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F220: 4800000C  b 0x8312f22c
	pc = 0x8312F22C; continue 'dispatch;
	// 8312F224: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312F228: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8312F22C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F240 size=72
    let mut pc: u32 = 0x8312F240;
    'dispatch: loop {
        match pc {
            0x8312F240 => {
    //   block [0x8312F240..0x8312F288)
	// 8312F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F24C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8312F250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312F254: 409A0018  bne cr6, 0x8312f26c
	if !ctx.cr[6].eq {
	pc = 0x8312F26C; continue 'dispatch;
	}
	// 8312F258: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F25C: 386B1200  addi r3, r11, 0x1200
	ctx.r[3].s64 = ctx.r[11].s64 + 4608;
	// 8312F260: 4BFF6FE1  bl 0x83126240
	ctx.lr = 0x8312F264;
	sub_83126240(ctx, base);
	// 8312F264: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F268: 48000010  b 0x8312f278
	pc = 0x8312F278; continue 'dispatch;
	// 8312F26C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312F270: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8312F274: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 8312F278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F288 size=448
    let mut pc: u32 = 0x8312F288;
    'dispatch: loop {
        match pc {
            0x8312F288 => {
    //   block [0x8312F288..0x8312F448)
	// 8312F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F28C: 48078EC1  bl 0x831a814c
	ctx.lr = 0x8312F290;
	sub_831A8130(ctx, base);
	// 8312F290: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F298: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 8312F29C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8312F2A0: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 8312F2A4: 839F0110  lwz r28, 0x110(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 8312F2A8: 2F1C0102  cmpwi cr6, r28, 0x102
	ctx.cr[6].compare_i32(ctx.r[28].s32, 258, &mut ctx.xer);
	// 8312F2AC: 41980018  blt cr6, 0x8312f2c4
	if ctx.cr[6].lt {
	pc = 0x8312F2C4; continue 'dispatch;
	}
	// 8312F2B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F2B4: 386B122C  addi r3, r11, 0x122c
	ctx.r[3].s64 = ctx.r[11].s64 + 4652;
	// 8312F2B8: 4BFF6F89  bl 0x83126240
	ctx.lr = 0x8312F2BC;
	sub_83126240(ctx, base);
	// 8312F2BC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F2C0: 48000180  b 0x8312f440
	pc = 0x8312F440; continue 'dispatch;
	// 8312F2C4: 7FDCEA14  add r30, r28, r29
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 8312F2C8: 3AE0005C  li r23, 0x5c
	ctx.r[23].s64 = 92;
	// 8312F2CC: 3B00002A  li r24, 0x2a
	ctx.r[24].s64 = 42;
	// 8312F2D0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8312F2D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312F2D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8312F2DC: 7EFCE9AE  stbx r23, r28, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32), ctx.r[23].u8) };
	// 8312F2E0: 9B1E0001  stb r24, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 8312F2E4: 9B5E0002  stb r26, 2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[26].u8 ) };
	// 8312F2E8: 48096101  bl 0x831c53e8
	ctx.lr = 0x8312F2EC;
	sub_831C53E8(ctx, base);
	// 8312F2EC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8312F2F0: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 8312F2F4: 419A0148  beq cr6, 0x8312f43c
	if ctx.cr[6].eq {
	pc = 0x8312F43C; continue 'dispatch;
	}
	// 8312F2F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F2FC: 3B2B1228  addi r25, r11, 0x1228
	ctx.r[25].s64 = ctx.r[11].s64 + 4648;
	// 8312F300: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312F304: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312F308: 418200BC  beq 0x8312f3c4
	if ctx.cr[0].eq {
	pc = 0x8312F3C4; continue 'dispatch;
	}
	// 8312F30C: 8961007C  lbz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8312F310: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 8312F314: 419A010C  beq cr6, 0x8312f420
	if ctx.cr[6].eq {
	pc = 0x8312F420; continue 'dispatch;
	}
	// 8312F318: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 8312F31C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8312F320: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F324: 4BA9DDB5  bl 0x82bcd0d8
	ctx.lr = 0x8312F328;
	sub_82BCD0D8(ctx, base);
	// 8312F328: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8312F32C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312F330: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F334: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F338: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8312F33C: 409AFFF4  bne cr6, 0x8312f330
	if !ctx.cr[6].eq {
	pc = 0x8312F330; continue 'dispatch;
	}
	// 8312F340: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312F344: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F348: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8312F34C: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8312F350: 554A07BD  rlwinm. r10, r10, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8312F354: 4182003C  beq 0x8312f390
	if ctx.cr[0].eq {
	pc = 0x8312F390; continue 'dispatch;
	}
	// 8312F358: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312F35C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312F360: 41820030  beq 0x8312f390
	if ctx.cr[0].eq {
	pc = 0x8312F390; continue 'dispatch;
	}
	// 8312F364: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312F368: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312F36C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312F370: 4E800421  bctrl
	ctx.lr = 0x8312F374;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312F374: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F378: 418000C8  blt 0x8312f440
	if ctx.cr[0].lt {
	pc = 0x8312F440; continue 'dispatch;
	}
	// 8312F37C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8312F380: 419A0010  beq cr6, 0x8312f390
	if ctx.cr[6].eq {
	pc = 0x8312F390; continue 'dispatch;
	}
	// 8312F384: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F388: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F38C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312F390: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8312F394: 419A001C  beq cr6, 0x8312f3b0
	if ctx.cr[6].eq {
	pc = 0x8312F3B0; continue 'dispatch;
	}
	// 8312F398: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8312F39C: 3895FFFF  addi r4, r21, -1
	ctx.r[4].s64 = ctx.r[21].s64 + -1;
	// 8312F3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F3A4: 4BFFFEE5  bl 0x8312f288
	ctx.lr = 0x8312F3A8;
	sub_8312F288(ctx, base);
	// 8312F3A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F3AC: 41800094  blt 0x8312f440
	if ctx.cr[0].lt {
	pc = 0x8312F440; continue 'dispatch;
	}
	// 8312F3B0: 9AFE0000  stb r23, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[23].u8 ) };
	// 8312F3B4: 9B1E0001  stb r24, 1(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(1 as u32), ctx.r[24].u8 ) };
	// 8312F3B8: 9B5E0002  stb r26, 2(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(2 as u32), ctx.r[26].u8 ) };
	// 8312F3BC: 939F0110  stw r28, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[28].u32 ) };
	// 8312F3C0: 48000060  b 0x8312f420
	pc = 0x8312F420; continue 'dispatch;
	// 8312F3C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F3C8: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312F3CC: 41820054  beq 0x8312f420
	if ctx.cr[0].eq {
	pc = 0x8312F420; continue 'dispatch;
	}
	// 8312F3D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312F3D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8312F3D8: 419A0048  beq cr6, 0x8312f420
	if ctx.cr[6].eq {
	pc = 0x8312F420; continue 'dispatch;
	}
	// 8312F3DC: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 8312F3E0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8312F3E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F3E8: 4BA9DCF1  bl 0x82bcd0d8
	ctx.lr = 0x8312F3EC;
	sub_82BCD0D8(ctx, base);
	// 8312F3EC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312F3F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312F3F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312F3F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312F3FC: 4E800421  bctrl
	ctx.lr = 0x8312F400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312F400: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F404: 4180003C  blt 0x8312f440
	if ctx.cr[0].lt {
	pc = 0x8312F440; continue 'dispatch;
	}
	// 8312F408: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8312F40C: 9B5E0000  stb r26, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u8 ) };
	// 8312F410: 419A0010  beq cr6, 0x8312f420
	if ctx.cr[6].eq {
	pc = 0x8312F420; continue 'dispatch;
	}
	// 8312F414: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F418: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F41C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312F420: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312F424: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8312F428: 48096051  bl 0x831c5478
	ctx.lr = 0x8312F42C;
	sub_831C5478(ctx, base);
	// 8312F42C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F430: 4082FED0  bne 0x8312f300
	if !ctx.cr[0].eq {
	pc = 0x8312F300; continue 'dispatch;
	}
	// 8312F434: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8312F438: 4BA9D5E9  bl 0x82bcca20
	ctx.lr = 0x8312F43C;
	sub_82BCCA20(ctx, base);
	// 8312F43C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312F440: 382101F0  addi r1, r1, 0x1f0
	ctx.r[1].s64 = ctx.r[1].s64 + 496;
	// 8312F444: 48078D58  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F448 size=184
    let mut pc: u32 = 0x8312F448;
    'dispatch: loop {
        match pc {
            0x8312F448 => {
    //   block [0x8312F448..0x8312F500)
	// 8312F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F44C: 48078D19  bl 0x831a8164
	ctx.lr = 0x8312F450;
	sub_831A8130(ctx, base);
	// 8312F450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F458: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8312F45C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8312F460: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312F464: 419A0084  beq cr6, 0x8312f4e8
	if ctx.cr[6].eq {
	pc = 0x8312F4E8; continue 'dispatch;
	}
	// 8312F468: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8312F46C: 419A007C  beq cr6, 0x8312f4e8
	if ctx.cr[6].eq {
	pc = 0x8312F4E8; continue 'dispatch;
	}
	// 8312F470: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8312F474: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8312F478: 419A0008  beq cr6, 0x8312f480
	if ctx.cr[6].eq {
	pc = 0x8312F480; continue 'dispatch;
	}
	// 8312F47C: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8312F480: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8312F484: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8312F488: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F48C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F490: 48000481  bl 0x8312f910
	ctx.lr = 0x8312F494;
	sub_8312F910(ctx, base);
	// 8312F494: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8312F498: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312F49C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F4A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F4A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8312F4A8: 409AFFF4  bne cr6, 0x8312f49c
	if !ctx.cr[6].eq {
	pc = 0x8312F49C; continue 'dispatch;
	}
	// 8312F4AC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312F4B0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8312F4B4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312F4B8: 7D4BFA14  add r10, r11, r31
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8312F4BC: 892A000B  lbz r9, 0xb(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(11 as u32) ) } as u64;
	// 8312F4C0: 2B09005C  cmplwi cr6, r9, 0x5c
	ctx.cr[6].compare_u32(ctx.r[9].u32, 92 as u32, &mut ctx.xer);
	// 8312F4C4: 409A000C  bne cr6, 0x8312f4d0
	if !ctx.cr[6].eq {
	pc = 0x8312F4D0; continue 'dispatch;
	}
	// 8312F4C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8312F4CC: 9B8A000B  stb r28, 0xb(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(11 as u32), ctx.r[28].u8 ) };
	// 8312F4D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312F4D4: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8312F4D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8312F4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F4E0: 4BFFFDA9  bl 0x8312f288
	ctx.lr = 0x8312F4E4;
	sub_8312F288(ctx, base);
	// 8312F4E4: 48000014  b 0x8312f4f8
	pc = 0x8312F4F8; continue 'dispatch;
	// 8312F4E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F4EC: 386B1254  addi r3, r11, 0x1254
	ctx.r[3].s64 = ctx.r[11].s64 + 4692;
	// 8312F4F0: 4BFF6D51  bl 0x83126240
	ctx.lr = 0x8312F4F4;
	sub_83126240(ctx, base);
	// 8312F4F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8312F4F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312F4FC: 48078CB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F500 size=52
    let mut pc: u32 = 0x8312F500;
    'dispatch: loop {
        match pc {
            0x8312F500 => {
    //   block [0x8312F500..0x8312F534)
	// 8312F500: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F504: 386B7C30  addi r3, r11, 0x7c30
	ctx.r[3].s64 = ctx.r[11].s64 + 31792;
	// 8312F508: 39030108  addi r8, r3, 0x108
	ctx.r[8].s64 = ctx.r[3].s64 + 264;
	// 8312F50C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8312F510: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312F514: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8312F518: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8312F51C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8312F520: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312F524: 4082FFE8  bne 0x8312f50c
	if !ctx.cr[0].eq {
	pc = 0x8312F50C; continue 'dispatch;
	}
	// 8312F528: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8312F52C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312F530: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F534 size=12
    let mut pc: u32 = 0x8312F534;
    'dispatch: loop {
        match pc {
            0x8312F534 => {
    //   block [0x8312F534..0x8312F540)
	// 8312F534: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8312F538: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312F53C: 48078CA4  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F540 size=4
    let mut pc: u32 = 0x8312F540;
    'dispatch: loop {
        match pc {
            0x8312F540 => {
    //   block [0x8312F540..0x8312F544)
	// 8312F540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F548 size=52
    let mut pc: u32 = 0x8312F548;
    'dispatch: loop {
        match pc {
            0x8312F548 => {
    //   block [0x8312F548..0x8312F57C)
	// 8312F548: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F54C: 386B7C30  addi r3, r11, 0x7c30
	ctx.r[3].s64 = ctx.r[11].s64 + 31792;
	// 8312F550: 39030108  addi r8, r3, 0x108
	ctx.r[8].s64 = ctx.r[3].s64 + 264;
	// 8312F554: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8312F558: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312F55C: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8312F560: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8312F564: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8312F568: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8312F56C: 4082FFE8  bne 0x8312f554
	if !ctx.cr[0].eq {
	pc = 0x8312F554; continue 'dispatch;
	}
	// 8312F570: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8312F574: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312F578: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F57C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F57C size=12
    let mut pc: u32 = 0x8312F57C;
    'dispatch: loop {
        match pc {
            0x8312F57C => {
    //   block [0x8312F57C..0x8312F588)
	// 8312F57C: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8312F580: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312F584: 48078C5C  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F588 size=4
    let mut pc: u32 = 0x8312F588;
    'dispatch: loop {
        match pc {
            0x8312F588 => {
    //   block [0x8312F588..0x8312F58C)
	// 8312F588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F590 size=392
    let mut pc: u32 = 0x8312F590;
    'dispatch: loop {
        match pc {
            0x8312F590 => {
    //   block [0x8312F590..0x8312F718)
	// 8312F590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F594: 48078BD9  bl 0x831a816c
	ctx.lr = 0x8312F598;
	sub_831A8130(ctx, base);
	// 8312F598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F59C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8312F5A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312F5A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8312F5A8: 409A001C  bne cr6, 0x8312f5c4
	if !ctx.cr[6].eq {
	pc = 0x8312F5C4; continue 'dispatch;
	}
	// 8312F5AC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F5B0: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F5B4: 38AB7C30  addi r5, r11, 0x7c30
	ctx.r[5].s64 = ctx.r[11].s64 + 31792;
	// 8312F5B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F5BC: 48000355  bl 0x8312f910
	ctx.lr = 0x8312F5C0;
	sub_8312F910(ctx, base);
	// 8312F5C0: 48000150  b 0x8312f710
	pc = 0x8312F710; continue 'dispatch;
	// 8312F5C4: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F5C8: 7D690775  extsb. r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8312F5CC: 4182FFE0  beq 0x8312f5ac
	if ctx.cr[0].eq {
	pc = 0x8312F5AC; continue 'dispatch;
	}
	// 8312F5D0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8312F5D4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312F5D8: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F5DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F5E0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8312F5E4: 409AFFF4  bne cr6, 0x8312f5d8
	if !ctx.cr[6].eq {
	pc = 0x8312F5D8; continue 'dispatch;
	}
	// 8312F5E8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312F5EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8312F5F0: 556B003F  rotlwi. r11, r11, 0
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312F5F4: 40810018  ble 0x8312f60c
	if !ctx.cr[0].gt {
	pc = 0x8312F60C; continue 'dispatch;
	}
	// 8312F5F8: 7D4BE8AE  lbzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8312F5FC: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 8312F600: 419A000C  beq cr6, 0x8312f60c
	if ctx.cr[6].eq {
	pc = 0x8312F60C; continue 'dispatch;
	}
	// 8312F604: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312F608: 4181FFF0  bgt 0x8312f5f8
	if ctx.cr[0].gt {
	pc = 0x8312F5F8; continue 'dispatch;
	}
	// 8312F60C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312F610: 409A0070  bne cr6, 0x8312f680
	if !ctx.cr[6].eq {
	pc = 0x8312F680; continue 'dispatch;
	}
	// 8312F614: 2F09005C  cmpwi cr6, r9, 0x5c
	ctx.cr[6].compare_i32(ctx.r[9].s32, 92, &mut ctx.xer);
	// 8312F618: 419A0024  beq cr6, 0x8312f63c
	if ctx.cr[6].eq {
	pc = 0x8312F63C; continue 'dispatch;
	}
	// 8312F61C: 2F09002F  cmpwi cr6, r9, 0x2f
	ctx.cr[6].compare_i32(ctx.r[9].s32, 47, &mut ctx.xer);
	// 8312F620: 419A001C  beq cr6, 0x8312f63c
	if ctx.cr[6].eq {
	pc = 0x8312F63C; continue 'dispatch;
	}
	// 8312F624: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F628: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F62C: 38AB7C30  addi r5, r11, 0x7c30
	ctx.r[5].s64 = ctx.r[11].s64 + 31792;
	// 8312F630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F634: 480002DD  bl 0x8312f910
	ctx.lr = 0x8312F638;
	sub_8312F910(ctx, base);
	// 8312F638: 48000034  b 0x8312f66c
	pc = 0x8312F66C; continue 'dispatch;
	// 8312F63C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F640: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F644: 3BEB7C30  addi r31, r11, 0x7c30
	ctx.r[31].s64 = ctx.r[11].s64 + 31792;
	// 8312F648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F64C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8312F650: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8312F654: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 8312F658: 480002E9  bl 0x8312f940
	ctx.lr = 0x8312F65C;
	sub_8312F940(ctx, base);
	// 8312F65C: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 8312F660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8312F664: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8312F668: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8312F66C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F670: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F674: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312F678: 48000329  bl 0x8312f9a0
	ctx.lr = 0x8312F67C;
	sub_8312F9A0(ctx, base);
	// 8312F67C: 48000048  b 0x8312f6c4
	pc = 0x8312F6C4; continue 'dispatch;
	// 8312F680: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F684: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8312F688: 386B127C  addi r3, r11, 0x127c
	ctx.r[3].s64 = ctx.r[11].s64 + 4732;
	// 8312F68C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8312F690: 48080969  bl 0x831afff8
	ctx.lr = 0x8312F694;
	sub_831AFFF8(ctx, base);
	// 8312F694: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F698: 4082001C  bne 0x8312f6b4
	if !ctx.cr[0].eq {
	pc = 0x8312F6B4; continue 'dispatch;
	}
	// 8312F69C: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 8312F6A0: 38BD0004  addi r5, r29, 4
	ctx.r[5].s64 = ctx.r[29].s64 + 4;
	// 8312F6A4: 38800103  li r4, 0x103
	ctx.r[4].s64 = 259;
	// 8312F6A8: 387E0001  addi r3, r30, 1
	ctx.r[3].s64 = ctx.r[30].s64 + 1;
	// 8312F6AC: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8312F6B0: 48000010  b 0x8312f6c0
	pc = 0x8312F6C0; continue 'dispatch;
	// 8312F6B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312F6B8: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F6BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F6C0: 48000251  bl 0x8312f910
	ctx.lr = 0x8312F6C4;
	sub_8312F910(ctx, base);
	// 8312F6C4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8312F6C8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312F6CC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F6D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F6D4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8312F6D8: 409AFFF4  bne cr6, 0x8312f6cc
	if !ctx.cr[6].eq {
	pc = 0x8312F6CC; continue 'dispatch;
	}
	// 8312F6DC: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312F6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312F6E4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8312F6E8: 554A003F  rotlwi. r10, r10, 0
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(0)) as u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8312F6EC: 41800024  blt 0x8312f710
	if ctx.cr[0].lt {
	pc = 0x8312F710; continue 'dispatch;
	}
	// 8312F6F0: 7D2BF0AE  lbzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8312F6F4: 2B09002F  cmplwi cr6, r9, 0x2f
	ctx.cr[6].compare_u32(ctx.r[9].u32, 47 as u32, &mut ctx.xer);
	// 8312F6F8: 409A000C  bne cr6, 0x8312f704
	if !ctx.cr[6].eq {
	pc = 0x8312F704; continue 'dispatch;
	}
	// 8312F6FC: 3920005C  li r9, 0x5c
	ctx.r[9].s64 = 92;
	// 8312F700: 7D2BF1AE  stbx r9, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u8) };
	// 8312F704: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F708: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8312F70C: 4099FFE4  ble cr6, 0x8312f6f0
	if !ctx.cr[6].gt {
	pc = 0x8312F6F0; continue 'dispatch;
	}
	// 8312F710: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312F714: 48078AA8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F718 size=100
    let mut pc: u32 = 0x8312F718;
    'dispatch: loop {
        match pc {
            0x8312F718 => {
    //   block [0x8312F718..0x8312F77C)
	// 8312F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F728: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F72C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F730: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8312F734: 386B1284  addi r3, r11, 0x1284
	ctx.r[3].s64 = ctx.r[11].s64 + 4740;
	// 8312F738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312F73C: 480808BD  bl 0x831afff8
	ctx.lr = 0x8312F740;
	sub_831AFFF8(ctx, base);
	// 8312F740: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F744: 41820024  beq 0x8312f768
	if ctx.cr[0].eq {
	pc = 0x8312F768; continue 'dispatch;
	}
	// 8312F748: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F74C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8312F750: 386B127C  addi r3, r11, 0x127c
	ctx.r[3].s64 = ctx.r[11].s64 + 4732;
	// 8312F754: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8312F758: 480808A1  bl 0x831afff8
	ctx.lr = 0x8312F75C;
	sub_831AFFF8(ctx, base);
	// 8312F75C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 8312F760: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8312F764: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8312F768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F780 size=372
    let mut pc: u32 = 0x8312F780;
    'dispatch: loop {
        match pc {
            0x8312F780 => {
    //   block [0x8312F780..0x8312F8F4)
	// 8312F780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F784: 480789E5  bl 0x831a8168
	ctx.lr = 0x8312F788;
	sub_831A8130(ctx, base);
	// 8312F788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F78C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F790: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8312F794: 3BCB7C30  addi r30, r11, 0x7c30
	ctx.r[30].s64 = ctx.r[11].s64 + 31792;
	// 8312F798: 38A00104  li r5, 0x104
	ctx.r[5].s64 = 260;
	// 8312F79C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312F7A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F7A4: 48078A3D  bl 0x831a81e0
	ctx.lr = 0x8312F7A8;
	sub_831A81E0(ctx, base);
	// 8312F7A8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8312F7AC: 409A0020  bne cr6, 0x8312f7cc
	if !ctx.cr[6].eq {
	pc = 0x8312F7CC; continue 'dispatch;
	}
	// 8312F7B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F7B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F7B8: 388B12E0  addi r4, r11, 0x12e0
	ctx.r[4].s64 = ctx.r[11].s64 + 4832;
	// 8312F7BC: 4BA9D91D  bl 0x82bcd0d8
	ctx.lr = 0x8312F7C0;
	sub_82BCD0D8(ctx, base);
	// 8312F7C0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8312F7C4: 917E0104  stw r11, 0x104(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 8312F7C8: 48000124  b 0x8312f8ec
	pc = 0x8312F8EC; continue 'dispatch;
	// 8312F7CC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8312F7D0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312F7D4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F7D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F7DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8312F7E0: 409AFFF4  bne cr6, 0x8312f7d4
	if !ctx.cr[6].eq {
	pc = 0x8312F7D4; continue 'dispatch;
	}
	// 8312F7E4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312F7E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8312F7EC: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8312F7F0: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 8312F7F4: 40980014  bge cr6, 0x8312f808
	if !ctx.cr[6].lt {
	pc = 0x8312F808; continue 'dispatch;
	}
	// 8312F7F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F7FC: 386B12BC  addi r3, r11, 0x12bc
	ctx.r[3].s64 = ctx.r[11].s64 + 4796;
	// 8312F800: 4BFF6A41  bl 0x83126240
	ctx.lr = 0x8312F804;
	sub_83126240(ctx, base);
	// 8312F804: 480000E8  b 0x8312f8ec
	pc = 0x8312F8EC; continue 'dispatch;
	// 8312F808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312F80C: 3B80005C  li r28, 0x5c
	ctx.r[28].s64 = 92;
	// 8312F810: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8312F814: 41980044  blt cr6, 0x8312f858
	if ctx.cr[6].lt {
	pc = 0x8312F858; continue 'dispatch;
	}
	// 8312F818: 7D4BE8AE  lbzx r10, r11, r29
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8312F81C: 2B0A002F  cmplwi cr6, r10, 0x2f
	ctx.cr[6].compare_u32(ctx.r[10].u32, 47 as u32, &mut ctx.xer);
	// 8312F820: 409A0008  bne cr6, 0x8312f828
	if !ctx.cr[6].eq {
	pc = 0x8312F828; continue 'dispatch;
	}
	// 8312F824: 7F8BE9AE  stbx r28, r11, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u8) };
	// 8312F828: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F82C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8312F830: 4099FFE8  ble cr6, 0x8312f818
	if !ctx.cr[6].gt {
	pc = 0x8312F818; continue 'dispatch;
	}
	// 8312F834: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8312F838: 41980020  blt cr6, 0x8312f858
	if ctx.cr[6].lt {
	pc = 0x8312F858; continue 'dispatch;
	}
	// 8312F83C: 7D7FE8AE  lbzx r11, r31, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8312F840: 2B0B003A  cmplwi cr6, r11, 0x3a
	ctx.cr[6].compare_u32(ctx.r[11].u32, 58 as u32, &mut ctx.xer);
	// 8312F844: 419A000C  beq cr6, 0x8312f850
	if ctx.cr[6].eq {
	pc = 0x8312F850; continue 'dispatch;
	}
	// 8312F848: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8312F84C: 4080FFF0  bge 0x8312f83c
	if !ctx.cr[0].lt {
	pc = 0x8312F83C; continue 'dispatch;
	}
	// 8312F850: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8312F854: 41990010  bgt cr6, 0x8312f864
	if ctx.cr[6].gt {
	pc = 0x8312F864; continue 'dispatch;
	}
	// 8312F858: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F85C: 386B1288  addi r3, r11, 0x1288
	ctx.r[3].s64 = ctx.r[11].s64 + 4744;
	// 8312F860: 4BFFFFA0  b 0x8312f800
	pc = 0x8312F800; continue 'dispatch;
	// 8312F864: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8312F868: 4BFFFEB1  bl 0x8312f718
	ctx.lr = 0x8312F86C;
	sub_8312F718(ctx, base);
	// 8312F86C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312F870: 40820024  bne 0x8312f894
	if !ctx.cr[0].eq {
	pc = 0x8312F894; continue 'dispatch;
	}
	// 8312F874: 39600044  li r11, 0x44
	ctx.r[11].s64 = 68;
	// 8312F878: 7CBFEA14  add r5, r31, r29
	ctx.r[5].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 8312F87C: 38800103  li r4, 0x103
	ctx.r[4].s64 = 259;
	// 8312F880: 387E0001  addi r3, r30, 1
	ctx.r[3].s64 = ctx.r[30].s64 + 1;
	// 8312F884: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8312F888: 48000089  bl 0x8312f910
	ctx.lr = 0x8312F88C;
	sub_8312F910(ctx, base);
	// 8312F88C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8312F890: 48000014  b 0x8312f8a4
	pc = 0x8312F8A4; continue 'dispatch;
	// 8312F894: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8312F898: 38800104  li r4, 0x104
	ctx.r[4].s64 = 260;
	// 8312F89C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8312F8A0: 48000071  bl 0x8312f910
	ctx.lr = 0x8312F8A4;
	sub_8312F910(ctx, base);
	// 8312F8A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8312F8A8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 8312F8AC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312F8B0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312F8B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8312F8B8: 409AFFF4  bne cr6, 0x8312f8ac
	if !ctx.cr[6].eq {
	pc = 0x8312F8AC; continue 'dispatch;
	}
	// 8312F8BC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8312F8C0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8312F8C4: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8312F8C8: 7D4BF214  add r10, r11, r30
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8312F8CC: 892AFFFF  lbz r9, -1(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8312F8D0: 2B09005C  cmplwi cr6, r9, 0x5c
	ctx.cr[6].compare_u32(ctx.r[9].u32, 92 as u32, &mut ctx.xer);
	// 8312F8D4: 419A0014  beq cr6, 0x8312f8e8
	if ctx.cr[6].eq {
	pc = 0x8312F8E8; continue 'dispatch;
	}
	// 8312F8D8: 9B8A0000  stb r28, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u8 ) };
	// 8312F8DC: 393E0001  addi r9, r30, 1
	ctx.r[9].s64 = ctx.r[30].s64 + 1;
	// 8312F8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8312F8E4: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 8312F8E8: 93FE0104  stw r31, 0x104(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 8312F8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312F8F0: 480788C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312F8F8 size=20
    let mut pc: u32 = 0x8312F8F8;
    'dispatch: loop {
        match pc {
            0x8312F8F8 => {
    //   block [0x8312F8F8..0x8312F90C)
	// 8312F8F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312F8FC: 386B12E8  addi r3, r11, 0x12e8
	ctx.r[3].s64 = ctx.r[11].s64 + 4840;
	// 8312F900: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312F904: 906B7D3C  stw r3, 0x7d3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32060 as u32), ctx.r[3].u32 ) };
	// 8312F908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F910 size=48
    let mut pc: u32 = 0x8312F910;
    'dispatch: loop {
        match pc {
            0x8312F910 => {
    //   block [0x8312F910..0x8312F940)
	// 8312F910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F91C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F924: 4808115D  bl 0x831b0a80
	ctx.lr = 0x8312F928;
	sub_831B0A80(ctx, base);
	// 8312F928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F92C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F940 size=48
    let mut pc: u32 = 0x8312F940;
    'dispatch: loop {
        match pc {
            0x8312F940 => {
    //   block [0x8312F940..0x8312F970)
	// 8312F940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F948: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F94C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F954: 4808338D  bl 0x831b2ce0
	ctx.lr = 0x8312F958;
	sub_831B2CE0(ctx, base);
	// 8312F958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F95C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F970 size=48
    let mut pc: u32 = 0x8312F970;
    'dispatch: loop {
        match pc {
            0x8312F970 => {
    //   block [0x8312F970..0x8312F9A0)
	// 8312F970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F978: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F97C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F980: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F984: 480834CD  bl 0x831b2e50
	ctx.lr = 0x8312F988;
	sub_831B2E50(ctx, base);
	// 8312F988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F98C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F9A0 size=48
    let mut pc: u32 = 0x8312F9A0;
    'dispatch: loop {
        match pc {
            0x8312F9A0 => {
    //   block [0x8312F9A0..0x8312F9D0)
	// 8312F9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F9A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312F9AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F9B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312F9B4: 4808321D  bl 0x831b2bd0
	ctx.lr = 0x8312F9B8;
	sub_831B2BD0(ctx, base);
	// 8312F9B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312F9BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312F9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312F9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312F9C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312F9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312F9D0 size=68
    let mut pc: u32 = 0x8312F9D0;
    'dispatch: loop {
        match pc {
            0x8312F9D0 => {
    //   block [0x8312F9D0..0x8312FA14)
	// 8312F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312F9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312F9D8: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 8312F9DC: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 8312F9E0: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 8312F9E4: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 8312F9E8: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 8312F9EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312F9F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8312F9F4: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 8312F9F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8312F9FC: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312FA00: 480831C1  bl 0x831b2bc0
	ctx.lr = 0x8312FA04;
	sub_831B2BC0(ctx, base);
	// 8312FA04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312FA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8312FA18 size=4
    let mut pc: u32 = 0x8312FA18;
    'dispatch: loop {
        match pc {
            0x8312FA18 => {
    //   block [0x8312FA18..0x8312FA1C)
	// 8312FA18: 480831A8  b 0x831b2bc0
	sub_831B2BC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FA20 size=120
    let mut pc: u32 = 0x8312FA20;
    'dispatch: loop {
        match pc {
            0x8312FA20 => {
    //   block [0x8312FA20..0x8312FA98)
	// 8312FA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FA28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312FA2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FA30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FA34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312FA38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312FA3C: 419A0040  beq cr6, 0x8312fa7c
	if ctx.cr[6].eq {
	pc = 0x8312FA7C; continue 'dispatch;
	}
	// 8312FA40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312FA44: 419A0038  beq cr6, 0x8312fa7c
	if ctx.cr[6].eq {
	pc = 0x8312FA7C; continue 'dispatch;
	}
	// 8312FA48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8312FA4C: 4BA9D1A5  bl 0x82bccbf0
	ctx.lr = 0x8312FA50;
	sub_82BCCBF0(ctx, base);
	// 8312FA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312FA54: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8312FA58: 409A0010  bne cr6, 0x8312fa68
	if !ctx.cr[6].eq {
	pc = 0x8312FA68; continue 'dispatch;
	}
	// 8312FA5C: 4BA9E29D  bl 0x82bcdcf8
	ctx.lr = 0x8312FA60;
	sub_82BCDCF8(ctx, base);
	// 8312FA60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312FA64: 40820018  bne 0x8312fa7c
	if !ctx.cr[0].eq {
	pc = 0x8312FA7C; continue 'dispatch;
	}
	// 8312FA68: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312FA6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312FA70: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8312FA74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312FA78: 48000008  b 0x8312fa80
	pc = 0x8312FA80; continue 'dispatch;
	// 8312FA7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312FA80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312FA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FA8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312FA90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FA98 size=136
    let mut pc: u32 = 0x8312FA98;
    'dispatch: loop {
        match pc {
            0x8312FA98 => {
    //   block [0x8312FA98..0x8312FB20)
	// 8312FA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FAA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312FAA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FAA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FAAC: F8810088  std r4, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[4].u64 ) };
	// 8312FAB0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8312FAB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312FAB8: 409A000C  bne cr6, 0x8312fac4
	if !ctx.cr[6].eq {
	pc = 0x8312FAC4; continue 'dispatch;
	}
	// 8312FABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312FAC0: 48000048  b 0x8312fb08
	pc = 0x8312FB08; continue 'dispatch;
	// 8312FAC4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 8312FAC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8312FACC: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8312FAD0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8312FAD4: 48094EA5  bl 0x831c4978
	ctx.lr = 0x8312FAD8;
	sub_831C4978(ctx, base);
	// 8312FAD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312FADC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8312FAE0: 409A0010  bne cr6, 0x8312faf0
	if !ctx.cr[6].eq {
	pc = 0x8312FAF0; continue 'dispatch;
	}
	// 8312FAE4: 4BA9E215  bl 0x82bcdcf8
	ctx.lr = 0x8312FAE8;
	sub_82BCDCF8(ctx, base);
	// 8312FAE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312FAEC: 4082FFD0  bne 0x8312fabc
	if !ctx.cr[0].eq {
	pc = 0x8312FABC; continue 'dispatch;
	}
	// 8312FAF0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8312FAF4: 419A0010  beq cr6, 0x8312fb04
	if ctx.cr[6].eq {
	pc = 0x8312FB04; continue 'dispatch;
	}
	// 8312FAF8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312FAFC: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8312FB00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312FB04: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8312FB08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312FB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FB14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312FB18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FB20 size=96
    let mut pc: u32 = 0x8312FB20;
    'dispatch: loop {
        match pc {
            0x8312FB20 => {
    //   block [0x8312FB20..0x8312FB80)
	// 8312FB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FB28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FB2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FB30: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FB34: 394B1340  addi r10, r11, 0x1340
	ctx.r[10].s64 = ctx.r[11].s64 + 4928;
	// 8312FB38: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312FB3C: 3BEB7D40  addi r31, r11, 0x7d40
	ctx.r[31].s64 = ctx.r[11].s64 + 32064;
	// 8312FB40: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 8312FB44: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8312FB48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312FB4C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8312FB50: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312FB54: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8312FB58: 409A0014  bne cr6, 0x8312fb6c
	if !ctx.cr[6].eq {
	pc = 0x8312FB6C; continue 'dispatch;
	}
	// 8312FB5C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8312FB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312FB64: 4BFFF32D  bl 0x8312ee90
	ctx.lr = 0x8312FB68;
	sub_8312EE90(ctx, base);
	// 8312FB68: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8312FB6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312FB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FB78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FB80 size=84
    let mut pc: u32 = 0x8312FB80;
    'dispatch: loop {
        match pc {
            0x8312FB80 => {
    //   block [0x8312FB80..0x8312FBD4)
	// 8312FB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FB88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FB8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FB90: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8312FB94: 3BEB7D64  addi r31, r11, 0x7d64
	ctx.r[31].s64 = ctx.r[11].s64 + 32100;
	// 8312FB98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FB9C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FBA0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8312FBA4: 4082001C  bne 0x8312fbc0
	if !ctx.cr[0].eq {
	pc = 0x8312FBC0; continue 'dispatch;
	}
	// 8312FBA8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312FBAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312FBB0: 419A0010  beq cr6, 0x8312fbc0
	if ctx.cr[6].eq {
	pc = 0x8312FBC0; continue 'dispatch;
	}
	// 8312FBB4: 4BFFF3BD  bl 0x8312ef70
	ctx.lr = 0x8312FBB8;
	sub_8312EF70(ctx, base);
	// 8312FBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312FBBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8312FBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8312FBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FBCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FBD8 size=200
    let mut pc: u32 = 0x8312FBD8;
    'dispatch: loop {
        match pc {
            0x8312FBD8 => {
    //   block [0x8312FBD8..0x8312FCA0)
	// 8312FBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FBE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312FBE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FBE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FBEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312FBF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8312FBF4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 8312FBF8: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8312FBFC: 48000068  b 0x8312fc64
	pc = 0x8312FC64; continue 'dispatch;
	// 8312FC00: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312FC04: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8312FC08: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8312FC0C: 419A001C  beq cr6, 0x8312fc28
	if ctx.cr[6].eq {
	pc = 0x8312FC28; continue 'dispatch;
	}
	// 8312FC10: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312FC14: 4BA9E2ED  bl 0x82bcdf00
	ctx.lr = 0x8312FC18;
	sub_82BCDF00(ctx, base);
	// 8312FC18: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8312FC1C: 419A000C  beq cr6, 0x8312fc28
	if ctx.cr[6].eq {
	pc = 0x8312FC28; continue 'dispatch;
	}
	// 8312FC20: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8312FC24: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8312FC28: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312FC2C: 4BFFF3D5  bl 0x8312f000
	ctx.lr = 0x8312FC30;
	sub_8312F000(ctx, base);
	// 8312FC30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FC34: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312FC38: 409A0024  bne cr6, 0x8312fc5c
	if !ctx.cr[6].eq {
	pc = 0x8312FC5C; continue 'dispatch;
	}
	// 8312FC3C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8312FC40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312FC44: 41820010  beq 0x8312fc54
	if ctx.cr[0].eq {
	pc = 0x8312FC54; continue 'dispatch;
	}
	// 8312FC48: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8312FC4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8312FC50: 4E800421  bctrl
	ctx.lr = 0x8312FC54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8312FC54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8312FC58: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8312FC5C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312FC60: 4BFFF439  bl 0x8312f098
	ctx.lr = 0x8312FC64;
	sub_8312F098(ctx, base);
	// 8312FC64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8312FC68: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FC6C: 4BA9E095  bl 0x82bcdd00
	ctx.lr = 0x8312FC70;
	sub_82BCDD00(ctx, base);
	// 8312FC70: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FC74: 480953FD  bl 0x831c5070
	ctx.lr = 0x8312FC78;
	sub_831C5070(ctx, base);
	// 8312FC78: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8312FC7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FC80: 419AFF80  beq cr6, 0x8312fc00
	if ctx.cr[6].eq {
	pc = 0x8312FC00; continue 'dispatch;
	}
	// 8312FC84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312FC88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312FC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FC94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312FC98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FCA0 size=336
    let mut pc: u32 = 0x8312FCA0;
    'dispatch: loop {
        match pc {
            0x8312FCA0 => {
    //   block [0x8312FCA0..0x8312FDF0)
	// 8312FCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FCA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FCAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FCB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8312FCB4: 409A0018  bne cr6, 0x8312fccc
	if !ctx.cr[6].eq {
	pc = 0x8312FCCC; continue 'dispatch;
	}
	// 8312FCB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FCBC: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 8312FCC0: 4BFF6581  bl 0x83126240
	ctx.lr = 0x8312FCC4;
	sub_83126240(ctx, base);
	// 8312FCC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312FCC8: 48000114  b 0x8312fddc
	pc = 0x8312FDDC; continue 'dispatch;
	// 8312FCCC: 2B040058  cmplwi cr6, r4, 0x58
	ctx.cr[6].compare_u32(ctx.r[4].u32, 88 as u32, &mut ctx.xer);
	// 8312FCD0: 40980010  bge cr6, 0x8312fce0
	if !ctx.cr[6].lt {
	pc = 0x8312FCE0; continue 'dispatch;
	}
	// 8312FCD4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FCD8: 386B1448  addi r3, r11, 0x1448
	ctx.r[3].s64 = ctx.r[11].s64 + 5192;
	// 8312FCDC: 4BFFFFE4  b 0x8312fcc0
	pc = 0x8312FCC0; continue 'dispatch;
	// 8312FCE0: 39630003  addi r11, r3, 3
	ctx.r[11].s64 = ctx.r[3].s64 + 3;
	// 8312FCE4: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8312FCE8: 557F003A  rlwinm r31, r11, 0, 0, 0x1d
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8312FCEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312FCF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312FCF4: 480784ED  bl 0x831a81e0
	ctx.lr = 0x8312FCF8;
	sub_831A81E0(ctx, base);
	// 8312FCF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8312FCFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8312FD00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8312FD04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312FD08: 48094DE1  bl 0x831c4ae8
	ctx.lr = 0x8312FD0C;
	sub_831C4AE8(ctx, base);
	// 8312FD0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312FD10: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8312FD14: 40820010  bne 0x8312fd24
	if !ctx.cr[0].eq {
	pc = 0x8312FD24; continue 'dispatch;
	}
	// 8312FD18: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FD1C: 386B141C  addi r3, r11, 0x141c
	ctx.r[3].s64 = ctx.r[11].s64 + 5148;
	// 8312FD20: 4BFFFFA0  b 0x8312fcc0
	pc = 0x8312FCC0; continue 'dispatch;
	// 8312FD24: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8312FD28: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8312FD2C: 4BFFF165  bl 0x8312ee90
	ctx.lr = 0x8312FD30;
	sub_8312EE90(ctx, base);
	// 8312FD30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312FD34: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8312FD38: 4082001C  bne 0x8312fd54
	if !ctx.cr[0].eq {
	pc = 0x8312FD54; continue 'dispatch;
	}
	// 8312FD3C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FD40: 386B13EC  addi r3, r11, 0x13ec
	ctx.r[3].s64 = ctx.r[11].s64 + 5100;
	// 8312FD44: 4BFF64FD  bl 0x83126240
	ctx.lr = 0x8312FD48;
	sub_83126240(ctx, base);
	// 8312FD48: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FD4C: 4BA9CCD5  bl 0x82bcca20
	ctx.lr = 0x8312FD50;
	sub_82BCCA20(ctx, base);
	// 8312FD50: 4BFFFF74  b 0x8312fcc4
	pc = 0x8312FCC4; continue 'dispatch;
	// 8312FD54: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8312FD58: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8312FD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8312FD60: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8312FD64: 38ABFBD8  addi r5, r11, -0x428
	ctx.r[5].s64 = ctx.r[11].s64 + -1064;
	// 8312FD68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8312FD6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8312FD70: 48094E19  bl 0x831c4b88
	ctx.lr = 0x8312FD74;
	sub_831C4B88(ctx, base);
	// 8312FD74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8312FD78: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8312FD7C: 4082002C  bne 0x8312fda8
	if !ctx.cr[0].eq {
	pc = 0x8312FDA8; continue 'dispatch;
	}
	// 8312FD80: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FD84: 386B13C4  addi r3, r11, 0x13c4
	ctx.r[3].s64 = ctx.r[11].s64 + 5060;
	// 8312FD88: 4BFF64B9  bl 0x83126240
	ctx.lr = 0x8312FD8C;
	sub_83126240(ctx, base);
	// 8312FD8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FD90: 4BA9CC91  bl 0x82bcca20
	ctx.lr = 0x8312FD94;
	sub_82BCCA20(ctx, base);
	// 8312FD94: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312FD98: 4BFFF1D9  bl 0x8312ef70
	ctx.lr = 0x8312FD9C;
	sub_8312EF70(ctx, base);
	// 8312FD9C: 4BFFFF28  b 0x8312fcc4
	pc = 0x8312FCC4; continue 'dispatch;
	// 8312FDA0: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 8312FDA4: 4BA9CB55  bl 0x82bcc8f8
	ctx.lr = 0x8312FDA8;
	sub_82BCC8F8(ctx, base);
	// 8312FDA8: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8312FDAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FDB0: 419AFFF0  beq cr6, 0x8312fda0
	if ctx.cr[6].eq {
	pc = 0x8312FDA0; continue 'dispatch;
	}
	// 8312FDB4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8312FDB8: 4BA9DFE1  bl 0x82bcdd98
	ctx.lr = 0x8312FDBC;
	sub_82BCDD98(ctx, base);
	// 8312FDBC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8312FDC0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8312FDC4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8312FDC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312FDCC: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8312FDD0: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8312FDD4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8312FDD8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8312FDDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312FDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FDE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FDF0 size=148
    let mut pc: u32 = 0x8312FDF0;
    'dispatch: loop {
        match pc {
            0x8312FDF0 => {
    //   block [0x8312FDF0..0x8312FE84)
	// 8312FDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FDF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8312FDFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FE00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FE04: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 8312FE08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312FE0C: 807E7D64  lwz r3, 0x7d64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8312FE10: 4BFFF1F1  bl 0x8312f000
	ctx.lr = 0x8312FE14;
	sub_8312F000(ctx, base);
	// 8312FE14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312FE18: 409A0018  bne cr6, 0x8312fe30
	if !ctx.cr[6].eq {
	pc = 0x8312FE30; continue 'dispatch;
	}
	// 8312FE1C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FE20: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 8312FE24: 4BFF641D  bl 0x83126240
	ctx.lr = 0x8312FE28;
	sub_83126240(ctx, base);
	// 8312FE28: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312FE2C: 48000034  b 0x8312fe60
	pc = 0x8312FE60; continue 'dispatch;
	// 8312FE30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312FE34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FE38: 409A0010  bne cr6, 0x8312fe48
	if !ctx.cr[6].eq {
	pc = 0x8312FE48; continue 'dispatch;
	}
	// 8312FE3C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FE40: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 8312FE44: 4BFFFFE0  b 0x8312fe24
	pc = 0x8312FE24; continue 'dispatch;
	// 8312FE48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FE4C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312FE50: 409A000C  bne cr6, 0x8312fe5c
	if !ctx.cr[6].eq {
	pc = 0x8312FE5C; continue 'dispatch;
	}
	// 8312FE54: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FE58: 48095339  bl 0x831c5190
	ctx.lr = 0x8312FE5C;
	sub_831C5190(ctx, base);
	// 8312FE5C: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FE60: 807E7D64  lwz r3, 0x7d64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8312FE64: 4BFFF235  bl 0x8312f098
	ctx.lr = 0x8312FE68;
	sub_8312F098(ctx, base);
	// 8312FE68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312FE6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8312FE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8312FE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8312FE78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8312FE7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8312FE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FE88 size=200
    let mut pc: u32 = 0x8312FE88;
    'dispatch: loop {
        match pc {
            0x8312FE88 => {
    //   block [0x8312FE88..0x8312FF50)
	// 8312FE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FE8C: 480782DD  bl 0x831a8168
	ctx.lr = 0x8312FE90;
	sub_831A8130(ctx, base);
	// 8312FE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FE94: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 8312FE98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312FE9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8312FEA0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8312FEA4: 807C7D64  lwz r3, 0x7d64(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8312FEA8: 4BFFF159  bl 0x8312f000
	ctx.lr = 0x8312FEAC;
	sub_8312F000(ctx, base);
	// 8312FEAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312FEB0: 409A0018  bne cr6, 0x8312fec8
	if !ctx.cr[6].eq {
	pc = 0x8312FEC8; continue 'dispatch;
	}
	// 8312FEB4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FEB8: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 8312FEBC: 4BFF6385  bl 0x83126240
	ctx.lr = 0x8312FEC0;
	sub_83126240(ctx, base);
	// 8312FEC0: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 8312FEC4: 48000078  b 0x8312ff3c
	pc = 0x8312FF3C; continue 'dispatch;
	// 8312FEC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312FECC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FED0: 409A0010  bne cr6, 0x8312fee0
	if !ctx.cr[6].eq {
	pc = 0x8312FEE0; continue 'dispatch;
	}
	// 8312FED4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FED8: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 8312FEDC: 4BFFFFE0  b 0x8312febc
	pc = 0x8312FEBC; continue 'dispatch;
	// 8312FEE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FEE4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FEE8: 409AFFD8  bne cr6, 0x8312fec0
	if !ctx.cr[6].eq {
	pc = 0x8312FEC0; continue 'dispatch;
	}
	// 8312FEEC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312FEF0: 4BFFF111  bl 0x8312f000
	ctx.lr = 0x8312FEF4;
	sub_8312F000(ctx, base);
	// 8312FEF4: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8312FEF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8312FEFC: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 8312FF00: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 8312FF04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8312FF08: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8312FF0C: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 8312FF10: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8312FF14: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8312FF18: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8312FF1C: 409A0008  bne cr6, 0x8312ff24
	if !ctx.cr[6].eq {
	pc = 0x8312FF24; continue 'dispatch;
	}
	// 8312FF20: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 8312FF24: 83DF003C  lwz r30, 0x3c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8312FF28: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312FF2C: 4BFFF16D  bl 0x8312f098
	ctx.lr = 0x8312FF30;
	sub_8312F098(ctx, base);
	// 8312FF30: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FF34: 4809525D  bl 0x831c5190
	ctx.lr = 0x8312FF38;
	sub_831C5190(ctx, base);
	// 8312FF38: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8312FF3C: 807C7D64  lwz r3, 0x7d64(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8312FF40: 4BFFF159  bl 0x8312f098
	ctx.lr = 0x8312FF44;
	sub_8312F098(ctx, base);
	// 8312FF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8312FF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8312FF4C: 4807826C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8312FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8312FF50 size=240
    let mut pc: u32 = 0x8312FF50;
    'dispatch: loop {
        match pc {
            0x8312FF50 => {
    //   block [0x8312FF50..0x83130040)
	// 8312FF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8312FF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8312FF58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8312FF5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8312FF60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8312FF64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8312FF68: 409A0014  bne cr6, 0x8312ff7c
	if !ctx.cr[6].eq {
	pc = 0x8312FF7C; continue 'dispatch;
	}
	// 8312FF6C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FF70: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 8312FF74: 4BFF62CD  bl 0x83126240
	ctx.lr = 0x8312FF78;
	sub_83126240(ctx, base);
	// 8312FF78: 480000B4  b 0x8313002c
	pc = 0x8313002C; continue 'dispatch;
	// 8312FF7C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312FF80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FF84: 409A0010  bne cr6, 0x8312ff94
	if !ctx.cr[6].eq {
	pc = 0x8312FF94; continue 'dispatch;
	}
	// 8312FF88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FF8C: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 8312FF90: 4BFFFFE4  b 0x8312ff74
	pc = 0x8312FF74; continue 'dispatch;
	// 8312FF94: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8312FF98: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8312FF9C: 409A0090  bne cr6, 0x8313002c
	if !ctx.cr[6].eq {
	pc = 0x8313002C; continue 'dispatch;
	}
	// 8312FFA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FFA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FFA8: 419A0084  beq cr6, 0x8313002c
	if ctx.cr[6].eq {
	pc = 0x8313002C; continue 'dispatch;
	}
	// 8312FFAC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FFB0: 480951E1  bl 0x831c5190
	ctx.lr = 0x8312FFB4;
	sub_831C5190(ctx, base);
	// 8312FFB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8312FFB8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8312FFBC: 409A0014  bne cr6, 0x8312ffd0
	if !ctx.cr[6].eq {
	pc = 0x8312FFD0; continue 'dispatch;
	}
	// 8312FFC0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8312FFC4: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 8312FFC8: 4BFF6279  bl 0x83126240
	ctx.lr = 0x8312FFCC;
	sub_83126240(ctx, base);
	// 8312FFCC: 48000024  b 0x8312fff0
	pc = 0x8312FFF0; continue 'dispatch;
	// 8312FFD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FFD4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8312FFD8: 409A000C  bne cr6, 0x8312ffe4
	if !ctx.cr[6].eq {
	pc = 0x8312FFE4; continue 'dispatch;
	}
	// 8312FFDC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8312FFE0: 480951B1  bl 0x831c5190
	ctx.lr = 0x8312FFE4;
	sub_831C5190(ctx, base);
	// 8312FFE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FFE8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8312FFEC: 41820040  beq 0x8313002c
	if ctx.cr[0].eq {
	pc = 0x8313002C; continue 'dispatch;
	}
	// 8312FFF0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8312FFF4: 4BFFF00D  bl 0x8312f000
	ctx.lr = 0x8312FFF8;
	sub_8312F000(ctx, base);
	// 8312FFF8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8312FFFC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83130000: 409A0024  bne cr6, 0x83130024
	if !ctx.cr[6].eq {
	pc = 0x83130024; continue 'dispatch;
	}
	// 83130004: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83130008: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8313000C: 41820010  beq 0x8313001c
	if ctx.cr[0].eq {
	pc = 0x8313001C; continue 'dispatch;
	}
	// 83130010: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83130014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130018: 4E800421  bctrl
	ctx.lr = 0x8313001C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313001C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83130020: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83130024: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83130028: 4BFFF071  bl 0x8312f098
	ctx.lr = 0x8313002C;
	sub_8312F098(ctx, base);
	// 8313002C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83130030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130038: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313003C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130040 size=144
    let mut pc: u32 = 0x83130040;
    'dispatch: loop {
        match pc {
            0x83130040 => {
    //   block [0x83130040..0x831300D0)
	// 83130040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130044: 48078129  bl 0x831a816c
	ctx.lr = 0x83130048;
	sub_831A8130(ctx, base);
	// 83130048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313004C: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 83130050: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83130054: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83130058: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8313005C: 4BFFEFA5  bl 0x8312f000
	ctx.lr = 0x83130060;
	sub_8312F000(ctx, base);
	// 83130060: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83130064: 409A0014  bne cr6, 0x83130078
	if !ctx.cr[6].eq {
	pc = 0x83130078; continue 'dispatch;
	}
	// 83130068: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313006C: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 83130070: 4BFF61D1  bl 0x83126240
	ctx.lr = 0x83130074;
	sub_83126240(ctx, base);
	// 83130074: 48000044  b 0x831300b8
	pc = 0x831300B8; continue 'dispatch;
	// 83130078: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313007C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130080: 409A0010  bne cr6, 0x83130090
	if !ctx.cr[6].eq {
	pc = 0x83130090; continue 'dispatch;
	}
	// 83130084: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130088: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 8313008C: 4BFFFFE4  b 0x83130070
	pc = 0x83130070; continue 'dispatch;
	// 83130090: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83130094: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83130098: 419A000C  beq cr6, 0x831300a4
	if ctx.cr[6].eq {
	pc = 0x831300A4; continue 'dispatch;
	}
	// 8313009C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 831300A0: 4800001C  b 0x831300bc
	pc = 0x831300BC; continue 'dispatch;
	// 831300A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831300A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831300AC: 419AFFF0  beq cr6, 0x8313009c
	if ctx.cr[6].eq {
	pc = 0x8313009C; continue 'dispatch;
	}
	// 831300B0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831300B4: 480950DD  bl 0x831c5190
	ctx.lr = 0x831300B8;
	sub_831C5190(ctx, base);
	// 831300B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831300BC: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 831300C0: 4BFFEFD9  bl 0x8312f098
	ctx.lr = 0x831300C4;
	sub_8312F098(ctx, base);
	// 831300C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831300C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831300CC: 480780F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831300D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831300D0 size=132
    let mut pc: u32 = 0x831300D0;
    'dispatch: loop {
        match pc {
            0x831300D0 => {
    //   block [0x831300D0..0x83130154)
	// 831300D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831300D4: 48078099  bl 0x831a816c
	ctx.lr = 0x831300D8;
	sub_831A8130(ctx, base);
	// 831300D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831300DC: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 831300E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831300E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831300E8: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 831300EC: 4BFFEF15  bl 0x8312f000
	ctx.lr = 0x831300F0;
	sub_8312F000(ctx, base);
	// 831300F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831300F4: 409A0014  bne cr6, 0x83130108
	if !ctx.cr[6].eq {
	pc = 0x83130108; continue 'dispatch;
	}
	// 831300F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831300FC: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 83130100: 4BFF6141  bl 0x83126240
	ctx.lr = 0x83130104;
	sub_83126240(ctx, base);
	// 83130104: 48000040  b 0x83130144
	pc = 0x83130144; continue 'dispatch;
	// 83130108: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313010C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130110: 409A0010  bne cr6, 0x83130120
	if !ctx.cr[6].eq {
	pc = 0x83130120; continue 'dispatch;
	}
	// 83130114: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130118: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 8313011C: 4BFFFFE4  b 0x83130100
	pc = 0x83130100; continue 'dispatch;
	// 83130120: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83130124: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83130128: 419A001C  beq cr6, 0x83130144
	if ctx.cr[6].eq {
	pc = 0x83130144; continue 'dispatch;
	}
	// 8313012C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83130130: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83130134: 4BA9DBE5  bl 0x82bcdd18
	ctx.lr = 0x83130138;
	sub_82BCDD18(ctx, base);
	// 83130138: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313013C: 4BA9DC5D  bl 0x82bcdd98
	ctx.lr = 0x83130140;
	sub_82BCDD98(ctx, base);
	// 83130140: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 83130144: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 83130148: 4BFFEF51  bl 0x8312f098
	ctx.lr = 0x8313014C;
	sub_8312F098(ctx, base);
	// 8313014C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130150: 4807806C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130158 size=128
    let mut pc: u32 = 0x83130158;
    'dispatch: loop {
        match pc {
            0x83130158 => {
    //   block [0x83130158..0x831301D8)
	// 83130158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313015C: 48078011  bl 0x831a816c
	ctx.lr = 0x83130160;
	sub_831A8130(ctx, base);
	// 83130160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130164: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 83130168: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313016C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83130170: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 83130174: 4BFFEE8D  bl 0x8312f000
	ctx.lr = 0x83130178;
	sub_8312F000(ctx, base);
	// 83130178: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313017C: 409A0014  bne cr6, 0x83130190
	if !ctx.cr[6].eq {
	pc = 0x83130190; continue 'dispatch;
	}
	// 83130180: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130184: 386B13AC  addi r3, r11, 0x13ac
	ctx.r[3].s64 = ctx.r[11].s64 + 5036;
	// 83130188: 4BFF60B9  bl 0x83126240
	ctx.lr = 0x8313018C;
	sub_83126240(ctx, base);
	// 8313018C: 4800003C  b 0x831301c8
	pc = 0x831301C8; continue 'dispatch;
	// 83130190: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130194: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130198: 409A0010  bne cr6, 0x831301a8
	if !ctx.cr[6].eq {
	pc = 0x831301A8; continue 'dispatch;
	}
	// 8313019C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831301A0: 386B1390  addi r3, r11, 0x1390
	ctx.r[3].s64 = ctx.r[11].s64 + 5008;
	// 831301A4: 4BFFFFE4  b 0x83130188
	pc = 0x83130188; continue 'dispatch;
	// 831301A8: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831301AC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 831301B0: 419A0018  beq cr6, 0x831301c8
	if ctx.cr[6].eq {
	pc = 0x831301C8; continue 'dispatch;
	}
	// 831301B4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831301B8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 831301BC: 48094FD5  bl 0x831c5190
	ctx.lr = 0x831301C0;
	sub_831C5190(ctx, base);
	// 831301C0: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 831301C4: 4BA9C735  bl 0x82bcc8f8
	ctx.lr = 0x831301C8;
	sub_82BCC8F8(ctx, base);
	// 831301C8: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 831301CC: 4BFFEECD  bl 0x8312f098
	ctx.lr = 0x831301D0;
	sub_8312F098(ctx, base);
	// 831301D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831301D4: 48077FE8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831301D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831301D8 size=68
    let mut pc: u32 = 0x831301D8;
    'dispatch: loop {
        match pc {
            0x831301D8 => {
    //   block [0x831301D8..0x8313021C)
	// 831301D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831301DC: 48077F91  bl 0x831a816c
	ctx.lr = 0x831301E0;
	sub_831A8130(ctx, base);
	// 831301E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831301E4: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 831301E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831301EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831301F0: 807F7D64  lwz r3, 0x7d64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32100 as u32) ) } as u64;
	// 831301F4: 4BFFEE0D  bl 0x8312f000
	ctx.lr = 0x831301F8;
	sub_8312F000(ctx, base);
	// 831301F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831301FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83130200: 4BFFFAA1  bl 0x8312fca0
	ctx.lr = 0x83130204;
	sub_8312FCA0(ctx, base);
	// 83130204: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83130208: 807F7D64  lwz r3, 0x7d64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8313020C: 4BFFEE8D  bl 0x8312f098
	ctx.lr = 0x83130210;
	sub_8312F098(ctx, base);
	// 83130210: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83130214: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130218: 48077FA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130220 size=60
    let mut pc: u32 = 0x83130220;
    'dispatch: loop {
        match pc {
            0x83130220 => {
    //   block [0x83130220..0x8313025C)
	// 83130220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130224: 48077F49  bl 0x831a816c
	ctx.lr = 0x83130228;
	sub_831A8130(ctx, base);
	// 83130228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313022C: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83130230: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83130234: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83130238: 807F7D64  lwz r3, 0x7d64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32100 as u32) ) } as u64;
	// 8313023C: 4BFFEDC5  bl 0x8312f000
	ctx.lr = 0x83130240;
	sub_8312F000(ctx, base);
	// 83130240: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83130244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83130248: 4BFFFD09  bl 0x8312ff50
	ctx.lr = 0x8313024C;
	sub_8312FF50(ctx, base);
	// 8313024C: 807F7D64  lwz r3, 0x7d64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32100 as u32) ) } as u64;
	// 83130250: 4BFFEE49  bl 0x8312f098
	ctx.lr = 0x83130254;
	sub_8312F098(ctx, base);
	// 83130254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130258: 48077F64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130260 size=228
    let mut pc: u32 = 0x83130260;
    'dispatch: loop {
        match pc {
            0x83130260 => {
    //   block [0x83130260..0x83130344)
	// 83130260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130264: 48077F09  bl 0x831a816c
	ctx.lr = 0x83130268;
	sub_831A8130(ctx, base);
	// 83130268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313026C: 3FA08339  lis r29, -0x7cc7
	ctx.r[29].s64 = -2093416448;
	// 83130270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83130274: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 83130278: 4BFFED89  bl 0x8312f000
	ctx.lr = 0x8313027C;
	sub_8312F000(ctx, base);
	// 8313027C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83130280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130284: 419A0010  beq cr6, 0x83130294
	if ctx.cr[6].eq {
	pc = 0x83130294; continue 'dispatch;
	}
	// 83130288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313028C: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83130290: 4BFFFCC1  bl 0x8312ff50
	ctx.lr = 0x83130294;
	sub_8312FF50(ctx, base);
	// 83130294: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83130298: 48000018  b 0x831302b0
	pc = 0x831302B0; continue 'dispatch;
	// 8313029C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831302A0: 2B0B0103  cmplwi cr6, r11, 0x103
	ctx.cr[6].compare_u32(ctx.r[11].u32, 259 as u32, &mut ctx.xer);
	// 831302A4: 409A002C  bne cr6, 0x831302d0
	if !ctx.cr[6].eq {
	pc = 0x831302D0; continue 'dispatch;
	}
	// 831302A8: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 831302AC: 4BA9C64D  bl 0x82bcc8f8
	ctx.lr = 0x831302B0;
	sub_82BCC8F8(ctx, base);
	// 831302B0: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 831302B4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831302B8: 48094ED9  bl 0x831c5190
	ctx.lr = 0x831302BC;
	sub_831C5190(ctx, base);
	// 831302BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831302C0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831302C4: 4BA9DB95  bl 0x82bcde58
	ctx.lr = 0x831302C8;
	sub_82BCDE58(ctx, base);
	// 831302C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831302CC: 4082FFD0  bne 0x8313029c
	if !ctx.cr[0].eq {
	pc = 0x8313029C; continue 'dispatch;
	}
	// 831302D0: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 831302D4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831302D8: 4BA9DA29  bl 0x82bcdd00
	ctx.lr = 0x831302DC;
	sub_82BCDD00(ctx, base);
	// 831302DC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831302E0: 48094D91  bl 0x831c5070
	ctx.lr = 0x831302E4;
	sub_831C5070(ctx, base);
	// 831302E4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831302E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831302EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831302F0: 4182000C  beq 0x831302fc
	if ctx.cr[0].eq {
	pc = 0x831302FC; continue 'dispatch;
	}
	// 831302F4: 4BA9C72D  bl 0x82bcca20
	ctx.lr = 0x831302F8;
	sub_82BCCA20(ctx, base);
	// 831302F8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 831302FC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83130300: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83130304: 4182000C  beq 0x83130310
	if ctx.cr[0].eq {
	pc = 0x83130310; continue 'dispatch;
	}
	// 83130308: 4BFFEC69  bl 0x8312ef70
	ctx.lr = 0x8313030C;
	sub_8312EF70(ctx, base);
	// 8313030C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83130310: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83130314: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83130318: 4182000C  beq 0x83130324
	if ctx.cr[0].eq {
	pc = 0x83130324; continue 'dispatch;
	}
	// 8313031C: 4BA9C705  bl 0x82bcca20
	ctx.lr = 0x83130320;
	sub_82BCCA20(ctx, base);
	// 83130320: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83130324: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 83130328: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313032C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130330: 48077EB1  bl 0x831a81e0
	ctx.lr = 0x83130334;
	sub_831A81E0(ctx, base);
	// 83130334: 807D7D64  lwz r3, 0x7d64(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32100 as u32) ) } as u64;
	// 83130338: 4BFFED61  bl 0x8312f098
	ctx.lr = 0x8313033C;
	sub_8312F098(ctx, base);
	// 8313033C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83130340: 48077E7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83130348 size=288
    let mut pc: u32 = 0x83130348;
    'dispatch: loop {
        match pc {
            0x83130348 => {
    //   block [0x83130348..0x83130468)
	// 83130348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313034C: 48077E21  bl 0x831a816c
	ctx.lr = 0x83130350;
	sub_831A8130(ctx, base);
	// 83130350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83130358: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 8313035C: 4BFF8895  bl 0x83128bf0
	ctx.lr = 0x83130360;
	sub_83128BF0(ctx, base);
	// 83130360: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83130364: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130368: 4BFF8889  bl 0x83128bf0
	ctx.lr = 0x8313036C;
	sub_83128BF0(ctx, base);
	// 8313036C: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 83130370: 419A00E8  beq cr6, 0x83130458
	if ctx.cr[6].eq {
	pc = 0x83130458; continue 'dispatch;
	}
	// 83130374: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83130378: 419A00E0  beq cr6, 0x83130458
	if ctx.cr[6].eq {
	pc = 0x83130458; continue 'dispatch;
	}
	// 8313037C: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 83130380: 419A0078  beq cr6, 0x831303f8
	if ctx.cr[6].eq {
	pc = 0x831303F8; continue 'dispatch;
	}
	// 83130384: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83130388: 419A0070  beq cr6, 0x831303f8
	if ctx.cr[6].eq {
	pc = 0x831303F8; continue 'dispatch;
	}
	// 8313038C: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 83130390: 809F033C  lwz r4, 0x33c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 83130394: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83130398: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8313039C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 831303A0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831303A4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831303A8: 396B00D1  addi r11, r11, 0xd1
	ctx.r[11].s64 = ctx.r[11].s64 + 209;
	// 831303AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831303B0: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831303B4: 4BFF84BD  bl 0x83128870
	ctx.lr = 0x831303B8;
	sub_83128870(ctx, base);
	// 831303B8: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 831303BC: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831303C0: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831303C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831303C8: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 831303CC: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 831303D0: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831303D4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831303D8: 396B00D1  addi r11, r11, 0xd1
	ctx.r[11].s64 = ctx.r[11].s64 + 209;
	// 831303DC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831303E0: 7CABF82E  lwzx r5, r11, r31
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 831303E4: 4BFF8415  bl 0x831287f8
	ctx.lr = 0x831303E8;
	sub_831287F8(ctx, base);
	// 831303E8: 817F0340  lwz r11, 0x340(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(832 as u32) ) } as u64;
	// 831303EC: 907F033C  stw r3, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[3].u32 ) };
	// 831303F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831303F4: 917F0340  stw r11, 0x340(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(832 as u32), ctx.r[11].u32 ) };
	// 831303F8: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831303FC: 4BFF8355  bl 0x83128750
	ctx.lr = 0x83130400;
	sub_83128750(ctx, base);
	// 83130400: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83130404: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130408: 4BFF8349  bl 0x83128750
	ctx.lr = 0x8313040C;
	sub_83128750(ctx, base);
	// 8313040C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83130410: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130414: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 83130418: 796B5D24  sldi r11, r11, 0xb
	ctx.r[11].u64 = ctx.r[11].u64.wrapping_shl(11);
	ctx.r[11].u32 = ctx.r[11].u64 as u32;
	// 8313041C: F97F0330  std r11, 0x330(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(816 as u32), ctx.r[11].u64 ) };
	// 83130420: 4BFF87D1  bl 0x83128bf0
	ctx.lr = 0x83130424;
	sub_83128BF0(ctx, base);
	// 83130424: 817F0338  lwz r11, 0x338(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(824 as u32) ) } as u64;
	// 83130428: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313042C: 41980034  blt cr6, 0x83130460
	if ctx.cr[6].lt {
	pc = 0x83130460; continue 'dispatch;
	}
	// 83130430: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83130434: 4198002C  blt cr6, 0x83130460
	if ctx.cr[6].lt {
	pc = 0x83130460; continue 'dispatch;
	}
	// 83130438: 817F033C  lwz r11, 0x33c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(828 as u32) ) } as u64;
	// 8313043C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130440: 41990020  bgt cr6, 0x83130460
	if ctx.cr[6].gt {
	pc = 0x83130460; continue 'dispatch;
	}
	// 83130444: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83130448: 409A0018  bne cr6, 0x83130460
	if !ctx.cr[6].eq {
	pc = 0x83130460; continue 'dispatch;
	}
	// 8313044C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83130450: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83130454: 4800000C  b 0x83130460
	pc = 0x83130460; continue 'dispatch;
	// 83130458: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8313045C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83130460: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130464: 48077D58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130468 size=180
    let mut pc: u32 = 0x83130468;
    'dispatch: loop {
        match pc {
            0x83130468 => {
    //   block [0x83130468..0x8313051C)
	// 83130468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313046C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313047C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83130480: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 83130484: 4BFF876D  bl 0x83128bf0
	ctx.lr = 0x83130488;
	sub_83128BF0(ctx, base);
	// 83130488: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8313048C: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130490: 4BFF8761  bl 0x83128bf0
	ctx.lr = 0x83130494;
	sub_83128BF0(ctx, base);
	// 83130494: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 83130498: 419A0064  beq cr6, 0x831304fc
	if ctx.cr[6].eq {
	pc = 0x831304FC; continue 'dispatch;
	}
	// 8313049C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831304A0: 419A005C  beq cr6, 0x831304fc
	if ctx.cr[6].eq {
	pc = 0x831304FC; continue 'dispatch;
	}
	// 831304A4: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 831304A8: 4BFF8A01  bl 0x83128ea8
	ctx.lr = 0x831304AC;
	sub_83128EA8(ctx, base);
	// 831304AC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831304B0: 41820014  beq 0x831304c4
	if ctx.cr[0].eq {
	pc = 0x831304C4; continue 'dispatch;
	}
	// 831304B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831304B8: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 831304BC: 4BFF852D  bl 0x831289e8
	ctx.lr = 0x831304C0;
	sub_831289E8(ctx, base);
	// 831304C0: 48000044  b 0x83130504
	pc = 0x83130504; continue 'dispatch;
	// 831304C4: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831304C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831304CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831304D0: 4182000C  beq 0x831304dc
	if ctx.cr[0].eq {
	pc = 0x831304DC; continue 'dispatch;
	}
	// 831304D4: 4BFF8115  bl 0x831285e8
	ctx.lr = 0x831304D8;
	sub_831285E8(ctx, base);
	// 831304D8: 93DF031C  stw r30, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[30].u32 ) };
	// 831304DC: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 831304E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831304E4: 4182000C  beq 0x831304f0
	if ctx.cr[0].eq {
	pc = 0x831304F0; continue 'dispatch;
	}
	// 831304E8: 4BFF8101  bl 0x831285e8
	ctx.lr = 0x831304EC;
	sub_831285E8(ctx, base);
	// 831304EC: 93DF0320  stw r30, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[30].u32 ) };
	// 831304F0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 831304F4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831304F8: 4800000C  b 0x83130504
	pc = 0x83130504; continue 'dispatch;
	// 831304FC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83130500: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83130504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313050C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130520 size=124
    let mut pc: u32 = 0x83130520;
    'dispatch: loop {
        match pc {
            0x83130520 => {
    //   block [0x83130520..0x8313059C)
	// 83130520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8313052C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83130538: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313053C: 409A0014  bne cr6, 0x83130550
	if !ctx.cr[6].eq {
	pc = 0x83130550; continue 'dispatch;
	}
	// 83130540: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130544: 386B1468  addi r3, r11, 0x1468
	ctx.r[3].s64 = ctx.r[11].s64 + 5224;
	// 83130548: 4BFF5CF9  bl 0x83126240
	ctx.lr = 0x8313054C;
	sub_83126240(ctx, base);
	// 8313054C: 48000038  b 0x83130584
	pc = 0x83130584; continue 'dispatch;
	// 83130550: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 83130554: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83130558: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8313055C: 4182000C  beq 0x83130568
	if ctx.cr[0].eq {
	pc = 0x83130568; continue 'dispatch;
	}
	// 83130560: 4BFF8089  bl 0x831285e8
	ctx.lr = 0x83130564;
	sub_831285E8(ctx, base);
	// 83130564: 93DF031C  stw r30, 0x31c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(796 as u32), ctx.r[30].u32 ) };
	// 83130568: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 8313056C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83130570: 4182000C  beq 0x8313057c
	if ctx.cr[0].eq {
	pc = 0x8313057C; continue 'dispatch;
	}
	// 83130574: 4BFF8075  bl 0x831285e8
	ctx.lr = 0x83130578;
	sub_831285E8(ctx, base);
	// 83130578: 93DF0320  stw r30, 0x320(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(800 as u32), ctx.r[30].u32 ) };
	// 8313057C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83130580: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83130584: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313058C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130590: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130594: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831305A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831305A0 size=140
    let mut pc: u32 = 0x831305A0;
    'dispatch: loop {
        match pc {
            0x831305A0 => {
    //   block [0x831305A0..0x8313062C)
	// 831305A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831305A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831305A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831305AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831305B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831305B4: 3BEB7D6C  addi r31, r11, 0x7d6c
	ctx.r[31].s64 = ctx.r[11].s64 + 32108;
	// 831305B8: 391F0020  addi r8, r31, 0x20
	ctx.r[8].s64 = ctx.r[31].s64 + 32;
	// 831305BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 831305C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 831305C4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 831305C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831305CC: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 831305D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 831305D4: 4082FFE8  bne 0x831305bc
	if !ctx.cr[0].eq {
	pc = 0x831305BC; continue 'dispatch;
	}
	// 831305D8: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831305DC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831305E0: 409A0038  bne cr6, 0x83130618
	if !ctx.cr[6].eq {
	pc = 0x83130618; continue 'dispatch;
	}
	// 831305E4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 831305E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831305EC: 4BFFE8A5  bl 0x8312ee90
	ctx.lr = 0x831305F0;
	sub_8312EE90(ctx, base);
	// 831305F0: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 831305F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831305F8: 40820014  bne 0x8313060c
	if !ctx.cr[0].eq {
	pc = 0x8313060C; continue 'dispatch;
	}
	// 831305FC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130600: 386BFD4C  addi r3, r11, -0x2b4
	ctx.r[3].s64 = ctx.r[11].s64 + -692;
	// 83130604: 4BFF5C3D  bl 0x83126240
	ctx.lr = 0x83130608;
	sub_83126240(ctx, base);
	// 83130608: 48000010  b 0x83130618
	pc = 0x83130618; continue 'dispatch;
	// 8313060C: 4BFFE9F5  bl 0x8312f000
	ctx.lr = 0x83130610;
	sub_8312F000(ctx, base);
	// 83130610: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83130614: 4BFFEA85  bl 0x8312f098
	ctx.lr = 0x83130618;
	sub_8312F098(ctx, base);
	// 83130618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313061C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130630 size=308
    let mut pc: u32 = 0x83130630;
    'dispatch: loop {
        match pc {
            0x83130630 => {
    //   block [0x83130630..0x83130764)
	// 83130630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130634: 48077B39  bl 0x831a816c
	ctx.lr = 0x83130638;
	sub_831A8130(ctx, base);
	// 83130638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313063C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83130640: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83130644: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83130648: 419A010C  beq cr6, 0x83130754
	if ctx.cr[6].eq {
	pc = 0x83130754; continue 'dispatch;
	}
	// 8313064C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83130650: 419A00A8  beq cr6, 0x831306f8
	if ctx.cr[6].eq {
	pc = 0x831306F8; continue 'dispatch;
	}
	// 83130654: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83130658: 419A0094  beq cr6, 0x831306ec
	if ctx.cr[6].eq {
	pc = 0x831306EC; continue 'dispatch;
	}
	// 8313065C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83130660: 409A00FC  bne cr6, 0x8313075c
	if !ctx.cr[6].eq {
	pc = 0x8313075C; continue 'dispatch;
	}
	// 83130664: 3BDF0114  addi r30, r31, 0x114
	ctx.r[30].s64 = ctx.r[31].s64 + 276;
	// 83130668: 3BBF0218  addi r29, r31, 0x218
	ctx.r[29].s64 = ctx.r[31].s64 + 536;
	// 8313066C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83130670: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83130674: 4809471D  bl 0x831c4d90
	ctx.lr = 0x83130678;
	sub_831C4D90(ctx, base);
	// 83130678: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313067C: 41820068  beq 0x831306e4
	if ctx.cr[0].eq {
	pc = 0x831306E4; continue 'dispatch;
	}
	// 83130680: 817F034C  lwz r11, 0x34c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(844 as u32) ) } as u64;
	// 83130684: 387F034C  addi r3, r31, 0x34c
	ctx.r[3].s64 = ctx.r[31].s64 + 844;
	// 83130688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8313068C: 419A0010  beq cr6, 0x8313069c
	if ctx.cr[6].eq {
	pc = 0x8313069C; continue 'dispatch;
	}
	// 83130690: 817F0350  lwz r11, 0x350(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(848 as u32) ) } as u64;
	// 83130694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83130698: 409A0020  bne cr6, 0x831306b8
	if !ctx.cr[6].eq {
	pc = 0x831306B8; continue 'dispatch;
	}
	// 8313069C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831306A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831306A4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831306A8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831306AC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831306B0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831306B4: 93DF0350  stw r30, 0x350(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(848 as u32), ctx.r[30].u32 ) };
	// 831306B8: 817F0364  lwz r11, 0x364(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(868 as u32) ) } as u64;
	// 831306BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831306C0: 409A0018  bne cr6, 0x831306d8
	if !ctx.cr[6].eq {
	pc = 0x831306D8; continue 'dispatch;
	}
	// 831306C4: 4BFF83E5  bl 0x83128aa8
	ctx.lr = 0x831306C8;
	sub_83128AA8(ctx, base);
	// 831306C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831306CC: 41800090  blt 0x8313075c
	if ctx.cr[0].lt {
	pc = 0x8313075C; continue 'dispatch;
	}
	// 831306D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831306D4: 917F0364  stw r11, 0x364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(868 as u32), ctx.r[11].u32 ) };
	// 831306D8: 817F0354  lwz r11, 0x354(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(852 as u32) ) } as u64;
	// 831306DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831306E0: 419A007C  beq cr6, 0x8313075c
	if ctx.cr[6].eq {
	pc = 0x8313075C; continue 'dispatch;
	}
	// 831306E4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831306E8: 48000064  b 0x8313074c
	pc = 0x8313074C; continue 'dispatch;
	// 831306EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831306F0: 4BFFFD79  bl 0x83130468
	ctx.lr = 0x831306F4;
	sub_83130468(ctx, base);
	// 831306F4: 48000068  b 0x8313075c
	pc = 0x8313075C; continue 'dispatch;
	// 831306F8: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831306FC: 4BFF84F5  bl 0x83128bf0
	ctx.lr = 0x83130700;
	sub_83128BF0(ctx, base);
	// 83130700: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83130704: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130708: 4BFF84E9  bl 0x83128bf0
	ctx.lr = 0x8313070C;
	sub_83128BF0(ctx, base);
	// 8313070C: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 83130710: 419A0038  beq cr6, 0x83130748
	if ctx.cr[6].eq {
	pc = 0x83130748; continue 'dispatch;
	}
	// 83130714: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83130718: 419A0030  beq cr6, 0x83130748
	if ctx.cr[6].eq {
	pc = 0x83130748; continue 'dispatch;
	}
	// 8313071C: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130720: 4BFF6F11  bl 0x83127630
	ctx.lr = 0x83130724;
	sub_83127630(ctx, base);
	// 83130724: E89F0328  ld r4, 0x328(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(808 as u32) ) };
	// 83130728: 7F232000  cmpd cr6, r3, r4
	ctx.cr[6].compare_i64(ctx.r[3].s64, ctx.r[4].s64, &mut ctx.xer);
	// 8313072C: 419A0010  beq cr6, 0x8313073c
	if ctx.cr[6].eq {
	pc = 0x8313073C; continue 'dispatch;
	}
	// 83130730: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 83130734: 4BFF81B5  bl 0x831288e8
	ctx.lr = 0x83130738;
	sub_831288E8(ctx, base);
	// 83130738: 48000024  b 0x8313075c
	pc = 0x8313075C; continue 'dispatch;
	// 8313073C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83130740: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83130744: 48000018  b 0x8313075c
	pc = 0x8313075C; continue 'dispatch;
	// 83130748: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8313074C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83130750: 4800000C  b 0x8313075c
	pc = 0x8313075C; continue 'dispatch;
	// 83130754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130758: 4BFFFBF1  bl 0x83130348
	ctx.lr = 0x8313075C;
	sub_83130348(ctx, base);
	// 8313075C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130760: 48077A5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130768 size=208
    let mut pc: u32 = 0x83130768;
    'dispatch: loop {
        match pc {
            0x83130768 => {
    //   block [0x83130768..0x83130838)
	// 83130768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313076C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313077C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83130780: 409A0014  bne cr6, 0x83130794
	if !ctx.cr[6].eq {
	pc = 0x83130794; continue 'dispatch;
	}
	// 83130784: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130788: 386B14B8  addi r3, r11, 0x14b8
	ctx.r[3].s64 = ctx.r[11].s64 + 5304;
	// 8313078C: 4BFF5AB5  bl 0x83126240
	ctx.lr = 0x83130790;
	sub_83126240(ctx, base);
	// 83130790: 48000094  b 0x83130824
	pc = 0x83130824; continue 'dispatch;
	// 83130794: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83130798: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313079C: 409A0088  bne cr6, 0x83130824
	if !ctx.cr[6].eq {
	pc = 0x83130824; continue 'dispatch;
	}
	// 831307A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831307A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831307A8: 419A0018  beq cr6, 0x831307c0
	if ctx.cr[6].eq {
	pc = 0x831307C0; continue 'dispatch;
	}
	// 831307AC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831307B0: 409A0074  bne cr6, 0x83130824
	if !ctx.cr[6].eq {
	pc = 0x83130824; continue 'dispatch;
	}
	// 831307B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831307B8: 4BFFFE79  bl 0x83130630
	ctx.lr = 0x831307BC;
	sub_83130630(ctx, base);
	// 831307BC: 48000068  b 0x83130824
	pc = 0x83130824; continue 'dispatch;
	// 831307C0: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831307C4: 4BFF86E5  bl 0x83128ea8
	ctx.lr = 0x831307C8;
	sub_83128EA8(ctx, base);
	// 831307C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831307CC: 41820058  beq 0x83130824
	if ctx.cr[0].eq {
	pc = 0x83130824; continue 'dispatch;
	}
	// 831307D0: 807F0320  lwz r3, 0x320(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(800 as u32) ) } as u64;
	// 831307D4: 4BFF86D5  bl 0x83128ea8
	ctx.lr = 0x831307D8;
	sub_83128EA8(ctx, base);
	// 831307D8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831307DC: 41820048  beq 0x83130824
	if ctx.cr[0].eq {
	pc = 0x83130824; continue 'dispatch;
	}
	// 831307E0: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831307E4: 4BFF6E4D  bl 0x83127630
	ctx.lr = 0x831307E8;
	sub_83127630(ctx, base);
	// 831307E8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 831307EC: 80BF0344  lwz r5, 0x344(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(836 as u32) ) } as u64;
	// 831307F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831307F4: 807F031C  lwz r3, 0x31c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(796 as u32) ) } as u64;
	// 831307F8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831307FC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83130800: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83130804: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 83130808: F97F0328  std r11, 0x328(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(808 as u32), ctx.r[11].u64 ) };
	// 8313080C: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 83130810: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83130814: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 83130818: 917F0338  stw r11, 0x338(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(824 as u32), ctx.r[11].u32 ) };
	// 8313081C: 4BFF7FDD  bl 0x831287f8
	ctx.lr = 0x83130820;
	sub_831287F8(ctx, base);
	// 83130820: 907F033C  stw r3, 0x33c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(828 as u32), ctx.r[3].u32 ) };
	// 83130824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83130828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313082C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130838 size=240
    let mut pc: u32 = 0x83130838;
    'dispatch: loop {
        match pc {
            0x83130838 => {
    //   block [0x83130838..0x83130928)
	// 83130838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313083C: 4807792D  bl 0x831a8168
	ctx.lr = 0x83130840;
	sub_831A8130(ctx, base);
	// 83130840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130844: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130848: 3BCB7D94  addi r30, r11, 0x7d94
	ctx.r[30].s64 = ctx.r[11].s64 + 32148;
	// 8313084C: 391EFFF8  addi r8, r30, -8
	ctx.r[8].s64 = ctx.r[30].s64 + -8;
	// 83130850: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 83130854: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83130858: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8313085C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 83130860: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83130864: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83130868: 4082FFE8  bne 0x83130850
	if !ctx.cr[0].eq {
	pc = 0x83130850; continue 'dispatch;
	}
	// 8313086C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83130870: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130874: 409A00AC  bne cr6, 0x83130920
	if !ctx.cr[6].eq {
	pc = 0x83130920; continue 'dispatch;
	}
	// 83130878: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8313087C: 4BFFE785  bl 0x8312f000
	ctx.lr = 0x83130880;
	sub_8312F000(ctx, base);
	// 83130880: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130884: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83130888: 419A0078  beq cr6, 0x83130900
	if ctx.cr[6].eq {
	pc = 0x83130900; continue 'dispatch;
	}
	// 8313088C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130890: 3B8B1490  addi r28, r11, 0x1490
	ctx.r[28].s64 = ctx.r[11].s64 + 5264;
	// 83130894: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 83130898: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313089C: 409A0014  bne cr6, 0x831308b0
	if !ctx.cr[6].eq {
	pc = 0x831308B0; continue 'dispatch;
	}
	// 831308A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831308A4: 4BFF599D  bl 0x83126240
	ctx.lr = 0x831308A8;
	sub_83126240(ctx, base);
	// 831308A8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831308AC: 4800004C  b 0x831308f8
	pc = 0x831308F8; continue 'dispatch;
	// 831308B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831308B4: 4BFFFC6D  bl 0x83130520
	ctx.lr = 0x831308B8;
	sub_83130520(ctx, base);
	// 831308B8: 815D0368  lwz r10, 0x368(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(872 as u32) ) } as u64;
	// 831308BC: 817D036C  lwz r11, 0x36c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(876 as u32) ) } as u64;
	// 831308C0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831308C4: 41820010  beq 0x831308d4
	if ctx.cr[0].eq {
	pc = 0x831308D4; continue 'dispatch;
	}
	// 831308C8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831308CC: 916A036C  stw r11, 0x36c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(876 as u32), ctx.r[11].u32 ) };
	// 831308D0: 4800000C  b 0x831308dc
	pc = 0x831308DC; continue 'dispatch;
	// 831308D4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 831308D8: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 831308DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831308E0: 419A0008  beq cr6, 0x831308e8
	if ctx.cr[6].eq {
	pc = 0x831308E8; continue 'dispatch;
	}
	// 831308E4: 914B0368  stw r10, 0x368(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(872 as u32), ctx.r[10].u32 ) };
	// 831308E8: 38A00370  li r5, 0x370
	ctx.r[5].s64 = 880;
	// 831308EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831308F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831308F4: 480778ED  bl 0x831a81e0
	ctx.lr = 0x831308F8;
	sub_831A81E0(ctx, base);
	// 831308F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831308FC: 409AFF98  bne cr6, 0x83130894
	if !ctx.cr[6].eq {
	pc = 0x83130894; continue 'dispatch;
	}
	// 83130900: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83130904: 4BFFE795  bl 0x8312f098
	ctx.lr = 0x83130908;
	sub_8312F098(ctx, base);
	// 83130908: 807EFFFC  lwz r3, -4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8313090C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83130910: 419A0010  beq cr6, 0x83130920
	if ctx.cr[6].eq {
	pc = 0x83130920; continue 'dispatch;
	}
	// 83130914: 4BFFE65D  bl 0x8312ef70
	ctx.lr = 0x83130918;
	sub_8312EF70(ctx, base);
	// 83130918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313091C: 917EFFFC  stw r11, -4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 83130920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83130924: 48077894  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130928 size=100
    let mut pc: u32 = 0x83130928;
    'dispatch: loop {
        match pc {
            0x83130928 => {
    //   block [0x83130928..0x8313098C)
	// 83130928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313092C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313093C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130940: 3BEB7D94  addi r31, r11, 0x7d94
	ctx.r[31].s64 = ctx.r[11].s64 + 32148;
	// 83130944: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83130948: 4BFFE6B9  bl 0x8312f000
	ctx.lr = 0x8313094C;
	sub_8312F000(ctx, base);
	// 8313094C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130950: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83130954: 419A0018  beq cr6, 0x8313096c
	if ctx.cr[6].eq {
	pc = 0x8313096C; continue 'dispatch;
	}
	// 83130958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8313095C: 4BFFFE0D  bl 0x83130768
	ctx.lr = 0x83130960;
	sub_83130768(ctx, base);
	// 83130960: 83DE036C  lwz r30, 0x36c(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(876 as u32) ) } as u64;
	// 83130964: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83130968: 4082FFF0  bne 0x83130958
	if !ctx.cr[0].eq {
	pc = 0x83130958; continue 'dispatch;
	}
	// 8313096C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83130970: 4BFFE729  bl 0x8312f098
	ctx.lr = 0x83130974;
	sub_8312F098(ctx, base);
	// 83130974: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313097C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130990 size=68
    let mut pc: u32 = 0x83130990;
    'dispatch: loop {
        match pc {
            0x83130990 => {
    //   block [0x83130990..0x831309D4)
	// 83130990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313099C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831309A0: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 831309A4: 386B6A40  addi r3, r11, 0x6a40
	ctx.r[3].s64 = ctx.r[11].s64 + 27200;
	// 831309A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831309AC: 48077835  bl 0x831a81e0
	ctx.lr = 0x831309B0;
	sub_831A81E0(ctx, base);
	// 831309B0: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831309B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831309B8: 916A7DB8  stw r11, 0x7db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32184 as u32), ctx.r[11].u32 ) };
	// 831309BC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831309C0: 916A7DBC  stw r11, 0x7dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32188 as u32), ctx.r[11].u32 ) };
	// 831309C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831309C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831309CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831309D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831309D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831309D8 size=132
    let mut pc: u32 = 0x831309D8;
    'dispatch: loop {
        match pc {
            0x831309D8 => {
    //   block [0x831309D8..0x83130A5C)
	// 831309D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831309DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831309E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831309E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831309E8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 831309EC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831309F0: 409A0010  bne cr6, 0x83130a00
	if !ctx.cr[6].eq {
	pc = 0x83130A00; continue 'dispatch;
	}
	// 831309F4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 831309F8: 386BEA50  addi r3, r11, -0x15b0
	ctx.r[3].s64 = ctx.r[11].s64 + -5552;
	// 831309FC: 48000048  b 0x83130a44
	pc = 0x83130A44; continue 'dispatch;
	// 83130A00: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83130A04: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 83130A08: 3BEB6A40  addi r31, r11, 0x6a40
	ctx.r[31].s64 = ctx.r[11].s64 + 27200;
	// 83130A0C: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 83130A10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130A14: 4BFFEF2D  bl 0x8312f940
	ctx.lr = 0x83130A18;
	sub_8312F940(ctx, base);
	// 83130A18: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130A1C: 814B7DB8  lwz r10, 0x7db8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32184 as u32) ) } as u64;
	// 83130A20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83130A24: 419A001C  beq cr6, 0x83130a40
	if ctx.cr[6].eq {
	pc = 0x83130A40; continue 'dispatch;
	}
	// 83130A28: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83130A2C: 816B7DB8  lwz r11, 0x7db8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32184 as u32) ) } as u64;
	// 83130A30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83130A34: 806A7DBC  lwz r3, 0x7dbc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32188 as u32) ) } as u64;
	// 83130A38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130A3C: 4E800421  bctrl
	ctx.lr = 0x83130A40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130A40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130A44: 4BFF58ED  bl 0x83126330
	ctx.lr = 0x83130A48;
	sub_83126330(ctx, base);
	// 83130A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83130A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130A60 size=172
    let mut pc: u32 = 0x83130A60;
    'dispatch: loop {
        match pc {
            0x83130A60 => {
    //   block [0x83130A60..0x83130B0C)
	// 83130A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130A74: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83130A78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83130A7C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83130A80: 419A0068  beq cr6, 0x83130ae8
	if ctx.cr[6].eq {
	pc = 0x83130AE8; continue 'dispatch;
	}
	// 83130A84: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83130A88: 419A0060  beq cr6, 0x83130ae8
	if ctx.cr[6].eq {
	pc = 0x83130AE8; continue 'dispatch;
	}
	// 83130A8C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83130A90: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 83130A94: 3BEB6A40  addi r31, r11, 0x6a40
	ctx.r[31].s64 = ctx.r[11].s64 + 27200;
	// 83130A98: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 83130A9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130AA0: 4BFFEEA1  bl 0x8312f940
	ctx.lr = 0x83130AA4;
	sub_8312F940(ctx, base);
	// 83130AA4: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 83130AA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83130AAC: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 83130AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130AB4: 4BFFEEBD  bl 0x8312f970
	ctx.lr = 0x83130AB8;
	sub_8312F970(ctx, base);
	// 83130AB8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130ABC: 814B7DB8  lwz r10, 0x7db8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32184 as u32) ) } as u64;
	// 83130AC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83130AC4: 419A001C  beq cr6, 0x83130ae0
	if ctx.cr[6].eq {
	pc = 0x83130AE0; continue 'dispatch;
	}
	// 83130AC8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83130ACC: 816B7DB8  lwz r11, 0x7db8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32184 as u32) ) } as u64;
	// 83130AD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83130AD4: 806A7DBC  lwz r3, 0x7dbc(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32188 as u32) ) } as u64;
	// 83130AD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130ADC: 4E800421  bctrl
	ctx.lr = 0x83130AE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130AE4: 4800000C  b 0x83130af0
	pc = 0x83130AF0; continue 'dispatch;
	// 83130AE8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 83130AEC: 386BEA50  addi r3, r11, -0x15b0
	ctx.r[3].s64 = ctx.r[11].s64 + -5552;
	// 83130AF0: 4BFF5841  bl 0x83126330
	ctx.lr = 0x83130AF4;
	sub_83126330(ctx, base);
	// 83130AF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130B00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83130B10 size=52
    let mut pc: u32 = 0x83130B10;
    'dispatch: loop {
        match pc {
            0x83130B10 => {
    //   block [0x83130B10..0x83130B44)
	// 83130B10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83130B14: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 83130B18: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83130B1C: 7D2353D6  divw r9, r3, r10
	ctx.r[9].s32 = ctx.r[3].s32 / ctx.r[10].s32;
	// 83130B20: 1D29000A  mulli r9, r9, 0xa
	ctx.r[9].s64 = ctx.r[9].s64 * 10;
	// 83130B24: 7D291850  subf r9, r9, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[9].s64;
	// 83130B28: 7C6353D7  divw. r3, r3, r10
	ctx.r[3].s32 = ctx.r[3].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83130B2C: 7D2B21AE  stbx r9, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[9].u8) };
	// 83130B30: 41820014  beq 0x83130b44
	if ctx.cr[0].eq {
		sub_83130B44(ctx, base);
		return;
	}
	// 83130B34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83130B38: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83130B3C: 4198FFE0  blt cr6, 0x83130b1c
	if ctx.cr[6].lt {
	pc = 0x83130B1C; continue 'dispatch;
	}
	// 83130B40: 48000008  b 0x83130b48
	sub_83130B44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83130B44 size=116
    let mut pc: u32 = 0x83130B44;
    'dispatch: loop {
        match pc {
            0x83130B44 => {
    //   block [0x83130B44..0x83130BB8)
	// 83130B44: 7D0B21AE  stbx r8, r11, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), ctx.r[8].u8) };
	// 83130B48: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130B4C: 392B7D98  addi r9, r11, 0x7d98
	ctx.r[9].s64 = ctx.r[11].s64 + 32152;
	// 83130B50: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 83130B54: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83130B58: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130B5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83130B60: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 83130B64: 409AFFF4  bne cr6, 0x83130b58
	if !ctx.cr[6].eq {
	pc = 0x83130B58; continue 'dispatch;
	}
	// 83130B68: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83130B6C: 3945FFFF  addi r10, r5, -1
	ctx.r[10].s64 = ctx.r[5].s64 + -1;
	// 83130B70: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83130B74: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83130B78: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83130B7C: 41980008  blt cr6, 0x83130b84
	if ctx.cr[6].lt {
	pc = 0x83130B84; continue 'dispatch;
	}
	// 83130B80: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83130B84: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 83130B88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130B8C: 40990024  ble cr6, 0x83130bb0
	if !ctx.cr[6].gt {
	pc = 0x83130BB0; continue 'dispatch;
	}
	// 83130B90: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83130B94: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83130B98: 88E90000  lbz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130B9C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83130BA0: 7CEA21AE  stbx r7, r10, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[7].u8) };
	// 83130BA4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83130BA8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83130BAC: 4198FFEC  blt cr6, 0x83130b98
	if ctx.cr[6].lt {
	pc = 0x83130B98; continue 'dispatch;
	}
	// 83130BB0: 7D0A21AE  stbx r8, r10, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[8].u8) };
	// 83130BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130BB8 size=216
    let mut pc: u32 = 0x83130BB8;
    'dispatch: loop {
        match pc {
            0x83130BB8 => {
    //   block [0x83130BB8..0x83130C90)
	// 83130BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130BC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130BC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130BC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130BCC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83130BD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83130BD4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83130BD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83130BDC: 4BFFFF35  bl 0x83130b10
	ctx.lr = 0x83130BE0;
	sub_83130B10(ctx, base);
	// 83130BE0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83130BE4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 83130BE8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130BEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83130BF0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83130BF4: 409AFFF4  bne cr6, 0x83130be8
	if !ctx.cr[6].eq {
	pc = 0x83130BE8; continue 'dispatch;
	}
	// 83130BF8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83130BFC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83130C00: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83130C04: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 83130C08: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83130C0C: 38AA9FD8  addi r5, r10, -0x6028
	ctx.r[5].s64 = ctx.r[10].s64 + -24616;
	// 83130C10: 7D6B3050  subf r11, r11, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 83130C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130C18: 38CBFFFF  addi r6, r11, -1
	ctx.r[6].s64 = ctx.r[11].s64 + -1;
	// 83130C1C: 4BFFED55  bl 0x8312f970
	ctx.lr = 0x83130C20;
	sub_8312F970(ctx, base);
	// 83130C20: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83130C24: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83130C28: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130C2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83130C30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83130C34: 409AFFF4  bne cr6, 0x83130c28
	if !ctx.cr[6].eq {
	pc = 0x83130C28; continue 'dispatch;
	}
	// 83130C38: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 83130C3C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 83130C40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83130C44: 5569003E  slwi r9, r11, 0
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83130C48: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83130C4C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130C50: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83130C54: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83130C58: 409AFFF4  bne cr6, 0x83130c4c
	if !ctx.cr[6].eq {
	pc = 0x83130C4C; continue 'dispatch;
	}
	// 83130C5C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83130C60: 20A90004  subfic r5, r9, 4
	ctx.xer.ca = ctx.r[9].u32 <= 4 as u32;
	ctx.r[5].s64 = (4 as i64) - ctx.r[9].s64;
	// 83130C64: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83130C68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83130C6C: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83130C70: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83130C74: 4BFFFE9D  bl 0x83130b10
	ctx.lr = 0x83130C78;
	sub_83130B10(ctx, base);
	// 83130C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130C90 size=116
    let mut pc: u32 = 0x83130C90;
    'dispatch: loop {
        match pc {
            0x83130C90 => {
    //   block [0x83130C90..0x83130D04)
	// 83130C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130C98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130C9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130CA0: 4BFF6109  bl 0x83126da8
	ctx.lr = 0x83130CA4;
	sub_83126DA8(ctx, base);
	// 83130CA4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130CA8: 3BEB7DC0  addi r31, r11, 0x7dc0
	ctx.r[31].s64 = ctx.r[11].s64 + 32192;
	// 83130CAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83130CB4: 419A0014  beq cr6, 0x83130cc8
	if ctx.cr[6].eq {
	pc = 0x83130CC8; continue 'dispatch;
	}
	// 83130CB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83130CBC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83130CC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130CC4: 4E800421  bctrl
	ctx.lr = 0x83130CC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130CC8: 48004CA9  bl 0x83135970
	ctx.lr = 0x83130CCC;
	sub_83135970(ctx, base);
	// 83130CCC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83130CD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83130CD4: 419A0018  beq cr6, 0x83130cec
	if ctx.cr[6].eq {
	pc = 0x83130CEC; continue 'dispatch;
	}
	// 83130CD8: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 83130CDC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83130CE0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130CE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130CE8: 4E800421  bctrl
	ctx.lr = 0x83130CEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130CEC: 4BFF60FD  bl 0x83126de8
	ctx.lr = 0x83130CF0;
	sub_83126DE8(ctx, base);
	// 83130CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83130CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130CFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130D08 size=244
    let mut pc: u32 = 0x83130D08;
    'dispatch: loop {
        match pc {
            0x83130D08 => {
    //   block [0x83130D08..0x83130DFC)
	// 83130D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130D10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130D14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130D18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130D1C: 4BFF608D  bl 0x83126da8
	ctx.lr = 0x83130D20;
	sub_83126DA8(ctx, base);
	// 83130D20: 4BF96DC1  bl 0x830c7ae0
	ctx.lr = 0x83130D24;
	sub_830C7AE0(ctx, base);
	// 83130D24: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83130D28: 817F7DD0  lwz r11, 0x7dd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32208 as u32) ) } as u64;
	// 83130D2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130D30: 419A000C  beq cr6, 0x83130d3c
	if ctx.cr[6].eq {
	pc = 0x83130D3C; continue 'dispatch;
	}
	// 83130D34: 4BF96DAD  bl 0x830c7ae0
	ctx.lr = 0x83130D38;
	sub_830C7AE0(ctx, base);
	// 83130D38: 480000A8  b 0x83130de0
	pc = 0x83130DE0; continue 'dispatch;
	// 83130D3C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83130D40: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130D44: 4BF96D9D  bl 0x830c7ae0
	ctx.lr = 0x83130D48;
	sub_830C7AE0(ctx, base);
	// 83130D48: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130D4C: 3BCB7DE4  addi r30, r11, 0x7de4
	ctx.r[30].s64 = ctx.r[11].s64 + 32228;
	// 83130D50: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 83130D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83130D58: 419A0018  beq cr6, 0x83130d70
	if ctx.cr[6].eq {
	pc = 0x83130D70; continue 'dispatch;
	}
	// 83130D5C: 397EFFF4  addi r11, r30, -0xc
	ctx.r[11].s64 = ctx.r[30].s64 + -12;
	// 83130D60: 807EFFF8  lwz r3, -8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130D64: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130D68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130D6C: 4E800421  bctrl
	ctx.lr = 0x83130D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130D70: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83130D74: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130D78: 480020C9  bl 0x83132e40
	ctx.lr = 0x83130D7C;
	sub_83132E40(ctx, base);
	// 83130D7C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83130D80: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130D84: 4800230D  bl 0x83133090
	ctx.lr = 0x83130D88;
	sub_83133090(ctx, base);
	// 83130D88: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 83130D8C: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130D90: 480067D9  bl 0x83137568
	ctx.lr = 0x83130D94;
	sub_83137568(ctx, base);
	// 83130D94: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 83130D98: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130D9C: 480022F5  bl 0x83133090
	ctx.lr = 0x83130DA0;
	sub_83133090(ctx, base);
	// 83130DA0: 39600007  li r11, 7
	ctx.r[11].s64 = 7;
	// 83130DA4: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130DA8: 48002099  bl 0x83132e40
	ctx.lr = 0x83130DAC;
	sub_83132E40(ctx, base);
	// 83130DAC: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 83130DB0: 917F7DD0  stw r11, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[11].u32 ) };
	// 83130DB4: 4800168D  bl 0x83132440
	ctx.lr = 0x83130DB8;
	sub_83132440(ctx, base);
	// 83130DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83130DBC: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83130DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83130DC4: 915F7DD0  stw r10, 0x7dd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32208 as u32), ctx.r[10].u32 ) };
	// 83130DC8: 419A0018  beq cr6, 0x83130de0
	if ctx.cr[6].eq {
	pc = 0x83130DE0; continue 'dispatch;
	}
	// 83130DCC: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 83130DD0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130DD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130DD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130DDC: 4E800421  bctrl
	ctx.lr = 0x83130DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130DE0: 4BFF6009  bl 0x83126de8
	ctx.lr = 0x83130DE4;
	sub_83126DE8(ctx, base);
	// 83130DE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130DF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83130E00 size=32
    let mut pc: u32 = 0x83130E00;
    'dispatch: loop {
        match pc {
            0x83130E00 => {
    //   block [0x83130E00..0x83130E20)
	// 83130E00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83130E04: 409A001C  bne cr6, 0x83130e20
	if !ctx.cr[6].eq {
		sub_83130E20(ctx, base);
		return;
	}
	// 83130E08: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83130E0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83130E10: 916A7DE8  stw r11, 0x7de8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32232 as u32), ctx.r[11].u32 ) };
	// 83130E14: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83130E18: 916A7DEC  stw r11, 0x7dec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32236 as u32), ctx.r[11].u32 ) };
	// 83130E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83130E20 size=20
    let mut pc: u32 = 0x83130E20;
    'dispatch: loop {
        match pc {
            0x83130E20 => {
    //   block [0x83130E20..0x83130E34)
	// 83130E20: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130E24: 906B7DE8  stw r3, 0x7de8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32232 as u32), ctx.r[3].u32 ) };
	// 83130E28: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130E2C: 908B7DEC  stw r4, 0x7dec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32236 as u32), ctx.r[4].u32 ) };
	// 83130E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130E38 size=144
    let mut pc: u32 = 0x83130E38;
    'dispatch: loop {
        match pc {
            0x83130E38 => {
    //   block [0x83130E38..0x83130EC8)
	// 83130E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130E44: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 83130E48: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 83130E4C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 83130E50: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 83130E54: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 83130E58: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 83130E5C: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 83130E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130E64: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83130E68: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83130E6C: 39210088  addi r9, r1, 0x88
	ctx.r[9].s64 = ctx.r[1].s64 + 136;
	// 83130E70: 3BEB6940  addi r31, r11, 0x6940
	ctx.r[31].s64 = ctx.r[11].s64 + 26944;
	// 83130E74: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83130E78: 38800100  li r4, 0x100
	ctx.r[4].s64 = 256;
	// 83130E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130E80: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83130E84: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83130E88: 4BFFEB91  bl 0x8312fa18
	ctx.lr = 0x83130E8C;
	sub_8312FA18(ctx, base);
	// 83130E8C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83130E90: 814B7DE8  lwz r10, 0x7de8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32232 as u32) ) } as u64;
	// 83130E94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83130E98: 419A001C  beq cr6, 0x83130eb4
	if ctx.cr[6].eq {
	pc = 0x83130EB4; continue 'dispatch;
	}
	// 83130E9C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83130EA0: 816B7DE8  lwz r11, 0x7de8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32232 as u32) ) } as u64;
	// 83130EA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83130EA8: 806A7DEC  lwz r3, 0x7dec(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32236 as u32) ) } as u64;
	// 83130EAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83130EB0: 4E800421  bctrl
	ctx.lr = 0x83130EB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83130EB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130EC8 size=128
    let mut pc: u32 = 0x83130EC8;
    'dispatch: loop {
        match pc {
            0x83130EC8 => {
    //   block [0x83130EC8..0x83130F48)
	// 83130EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130ED0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130ED4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130ED8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130EDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83130EE0: 816B1514  lwz r11, 0x1514(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5396 as u32) ) } as u64;
	// 83130EE4: 4BF96BFD  bl 0x830c7ae0
	ctx.lr = 0x83130EE8;
	sub_830C7AE0(ctx, base);
	// 83130EE8: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83130EEC: 817F7DF0  lwz r11, 0x7df0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32240 as u32) ) } as u64;
	// 83130EF0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130EF4: 409A0030  bne cr6, 0x83130f24
	if !ctx.cr[6].eq {
	pc = 0x83130F24; continue 'dispatch;
	}
	// 83130EF8: 4BFF5DB9  bl 0x83126cb0
	ctx.lr = 0x83130EFC;
	sub_83126CB0(ctx, base);
	// 83130EFC: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83130F00: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 83130F04: 386BDB40  addi r3, r11, -0x24c0
	ctx.r[3].s64 = ctx.r[11].s64 + -9408;
	// 83130F08: 60A58E00  ori r5, r5, 0x8e00
	ctx.r[5].u64 = ctx.r[5].u64 | 36352;
	// 83130F0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83130F10: 480772D1  bl 0x831a81e0
	ctx.lr = 0x83130F14;
	sub_831A81E0(ctx, base);
	// 83130F14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83130F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83130F1C: 4BFFFEE5  bl 0x83130e00
	ctx.lr = 0x83130F20;
	sub_83130E00(ctx, base);
	// 83130F20: 817F7DF0  lwz r11, 0x7df0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32240 as u32) ) } as u64;
	// 83130F24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83130F28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83130F2C: 917F7DF0  stw r11, 0x7df0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32240 as u32), ctx.r[11].u32 ) };
	// 83130F30: 4BF96BB1  bl 0x830c7ae0
	ctx.lr = 0x83130F34;
	sub_830C7AE0(ctx, base);
	// 83130F34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130F40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130F48 size=152
    let mut pc: u32 = 0x83130F48;
    'dispatch: loop {
        match pc {
            0x83130F48 => {
    //   block [0x83130F48..0x83130FE0)
	// 83130F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130F5C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83130F60: 816A7DF0  lwz r11, 0x7df0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32240 as u32) ) } as u64;
	// 83130F64: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83130F68: 916A7DF0  stw r11, 0x7df0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32240 as u32), ctx.r[11].u32 ) };
	// 83130F6C: 4082005C  bne 0x83130fc8
	if !ctx.cr[0].eq {
	pc = 0x83130FC8; continue 'dispatch;
	}
	// 83130F70: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83130F74: 3BCBDB40  addi r30, r11, -0x24c0
	ctx.r[30].s64 = ctx.r[11].s64 + -9408;
	// 83130F78: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83130F7C: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83130F80: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83130F84: 409A000C  bne cr6, 0x83130f90
	if !ctx.cr[6].eq {
	pc = 0x83130F90; continue 'dispatch;
	}
	// 83130F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83130F8C: 48004CCD  bl 0x83135c58
	ctx.lr = 0x83130F90;
	sub_83135C58(ctx, base);
	// 83130F90: 3D7E0001  addis r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 65536;
	// 83130F94: 3BFF0238  addi r31, r31, 0x238
	ctx.r[31].s64 = ctx.r[31].s64 + 568;
	// 83130F98: 396B8E00  addi r11, r11, -0x7200
	ctx.r[11].s64 = ctx.r[11].s64 + -29184;
	// 83130F9C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83130FA0: 4198FFDC  blt cr6, 0x83130f7c
	if ctx.cr[6].lt {
	pc = 0x83130F7C; continue 'dispatch;
	}
	// 83130FA4: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 83130FA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83130FAC: 60A58E00  ori r5, r5, 0x8e00
	ctx.r[5].u64 = ctx.r[5].u64 | 36352;
	// 83130FB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83130FB4: 4807722D  bl 0x831a81e0
	ctx.lr = 0x83130FB8;
	sub_831A81E0(ctx, base);
	// 83130FB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83130FBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83130FC0: 4BFFFE41  bl 0x83130e00
	ctx.lr = 0x83130FC4;
	sub_83130E00(ctx, base);
	// 83130FC4: 4BFF5D6D  bl 0x83126d30
	ctx.lr = 0x83130FC8;
	sub_83126D30(ctx, base);
	// 83130FC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83130FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83130FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83130FD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83130FD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83130FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83130FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83130FE0 size=224
    let mut pc: u32 = 0x83130FE0;
    'dispatch: loop {
        match pc {
            0x83130FE0 => {
    //   block [0x83130FE0..0x831310C0)
	// 83130FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83130FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83130FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83130FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83130FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83130FF4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83130FF8: 3FC08339  lis r30, -0x7cc7
	ctx.r[30].s64 = -2093416448;
	// 83130FFC: 396B1518  addi r11, r11, 0x1518
	ctx.r[11].s64 = ctx.r[11].s64 + 5400;
	// 83131000: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83131004: 83FE7DF8  lwz r31, 0x7df8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32248 as u32) ) } as u64;
	// 83131008: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8313100C: 916A7DF4  stw r11, 0x7df4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32244 as u32), ctx.r[11].u32 ) };
	// 83131010: 409A0090  bne cr6, 0x831310a0
	if !ctx.cr[6].eq {
	pc = 0x831310A0; continue 'dispatch;
	}
	// 83131014: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83131018: 38A00340  li r5, 0x340
	ctx.r[5].s64 = 832;
	// 8313101C: 386BD800  addi r3, r11, -0x2800
	ctx.r[3].s64 = ctx.r[11].s64 + -10240;
	// 83131020: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131024: 480771BD  bl 0x831a81e0
	ctx.lr = 0x83131028;
	sub_831A81E0(ctx, base);
	// 83131028: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8313102C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 83131030: 386BD400  addi r3, r11, -0x2c00
	ctx.r[3].s64 = ctx.r[11].s64 + -11264;
	// 83131034: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131038: 480771A9  bl 0x831a81e0
	ctx.lr = 0x8313103C;
	sub_831A81E0(ctx, base);
	// 8313103C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83131040: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 83131044: 386BD2A0  addi r3, r11, -0x2d60
	ctx.r[3].s64 = ctx.r[11].s64 + -11616;
	// 83131048: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 8313104C: 48077195  bl 0x831a81e0
	ctx.lr = 0x83131050;
	sub_831A81E0(ctx, base);
	// 83131050: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83131054: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 83131058: 386BD3C0  addi r3, r11, -0x2c40
	ctx.r[3].s64 = ctx.r[11].s64 + -11328;
	// 8313105C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131060: 48077181  bl 0x831a81e0
	ctx.lr = 0x83131064;
	sub_831A81E0(ctx, base);
	// 83131064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83131068: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 8313106C: 916AD3A0  stw r11, -0x2c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11360 as u32), ctx.r[11].u32 ) };
	// 83131070: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83131074: 916AD3A4  stw r11, -0x2c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11356 as u32), ctx.r[11].u32 ) };
	// 83131078: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 8313107C: 916AD3E4  stw r11, -0x2c1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11292 as u32), ctx.r[11].u32 ) };
	// 83131080: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83131084: 916AD3E0  stw r11, -0x2c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11296 as u32), ctx.r[11].u32 ) };
	// 83131088: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 8313108C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83131090: 916AD280  stw r11, -0x2d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11648 as u32), ctx.r[11].u32 ) };
	// 83131094: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83131098: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8313109C: 916AD284  stw r11, -0x2d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11644 as u32), ctx.r[11].u32 ) };
	// 831310A0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 831310A4: 917E7DF8  stw r11, 0x7df8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32248 as u32), ctx.r[11].u32 ) };
	// 831310A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831310AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831310B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831310B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831310B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831310BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831310C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831310C0 size=192
    let mut pc: u32 = 0x831310C0;
    'dispatch: loop {
        match pc {
            0x831310C0 => {
    //   block [0x831310C0..0x83131180)
	// 831310C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831310C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831310C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831310CC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831310D0: 816A7DF8  lwz r11, 0x7df8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32248 as u32) ) } as u64;
	// 831310D4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831310D8: 916A7DF8  stw r11, 0x7df8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32248 as u32), ctx.r[11].u32 ) };
	// 831310DC: 40820094  bne 0x83131170
	if !ctx.cr[0].eq {
	pc = 0x83131170; continue 'dispatch;
	}
	// 831310E0: 480066E9  bl 0x831377c8
	ctx.lr = 0x831310E4;
	sub_831377C8(ctx, base);
	// 831310E4: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831310E8: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 831310EC: 386BD3C0  addi r3, r11, -0x2c40
	ctx.r[3].s64 = ctx.r[11].s64 + -11328;
	// 831310F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831310F4: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 831310F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831310FC: 916AD284  stw r11, -0x2d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11644 as u32), ctx.r[11].u32 ) };
	// 83131100: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83131104: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83131108: 916AD280  stw r11, -0x2d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11648 as u32), ctx.r[11].u32 ) };
	// 8313110C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83131110: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83131114: 916AD3E0  stw r11, -0x2c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11296 as u32), ctx.r[11].u32 ) };
	// 83131118: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 8313111C: 916AD3E4  stw r11, -0x2c1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11292 as u32), ctx.r[11].u32 ) };
	// 83131120: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 83131124: 916AD3A4  stw r11, -0x2c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11356 as u32), ctx.r[11].u32 ) };
	// 83131128: 3D408344  lis r10, -0x7cbc
	ctx.r[10].s64 = -2092695552;
	// 8313112C: 916AD3A0  stw r11, -0x2c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11360 as u32), ctx.r[11].u32 ) };
	// 83131130: 480770B1  bl 0x831a81e0
	ctx.lr = 0x83131134;
	sub_831A81E0(ctx, base);
	// 83131134: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83131138: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8313113C: 386BD2A0  addi r3, r11, -0x2d60
	ctx.r[3].s64 = ctx.r[11].s64 + -11616;
	// 83131140: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 83131144: 4807709D  bl 0x831a81e0
	ctx.lr = 0x83131148;
	sub_831A81E0(ctx, base);
	// 83131148: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 8313114C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 83131150: 386BD400  addi r3, r11, -0x2c00
	ctx.r[3].s64 = ctx.r[11].s64 + -11264;
	// 83131154: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131158: 48077089  bl 0x831a81e0
	ctx.lr = 0x8313115C;
	sub_831A81E0(ctx, base);
	// 8313115C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83131160: 38A00340  li r5, 0x340
	ctx.r[5].s64 = 832;
	// 83131164: 386BD800  addi r3, r11, -0x2800
	ctx.r[3].s64 = ctx.r[11].s64 + -10240;
	// 83131168: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313116C: 48077075  bl 0x831a81e0
	ctx.lr = 0x83131170;
	sub_831A81E0(ctx, base);
	// 83131170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83131174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83131178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313117C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83131180 size=88
    let mut pc: u32 = 0x83131180;
    'dispatch: loop {
        match pc {
            0x83131180 => {
    //   block [0x83131180..0x831311D8)
	// 83131180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83131188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313118C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131190: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 83131194: 817F7DFC  lwz r11, 0x7dfc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32252 as u32) ) } as u64;
	// 83131198: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313119C: 409A001C  bne cr6, 0x831311b8
	if !ctx.cr[6].eq {
	pc = 0x831311B8; continue 'dispatch;
	}
	// 831311A0: 48006851  bl 0x831379f0
	ctx.lr = 0x831311A4;
	sub_831379F0(ctx, base);
	// 831311A4: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831311A8: 38A01680  li r5, 0x1680
	ctx.r[5].s64 = 5760;
	// 831311AC: 386BBC00  addi r3, r11, -0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + -17408;
	// 831311B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831311B4: 4807702D  bl 0x831a81e0
	ctx.lr = 0x831311B8;
	sub_831A81E0(ctx, base);
	// 831311B8: 817F7DFC  lwz r11, 0x7dfc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32252 as u32) ) } as u64;
	// 831311BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831311C0: 917F7DFC  stw r11, 0x7dfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32252 as u32), ctx.r[11].u32 ) };
	// 831311C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831311C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831311CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831311D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831311D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831311D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831311D8 size=28
    let mut pc: u32 = 0x831311D8;
    'dispatch: loop {
        match pc {
            0x831311D8 => {
    //   block [0x831311D8..0x831311F4)
	// 831311D8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831311DC: 814B7DFC  lwz r10, 0x7dfc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32252 as u32) ) } as u64;
	// 831311E0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 831311E4: 914B7DFC  stw r10, 0x7dfc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32252 as u32), ctx.r[10].u32 ) };
	// 831311E8: 816B7DFC  lwz r11, 0x7dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32252 as u32) ) } as u64;
	// 831311EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831311F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831311F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831311F4 size=20
    let mut pc: u32 = 0x831311F4;
    'dispatch: loop {
        match pc {
            0x831311F4 => {
    //   block [0x831311F4..0x83131208)
	// 831311F4: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831311F8: 38A01680  li r5, 0x1680
	ctx.r[5].s64 = 5760;
	// 831311FC: 386BBC00  addi r3, r11, -0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + -17408;
	// 83131200: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131204: 48076FDC  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131208 size=4
    let mut pc: u32 = 0x83131208;
    'dispatch: loop {
        match pc {
            0x83131208 => {
    //   block [0x83131208..0x8313120C)
	// 83131208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83131210 size=96
    let mut pc: u32 = 0x83131210;
    'dispatch: loop {
        match pc {
            0x83131210 => {
    //   block [0x83131210..0x83131270)
	// 83131210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83131218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313121C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83131224: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83131228: 419A0034  beq cr6, 0x8313125c
	if ctx.cr[6].eq {
	pc = 0x8313125C; continue 'dispatch;
	}
	// 8313122C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131230: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83131234: 41820010  beq 0x83131244
	if ctx.cr[0].eq {
	pc = 0x83131244; continue 'dispatch;
	}
	// 83131238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8313123C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83131240: 48006851  bl 0x83137a90
	ctx.lr = 0x83131244;
	sub_83137A90(ctx, base);
	// 83131244: 4BF9689D  bl 0x830c7ae0
	ctx.lr = 0x83131248;
	sub_830C7AE0(ctx, base);
	// 83131248: 38A000B4  li r5, 0xb4
	ctx.r[5].s64 = 180;
	// 8313124C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83131254: 48076F8D  bl 0x831a81e0
	ctx.lr = 0x83131258;
	sub_831A81E0(ctx, base);
	// 83131258: 4BF96889  bl 0x830c7ae0
	ctx.lr = 0x8313125C;
	sub_830C7AE0(ctx, base);
	// 8313125C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83131260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83131264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83131268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313126C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131270 size=16
    let mut pc: u32 = 0x83131270;
    'dispatch: loop {
        match pc {
            0x83131270 => {
    //   block [0x83131270..0x83131280)
	// 83131270: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83131274: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131278: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8313127C: 480075C4  b 0x83138840
	sub_83138840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131280 size=16
    let mut pc: u32 = 0x83131280;
    'dispatch: loop {
        match pc {
            0x83131280 => {
    //   block [0x83131280..0x83131290)
	// 83131280: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83131284: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131288: 908B0038  stw r4, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 8313128C: 480075D4  b 0x83138860
	sub_83138860(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131290 size=8
    let mut pc: u32 = 0x83131290;
    'dispatch: loop {
        match pc {
            0x83131290 => {
    //   block [0x83131290..0x83131298)
	// 83131290: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131294: 4800763C  b 0x831388d0
	sub_831388D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131298 size=72
    let mut pc: u32 = 0x83131298;
    'dispatch: loop {
        match pc {
            0x83131298 => {
    //   block [0x83131298..0x831312E0)
	// 83131298: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8313129C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831312A0: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 831312A4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 831312A8: 916300A0  stw r11, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 831312AC: 91430038  stw r10, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 831312B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 831312B4: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 831312B8: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 831312BC: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 831312C0: 9123003C  stw r9, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 831312C4: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 831312C8: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 831312CC: 99630003  stb r11, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 831312D0: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 831312D4: 916300AC  stw r11, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 831312D8: 99430001  stb r10, 1(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 831312DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831312E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831312E0 size=56
    let mut pc: u32 = 0x831312E0;
    'dispatch: loop {
        match pc {
            0x831312E0 => {
    //   block [0x831312E0..0x83131318)
	// 831312E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831312E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831312E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831312EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831312F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831312F4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831312F8: 48006B91  bl 0x83137e88
	ctx.lr = 0x831312FC;
	sub_83137E88(ctx, base);
	// 831312FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83131300: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83131304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83131308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313130C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83131310: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83131314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83131318 size=716
    let mut pc: u32 = 0x83131318;
    'dispatch: loop {
        match pc {
            0x83131318 => {
    //   block [0x83131318..0x831315E4)
	// 83131318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313131C: 48076E41  bl 0x831a815c
	ctx.lr = 0x83131320;
	sub_831A8130(ctx, base);
	// 83131320: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131324: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83131328: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 8313132C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83131330: 60A5C800  ori r5, r5, 0xc800
	ctx.r[5].u64 = ctx.r[5].u64 | 51200;
	// 83131334: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131338: 837A0008  lwz r27, 8(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313133C: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131340: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83131344: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131348: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8313134C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131350: 4E800421  bctrl
	ctx.lr = 0x83131354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131354: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83131358: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313135C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83131360: 40990020  ble cr6, 0x83131380
	if !ctx.cr[6].gt {
	pc = 0x83131380; continue 'dispatch;
	}
	// 83131364: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83131368: 7D4A20AE  lbzx r10, r10, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 8313136C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83131370: 409A0010  bne cr6, 0x83131380
	if !ctx.cr[6].eq {
	pc = 0x83131380; continue 'dispatch;
	}
	// 83131374: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 83131378: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313137C: 4198FFE8  blt cr6, 0x83131364
	if ctx.cr[6].lt {
	pc = 0x83131364; continue 'dispatch;
	}
	// 83131380: 7C8B0E70  srawi r11, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 83131384: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83131388: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8313138C: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 83131390: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83131394: 409A004C  bne cr6, 0x831313e0
	if !ctx.cr[6].eq {
	pc = 0x831313E0; continue 'dispatch;
	}
	// 83131398: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313139C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831313A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831313A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831313A8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831313AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831313B0: 4E800421  bctrl
	ctx.lr = 0x831313B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831313B4: 4800662D  bl 0x831379e0
	ctx.lr = 0x831313B8;
	sub_831379E0(ctx, base);
	// 831313B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831313BC: 40820018  bne 0x831313d4
	if !ctx.cr[0].eq {
	pc = 0x831313D4; continue 'dispatch;
	}
	// 831313C0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831313C4: 388B15C8  addi r4, r11, 0x15c8
	ctx.r[4].s64 = ctx.r[11].s64 + 5576;
	// 831313C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831313CC: 386B15A8  addi r3, r11, 0x15a8
	ctx.r[3].s64 = ctx.r[11].s64 + 5544;
	// 831313D0: 4BFFF691  bl 0x83130a60
	ctx.lr = 0x831313D4;
	sub_83130A60(ctx, base);
	// 831313D4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 831313D8: 997A0001  stb r11, 1(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 831313DC: 48000200  b 0x831315dc
	pc = 0x831315DC; continue 'dispatch;
	// 831313E0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831313E4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831313E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831313EC: 48003DA5  bl 0x83135190
	ctx.lr = 0x831313F0;
	sub_83135190(ctx, base);
	// 831313F0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831313F4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831313F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831313FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83131400: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83131404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131408: 4E800421  bctrl
	ctx.lr = 0x8313140C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313140C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83131410: 2F050010  cmpwi cr6, r5, 0x10
	ctx.cr[6].compare_i32(ctx.r[5].s32, 16, &mut ctx.xer);
	// 83131414: 419801AC  blt cr6, 0x831315c0
	if ctx.cr[6].lt {
	pc = 0x831315C0; continue 'dispatch;
	}
	// 83131418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313141C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83131420: 48007249  bl 0x83138668
	ctx.lr = 0x83131424;
	sub_83138668(ctx, base);
	// 83131424: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83131428: 41820198  beq 0x831315c0
	if ctx.cr[0].eq {
	pc = 0x831315C0; continue 'dispatch;
	}
	// 8313142C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83131430: 7F195800  cmpw cr6, r25, r11
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131434: 4199018C  bgt cr6, 0x831315c0
	if ctx.cr[6].gt {
	pc = 0x831315C0; continue 'dispatch;
	}
	// 83131438: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8313143C: 4098001C  bge cr6, 0x83131458
	if !ctx.cr[6].lt {
	pc = 0x83131458; continue 'dispatch;
	}
	// 83131440: A17F009A  lhz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 83131444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83131448: 419A0118  beq cr6, 0x83131560
	if ctx.cr[6].eq {
	pc = 0x83131560; continue 'dispatch;
	}
	// 8313144C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83131450: 480066A1  bl 0x83137af0
	ctx.lr = 0x83131454;
	sub_83137AF0(ctx, base);
	// 83131454: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83131458: 817A0050  lwz r11, 0x50(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(80 as u32) ) } as u64;
	// 8313145C: 933A00A0  stw r25, 0xa0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(160 as u32), ctx.r[25].u32 ) };
	// 83131460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83131464: 419A0050  beq cr6, 0x831314b4
	if ctx.cr[6].eq {
	pc = 0x831314B4; continue 'dispatch;
	}
	// 83131468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313146C: 4800673D  bl 0x83137ba8
	ctx.lr = 0x83131470;
	sub_83137BA8(ctx, base);
	// 83131470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83131474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83131478: 48006739  bl 0x83137bb0
	ctx.lr = 0x8313147C;
	sub_83137BB0(ctx, base);
	// 8313147C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83131480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83131484: 4BF21CBD  bl 0x83053140
	ctx.lr = 0x83131488;
	sub_83053140(ctx, base);
	// 83131488: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8313148C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83131490: 4BF21CB9  bl 0x83053148
	ctx.lr = 0x83131494;
	sub_83053148(ctx, base);
	// 83131494: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 83131498: 807A0054  lwz r3, 0x54(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(84 as u32) ) } as u64;
	// 8313149C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 831314A0: 817A0050  lwz r11, 0x50(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(80 as u32) ) } as u64;
	// 831314A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831314A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831314AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831314B0: 4E800421  bctrl
	ctx.lr = 0x831314B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831314B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831314B8: 480066F1  bl 0x83137ba8
	ctx.lr = 0x831314BC;
	sub_83137BA8(ctx, base);
	// 831314BC: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831314C0: 409A000C  bne cr6, 0x831314cc
	if !ctx.cr[6].eq {
	pc = 0x831314CC; continue 'dispatch;
	}
	// 831314C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831314C8: 997A0003  stb r11, 3(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 831314CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831314D0: 480066D9  bl 0x83137ba8
	ctx.lr = 0x831314D4;
	sub_83137BA8(ctx, base);
	// 831314D4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831314D8: 409A0020  bne cr6, 0x831314f8
	if !ctx.cr[6].eq {
	pc = 0x831314F8; continue 'dispatch;
	}
	// 831314DC: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831314E0: 2F050040  cmpwi cr6, r5, 0x40
	ctx.cr[6].compare_i32(ctx.r[5].s32, 64, &mut ctx.xer);
	// 831314E4: 41980008  blt cr6, 0x831314ec
	if ctx.cr[6].lt {
	pc = 0x831314EC; continue 'dispatch;
	}
	// 831314E8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 831314EC: 387A0060  addi r3, r26, 0x60
	ctx.r[3].s64 = ctx.r[26].s64 + 96;
	// 831314F0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831314F4: 4807701D  bl 0x831a8510
	ctx.lr = 0x831314F8;
	sub_831A8510(ctx, base);
	// 831314F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831314FC: 480066AD  bl 0x83137ba8
	ctx.lr = 0x83131500;
	sub_83137BA8(ctx, base);
	// 83131500: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 83131504: 419A0098  beq cr6, 0x8313159c
	if ctx.cr[6].eq {
	pc = 0x8313159C; continue 'dispatch;
	}
	// 83131508: 2F03000B  cmpwi cr6, r3, 0xb
	ctx.cr[6].compare_i32(ctx.r[3].s32, 11, &mut ctx.xer);
	// 8313150C: 419A0090  beq cr6, 0x8313159c
	if ctx.cr[6].eq {
	pc = 0x8313159C; continue 'dispatch;
	}
	// 83131510: 2F03000C  cmpwi cr6, r3, 0xc
	ctx.cr[6].compare_i32(ctx.r[3].s32, 12, &mut ctx.xer);
	// 83131514: 419A0088  beq cr6, 0x8313159c
	if ctx.cr[6].eq {
	pc = 0x8313159C; continue 'dispatch;
	}
	// 83131518: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 8313151C: 419A0080  beq cr6, 0x8313159c
	if ctx.cr[6].eq {
	pc = 0x8313159C; continue 'dispatch;
	}
	// 83131520: 2F03000F  cmpwi cr6, r3, 0xf
	ctx.cr[6].compare_i32(ctx.r[3].s32, 15, &mut ctx.xer);
	// 83131524: 419A0078  beq cr6, 0x8313159c
	if ctx.cr[6].eq {
	pc = 0x8313159C; continue 'dispatch;
	}
	// 83131528: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8313152C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83131530: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83131534: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83131538: 48003C59  bl 0x83135190
	ctx.lr = 0x8313153C;
	sub_83135190(ctx, base);
	// 8313153C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131540: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83131544: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131548: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8313154C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83131550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131554: 4E800421  bctrl
	ctx.lr = 0x83131558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131558: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8313155C: 48000044  b 0x831315a0
	pc = 0x831315A0; continue 'dispatch;
	// 83131560: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131564: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83131568: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8313156C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83131570: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131578: 4E800421  bctrl
	ctx.lr = 0x8313157C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313157C: 48006465  bl 0x831379e0
	ctx.lr = 0x83131580;
	sub_831379E0(ctx, base);
	// 83131580: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131584: 4082FE50  bne 0x831313d4
	if !ctx.cr[0].eq {
	pc = 0x831313D4; continue 'dispatch;
	}
	// 83131588: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313158C: 388B1584  addi r4, r11, 0x1584
	ctx.r[4].s64 = ctx.r[11].s64 + 5508;
	// 83131590: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83131594: 386B1564  addi r3, r11, 0x1564
	ctx.r[3].s64 = ctx.r[11].s64 + 5476;
	// 83131598: 4BFFFE38  b 0x831313d0
	pc = 0x831313D0; continue 'dispatch;
	// 8313159C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831315A0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831315A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831315A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831315AC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831315B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831315B4: 4E800421  bctrl
	ctx.lr = 0x831315B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831315B8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831315BC: 4BFFFE1C  b 0x831313d8
	pc = 0x831313D8; continue 'dispatch;
	// 831315C0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 831315C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831315C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831315CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831315D0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831315D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831315D8: 4E800421  bctrl
	ctx.lr = 0x831315DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831315DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831315E0: 48076BCC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831315E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831315E8 size=232
    let mut pc: u32 = 0x831315E8;
    'dispatch: loop {
        match pc {
            0x831315E8 => {
    //   block [0x831315E8..0x831316D0)
	// 831315E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831315EC: 48076B6D  bl 0x831a8158
	ctx.lr = 0x831315F0;
	sub_831A8130(ctx, base);
	// 831315F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831315F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831315F8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 831315FC: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83131600: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 83131604: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 83131608: 835F000C  lwz r26, 0xc(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8313160C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83131610: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131614: 4800659D  bl 0x83137bb0
	ctx.lr = 0x83131618;
	sub_83137BB0(ctx, base);
	// 83131618: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8313161C: 40810044  ble 0x83131660
	if !ctx.cr[0].gt {
	pc = 0x83131660; continue 'dispatch;
	}
	// 83131620: 3BBF001C  addi r29, r31, 0x1c
	ctx.r[29].s64 = ctx.r[31].s64 + 28;
	// 83131624: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131628: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8313162C: 38A04000  li r5, 0x4000
	ctx.r[5].s64 = 16384;
	// 83131630: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131634: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131638: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8313163C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131640: 4E800421  bctrl
	ctx.lr = 0x83131644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131644: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131648: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8313164C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83131650: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 83131654: 4800655D  bl 0x83137bb0
	ctx.lr = 0x83131658;
	sub_83137BB0(ctx, base);
	// 83131658: 7F1C1800  cmpw cr6, r28, r3
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8313165C: 4198FFC8  blt cr6, 0x83131624
	if ctx.cr[6].lt {
	pc = 0x83131624; continue 'dispatch;
	}
	// 83131660: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83131664: 48002F65  bl 0x831345c8
	ctx.lr = 0x83131668;
	sub_831345C8(ctx, base);
	// 83131668: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313166C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83131670: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83131674: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83131678: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8313167C: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83131680: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83131684: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83131688: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8313168C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131690: 40980008  bge cr6, 0x83131698
	if !ctx.cr[6].lt {
	pc = 0x83131698; continue 'dispatch;
	}
	// 83131694: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 83131698: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8313169C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831316A0: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831316A4: 41800010  blt 0x831316b4
	if ctx.cr[0].lt {
	pc = 0x831316B4; continue 'dispatch;
	}
	// 831316A8: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 831316AC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831316B0: 4800000C  b 0x831316bc
	pc = 0x831316BC; continue 'dispatch;
	// 831316B4: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 831316B8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 831316BC: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831316C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831316C4: 4BED7765  bl 0x83008e28
	ctx.lr = 0x831316C8;
	sub_83008E28(ctx, base);
	// 831316C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831316CC: 48076ADC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831316D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831316D0 size=304
    let mut pc: u32 = 0x831316D0;
    'dispatch: loop {
        match pc {
            0x831316D0 => {
    //   block [0x831316D0..0x83131800)
	// 831316D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831316D4: 48076A91  bl 0x831a8164
	ctx.lr = 0x831316D8;
	sub_831A8130(ctx, base);
	// 831316D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831316DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831316E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831316E4: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831316E8: 815E00A8  lwz r10, 0xa8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 831316EC: 555C083C  slwi r28, r10, 1
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 831316F0: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831316F4: 40810070  ble 0x83131764
	if !ctx.cr[0].gt {
	pc = 0x83131764; continue 'dispatch;
	}
	// 831316F8: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 831316FC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131700: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83131704: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83131708: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8313170C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131714: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313171C: 4E800421  bctrl
	ctx.lr = 0x83131720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131720: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83131724: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131728: 41980008  blt cr6, 0x83131730
	if ctx.cr[6].lt {
	pc = 0x83131730; continue 'dispatch;
	}
	// 8313172C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83131730: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131734: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83131738: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313173C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131740: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131748: 4E800421  bctrl
	ctx.lr = 0x8313174C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313174C: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 83131750: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83131754: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83131758: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8313175C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131760: 4198FF9C  blt cr6, 0x831316fc
	if ctx.cr[6].lt {
	pc = 0x831316FC; continue 'dispatch;
	}
	// 83131764: 7F8B0E70  srawi r11, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 83131768: 7F6B0194  addze r27, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[27].s64 = tmp.s64;
	// 8313176C: 577C083D  rlwinm. r28, r27, 1, 0, 0x1e
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83131770: 40810088  ble 0x831317f8
	if !ctx.cr[0].gt {
	pc = 0x831317F8; continue 'dispatch;
	}
	// 83131774: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 83131778: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8313177C: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83131780: 4081006C  ble 0x831317ec
	if !ctx.cr[0].gt {
	pc = 0x831317EC; continue 'dispatch;
	}
	// 83131784: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 83131788: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313178C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83131790: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83131794: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313179C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831317A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831317A4: 4E800421  bctrl
	ctx.lr = 0x831317A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831317A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831317AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831317B0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831317B4: 48076A2D  bl 0x831a81e0
	ctx.lr = 0x831317B8;
	sub_831A81E0(ctx, base);
	// 831317B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831317BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831317C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831317C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831317C8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831317CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831317D0: 4E800421  bctrl
	ctx.lr = 0x831317D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831317D4: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831317D8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831317DC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831317E0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831317E4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831317E8: 4198FFA0  blt cr6, 0x83131788
	if ctx.cr[6].lt {
	pc = 0x83131788; continue 'dispatch;
	}
	// 831317EC: 817E00A8  lwz r11, 0xa8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 831317F0: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 831317F4: 917E00A8  stw r11, 0xa8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 831317F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831317FC: 480769B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83131800 size=288
    let mut pc: u32 = 0x83131800;
    'dispatch: loop {
        match pc {
            0x83131800 => {
    //   block [0x83131800..0x83131920)
	// 83131800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131804: 48076961  bl 0x831a8164
	ctx.lr = 0x83131808;
	sub_831A8130(ctx, base);
	// 83131808: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313180C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83131810: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83131814: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 83131818: 815E00AC  lwz r10, 0xac(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 8313181C: 555C083C  slwi r28, r10, 1
	ctx.r[28].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83131820: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83131824: 40810070  ble 0x83131894
	if !ctx.cr[0].gt {
	pc = 0x83131894; continue 'dispatch;
	}
	// 83131828: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 8313182C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131830: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83131834: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83131838: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 8313183C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131840: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131844: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131848: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313184C: 4E800421  bctrl
	ctx.lr = 0x83131850;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131850: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83131854: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131858: 41980008  blt cr6, 0x83131860
	if ctx.cr[6].lt {
	pc = 0x83131860; continue 'dispatch;
	}
	// 8313185C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83131860: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131864: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83131868: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8313186C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131870: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131874: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131878: 4E800421  bctrl
	ctx.lr = 0x8313187C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313187C: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 83131880: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83131884: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83131888: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8313188C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131890: 4198FF9C  blt cr6, 0x8313182c
	if ctx.cr[6].lt {
	pc = 0x8313182C; continue 'dispatch;
	}
	// 83131894: 7F8B0E70  srawi r11, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 83131898: 7F6B0194  addze r27, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[27].s64 = tmp.s64;
	// 8313189C: 577C083D  rlwinm. r28, r27, 1, 0, 0x1e
	ctx.r[28].u64 = ctx.r[27].u32 as u64 & 0x7FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831318A0: 40810078  ble 0x83131918
	if !ctx.cr[0].gt {
	pc = 0x83131918; continue 'dispatch;
	}
	// 831318A4: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831318A8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831318AC: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831318B0: 4081005C  ble 0x8313190c
	if !ctx.cr[0].gt {
	pc = 0x8313190C; continue 'dispatch;
	}
	// 831318B4: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 831318B8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831318BC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831318C0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831318C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831318C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831318CC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831318D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831318D4: 4E800421  bctrl
	ctx.lr = 0x831318D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831318D8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831318DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831318E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831318E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831318E8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831318EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831318F0: 4E800421  bctrl
	ctx.lr = 0x831318F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831318F4: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 831318F8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831318FC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83131900: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83131904: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131908: 4198FFB0  blt cr6, 0x831318b8
	if ctx.cr[6].lt {
	pc = 0x831318B8; continue 'dispatch;
	}
	// 8313190C: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 83131910: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 83131914: 917E00AC  stw r11, 0xac(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 83131918: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8313191C: 48076898  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131920 size=8
    let mut pc: u32 = 0x83131920;
    'dispatch: loop {
        match pc {
            0x83131920 => {
    //   block [0x83131920..0x83131928)
	// 83131920: 908300A4  stw r4, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 83131924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131928 size=12
    let mut pc: u32 = 0x83131928;
    'dispatch: loop {
        match pc {
            0x83131928 => {
    //   block [0x83131928..0x83131934)
	// 83131928: 90830050  stw r4, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 8313192C: 90A30054  stw r5, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 83131930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131938 size=12
    let mut pc: u32 = 0x83131938;
    'dispatch: loop {
        match pc {
            0x83131938 => {
    //   block [0x83131938..0x83131944)
	// 83131938: 90830048  stw r4, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[4].u32 ) };
	// 8313193C: 90A3004C  stw r5, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 83131940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131948 size=8
    let mut pc: u32 = 0x83131948;
    'dispatch: loop {
        match pc {
            0x83131948 => {
    //   block [0x83131948..0x83131950)
	// 83131948: 9083003C  stw r4, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 8313194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131950 size=8
    let mut pc: u32 = 0x83131950;
    'dispatch: loop {
        match pc {
            0x83131950 => {
    //   block [0x83131950..0x83131958)
	// 83131950: 90830044  stw r4, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 83131954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131958 size=8
    let mut pc: u32 = 0x83131958;
    'dispatch: loop {
        match pc {
            0x83131958 => {
    //   block [0x83131958..0x83131960)
	// 83131958: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313195C: 4800624C  b 0x83137ba8
	sub_83137BA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131960 size=8
    let mut pc: u32 = 0x83131960;
    'dispatch: loop {
        match pc {
            0x83131960 => {
    //   block [0x83131960..0x83131968)
	// 83131960: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131964: 4BF217DC  b 0x83053140
	sub_83053140(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131968 size=8
    let mut pc: u32 = 0x83131968;
    'dispatch: loop {
        match pc {
            0x83131968 => {
    //   block [0x83131968..0x83131970)
	// 83131968: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313196C: 48006244  b 0x83137bb0
	sub_83137BB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131970 size=8
    let mut pc: u32 = 0x83131970;
    'dispatch: loop {
        match pc {
            0x83131970 => {
    //   block [0x83131970..0x83131978)
	// 83131970: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131974: 4800628C  b 0x83137c00
	sub_83137C00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131978 size=8
    let mut pc: u32 = 0x83131978;
    'dispatch: loop {
        match pc {
            0x83131978 => {
    //   block [0x83131978..0x83131980)
	// 83131978: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313197C: 48022C7C  b 0x831545f8
	sub_831545F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131980 size=8
    let mut pc: u32 = 0x83131980;
    'dispatch: loop {
        match pc {
            0x83131980 => {
    //   block [0x83131980..0x83131988)
	// 83131980: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131984: 4BF217C4  b 0x83053148
	sub_83053148(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131988 size=8
    let mut pc: u32 = 0x83131988;
    'dispatch: loop {
        match pc {
            0x83131988 => {
    //   block [0x83131988..0x83131990)
	// 83131988: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313198C: 480062CC  b 0x83137c58
	sub_83137C58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131990 size=8
    let mut pc: u32 = 0x83131990;
    'dispatch: loop {
        match pc {
            0x83131990 => {
    //   block [0x83131990..0x83131998)
	// 83131990: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131994: 4BF226AC  b 0x83054040
	sub_83054040(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131998 size=12
    let mut pc: u32 = 0x83131998;
    'dispatch: loop {
        match pc {
            0x83131998 => {
    //   block [0x83131998..0x831319A4)
	// 83131998: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8313199C: 409A0008  bne cr6, 0x831319a4
	if !ctx.cr[6].eq {
		sub_831319A4(ctx, base);
		return;
	}
	// 831319A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831319A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831319A4 size=8
    let mut pc: u32 = 0x831319A4;
    'dispatch: loop {
        match pc {
            0x831319A4 => {
    //   block [0x831319A4..0x831319AC)
	// 831319A4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831319A8: 480062B8  b 0x83137c60
	sub_83137C60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831319B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831319B0 size=8
    let mut pc: u32 = 0x831319B0;
    'dispatch: loop {
        match pc {
            0x831319B0 => {
    //   block [0x831319B0..0x831319B8)
	// 831319B0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831319B4: 4BEA29E4  b 0x82fd4398
	sub_82FD4398(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831319B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831319B8 size=8
    let mut pc: u32 = 0x831319B8;
    'dispatch: loop {
        match pc {
            0x831319B8 => {
    //   block [0x831319B8..0x831319C0)
	// 831319B8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831319BC: 4BED7AC4  b 0x83009480
	sub_83009480(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831319C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831319C0 size=96
    let mut pc: u32 = 0x831319C0;
    'dispatch: loop {
        match pc {
            0x831319C0 => {
    //   block [0x831319C0..0x83131A20)
	// 831319C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831319C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831319C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831319CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831319D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831319D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831319D8: 48006299  bl 0x83137c70
	ctx.lr = 0x831319DC;
	sub_83137C70(ctx, base);
	// 831319DC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831319E0: 40810028  ble 0x83131a08
	if !ctx.cr[0].gt {
	pc = 0x83131A08; continue 'dispatch;
	}
	// 831319E4: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 831319E8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 831319EC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831319F0: 419A000C  beq cr6, 0x831319fc
	if ctx.cr[6].eq {
	pc = 0x831319FC; continue 'dispatch;
	}
	// 831319F4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 831319F8: 409A0010  bne cr6, 0x83131a08
	if !ctx.cr[6].eq {
	pc = 0x83131A08; continue 'dispatch;
	}
	// 831319FC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131A00: 48006279  bl 0x83137c78
	ctx.lr = 0x83131A04;
	sub_83137C78(ctx, base);
	// 83131A04: 48000008  b 0x83131a0c
	pc = 0x83131A0C; continue 'dispatch;
	// 83131A08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83131A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83131A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83131A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83131A18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83131A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83131A20 size=112
    let mut pc: u32 = 0x83131A20;
    'dispatch: loop {
        match pc {
            0x83131A20 => {
    //   block [0x83131A20..0x83131A90)
	// 83131A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83131A28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83131A2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83131A30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131A34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83131A38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83131A3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131A40: 48006231  bl 0x83137c70
	ctx.lr = 0x83131A44;
	sub_83137C70(ctx, base);
	// 83131A44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131A48: 4081002C  ble 0x83131a74
	if !ctx.cr[0].gt {
	pc = 0x83131A74; continue 'dispatch;
	}
	// 83131A4C: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83131A50: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83131A54: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83131A58: 419A000C  beq cr6, 0x83131a64
	if ctx.cr[6].eq {
	pc = 0x83131A64; continue 'dispatch;
	}
	// 83131A5C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83131A60: 409A0014  bne cr6, 0x83131a74
	if !ctx.cr[6].eq {
	pc = 0x83131A74; continue 'dispatch;
	}
	// 83131A64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83131A68: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131A6C: 48006215  bl 0x83137c80
	ctx.lr = 0x83131A70;
	sub_83137C80(ctx, base);
	// 83131A70: 48000008  b 0x83131a78
	pc = 0x83131A78; continue 'dispatch;
	// 83131A74: 3860FF80  li r3, -0x80
	ctx.r[3].s64 = -128;
	// 83131A78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83131A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83131A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83131A84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83131A88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83131A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131A90 size=8
    let mut pc: u32 = 0x83131A90;
    'dispatch: loop {
        match pc {
            0x83131A90 => {
    //   block [0x83131A90..0x83131A98)
	// 83131A90: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131A94: 480061FC  b 0x83137c90
	sub_83137C90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83131A98 size=8
    let mut pc: u32 = 0x83131A98;
    'dispatch: loop {
        match pc {
            0x83131A98 => {
    //   block [0x83131A98..0x83131AA0)
	// 83131A98: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131A9C: 48006244  b 0x83137ce0
	sub_83137CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83131AA0 size=344
    let mut pc: u32 = 0x83131AA0;
    'dispatch: loop {
        match pc {
            0x83131AA0 => {
    //   block [0x83131AA0..0x83131BF8)
	// 83131AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131AA4: 480766B9  bl 0x831a815c
	ctx.lr = 0x83131AA8;
	sub_831A8130(ctx, base);
	// 83131AA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131AAC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83131AB0: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83131AB4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83131AB8: 396BBC00  addi r11, r11, -0x4400
	ctx.r[11].s64 = ctx.r[11].s64 + -17408;
	// 83131ABC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83131AC0: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131AC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83131AC8: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 83131ACC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83131AD0: 89090000  lbz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131AD4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83131AD8: 419A0018  beq cr6, 0x83131af0
	if ctx.cr[6].eq {
	pc = 0x83131AF0; continue 'dispatch;
	}
	// 83131ADC: 392900B4  addi r9, r9, 0xb4
	ctx.r[9].s64 = ctx.r[9].s64 + 180;
	// 83131AE0: 390B1680  addi r8, r11, 0x1680
	ctx.r[8].s64 = ctx.r[11].s64 + 5760;
	// 83131AE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83131AE8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83131AEC: 4198FFE4  blt cr6, 0x83131ad0
	if ctx.cr[6].lt {
	pc = 0x83131AD0; continue 'dispatch;
	}
	// 83131AF0: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 83131AF4: 409A000C  bne cr6, 0x83131b00
	if !ctx.cr[6].eq {
	pc = 0x83131B00; continue 'dispatch;
	}
	// 83131AF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83131AFC: 480000F4  b 0x83131bf0
	pc = 0x83131BF0; continue 'dispatch;
	// 83131B00: 1D4A00B4  mulli r10, r10, 0xb4
	ctx.r[10].s64 = ctx.r[10].s64 * 180;
	// 83131B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131B08: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83131B0C: 48002ABD  bl 0x831345c8
	ctx.lr = 0x83131B10;
	sub_831345C8(ctx, base);
	// 83131B10: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83131B14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131B18: 48002B31  bl 0x83134648
	ctx.lr = 0x83131B1C;
	sub_83134648(ctx, base);
	// 83131B1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83131B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131B24: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83131B28: 7FCB0194  addze r30, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[30].s64 = tmp.s64;
	// 83131B2C: 48002B9D  bl 0x831346c8
	ctx.lr = 0x83131B30;
	sub_831346C8(ctx, base);
	// 83131B30: 7C6B0E70  srawi r11, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 83131B34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83131B38: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83131B3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83131B40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83131B44: 7CCBF214  add r6, r11, r30
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83131B48: 48006769  bl 0x831382b0
	ctx.lr = 0x83131B4C;
	sub_831382B0(ctx, base);
	// 83131B4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83131B50: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83131B54: 4182FFA4  beq 0x83131af8
	if ctx.cr[0].eq {
	pc = 0x83131AF8; continue 'dispatch;
	}
	// 83131B58: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 83131B5C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83131B60: 388B15E8  addi r4, r11, 0x15e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5608;
	// 83131B64: 48006035  bl 0x83137b98
	ctx.lr = 0x83131B68;
	sub_83137B98(ctx, base);
	// 83131B68: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83131B6C: 935F0008  stw r26, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 83131B70: 9BBF0002  stb r29, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[29].u8 ) };
	// 83131B74: 40990024  ble cr6, 0x83131b98
	if !ctx.cr[6].gt {
	pc = 0x83131B98; continue 'dispatch;
	}
	// 83131B78: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 83131B7C: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 83131B80: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131B84: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83131B88: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83131B8C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83131B90: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83131B94: 4082FFEC  bne 0x83131b80
	if !ctx.cr[0].eq {
	pc = 0x83131B80; continue 'dispatch;
	}
	// 83131B98: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 83131B9C: 9B3F0001  stb r25, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[25].u8 ) };
	// 83131BA0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83131BA4: 933F00A0  stw r25, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[25].u32 ) };
	// 83131BA8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 83131BAC: 933F002C  stw r25, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[25].u32 ) };
	// 83131BB0: 933F0030  stw r25, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[25].u32 ) };
	// 83131BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83131BB8: 933F0034  stw r25, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[25].u32 ) };
	// 83131BBC: 933F0040  stw r25, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[25].u32 ) };
	// 83131BC0: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 83131BC4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83131BC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83131BCC: 933F0044  stw r25, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[25].u32 ) };
	// 83131BD0: 9B3F0003  stb r25, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[25].u8 ) };
	// 83131BD4: 933F00A8  stw r25, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[25].u32 ) };
	// 83131BD8: 933F00AC  stw r25, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[25].u32 ) };
	// 83131BDC: 933F0048  stw r25, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[25].u32 ) };
	// 83131BE0: 933F004C  stw r25, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[25].u32 ) };
	// 83131BE4: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 83131BE8: 933F005C  stw r25, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 83131BEC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83131BF0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83131BF4: 480765B8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83131BF8 size=952
    let mut pc: u32 = 0x83131BF8;
    'dispatch: loop {
        match pc {
            0x83131BF8 => {
    //   block [0x83131BF8..0x83131FB0)
	// 83131BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131BFC: 48076561  bl 0x831a815c
	ctx.lr = 0x83131C00;
	sub_831A8130(ctx, base);
	// 83131C00: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83131C08: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83131C0C: 833F0004  lwz r25, 4(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131C10: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83131C14: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131C18: 41800028  blt 0x83131c40
	if ctx.cr[0].lt {
	pc = 0x83131C40; continue 'dispatch;
	}
	// 83131C1C: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83131C20: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131C24: 4198001C  blt cr6, 0x83131c40
	if ctx.cr[6].lt {
	pc = 0x83131C40; continue 'dispatch;
	}
	// 83131C28: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83131C2C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83131C30: 41820010  beq 0x83131c40
	if ctx.cr[0].eq {
	pc = 0x83131C40; continue 'dispatch;
	}
	// 83131C34: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83131C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131C3C: 4E800421  bctrl
	ctx.lr = 0x83131C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131C40: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 83131C44: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83131C48: 409A0030  bne cr6, 0x83131c78
	if !ctx.cr[6].eq {
	pc = 0x83131C78; continue 'dispatch;
	}
	// 83131C4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131C50: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131C54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131C58: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83131C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131C60: 4E800421  bctrl
	ctx.lr = 0x83131C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131C64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131C68: 40820010  bne 0x83131c78
	if !ctx.cr[0].eq {
	pc = 0x83131C78; continue 'dispatch;
	}
	// 83131C6C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83131C70: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83131C74: 48000334  b 0x83131fa8
	pc = 0x83131FA8; continue 'dispatch;
	// 83131C78: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131C7C: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83131C80: 3BBF0014  addi r29, r31, 0x14
	ctx.r[29].s64 = ctx.r[31].s64 + 20;
	// 83131C84: 615AFFFF  ori r26, r10, 0xffff
	ctx.r[26].u64 = ctx.r[10].u64 | 65535;
	// 83131C88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131C8C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83131C90: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131C94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131C98: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83131C9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131CA0: 4E800421  bctrl
	ctx.lr = 0x83131CA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131CA4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131CA8: 48005F01  bl 0x83137ba8
	ctx.lr = 0x83131CAC;
	sub_83137BA8(ctx, base);
	// 83131CAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131CB0: 40820170  bne 0x83131e20
	if !ctx.cr[0].eq {
	pc = 0x83131E20; continue 'dispatch;
	}
	// 83131CB4: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131CB8: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 83131CBC: 41980164  blt cr6, 0x83131e20
	if ctx.cr[6].lt {
	pc = 0x83131E20; continue 'dispatch;
	}
	// 83131CC0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131CC4: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131CC8: 2B0B8001  cmplwi cr6, r11, 0x8001
	ctx.cr[6].compare_u32(ctx.r[11].u32, 32769 as u32, &mut ctx.xer);
	// 83131CCC: 409A0154  bne cr6, 0x83131e20
	if !ctx.cr[6].eq {
	pc = 0x83131E20; continue 'dispatch;
	}
	// 83131CD0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83131CD4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83131CD8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83131CDC: 480045CD  bl 0x831362a8
	ctx.lr = 0x83131CE0;
	sub_831362A8(ctx, base);
	// 83131CE0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131CE4: 40820078  bne 0x83131d5c
	if !ctx.cr[0].eq {
	pc = 0x83131D5C; continue 'dispatch;
	}
	// 83131CE8: A8810050  lha r4, 0x50(r1)
	ctx.r[4].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i16) as i64;
	// 83131CEC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131CF0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131CF4: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83131CF8: 40990020  ble cr6, 0x83131d18
	if !ctx.cr[6].gt {
	pc = 0x83131D18; continue 'dispatch;
	}
	// 83131CFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131D00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131D04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131D08: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131D0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131D10: 4E800421  bctrl
	ctx.lr = 0x83131D14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131D14: 48000294  b 0x83131fa8
	pc = 0x83131FA8; continue 'dispatch;
	// 83131D18: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83131D1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83131D20: 48003471  bl 0x83135190
	ctx.lr = 0x83131D24;
	sub_83135190(ctx, base);
	// 83131D24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131D28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131D2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131D34: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83131D38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131D3C: 4E800421  bctrl
	ctx.lr = 0x83131D40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131D40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131D44: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83131D48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131D4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131D50: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131D54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131D58: 4E800421  bctrl
	ctx.lr = 0x83131D5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131D5C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83131D60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83131D64: 419A0244  beq cr6, 0x83131fa8
	if ctx.cr[6].eq {
	pc = 0x83131FA8; continue 'dispatch;
	}
	// 83131D68: 48000088  b 0x83131df0
	pc = 0x83131DF0; continue 'dispatch;
	// 83131D6C: 837F0018  lwz r27, 0x18(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131D70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83131D74: 2C1B0000  cmpwi r27, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83131D78: 40810024  ble 0x83131d9c
	if !ctx.cr[0].gt {
	pc = 0x83131D9C; continue 'dispatch;
	}
	// 83131D7C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131D80: 7D4BE0AE  lbzx r10, r11, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 83131D84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83131D88: 409A0014  bne cr6, 0x83131d9c
	if !ctx.cr[6].eq {
	pc = 0x83131D9C; continue 'dispatch;
	}
	// 83131D8C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131D90: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83131D94: 7F1C5000  cmpw cr6, r28, r10
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83131D98: 4198FFE8  blt cr6, 0x83131d80
	if ctx.cr[6].lt {
	pc = 0x83131D80; continue 'dispatch;
	}
	// 83131D9C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83131DA0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131DA4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83131DA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83131DAC: 480033E5  bl 0x83135190
	ctx.lr = 0x83131DB0;
	sub_83135190(ctx, base);
	// 83131DB0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131DB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131DB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131DBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131DC0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83131DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131DC8: 4E800421  bctrl
	ctx.lr = 0x83131DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131DCC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131DD0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83131DD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131DD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131DDC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131DE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131DE4: 4E800421  bctrl
	ctx.lr = 0x83131DE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131DE8: 7F1CD800  cmpw cr6, r28, r27
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83131DEC: 419801BC  blt cr6, 0x83131fa8
	if ctx.cr[6].lt {
	pc = 0x83131FA8; continue 'dispatch;
	}
	// 83131DF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131DF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131DF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131DFC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83131E00: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83131E04: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131E0C: 4E800421  bctrl
	ctx.lr = 0x83131E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131E10: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131E14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83131E18: 409AFF54  bne cr6, 0x83131d6c
	if !ctx.cr[6].eq {
	pc = 0x83131D6C; continue 'dispatch;
	}
	// 83131E1C: 4800018C  b 0x83131fa8
	pc = 0x83131FA8; continue 'dispatch;
	// 83131E20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131E24: 4BF21325  bl 0x83053148
	ctx.lr = 0x83131E28;
	sub_83053148(ctx, base);
	// 83131E28: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83131E2C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83131E30: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131E34: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83131E38: 4198005C  blt cr6, 0x83131e94
	if ctx.cr[6].lt {
	pc = 0x83131E94; continue 'dispatch;
	}
	// 83131E3C: 48005D6D  bl 0x83137ba8
	ctx.lr = 0x83131E40;
	sub_83137BA8(ctx, base);
	// 83131E40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83131E44: 409A006C  bne cr6, 0x83131eb0
	if !ctx.cr[6].eq {
	pc = 0x83131EB0; continue 'dispatch;
	}
	// 83131E48: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 83131E4C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83131E50: 409A0060  bne cr6, 0x83131eb0
	if !ctx.cr[6].eq {
	pc = 0x83131EB0; continue 'dispatch;
	}
	// 83131E54: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131E58: 480227A1  bl 0x831545f8
	ctx.lr = 0x83131E5C;
	sub_831545F8(ctx, base);
	// 83131E5C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83131E60: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83131E64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83131E68: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83131E6C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131E70: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 83131E74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131E78: 4E800421  bctrl
	ctx.lr = 0x83131E7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131E7C: 7C6B0E70  srawi r11, r3, 1
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 1) as i64;
	// 83131E80: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83131E84: 7F0BD800  cmpw cr6, r11, r27
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[27].s32, &mut ctx.xer);
	// 83131E88: 40980038  bge cr6, 0x83131ec0
	if !ctx.cr[6].lt {
	pc = 0x83131EC0; continue 'dispatch;
	}
	// 83131E8C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131E90: 4BFFFE6C  b 0x83131cfc
	pc = 0x83131CFC; continue 'dispatch;
	// 83131E94: 48005D15  bl 0x83137ba8
	ctx.lr = 0x83131E98;
	sub_83137BA8(ctx, base);
	// 83131E98: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 83131E9C: 409AFFB8  bne cr6, 0x83131e54
	if !ctx.cr[6].eq {
	pc = 0x83131E54; continue 'dispatch;
	}
	// 83131EA0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83131EA4: 396B0240  addi r11, r11, 0x240
	ctx.r[11].s64 = ctx.r[11].s64 + 576;
	// 83131EA8: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83131EAC: 4198FFA8  blt cr6, 0x83131e54
	if ctx.cr[6].lt {
	pc = 0x83131E54; continue 'dispatch;
	}
	// 83131EB0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83131EB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131EB8: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83131EBC: 4BFFFE40  b 0x83131cfc
	pc = 0x83131CFC; continue 'dispatch;
	// 83131EC0: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 83131EC4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83131EC8: 419A009C  beq cr6, 0x83131f64
	if ctx.cr[6].eq {
	pc = 0x83131F64; continue 'dispatch;
	}
	// 83131ECC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131ED0: 48005CD9  bl 0x83137ba8
	ctx.lr = 0x83131ED4;
	sub_83137BA8(ctx, base);
	// 83131ED4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83131ED8: 409A008C  bne cr6, 0x83131f64
	if !ctx.cr[6].eq {
	pc = 0x83131F64; continue 'dispatch;
	}
	// 83131EDC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131EE0: 48005D11  bl 0x83137bf0
	ctx.lr = 0x83131EE4;
	sub_83137BF0(ctx, base);
	// 83131EE4: 2F030010  cmpwi cr6, r3, 0x10
	ctx.cr[6].compare_i32(ctx.r[3].s32, 16, &mut ctx.xer);
	// 83131EE8: 409A0068  bne cr6, 0x83131f50
	if !ctx.cr[6].eq {
	pc = 0x83131F50; continue 'dispatch;
	}
	// 83131EEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131EF0: 48005CC1  bl 0x83137bb0
	ctx.lr = 0x83131EF4;
	sub_83137BB0(ctx, base);
	// 83131EF4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131EF8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83131EFC: 7D4A1BD6  divw r10, r10, r3
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[3].s32;
	// 83131F00: 7D4A0E70  srawi r10, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 83131F04: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83131F08: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83131F0C: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83131F10: 40990054  ble cr6, 0x83131f64
	if !ctx.cr[6].gt {
	pc = 0x83131F64; continue 'dispatch;
	}
	// 83131F14: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83131F18: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83131F1C: 7D6B19D6  mullw r11, r11, r3
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[3].s32 as i64);
	// 83131F20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131F24: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83131F28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83131F2C: 48003265  bl 0x83135190
	ctx.lr = 0x83131F30;
	sub_83135190(ctx, base);
	// 83131F30: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131F34: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83131F38: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131F3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131F40: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131F44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131F48: 4E800421  bctrl
	ctx.lr = 0x83131F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131F4C: 48000018  b 0x83131f64
	pc = 0x83131F64; continue 'dispatch;
	// 83131F50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83131F54: 388B160C  addi r4, r11, 0x160c
	ctx.r[4].s64 = ctx.r[11].s64 + 5644;
	// 83131F58: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83131F5C: 386B15E8  addi r3, r11, 0x15e8
	ctx.r[3].s64 = ctx.r[11].s64 + 5608;
	// 83131F60: 4BFFEB01  bl 0x83130a60
	ctx.lr = 0x83131F64;
	sub_83130A60(ctx, base);
	// 83131F64: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131F68: 48005C41  bl 0x83137ba8
	ctx.lr = 0x83131F6C;
	sub_83137BA8(ctx, base);
	// 83131F6C: 2F03000A  cmpwi cr6, r3, 0xa
	ctx.cr[6].compare_i32(ctx.r[3].s32, 10, &mut ctx.xer);
	// 83131F70: 409A0020  bne cr6, 0x83131f90
	if !ctx.cr[6].eq {
	pc = 0x83131F90; continue 'dispatch;
	}
	// 83131F74: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131F78: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83131F7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83131F80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83131F84: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83131F88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83131F8C: 4E800421  bctrl
	ctx.lr = 0x83131F90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83131F90: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131F94: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83131F98: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83131F9C: 48005E75  bl 0x83137e10
	ctx.lr = 0x83131FA0;
	sub_83137E10(ctx, base);
	// 83131FA0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83131FA4: 48005ECD  bl 0x83137e70
	ctx.lr = 0x83131FA8;
	sub_83137E70(ctx, base);
	// 83131FA8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83131FAC: 48076200  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83131FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83131FB0 size=432
    let mut pc: u32 = 0x83131FB0;
    'dispatch: loop {
        match pc {
            0x83131FB0 => {
    //   block [0x83131FB0..0x83132160)
	// 83131FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83131FB4: 480761A5  bl 0x831a8158
	ctx.lr = 0x83131FB8;
	sub_831A8130(ctx, base);
	// 83131FB8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83131FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83131FC0: 835F0004  lwz r26, 4(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83131FC4: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83131FC8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83131FCC: 4BF2117D  bl 0x83053148
	ctx.lr = 0x83131FD0;
	sub_83053148(ctx, base);
	// 83131FD0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83131FD4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83131FD8: 48005F31  bl 0x83137f08
	ctx.lr = 0x83131FDC;
	sub_83137F08(ctx, base);
	// 83131FDC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83131FE0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83131FE4: 48005F2D  bl 0x83137f10
	ctx.lr = 0x83131FE8;
	sub_83137F10(ctx, base);
	// 83131FE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83131FEC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83131FF0: 48005BB9  bl 0x83137ba8
	ctx.lr = 0x83131FF4;
	sub_83137BA8(ctx, base);
	// 83131FF4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83131FF8: 409A0010  bne cr6, 0x83132008
	if !ctx.cr[6].eq {
	pc = 0x83132008; continue 'dispatch;
	}
	// 83131FFC: 817F00B0  lwz r11, 0xb0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 83132000: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83132004: 419A0018  beq cr6, 0x8313201c
	if ctx.cr[6].eq {
	pc = 0x8313201C; continue 'dispatch;
	}
	// 83132008: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8313200C: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83132010: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132014: 41980008  blt cr6, 0x8313201c
	if ctx.cr[6].lt {
	pc = 0x8313201C; continue 'dispatch;
	}
	// 83132018: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 8313201C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83132020: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83132024: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83132028: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8313202C: 48003165  bl 0x83135190
	ctx.lr = 0x83132030;
	sub_83135190(ctx, base);
	// 83132030: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132034: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83132038: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313203C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132040: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83132044: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132048: 4E800421  bctrl
	ctx.lr = 0x8313204C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313204C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132050: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83132054: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83132058: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313205C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83132060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132064: 4E800421  bctrl
	ctx.lr = 0x83132068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132068: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313206C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83132070: 48005B41  bl 0x83137bb0
	ctx.lr = 0x83132074;
	sub_83137BB0(ctx, base);
	// 83132074: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83132078: 4081009C  ble 0x83132114
	if !ctx.cr[0].gt {
	pc = 0x83132114; continue 'dispatch;
	}
	// 8313207C: 57D8083C  slwi r24, r30, 1
	ctx.r[24].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 83132080: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 83132084: 3B9F001C  addi r28, r31, 0x1c
	ctx.r[28].s64 = ctx.r[31].s64 + 28;
	// 83132088: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8313208C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83132090: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83132094: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83132098: 480030F9  bl 0x83135190
	ctx.lr = 0x8313209C;
	sub_83135190(ctx, base);
	// 8313209C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 831320A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831320A4: 4182001C  beq 0x831320c0
	if ctx.cr[0].eq {
	pc = 0x831320C0; continue 'dispatch;
	}
	// 831320A8: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 831320AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831320B0: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831320B4: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831320B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831320BC: 4E800421  bctrl
	ctx.lr = 0x831320C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831320C0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831320C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831320C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831320CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831320D0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831320D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831320D8: 4E800421  bctrl
	ctx.lr = 0x831320DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831320DC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831320E0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831320E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831320E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831320EC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 831320F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831320F4: 4E800421  bctrl
	ctx.lr = 0x831320F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831320F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831320FC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83132100: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83132104: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 83132108: 48005AA9  bl 0x83137bb0
	ctx.lr = 0x8313210C;
	sub_83137BB0(ctx, base);
	// 8313210C: 7F1B1800  cmpw cr6, r27, r3
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83132110: 4198FF78  blt cr6, 0x83132088
	if ctx.cr[6].lt {
	pc = 0x83132088; continue 'dispatch;
	}
	// 83132114: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83132118: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8313211C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83132120: 7D0AF214  add r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 83132124: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83132128: 7CEBCA14  add r7, r11, r25
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8313212C: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83132130: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83132134: 7D29F214  add r9, r9, r30
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 83132138: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8313213C: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 83132140: 911F002C  stw r8, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 83132144: 90FF0030  stw r7, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 83132148: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 8313214C: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 83132150: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83132154: 48005D6D  bl 0x83137ec0
	ctx.lr = 0x83132158;
	sub_83137EC0(ctx, base);
	// 83132158: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8313215C: 4807604C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132160 size=216
    let mut pc: u32 = 0x83132160;
    'dispatch: loop {
        match pc {
            0x83132160 => {
    //   block [0x83132160..0x83132238)
	// 83132160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132164: 48076005  bl 0x831a8168
	ctx.lr = 0x83132168;
	sub_831A8130(ctx, base);
	// 83132168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313216C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132170: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83132174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83132178: 4BF205D1  bl 0x83052748
	ctx.lr = 0x8313217C;
	sub_83052748(ctx, base);
	// 8313217C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83132180: 4082000C  bne 0x8313218c
	if !ctx.cr[0].eq {
	pc = 0x8313218C; continue 'dispatch;
	}
	// 83132184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132188: 4BFFFA71  bl 0x83131bf8
	ctx.lr = 0x8313218C;
	sub_83131BF8(ctx, base);
	// 8313218C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83132190: 480065C9  bl 0x83138758
	ctx.lr = 0x83132194;
	sub_83138758(ctx, base);
	// 83132194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83132198: 4BF205B1  bl 0x83052748
	ctx.lr = 0x8313219C;
	sub_83052748(ctx, base);
	// 8313219C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 831321A0: 409A000C  bne cr6, 0x831321ac
	if !ctx.cr[6].eq {
	pc = 0x831321AC; continue 'dispatch;
	}
	// 831321A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831321A8: 4BFFFE09  bl 0x83131fb0
	ctx.lr = 0x831321AC;
	sub_83131FB0(ctx, base);
	// 831321AC: A97E0098  lha r11, 0x98(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(152 as u32) ) } as i16) as i64;
	// 831321B0: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 831321B4: 419A0024  beq cr6, 0x831321d8
	if ctx.cr[6].eq {
	pc = 0x831321D8; continue 'dispatch;
	}
	// 831321B8: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 831321BC: 419A001C  beq cr6, 0x831321d8
	if ctx.cr[6].eq {
	pc = 0x831321D8; continue 'dispatch;
	}
	// 831321C0: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 831321C4: 419A0014  beq cr6, 0x831321d8
	if ctx.cr[6].eq {
	pc = 0x831321D8; continue 'dispatch;
	}
	// 831321C8: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 831321CC: 419A000C  beq cr6, 0x831321d8
	if ctx.cr[6].eq {
	pc = 0x831321D8; continue 'dispatch;
	}
	// 831321D0: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 831321D4: 409A005C  bne cr6, 0x83132230
	if !ctx.cr[6].eq {
	pc = 0x83132230; continue 'dispatch;
	}
	// 831321D8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831321DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831321E0: 4BF20F69  bl 0x83053148
	ctx.lr = 0x831321E4;
	sub_83053148(ctx, base);
	// 831321E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831321E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831321EC: 48005D1D  bl 0x83137f08
	ctx.lr = 0x831321F0;
	sub_83137F08(ctx, base);
	// 831321F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831321F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831321F8: 48005D19  bl 0x83137f10
	ctx.lr = 0x831321FC;
	sub_83137F10(ctx, base);
	// 831321FC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83132200: 7D4BE850  subf r10, r11, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 83132204: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83132208: 41980008  blt cr6, 0x83132210
	if ctx.cr[6].lt {
	pc = 0x83132210; continue 'dispatch;
	}
	// 8313220C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83132210: 7D2B1A14  add r9, r11, r3
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83132214: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83132218: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8313221C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83132220: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 83132224: 913F0034  stw r9, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[9].u32 ) };
	// 83132228: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8313222C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83132230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83132234: 48075F84  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132238 size=140
    let mut pc: u32 = 0x83132238;
    'dispatch: loop {
        match pc {
            0x83132238 => {
    //   block [0x83132238..0x831322C4)
	// 83132238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313223C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83132244: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313224C: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 83132250: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83132254: 40990014  ble cr6, 0x83132268
	if !ctx.cr[6].gt {
	pc = 0x83132268; continue 'dispatch;
	}
	// 83132258: 4BF95889  bl 0x830c7ae0
	ctx.lr = 0x8313225C;
	sub_830C7AE0(ctx, base);
	// 8313225C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132260: 4BFFF471  bl 0x831316d0
	ctx.lr = 0x83132264;
	sub_831316D0(ctx, base);
	// 83132264: 4BF9587D  bl 0x830c7ae0
	ctx.lr = 0x83132268;
	sub_830C7AE0(ctx, base);
	// 83132268: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8313226C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83132270: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83132274: 409A0010  bne cr6, 0x83132284
	if !ctx.cr[6].eq {
	pc = 0x83132284; continue 'dispatch;
	}
	// 83132278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313227C: 4BFFFEE5  bl 0x83132160
	ctx.lr = 0x83132280;
	sub_83132160(ctx, base);
	// 83132280: 48000014  b 0x83132294
	pc = 0x83132294; continue 'dispatch;
	// 83132284: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83132288: 409A000C  bne cr6, 0x83132294
	if !ctx.cr[6].eq {
	pc = 0x83132294; continue 'dispatch;
	}
	// 8313228C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132290: 4BFFF089  bl 0x83131318
	ctx.lr = 0x83132294;
	sub_83131318(ctx, base);
	// 83132294: 817F00AC  lwz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83132298: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8313229C: 40990014  ble cr6, 0x831322b0
	if !ctx.cr[6].gt {
	pc = 0x831322B0; continue 'dispatch;
	}
	// 831322A0: 4BF95841  bl 0x830c7ae0
	ctx.lr = 0x831322A4;
	sub_830C7AE0(ctx, base);
	// 831322A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831322A8: 4BFFF559  bl 0x83131800
	ctx.lr = 0x831322AC;
	sub_83131800(ctx, base);
	// 831322AC: 4BF95835  bl 0x830c7ae0
	ctx.lr = 0x831322B0;
	sub_830C7AE0(ctx, base);
	// 831322B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831322B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831322B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831322BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831322C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831322C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831322C8 size=140
    let mut pc: u32 = 0x831322C8;
    'dispatch: loop {
        match pc {
            0x831322C8 => {
    //   block [0x831322C8..0x83132354)
	// 831322C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831322CC: 48075EA1  bl 0x831a816c
	ctx.lr = 0x831322D0;
	sub_831A8130(ctx, base);
	// 831322D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831322D4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831322D8: 3BCB7E0C  addi r30, r11, 0x7e0c
	ctx.r[30].s64 = ctx.r[11].s64 + 32268;
	// 831322DC: 817EFFF4  lwz r11, -0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-12 as u32) ) } as u64;
	// 831322E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831322E4: 419A0018  beq cr6, 0x831322fc
	if ctx.cr[6].eq {
	pc = 0x831322FC; continue 'dispatch;
	}
	// 831322E8: 397EFFF4  addi r11, r30, -0xc
	ctx.r[11].s64 = ctx.r[30].s64 + -12;
	// 831322EC: 807EFFF8  lwz r3, -8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831322F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831322F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831322F8: 4E800421  bctrl
	ctx.lr = 0x831322FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831322FC: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83132300: 3BABBC00  addi r29, r11, -0x4400
	ctx.r[29].s64 = ctx.r[11].s64 + -17408;
	// 83132304: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83132308: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313230C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132310: 409A000C  bne cr6, 0x8313231c
	if !ctx.cr[6].eq {
	pc = 0x8313231C; continue 'dispatch;
	}
	// 83132314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132318: 4BFFFF21  bl 0x83132238
	ctx.lr = 0x8313231C;
	sub_83132238(ctx, base);
	// 8313231C: 3BFF00B4  addi r31, r31, 0xb4
	ctx.r[31].s64 = ctx.r[31].s64 + 180;
	// 83132320: 397D1680  addi r11, r29, 0x1680
	ctx.r[11].s64 = ctx.r[29].s64 + 5760;
	// 83132324: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132328: 4198FFE0  blt cr6, 0x83132308
	if ctx.cr[6].lt {
	pc = 0x83132308; continue 'dispatch;
	}
	// 8313232C: 817EFFFC  lwz r11, -4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83132330: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132334: 419A0018  beq cr6, 0x8313234c
	if ctx.cr[6].eq {
	pc = 0x8313234C; continue 'dispatch;
	}
	// 83132338: 397EFFFC  addi r11, r30, -4
	ctx.r[11].s64 = ctx.r[30].s64 + -4;
	// 8313233C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132340: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132344: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132348: 4E800421  bctrl
	ctx.lr = 0x8313234C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8313234C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83132350: 48075E6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132358 size=76
    let mut pc: u32 = 0x83132358;
    'dispatch: loop {
        match pc {
            0x83132358 => {
    //   block [0x83132358..0x831323A4)
	// 83132358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132364: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 83132368: 816A7E10  lwz r11, 0x7e10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32272 as u32) ) } as u64;
	// 8313236C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83132370: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83132374: 916A7E10  stw r11, 0x7e10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32272 as u32), ctx.r[11].u32 ) };
	// 83132378: 409A0018  bne cr6, 0x83132390
	if !ctx.cr[6].eq {
	pc = 0x83132390; continue 'dispatch;
	}
	// 8313237C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83132380: 38A02080  li r5, 0x2080
	ctx.r[5].s64 = 8320;
	// 83132384: 386B9B80  addi r3, r11, -0x6480
	ctx.r[3].s64 = ctx.r[11].s64 + -25728;
	// 83132388: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313238C: 48075E55  bl 0x831a81e0
	ctx.lr = 0x83132390;
	sub_831A81E0(ctx, base);
	// 83132390: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83132394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83132398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313239C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831323A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831323A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831323A8 size=20
    let mut pc: u32 = 0x831323A8;
    'dispatch: loop {
        match pc {
            0x831323A8 => {
    //   block [0x831323A8..0x831323BC)
	// 831323A8: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831323AC: 816A7E10  lwz r11, 0x7e10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32272 as u32) ) } as u64;
	// 831323B0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831323B4: 916A7E10  stw r11, 0x7e10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32272 as u32), ctx.r[11].u32 ) };
	// 831323B8: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831323BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831323BC size=20
    let mut pc: u32 = 0x831323BC;
    'dispatch: loop {
        match pc {
            0x831323BC => {
    //   block [0x831323BC..0x831323D0)
	// 831323BC: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831323C0: 38A02080  li r5, 0x2080
	ctx.r[5].s64 = 8320;
	// 831323C4: 386B9B80  addi r3, r11, -0x6480
	ctx.r[3].s64 = ctx.r[11].s64 + -25728;
	// 831323C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831323CC: 48075E14  b 0x831a81e0
	sub_831A81E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831323D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831323D0 size=4
    let mut pc: u32 = 0x831323D0;
    'dispatch: loop {
        match pc {
            0x831323D0 => {
    //   block [0x831323D0..0x831323D4)
	// 831323D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831323D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831323D8 size=20
    let mut pc: u32 = 0x831323D8;
    'dispatch: loop {
        match pc {
            0x831323D8 => {
    //   block [0x831323D8..0x831323EC)
	// 831323D8: 8963004A  lbz r11, 0x4a(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(74 as u32) ) } as u64;
	// 831323DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831323E0: 419A000C  beq cr6, 0x831323ec
	if ctx.cr[6].eq {
		sub_831323EC(ctx, base);
		return;
	}
	// 831323E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831323E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831323EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831323EC size=20
    let mut pc: u32 = 0x831323EC;
    'dispatch: loop {
        match pc {
            0x831323EC => {
    //   block [0x831323EC..0x83132400)
	// 831323EC: 89630049  lbz r11, 0x49(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(73 as u32) ) } as u64;
	// 831323F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831323F4: 419A000C  beq cr6, 0x83132400
	if ctx.cr[6].eq {
		sub_83132400(ctx, base);
		return;
	}
	// 831323F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831323FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83132400 size=24
    let mut pc: u32 = 0x83132400;
    'dispatch: loop {
        match pc {
            0x83132400 => {
    //   block [0x83132400..0x83132418)
	// 83132400: 8963004D  lbz r11, 0x4d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(77 as u32) ) } as u64;
	// 83132404: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83132408: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8313240C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83132410: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83132414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83132418 size=28
    let mut pc: u32 = 0x83132418;
    'dispatch: loop {
        match pc {
            0x83132418 => {
    //   block [0x83132418..0x83132434)
	// 83132418: 89630049  lbz r11, 0x49(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(73 as u32) ) } as u64;
	// 8313241C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132420: 409A0014  bne cr6, 0x83132434
	if !ctx.cr[6].eq {
		sub_83132434(ctx, base);
		return;
	}
	// 83132424: 8963004D  lbz r11, 0x4d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(77 as u32) ) } as u64;
	// 83132428: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313242C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132430: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83132434 size=8
    let mut pc: u32 = 0x83132434;
    'dispatch: loop {
        match pc {
            0x83132434 => {
    //   block [0x83132434..0x8313243C)
	// 83132434: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83132438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132440 size=36
    let mut pc: u32 = 0x83132440;
    'dispatch: loop {
        match pc {
            0x83132440 => {
    //   block [0x83132440..0x83132464)
	// 83132440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313244C: 4BFF6FB5  bl 0x83129400
	ctx.lr = 0x83132450;
	sub_83129400(ctx, base);
	// 83132450: 4BFF7679  bl 0x83129ac8
	ctx.lr = 0x83132454;
	sub_83129AC8(ctx, base);
	// 83132454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83132458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8313245C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83132460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83132468 size=12
    let mut pc: u32 = 0x83132468;
    'dispatch: loop {
        match pc {
            0x83132468 => {
    //   block [0x83132468..0x83132474)
	// 83132468: 8963004D  lbz r11, 0x4d(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(77 as u32) ) } as u64;
	// 8313246C: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 83132470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83132478 size=240
    let mut pc: u32 = 0x83132478;
    'dispatch: loop {
        match pc {
            0x83132478 => {
    //   block [0x83132478..0x83132568)
	// 83132478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313247C: 48075CE1  bl 0x831a815c
	ctx.lr = 0x83132480;
	sub_831A8130(ctx, base);
	// 83132480: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132488: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8313248C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83132490: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83132494: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83132498: 4BF95649  bl 0x830c7ae0
	ctx.lr = 0x8313249C;
	sub_830C7AE0(ctx, base);
	// 8313249C: 7FCB5E70  srawi r11, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 831324A0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 831324A4: 7FCA07B4  extsw r10, r30
	ctx.r[10].s64 = ctx.r[30].s32 as i64;
	// 831324A8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 831324AC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 831324B0: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 831324B4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831324B8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831324BC: 556B5828  slwi r11, r11, 0xb
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831324C0: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 831324C4: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 831324C8: 9B3F0001  stb r25, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[25].u8 ) };
	// 831324CC: 7D6BF051  subf. r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831324D0: 9B5F0002  stb r26, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[26].u8 ) };
	// 831324D4: 41810008  bgt 0x831324dc
	if ctx.cr[0].gt {
	pc = 0x831324DC; continue 'dispatch;
	}
	// 831324D8: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 831324DC: 7FCB5E70  srawi r11, r30, 0xb
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 11) as i64;
	// 831324E0: 935F005C  stw r26, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 831324E4: 39200200  li r9, 0x200
	ctx.r[9].s64 = 512;
	// 831324E8: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 831324EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831324F0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831324F4: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 831324F8: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 831324FC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 83132500: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83132504: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83132508: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8313250C: 419A0048  beq cr6, 0x83132554
	if ctx.cr[6].eq {
	pc = 0x83132554; continue 'dispatch;
	}
	// 83132510: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132514: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83132518: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313251C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83132520: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132524: 4E800421  bctrl
	ctx.lr = 0x83132528;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132528: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313252C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83132530: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132534: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132538: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8313253C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132540: 4E800421  bctrl
	ctx.lr = 0x83132544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132544: 7D7E1A14  add r11, r30, r3
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 83132548: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8313254C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83132550: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83132554: 9B5F0048  stb r26, 0x48(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u8 ) };
	// 83132558: 9B3F0000  stb r25, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[25].u8 ) };
	// 8313255C: 4BF95585  bl 0x830c7ae0
	ctx.lr = 0x83132560;
	sub_830C7AE0(ctx, base);
	// 83132560: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83132564: 48075C48  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132568 size=284
    let mut pc: u32 = 0x83132568;
    'dispatch: loop {
        match pc {
            0x83132568 => {
    //   block [0x83132568..0x83132684)
	// 83132568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313256C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132570: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83132574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132578: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8313257C: 2F040100  cmpwi cr6, r4, 0x100
	ctx.cr[6].compare_i32(ctx.r[4].s32, 256, &mut ctx.xer);
	// 83132580: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83132584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83132588: 40980070  bge cr6, 0x831325f8
	if !ctx.cr[6].lt {
	pc = 0x831325F8; continue 'dispatch;
	}
	// 8313258C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83132590: 810B5A60  lwz r8, 0x5a60(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23136 as u32) ) } as u64;
	// 83132594: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83132598: 4099003C  ble cr6, 0x831325d4
	if !ctx.cr[6].gt {
	pc = 0x831325D4; continue 'dispatch;
	}
	// 8313259C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831325A0: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831325A4: 396B9B80  addi r11, r11, -0x6480
	ctx.r[11].s64 = ctx.r[11].s64 + -25728;
	// 831325A8: 814A7E14  lwz r10, 0x7e14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32276 as u32) ) } as u64;
	// 831325AC: 1D4A0068  mulli r10, r10, 0x68
	ctx.r[10].s64 = ctx.r[10].s64 * 104;
	// 831325B0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831325B4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831325B8: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 831325BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831325C0: 419A0014  beq cr6, 0x831325d4
	if ctx.cr[6].eq {
	pc = 0x831325D4; continue 'dispatch;
	}
	// 831325C4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 831325C8: 396B0068  addi r11, r11, 0x68
	ctx.r[11].s64 = ctx.r[11].s64 + 104;
	// 831325CC: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831325D0: 4198FFE4  blt cr6, 0x831325b4
	if ctx.cr[6].lt {
	pc = 0x831325B4; continue 'dispatch;
	}
	// 831325D4: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 831325D8: 419A0070  beq cr6, 0x83132648
	if ctx.cr[6].eq {
	pc = 0x83132648; continue 'dispatch;
	}
	// 831325DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831325E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831325E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831325E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831325EC: 4BFFFE8D  bl 0x83132478
	ctx.lr = 0x831325F0;
	sub_83132478(ctx, base);
	// 831325F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831325F4: 48000074  b 0x83132668
	pc = 0x83132668; continue 'dispatch;
	// 831325F8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 831325FC: 810B5A68  lwz r8, 0x5a68(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23144 as u32) ) } as u64;
	// 83132600: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83132604: 4099003C  ble cr6, 0x83132640
	if !ctx.cr[6].gt {
	pc = 0x83132640; continue 'dispatch;
	}
	// 83132608: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8313260C: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 83132610: 396B9B80  addi r11, r11, -0x6480
	ctx.r[11].s64 = ctx.r[11].s64 + -25728;
	// 83132614: 814A5A64  lwz r10, 0x5a64(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(23140 as u32) ) } as u64;
	// 83132618: 1D4A0068  mulli r10, r10, 0x68
	ctx.r[10].s64 = ctx.r[10].s64 * 104;
	// 8313261C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83132620: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132624: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83132628: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8313262C: 419A0014  beq cr6, 0x83132640
	if ctx.cr[6].eq {
	pc = 0x83132640; continue 'dispatch;
	}
	// 83132630: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83132634: 396B0068  addi r11, r11, 0x68
	ctx.r[11].s64 = ctx.r[11].s64 + 104;
	// 83132638: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8313263C: 4198FFE4  blt cr6, 0x83132620
	if ctx.cr[6].lt {
	pc = 0x83132620; continue 'dispatch;
	}
	// 83132640: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 83132644: 409A000C  bne cr6, 0x83132650
	if !ctx.cr[6].eq {
	pc = 0x83132650; continue 'dispatch;
	}
	// 83132648: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8313264C: 48000024  b 0x83132670
	pc = 0x83132670; continue 'dispatch;
	// 83132650: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83132654: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83132658: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313265C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132660: 4BFFFE19  bl 0x83132478
	ctx.lr = 0x83132664;
	sub_83132478(ctx, base);
	// 83132664: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83132668: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8313266C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83132674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83132678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8313267C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83132680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132688 size=60
    let mut pc: u32 = 0x83132688;
    'dispatch: loop {
        match pc {
            0x83132688 => {
    //   block [0x83132688..0x831326C4)
	// 83132688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8313268C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83132694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8313269C: 4BFF470D  bl 0x83126da8
	ctx.lr = 0x831326A0;
	sub_83126DA8(ctx, base);
	// 831326A0: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 831326A4: 7D7F0774  extsb r31, r11
	ctx.r[31].s64 = ctx.r[11].s8 as i64;
	// 831326A8: 4BFF4741  bl 0x83126de8
	ctx.lr = 0x831326AC;
	sub_83126DE8(ctx, base);
	// 831326AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831326B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831326B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831326B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831326BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831326C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831326C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831326C8 size=88
    let mut pc: u32 = 0x831326C8;
    'dispatch: loop {
        match pc {
            0x831326C8 => {
    //   block [0x831326C8..0x83132720)
	// 831326C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831326CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831326D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831326D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831326D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831326DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831326E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831326E4: 4BFF46C5  bl 0x83126da8
	ctx.lr = 0x831326E8;
	sub_83126DA8(ctx, base);
	// 831326E8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831326EC: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 831326F0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831326F4: 40990008  ble cr6, 0x831326fc
	if !ctx.cr[6].gt {
	pc = 0x831326FC; continue 'dispatch;
	}
	// 831326F8: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831326FC: 83FF005C  lwz r31, 0x5c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83132700: 4BFF46E9  bl 0x83126de8
	ctx.lr = 0x83132704;
	sub_83126DE8(ctx, base);
	// 83132704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132708: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313270C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83132710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83132714: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83132718: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313271C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132720 size=76
    let mut pc: u32 = 0x83132720;
    'dispatch: loop {
        match pc {
            0x83132720 => {
    //   block [0x83132720..0x8313276C)
	// 83132720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132728: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313272C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132734: 4BFF4675  bl 0x83126da8
	ctx.lr = 0x83132738;
	sub_83126DA8(ctx, base);
	// 83132738: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8313273C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132740: 419A000C  beq cr6, 0x8313274c
	if ctx.cr[6].eq {
	pc = 0x8313274C; continue 'dispatch;
	}
	// 83132744: 83FF005C  lwz r31, 0x5c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83132748: 48000008  b 0x83132750
	pc = 0x83132750; continue 'dispatch;
	// 8313274C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83132750: 4BFF4699  bl 0x83126de8
	ctx.lr = 0x83132754;
	sub_83126DE8(ctx, base);
	// 83132754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313275C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83132760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83132764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83132768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132770 size=48
    let mut pc: u32 = 0x83132770;
    'dispatch: loop {
        match pc {
            0x83132770 => {
    //   block [0x83132770..0x831327A0)
	// 83132770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132774: 480759F9  bl 0x831a816c
	ctx.lr = 0x83132778;
	sub_831A8130(ctx, base);
	// 83132778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313277C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132780: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83132784: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83132788: 4BFF4621  bl 0x83126da8
	ctx.lr = 0x8313278C;
	sub_83126DA8(ctx, base);
	// 8313278C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 83132790: 93BF0040  stw r29, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 83132794: 4BFF4655  bl 0x83126de8
	ctx.lr = 0x83132798;
	sub_83126DE8(ctx, base);
	// 83132798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8313279C: 48075A20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831327A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831327A0 size=84
    let mut pc: u32 = 0x831327A0;
    'dispatch: loop {
        match pc {
            0x831327A0 => {
    //   block [0x831327A0..0x831327F4)
	// 831327A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831327A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831327A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831327AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831327B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831327B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831327B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831327BC: 4BFF45ED  bl 0x83126da8
	ctx.lr = 0x831327C0;
	sub_83126DA8(ctx, base);
	// 831327C0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831327C4: 4198000C  blt cr6, 0x831327d0
	if ctx.cr[6].lt {
	pc = 0x831327D0; continue 'dispatch;
	}
	// 831327C8: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 831327CC: 4800000C  b 0x831327d8
	pc = 0x831327D8; continue 'dispatch;
	// 831327D0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831327D4: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 831327D8: 4BFF4611  bl 0x83126de8
	ctx.lr = 0x831327DC;
	sub_83126DE8(ctx, base);
	// 831327DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831327E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831327E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831327E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831327EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831327F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831327F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831327F8 size=940
    let mut pc: u32 = 0x831327F8;
    'dispatch: loop {
        match pc {
            0x831327F8 => {
    //   block [0x831327F8..0x83132BA4)
	// 831327F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831327FC: 48075959  bl 0x831a8154
	ctx.lr = 0x83132800;
	sub_831A8130(ctx, base);
	// 83132800: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132808: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313280C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132810: 4BFF7331  bl 0x83129b40
	ctx.lr = 0x83132814;
	sub_83129B40(ctx, base);
	// 83132814: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83132818: 4BF952C9  bl 0x830c7ae0
	ctx.lr = 0x8313281C;
	sub_830C7AE0(ctx, base);
	// 8313281C: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83132820: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 83132824: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83132828: 3B400003  li r26, 3
	ctx.r[26].s64 = 3;
	// 8313282C: 6159FFFF  ori r25, r10, 0xffff
	ctx.r[25].u64 = ctx.r[10].u64 | 65535;
	// 83132830: 3AE00004  li r23, 4
	ctx.r[23].s64 = 4;
	// 83132834: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132838: 3F008339  lis r24, -0x7cc7
	ctx.r[24].s64 = -2093416448;
	// 8313283C: 409A0154  bne cr6, 0x83132990
	if !ctx.cr[6].eq {
	pc = 0x83132990; continue 'dispatch;
	}
	// 83132840: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83132844: 409A00E0  bne cr6, 0x83132924
	if !ctx.cr[6].eq {
	pc = 0x83132924; continue 'dispatch;
	}
	// 83132848: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 8313284C: 4BF95295  bl 0x830c7ae0
	ctx.lr = 0x83132850;
	sub_830C7AE0(ctx, base);
	// 83132850: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83132854: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83132858: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8313285C: 557E5828  slwi r30, r11, 0xb
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83132860: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83132864: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83132868: 48002929  bl 0x83135190
	ctx.lr = 0x8313286C;
	sub_83135190(ctx, base);
	// 8313286C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132870: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83132874: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83132878: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8313287C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83132880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132884: 4E800421  bctrl
	ctx.lr = 0x83132888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132888: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313288C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83132890: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132894: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132898: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8313289C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831328A0: 4E800421  bctrl
	ctx.lr = 0x831328A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831328A4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 831328A8: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 831328AC: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 831328B0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 831328B4: 813F0034  lwz r9, 0x34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831328B8: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 831328BC: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831328C0: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 831328C4: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 831328C8: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 831328CC: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831328D0: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 831328D4: 409A001C  bne cr6, 0x831328f0
	if !ctx.cr[6].eq {
	pc = 0x831328F0; continue 'dispatch;
	}
	// 831328D8: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831328DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831328E0: 41820010  beq 0x831328f0
	if ctx.cr[0].eq {
	pc = 0x831328F0; continue 'dispatch;
	}
	// 831328E4: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 831328E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831328EC: 4E800421  bctrl
	ctx.lr = 0x831328F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831328F0: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 831328F4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831328F8: 40980020  bge cr6, 0x83132918
	if !ctx.cr[6].lt {
	pc = 0x83132918; continue 'dispatch;
	}
	// 831328FC: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83132900: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83132904: 554AAAFE  srwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83132908: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8313290C: 41980010  blt cr6, 0x8313291c
	if ctx.cr[6].lt {
	pc = 0x8313291C; continue 'dispatch;
	}
	// 83132910: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 83132914: 40980008  bge cr6, 0x8313291c
	if !ctx.cr[6].lt {
	pc = 0x8313291C; continue 'dispatch;
	}
	// 83132918: 9B5F0001  stb r26, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 8313291C: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 83132920: 48000074  b 0x83132994
	pc = 0x83132994; continue 'dispatch;
	// 83132924: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 83132928: 409A0068  bne cr6, 0x83132990
	if !ctx.cr[6].eq {
	pc = 0x83132990; continue 'dispatch;
	}
	// 8313292C: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 83132930: 4BF951B1  bl 0x830c7ae0
	ctx.lr = 0x83132934;
	sub_830C7AE0(ctx, base);
	// 83132934: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132938: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 8313293C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132940: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132944: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83132948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8313294C: 4E800421  bctrl
	ctx.lr = 0x83132950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132950: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 83132954: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 83132958: 81787E20  lwz r11, 0x7e20(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32288 as u32) ) } as u64;
	// 8313295C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83132960: 41980018  blt cr6, 0x83132978
	if ctx.cr[6].lt {
	pc = 0x83132978; continue 'dispatch;
	}
	// 83132964: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83132968: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8313296C: 4198000C  blt cr6, 0x83132978
	if ctx.cr[6].lt {
	pc = 0x83132978; continue 'dispatch;
	}
	// 83132970: 9AFF0001  stb r23, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[23].u8 ) };
	// 83132974: 48000020  b 0x83132994
	pc = 0x83132994; continue 'dispatch;
	// 83132978: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8313297C: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 83132980: 40980014  bge cr6, 0x83132994
	if !ctx.cr[6].lt {
	pc = 0x83132994; continue 'dispatch;
	}
	// 83132984: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83132988: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8313298C: 48000008  b 0x83132994
	pc = 0x83132994; continue 'dispatch;
	// 83132990: 4BF95151  bl 0x830c7ae0
	ctx.lr = 0x83132994;
	sub_830C7AE0(ctx, base);
	// 83132994: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83132998: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 8313299C: 419A0200  beq cr6, 0x83132b9c
	if ctx.cr[6].eq {
	pc = 0x83132B9C; continue 'dispatch;
	}
	// 831329A0: 4BF95141  bl 0x830c7ae0
	ctx.lr = 0x831329A4;
	sub_830C7AE0(ctx, base);
	// 831329A4: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 831329A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831329AC: 409A01EC  bne cr6, 0x83132b98
	if !ctx.cr[6].eq {
	pc = 0x83132B98; continue 'dispatch;
	}
	// 831329B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831329B4: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 831329B8: 3B7F0028  addi r27, r31, 0x28
	ctx.r[27].s64 = ctx.r[31].s64 + 40;
	// 831329BC: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 831329C0: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 831329C4: 4BF9511D  bl 0x830c7ae0
	ctx.lr = 0x831329C8;
	sub_830C7AE0(ctx, base);
	// 831329C8: 897F0048  lbz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831329CC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 831329D0: 419A01C0  beq cr6, 0x83132b90
	if ctx.cr[6].eq {
	pc = 0x83132B90; continue 'dispatch;
	}
	// 831329D4: 897F004C  lbz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831329D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 831329DC: 419A01B4  beq cr6, 0x83132b90
	if ctx.cr[6].eq {
	pc = 0x83132B90; continue 'dispatch;
	}
	// 831329E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831329E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831329E8: 409A0014  bne cr6, 0x831329fc
	if !ctx.cr[6].eq {
	pc = 0x831329FC; continue 'dispatch;
	}
	// 831329EC: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 831329F0: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 831329F4: 9B5F0001  stb r26, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[26].u8 ) };
	// 831329F8: 480001A4  b 0x83132b9c
	pc = 0x83132B9C; continue 'dispatch;
	// 831329FC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83132A00: 419A0178  beq cr6, 0x83132b78
	if ctx.cr[6].eq {
	pc = 0x83132B78; continue 'dispatch;
	}
	// 83132A04: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132A08: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83132A0C: 4182016C  beq 0x83132b78
	if ctx.cr[0].eq {
	pc = 0x83132B78; continue 'dispatch;
	}
	// 83132A10: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83132A14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132A18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132A1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132A20: 4E800421  bctrl
	ctx.lr = 0x83132A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132A24: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83132A28: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83132A2C: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 83132A30: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83132A34: 4098015C  bge cr6, 0x83132b90
	if !ctx.cr[6].lt {
	pc = 0x83132B90; continue 'dispatch;
	}
	// 83132A38: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132A3C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83132A40: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83132A44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132A48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132A4C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83132A50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132A54: 4E800421  bctrl
	ctx.lr = 0x83132A58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132A58: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83132A5C: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 83132A60: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83132A64: 7D295E70  srawi r9, r9, 0xb
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 11) as i64;
	// 83132A68: 7FC90194  addze r30, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[30].s64 = tmp.s64;
	// 83132A6C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83132A70: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132A74: 41980008  blt cr6, 0x83132a7c
	if ctx.cr[6].lt {
	pc = 0x83132A7C; continue 'dispatch;
	}
	// 83132A78: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83132A7C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83132A80: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83132A84: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132A88: 41980008  blt cr6, 0x83132a90
	if ctx.cr[6].lt {
	pc = 0x83132A90; continue 'dispatch;
	}
	// 83132A8C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83132A90: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83132A94: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132A98: 41980008  blt cr6, 0x83132aa0
	if ctx.cr[6].lt {
	pc = 0x83132AA0; continue 'dispatch;
	}
	// 83132A9C: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83132AA0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83132AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83132AA8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132AAC: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83132AB0: 4BFF6E31  bl 0x831298e0
	ctx.lr = 0x83132AB4;
	sub_831298E0(ctx, base);
	// 83132AB4: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83132AB8: 7F0BC840  cmplw cr6, r11, r25
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[25].u32, &mut ctx.xer);
	// 83132ABC: 419A0020  beq cr6, 0x83132adc
	if ctx.cr[6].eq {
	pc = 0x83132ADC; continue 'dispatch;
	}
	// 83132AC0: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83132AC4: 7D4A5E70  srawi r10, r10, 0xb
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 11) as i64;
	// 83132AC8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83132ACC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83132AD0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132AD4: 41980008  blt cr6, 0x83132adc
	if ctx.cr[6].lt {
	pc = 0x83132ADC; continue 'dispatch;
	}
	// 83132AD8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83132ADC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83132AE0: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83132AE4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132AE8: 4BFF6EB9  bl 0x831299a0
	ctx.lr = 0x83132AEC;
	sub_831299A0(ctx, base);
	// 83132AEC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83132AF0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83132AF4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83132AF8: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83132AFC: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83132B00: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 83132B04: 41810098  bgt 0x83132b9c
	if ctx.cr[0].gt {
	pc = 0x83132B9C; continue 'dispatch;
	}
	// 83132B08: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83132B0C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83132B10: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132B14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83132B18: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83132B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83132B20: 4E800421  bctrl
	ctx.lr = 0x83132B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83132B24: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132B28: 939B0000  stw r28, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83132B2C: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 83132B30: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 83132B34: 4BFF700D  bl 0x83129b40
	ctx.lr = 0x83132B38;
	sub_83129B40(ctx, base);
	// 83132B38: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83132B3C: 409A0060  bne cr6, 0x83132b9c
	if !ctx.cr[6].eq {
	pc = 0x83132B9C; continue 'dispatch;
	}
	// 83132B40: 81787E20  lwz r11, 0x7e20(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32288 as u32) ) } as u64;
	// 83132B44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83132B48: 41980018  blt cr6, 0x83132b60
	if ctx.cr[6].lt {
	pc = 0x83132B60; continue 'dispatch;
	}
	// 83132B4C: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83132B50: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83132B54: 4198000C  blt cr6, 0x83132b60
	if ctx.cr[6].lt {
	pc = 0x83132B60; continue 'dispatch;
	}
	// 83132B58: 9AFF0001  stb r23, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[23].u8 ) };
	// 83132B5C: 48000040  b 0x83132b9c
	pc = 0x83132B9C; continue 'dispatch;
	// 83132B60: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83132B64: 7F0BC800  cmpw cr6, r11, r25
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[25].s32, &mut ctx.xer);
	// 83132B68: 40980034  bge cr6, 0x83132b9c
	if !ctx.cr[6].lt {
	pc = 0x83132B9C; continue 'dispatch;
	}
	// 83132B6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83132B70: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83132B74: 48000028  b 0x83132b9c
	pc = 0x83132B9C; continue 'dispatch;
	// 83132B78: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83132B7C: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 83132B80: 814B7E24  lwz r10, 0x7e24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32292 as u32) ) } as u64;
	// 83132B84: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83132B88: 914B7E24  stw r10, 0x7e24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32292 as u32), ctx.r[10].u32 ) };
	// 83132B8C: 48000010  b 0x83132b9c
	pc = 0x83132B9C; continue 'dispatch;
	// 83132B90: 9B9F0002  stb r28, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[28].u8 ) };
	// 83132B94: 48000008  b 0x83132b9c
	pc = 0x83132B9C; continue 'dispatch;
	// 83132B98: 4BF94F49  bl 0x830c7ae0
	ctx.lr = 0x83132B9C;
	sub_830C7AE0(ctx, base);
	// 83132B9C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83132BA0: 48075604  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132BA8 size=660
    let mut pc: u32 = 0x83132BA8;
    'dispatch: loop {
        match pc {
            0x83132BA8 => {
    //   block [0x83132BA8..0x83132E3C)
	// 83132BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132BAC: 480755BD  bl 0x831a8168
	ctx.lr = 0x83132BB0;
	sub_831A8130(ctx, base);
	// 83132BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132BB8: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83132BBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132BC0: 409A0248  bne cr6, 0x83132e08
	if !ctx.cr[6].eq {
	pc = 0x83132E08; continue 'dispatch;
	}
	// 83132BC4: 897F004C  lbz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83132BC8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83132BCC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83132BD0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132BD4: 409A0018  bne cr6, 0x83132bec
	if !ctx.cr[6].eq {
	pc = 0x83132BEC; continue 'dispatch;
	}
	// 83132BD8: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83132BDC: 9B9F004C  stb r28, 0x4c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[28].u8 ) };
	// 83132BE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132BE4: 409A0008  bne cr6, 0x83132bec
	if !ctx.cr[6].eq {
	pc = 0x83132BEC; continue 'dispatch;
	}
	// 83132BE8: 9BBF0001  stb r29, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[29].u8 ) };
	// 83132BEC: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83132BF0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132BF4: 409A0028  bne cr6, 0x83132c1c
	if !ctx.cr[6].eq {
	pc = 0x83132C1C; continue 'dispatch;
	}
	// 83132BF8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132BFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83132C00: 4182000C  beq 0x83132c0c
	if ctx.cr[0].eq {
	pc = 0x83132C0C; continue 'dispatch;
	}
	// 83132C04: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83132C08: 4BFF6B69  bl 0x83129770
	ctx.lr = 0x83132C0C;
	sub_83129770(ctx, base);
	// 83132C0C: 4BF94ED5  bl 0x830c7ae0
	ctx.lr = 0x83132C10;
	sub_830C7AE0(ctx, base);
	// 83132C10: 9B9F004A  stb r28, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[28].u8 ) };
	// 83132C14: 9B9F004D  stb r28, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[28].u8 ) };
	// 83132C18: 4BF94EC9  bl 0x830c7ae0
	ctx.lr = 0x83132C1C;
	sub_830C7AE0(ctx, base);
	// 83132C1C: 4BF94EC5  bl 0x830c7ae0
	ctx.lr = 0x83132C20;
	sub_830C7AE0(ctx, base);
	// 83132C20: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 83132C24: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83132C28: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132C2C: 409A01C8  bne cr6, 0x83132df4
	if !ctx.cr[6].eq {
	pc = 0x83132DF4; continue 'dispatch;
	}
	// 83132C30: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83132C34: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132C38: 419A01FC  beq cr6, 0x83132e34
	if ctx.cr[6].eq {
	pc = 0x83132E34; continue 'dispatch;
	}
	// 83132C3C: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 83132C40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132C44: 409A005C  bne cr6, 0x83132ca0
	if !ctx.cr[6].eq {
	pc = 0x83132CA0; continue 'dispatch;
	}
	// 83132C48: 9BBF004D  stb r29, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[29].u8 ) };
	// 83132C4C: 4BF94E95  bl 0x830c7ae0
	ctx.lr = 0x83132C50;
	sub_830C7AE0(ctx, base);
	// 83132C50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132C54: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 83132C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132C5C: 409A0044  bne cr6, 0x83132ca0
	if !ctx.cr[6].eq {
	pc = 0x83132CA0; continue 'dispatch;
	}
	// 83132C60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83132C64: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83132C68: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83132C6C: 4BFF75AD  bl 0x8312a218
	ctx.lr = 0x83132C70;
	sub_8312A218(ctx, base);
	// 83132C70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83132C74: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83132C78: 40820028  bne 0x83132ca0
	if !ctx.cr[0].eq {
	pc = 0x83132CA0; continue 'dispatch;
	}
	// 83132C7C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83132C80: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83132C84: 386B1668  addi r3, r11, 0x1668
	ctx.r[3].s64 = ctx.r[11].s64 + 5736;
	// 83132C88: 4BFFDDD9  bl 0x83130a60
	ctx.lr = 0x83132C8C;
	sub_83130A60(ctx, base);
	// 83132C8C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83132C90: 9B9F004D  stb r28, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[28].u8 ) };
	// 83132C94: 9B9F0049  stb r28, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[28].u8 ) };
	// 83132C98: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83132C9C: 48000198  b 0x83132e34
	pc = 0x83132E34; continue 'dispatch;
	// 83132CA0: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 83132CA4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132CA8: 409A0150  bne cr6, 0x83132df8
	if !ctx.cr[6].eq {
	pc = 0x83132DF8; continue 'dispatch;
	}
	// 83132CAC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83132CB0: 409A0008  bne cr6, 0x83132cb8
	if !ctx.cr[6].eq {
	pc = 0x83132CB8; continue 'dispatch;
	}
	// 83132CB4: 4BF94E2D  bl 0x830c7ae0
	ctx.lr = 0x83132CB8;
	sub_830C7AE0(ctx, base);
	// 83132CB8: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132CBC: 4BFF6F45  bl 0x83129c00
	ctx.lr = 0x83132CC0;
	sub_83129C00(ctx, base);
	// 83132CC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83132CC4: 418200F8  beq 0x83132dbc
	if ctx.cr[0].eq {
	pc = 0x83132DBC; continue 'dispatch;
	}
	// 83132CC8: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 83132CCC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132CD0: 409A0010  bne cr6, 0x83132ce0
	if !ctx.cr[6].eq {
	pc = 0x83132CE0; continue 'dispatch;
	}
	// 83132CD4: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83132CD8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132CDC: 419A0158  beq cr6, 0x83132e34
	if ctx.cr[6].eq {
	pc = 0x83132E34; continue 'dispatch;
	}
	// 83132CE0: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83132CE4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132CE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132CEC: 409A003C  bne cr6, 0x83132d28
	if !ctx.cr[6].eq {
	pc = 0x83132D28; continue 'dispatch;
	}
	// 83132CF0: 4BFF70A9  bl 0x83129d98
	ctx.lr = 0x83132CF4;
	sub_83129D98(ctx, base);
	// 83132CF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83132CF8: 2F3D0000  cmpdi cr6, r29, 0
	ctx.cr[6].compare_i64(ctx.r[29].s64, 0, &mut ctx.xer);
	// 83132CFC: 40980010  bge cr6, 0x83132d0c
	if !ctx.cr[6].lt {
	pc = 0x83132D0C; continue 'dispatch;
	}
	// 83132D00: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83132D04: 4BFF76AD  bl 0x8312a3b0
	ctx.lr = 0x83132D08;
	sub_8312A3B0(ctx, base);
	// 83132D08: 7C7D07B4  extsw r29, r3
	ctx.r[29].s64 = ctx.r[3].s32 as i64;
	// 83132D0C: 397D07FF  addi r11, r29, 0x7ff
	ctx.r[11].s64 = ctx.r[29].s64 + 2047;
	// 83132D10: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 83132D14: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 83132D18: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83132D1C: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 83132D20: 7D7E07B4  extsw r30, r11
	ctx.r[30].s64 = ctx.r[11].s32 as i64;
	// 83132D24: 48000034  b 0x83132d58
	pc = 0x83132D58; continue 'dispatch;
	// 83132D28: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83132D2C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132D30: 4BFF6BB1  bl 0x831298e0
	ctx.lr = 0x83132D34;
	sub_831298E0(ctx, base);
	// 83132D34: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132D38: 4BFF6AE9  bl 0x83129820
	ctx.lr = 0x83132D3C;
	sub_83129820(ctx, base);
	// 83132D3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83132D40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83132D44: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132D48: 57CB5828  slwi r11, r30, 0xb
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(11);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83132D4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132D50: 7D7D07B4  extsw r29, r11
	ctx.r[29].s64 = ctx.r[11].s32 as i64;
	// 83132D54: 4BFF6B8D  bl 0x831298e0
	ctx.lr = 0x83132D58;
	sub_831298E0(ctx, base);
	// 83132D58: 3940F800  li r10, -0x800
	ctx.r[10].s64 = -2048;
	// 83132D5C: E97F0010  ld r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 83132D60: 794A0580  clrldi r10, r10, 0x16
	ctx.r[10].u64 = ctx.r[10].u64 & 0x000003FFFFFFFFFFu64;
	// 83132D64: 7F2B5000  cmpd cr6, r11, r10
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[10].s64, &mut ctx.xer);
	// 83132D68: 409A000C  bne cr6, 0x83132d74
	if !ctx.cr[6].eq {
	pc = 0x83132D74; continue 'dispatch;
	}
	// 83132D6C: FBBF0010  std r29, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u64 ) };
	// 83132D70: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83132D74: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83132D78: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83132D7C: 40990008  ble cr6, 0x83132d84
	if !ctx.cr[6].gt {
	pc = 0x83132D84; continue 'dispatch;
	}
	// 83132D80: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83132D84: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83132D88: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83132D8C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83132D90: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83132D94: 40990018  ble cr6, 0x83132dac
	if !ctx.cr[6].gt {
	pc = 0x83132DAC; continue 'dispatch;
	}
	// 83132D98: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 83132D9C: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 83132DA0: 794A5D24  sldi r10, r10, 0xb
	ctx.r[10].u64 = ctx.r[10].u64.wrapping_shl(11);
	ctx.r[10].u32 = ctx.r[10].u64 as u32;
	// 83132DA4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83132DA8: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 83132DAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83132DB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132DB4: 4BFFF915  bl 0x831326c8
	ctx.lr = 0x83132DB8;
	sub_831326C8(ctx, base);
	// 83132DB8: 9B9F0049  stb r28, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[28].u8 ) };
	// 83132DBC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132DC0: 4BFF6D81  bl 0x83129b40
	ctx.lr = 0x83132DC4;
	sub_83129B40(ctx, base);
	// 83132DC4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83132DC8: 409A0030  bne cr6, 0x83132df8
	if !ctx.cr[6].eq {
	pc = 0x83132DF8; continue 'dispatch;
	}
	// 83132DCC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83132DD0: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83132DD4: 386B163C  addi r3, r11, 0x163c
	ctx.r[3].s64 = ctx.r[11].s64 + 5692;
	// 83132DD8: 4BFFDC89  bl 0x83130a60
	ctx.lr = 0x83132DDC;
	sub_83130A60(ctx, base);
	// 83132DDC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83132DE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83132DE4: 4182FEA8  beq 0x83132c8c
	if ctx.cr[0].eq {
	pc = 0x83132C8C; continue 'dispatch;
	}
	// 83132DE8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 83132DEC: 4BFF6985  bl 0x83129770
	ctx.lr = 0x83132DF0;
	sub_83129770(ctx, base);
	// 83132DF0: 4BFFFE9C  b 0x83132c8c
	pc = 0x83132C8C; continue 'dispatch;
	// 83132DF4: 4BF94CED  bl 0x830c7ae0
	ctx.lr = 0x83132DF8;
	sub_830C7AE0(ctx, base);
	// 83132DF8: 897F004B  lbz r11, 0x4b(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83132DFC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132E00: 409A0008  bne cr6, 0x83132e08
	if !ctx.cr[6].eq {
	pc = 0x83132E08; continue 'dispatch;
	}
	// 83132E04: 9B9F004B  stb r28, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[28].u8 ) };
	// 83132E08: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83132E0C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 83132E10: 409A0024  bne cr6, 0x83132e34
	if !ctx.cr[6].eq {
	pc = 0x83132E34; continue 'dispatch;
	}
	// 83132E14: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 83132E18: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83132E1C: 409A0018  bne cr6, 0x83132e34
	if !ctx.cr[6].eq {
	pc = 0x83132E34; continue 'dispatch;
	}
	// 83132E20: 897F0049  lbz r11, 0x49(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(73 as u32) ) } as u64;
	// 83132E24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83132E28: 409A000C  bne cr6, 0x83132e34
	if !ctx.cr[6].eq {
	pc = 0x83132E34; continue 'dispatch;
	}
	// 83132E2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132E30: 4BFFF9C9  bl 0x831327f8
	ctx.lr = 0x83132E34;
	sub_831327F8(ctx, base);
	// 83132E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83132E38: 48075380  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132E40 size=40
    let mut pc: u32 = 0x83132E40;
    'dispatch: loop {
        match pc {
            0x83132E40 => {
    //   block [0x83132E40..0x83132E68)
	// 83132E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132E4C: 4BFF3F5D  bl 0x83126da8
	ctx.lr = 0x83132E50;
	sub_83126DA8(ctx, base);
	// 83132E50: 4BFF6C11  bl 0x83129a60
	ctx.lr = 0x83132E54;
	sub_83129A60(ctx, base);
	// 83132E54: 4BFF3F95  bl 0x83126de8
	ctx.lr = 0x83132E58;
	sub_83126DE8(ctx, base);
	// 83132E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83132E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83132E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83132E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132E68 size=52
    let mut pc: u32 = 0x83132E68;
    'dispatch: loop {
        match pc {
            0x83132E68 => {
    //   block [0x83132E68..0x83132E9C)
	// 83132E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132E6C: 48075301  bl 0x831a816c
	ctx.lr = 0x83132E70;
	sub_831A8130(ctx, base);
	// 83132E70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132E74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132E78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83132E7C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83132E80: 4BFF3F29  bl 0x83126da8
	ctx.lr = 0x83132E84;
	sub_83126DA8(ctx, base);
	// 83132E84: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 83132E88: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 83132E8C: 4BFF3F5D  bl 0x83126de8
	ctx.lr = 0x83132E90;
	sub_83126DE8(ctx, base);
	// 83132E90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83132E94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83132E98: 48075324  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132EA0 size=56
    let mut pc: u32 = 0x83132EA0;
    'dispatch: loop {
        match pc {
            0x83132EA0 => {
    //   block [0x83132EA0..0x83132ED8)
	// 83132EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132EA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83132EAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132EB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132EB4: 4BFF3EF5  bl 0x83126da8
	ctx.lr = 0x83132EB8;
	sub_83126DA8(ctx, base);
	// 83132EB8: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83132EBC: 4BFF3F2D  bl 0x83126de8
	ctx.lr = 0x83132EC0;
	sub_83126DE8(ctx, base);
	// 83132EC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132EC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83132EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83132ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83132ED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83132ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132ED8 size=80
    let mut pc: u32 = 0x83132ED8;
    'dispatch: loop {
        match pc {
            0x83132ED8 => {
    //   block [0x83132ED8..0x83132F28)
	// 83132ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83132EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83132EE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132EEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132EF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83132EF4: 4BFF3EB5  bl 0x83126da8
	ctx.lr = 0x83132EF8;
	sub_83126DA8(ctx, base);
	// 83132EF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83132EFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132F00: 4BFFF669  bl 0x83132568
	ctx.lr = 0x83132F04;
	sub_83132568(ctx, base);
	// 83132F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132F08: 4BFF3EE1  bl 0x83126de8
	ctx.lr = 0x83132F0C;
	sub_83126DE8(ctx, base);
	// 83132F0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83132F10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83132F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83132F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83132F1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83132F20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83132F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132F28 size=104
    let mut pc: u32 = 0x83132F28;
    'dispatch: loop {
        match pc {
            0x83132F28 => {
    //   block [0x83132F28..0x83132F90)
	// 83132F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132F2C: 48075239  bl 0x831a8164
	ctx.lr = 0x83132F30;
	sub_831A8130(ctx, base);
	// 83132F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132F34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132F38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83132F3C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83132F40: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83132F44: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83132F48: 4BFF3E61  bl 0x83126da8
	ctx.lr = 0x83132F4C;
	sub_83126DA8(ctx, base);
	// 83132F4C: 4BF94B95  bl 0x830c7ae0
	ctx.lr = 0x83132F50;
	sub_830C7AE0(ctx, base);
	// 83132F50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83132F54: 397E07FF  addi r11, r30, 0x7ff
	ctx.r[11].s64 = ctx.r[30].s64 + 2047;
	// 83132F58: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 83132F5C: FBDF0010  std r30, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u64 ) };
	// 83132F60: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83132F64: 939F0058  stw r28, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[28].u32 ) };
	// 83132F68: 995F0049  stb r10, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[10].u8 ) };
	// 83132F6C: 7D6A5674  sradi r10, r11, 0xa
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 10) - 1)) != 0);
	ctx.r[10].s64 = ctx.r[11].s64 >> 10;
	// 83132F70: 794A5D60  rldicl r10, r10, 0xb, 0x35
	ctx.r[10].u64 = ctx.r[10].u64 & 0x001FFFFFFFFFFFFFu64;
	// 83132F74: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83132F78: 7D6B5E74  sradi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s64 < 0) && ((ctx.r[11].u64 & ((1u64 << 11) - 1)) != 0);
	ctx.r[11].s64 = ctx.r[11].s64 >> 11;
	// 83132F7C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83132F80: 4BF94B61  bl 0x830c7ae0
	ctx.lr = 0x83132F84;
	sub_830C7AE0(ctx, base);
	// 83132F84: 4BFF3E65  bl 0x83126de8
	ctx.lr = 0x83132F88;
	sub_83126DE8(ctx, base);
	// 83132F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83132F8C: 48075228  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83132F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83132F90 size=128
    let mut pc: u32 = 0x83132F90;
    'dispatch: loop {
        match pc {
            0x83132F90 => {
    //   block [0x83132F90..0x83133010)
	// 83132F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83132F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83132F98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83132F9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83132FA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83132FA4: 4BFF3E05  bl 0x83126da8
	ctx.lr = 0x83132FA8;
	sub_83126DA8(ctx, base);
	// 83132FA8: 4BF94B39  bl 0x830c7ae0
	ctx.lr = 0x83132FAC;
	sub_830C7AE0(ctx, base);
	// 83132FAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83132FB0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83132FB4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83132FB8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83132FBC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83132FC0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83132FC4: 419A0008  beq cr6, 0x83132fcc
	if ctx.cr[6].eq {
	pc = 0x83132FCC; continue 'dispatch;
	}
	// 83132FC8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83132FCC: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 83132FD0: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 83132FD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83132FD8: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 83132FDC: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 83132FE0: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83132FE4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83132FE8: 995F004B  stb r10, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[10].u8 ) };
	// 83132FEC: 913F0060  stw r9, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 83132FF0: 4BF94AF1  bl 0x830c7ae0
	ctx.lr = 0x83132FF4;
	sub_830C7AE0(ctx, base);
	// 83132FF4: 4BFF3DF5  bl 0x83126de8
	ctx.lr = 0x83132FF8;
	sub_83126DE8(ctx, base);
	// 83132FF8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83132FFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83133000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313300C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133010 size=124
    let mut pc: u32 = 0x83133010;
    'dispatch: loop {
        match pc {
            0x83133010 => {
    //   block [0x83133010..0x8313308C)
	// 83133010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313301C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133024: 4BFF3D85  bl 0x83126da8
	ctx.lr = 0x83133028;
	sub_83126DA8(ctx, base);
	// 83133028: 4BF94AB9  bl 0x830c7ae0
	ctx.lr = 0x8313302C;
	sub_830C7AE0(ctx, base);
	// 8313302C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83133030: 4BFF6C61  bl 0x83129c90
	ctx.lr = 0x83133034;
	sub_83129C90(ctx, base);
	// 83133034: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83133038: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8313303C: 409A002C  bne cr6, 0x83133068
	if !ctx.cr[6].eq {
	pc = 0x83133068; continue 'dispatch;
	}
	// 83133040: 897F0002  lbz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 83133044: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83133048: 409A0020  bne cr6, 0x83133068
	if !ctx.cr[6].eq {
	pc = 0x83133068; continue 'dispatch;
	}
	// 8313304C: 895F004B  lbz r10, 0x4b(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(75 as u32) ) } as u64;
	// 83133050: 997F004C  stb r11, 0x4c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u8 ) };
	// 83133054: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 83133058: 409A0018  bne cr6, 0x83133070
	if !ctx.cr[6].eq {
	pc = 0x83133070; continue 'dispatch;
	}
	// 8313305C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133060: 997F004B  stb r11, 0x4b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(75 as u32), ctx.r[11].u8 ) };
	// 83133064: 4800000C  b 0x83133070
	pc = 0x83133070; continue 'dispatch;
	// 83133068: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8313306C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 83133070: 4BF94A71  bl 0x830c7ae0
	ctx.lr = 0x83133074;
	sub_830C7AE0(ctx, base);
	// 83133074: 4BFF3D75  bl 0x83126de8
	ctx.lr = 0x83133078;
	sub_83126DE8(ctx, base);
	// 83133078: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313307C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133084: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133090 size=108
    let mut pc: u32 = 0x83133090;
    'dispatch: loop {
        match pc {
            0x83133090 => {
    //   block [0x83133090..0x831330FC)
	// 83133090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133094: 480750D9  bl 0x831a816c
	ctx.lr = 0x83133098;
	sub_831A8130(ctx, base);
	// 83133098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8313309C: 4BFF3D0D  bl 0x83126da8
	ctx.lr = 0x831330A0;
	sub_83126DA8(ctx, base);
	// 831330A0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 831330A4: 3BAB7E18  addi r29, r11, 0x7e18
	ctx.r[29].s64 = ctx.r[11].s64 + 32280;
	// 831330A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831330AC: 4BFF3875  bl 0x83126920
	ctx.lr = 0x831330B0;
	sub_83126920(ctx, base);
	// 831330B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 831330B4: 4182003C  beq 0x831330f0
	if ctx.cr[0].eq {
	pc = 0x831330F0; continue 'dispatch;
	}
	// 831330B8: 3D608344  lis r11, -0x7cbc
	ctx.r[11].s64 = -2092695552;
	// 831330BC: 3BCB9B80  addi r30, r11, -0x6480
	ctx.r[30].s64 = ctx.r[11].s64 + -25728;
	// 831330C0: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 831330C4: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831330C8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 831330CC: 409A000C  bne cr6, 0x831330d8
	if !ctx.cr[6].eq {
	pc = 0x831330D8; continue 'dispatch;
	}
	// 831330D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831330D4: 4BFFFAD5  bl 0x83132ba8
	ctx.lr = 0x831330D8;
	sub_83132BA8(ctx, base);
	// 831330D8: 3BFF0068  addi r31, r31, 0x68
	ctx.r[31].s64 = ctx.r[31].s64 + 104;
	// 831330DC: 397E2080  addi r11, r30, 0x2080
	ctx.r[11].s64 = ctx.r[30].s64 + 8320;
	// 831330E0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831330E4: 4198FFE0  blt cr6, 0x831330c4
	if ctx.cr[6].lt {
	pc = 0x831330C4; continue 'dispatch;
	}
	// 831330E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831330EC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831330F0: 4BFF3CF9  bl 0x83126de8
	ctx.lr = 0x831330F4;
	sub_83126DE8(ctx, base);
	// 831330F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831330F8: 480750C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133100 size=92
    let mut pc: u32 = 0x83133100;
    'dispatch: loop {
        match pc {
            0x83133100 => {
    //   block [0x83133100..0x8313315C)
	// 83133100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133108: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313310C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133114: 4BFF3C95  bl 0x83126da8
	ctx.lr = 0x83133118;
	sub_83126DA8(ctx, base);
	// 83133118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313311C: 4BFFFEF5  bl 0x83133010
	ctx.lr = 0x83133120;
	sub_83133010(ctx, base);
	// 83133120: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 83133124: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83133128: 409A0010  bne cr6, 0x83133138
	if !ctx.cr[6].eq {
	pc = 0x83133138; continue 'dispatch;
	}
	// 8313312C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83133130: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83133134: 419A0010  beq cr6, 0x83133144
	if ctx.cr[6].eq {
	pc = 0x83133144; continue 'dispatch;
	}
	// 83133138: 4BFFDBD1  bl 0x83130d08
	ctx.lr = 0x8313313C;
	sub_83130D08(ctx, base);
	// 8313313C: 4BFFA9C5  bl 0x8312db00
	ctx.lr = 0x83133140;
	sub_8312DB00(ctx, base);
	// 83133140: 4BFFFFE0  b 0x83133120
	pc = 0x83133120; continue 'dispatch;
	// 83133144: 4BFF3CA5  bl 0x83126de8
	ctx.lr = 0x83133148;
	sub_83126DE8(ctx, base);
	// 83133148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313314C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133160 size=96
    let mut pc: u32 = 0x83133160;
    'dispatch: loop {
        match pc {
            0x83133160 => {
    //   block [0x83133160..0x831331C0)
	// 83133160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133168: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313316C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133170: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133174: 4BFF3C35  bl 0x83126da8
	ctx.lr = 0x83133178;
	sub_83126DA8(ctx, base);
	// 83133178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313317C: 4BFFFE95  bl 0x83133010
	ctx.lr = 0x83133180;
	sub_83133010(ctx, base);
	// 83133180: 4BF94961  bl 0x830c7ae0
	ctx.lr = 0x83133184;
	sub_830C7AE0(ctx, base);
	// 83133184: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 83133188: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8313318C: 409A0008  bne cr6, 0x83133194
	if !ctx.cr[6].eq {
	pc = 0x83133194; continue 'dispatch;
	}
	// 83133190: 997F004A  stb r11, 0x4a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(74 as u32), ctx.r[11].u8 ) };
	// 83133194: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313319C: 997F0049  stb r11, 0x49(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(73 as u32), ctx.r[11].u8 ) };
	// 831331A0: 4BFFFA09  bl 0x83132ba8
	ctx.lr = 0x831331A4;
	sub_83132BA8(ctx, base);
	// 831331A4: 4BF9493D  bl 0x830c7ae0
	ctx.lr = 0x831331A8;
	sub_830C7AE0(ctx, base);
	// 831331A8: 4BFF3C41  bl 0x83126de8
	ctx.lr = 0x831331AC;
	sub_83126DE8(ctx, base);
	// 831331AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831331B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831331B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831331B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831331BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831331C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831331C0 size=108
    let mut pc: u32 = 0x831331C0;
    'dispatch: loop {
        match pc {
            0x831331C0 => {
    //   block [0x831331C0..0x8313322C)
	// 831331C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831331C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831331C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831331CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831331D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831331D4: 4BFF3BD5  bl 0x83126da8
	ctx.lr = 0x831331D8;
	sub_83126DA8(ctx, base);
	// 831331D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831331DC: 4BFFFF25  bl 0x83133100
	ctx.lr = 0x831331E0;
	sub_83133100(ctx, base);
	// 831331E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831331E4: 4BFFFF7D  bl 0x83133160
	ctx.lr = 0x831331E8;
	sub_83133160(ctx, base);
	// 831331E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831331EC: 4BFFF9BD  bl 0x83132ba8
	ctx.lr = 0x831331F0;
	sub_83132BA8(ctx, base);
	// 831331F0: 897F004D  lbz r11, 0x4d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(77 as u32) ) } as u64;
	// 831331F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831331F8: 409A0010  bne cr6, 0x83133208
	if !ctx.cr[6].eq {
	pc = 0x83133208; continue 'dispatch;
	}
	// 831331FC: 897F004A  lbz r11, 0x4a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(74 as u32) ) } as u64;
	// 83133200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83133204: 419A0010  beq cr6, 0x83133214
	if ctx.cr[6].eq {
	pc = 0x83133214; continue 'dispatch;
	}
	// 83133208: 4BFFDB01  bl 0x83130d08
	ctx.lr = 0x8313320C;
	sub_83130D08(ctx, base);
	// 8313320C: 4BFFA8F5  bl 0x8312db00
	ctx.lr = 0x83133210;
	sub_8312DB00(ctx, base);
	// 83133210: 4BFFFFE0  b 0x831331f0
	pc = 0x831331F0; continue 'dispatch;
	// 83133214: 4BFF3BD5  bl 0x83126de8
	ctx.lr = 0x83133218;
	sub_83126DE8(ctx, base);
	// 83133218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8313321C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83133228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83133230 size=96
    let mut pc: u32 = 0x83133230;
    'dispatch: loop {
        match pc {
            0x83133230 => {
    //   block [0x83133230..0x83133290)
	// 83133230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83133234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83133238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8313323C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83133240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83133244: 4BFF3B65  bl 0x83126da8
	ctx.lr = 0x83133248;
	sub_83126DA8(ctx, base);
	// 83133248: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8313324C: 419A002C  beq cr6, 0x83133278
	if ctx.cr[6].eq {
	pc = 0x83133278; continue 'dispatch;
	}
	// 83133250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83133254: 4BFFFEAD  bl 0x83133100
	ctx.lr = 0x83133258;
	sub_83133100(ctx, base);
	// 83133258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8313325C: 4BFFFF65  bl 0x831331c0
	ctx.lr = 0x83133260;
	sub_831331C0(ctx, base);
	// 83133260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83133264: 38A00068  li r5, 0x68
	ctx.r[5].s64 = 104;
	// 83133268: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8313326C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83133270: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 83133274: 48074F6D  bl 0x831a81e0
	ctx.lr = 0x83133278;
	sub_831A81E0(ctx, base);
	// 83133278: 4BFF3B71  bl 0x83126de8
	ctx.lr = 0x8313327C;
	sub_83126DE8(ctx, base);
	// 8313327C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83133280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83133284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83133288: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8313328C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133290 size=12
    let mut pc: u32 = 0x83133290;
    'dispatch: loop {
        match pc {
            0x83133290 => {
    //   block [0x83133290..0x8313329C)
	// 83133290: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133294: 386B16A4  addi r3, r11, 0x16a4
	ctx.r[3].s64 = ctx.r[11].s64 + 5796;
	// 83133298: 48005658  b 0x831388f0
	sub_831388F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831332A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831332A0 size=80
    let mut pc: u32 = 0x831332A0;
    'dispatch: loop {
        match pc {
            0x831332A0 => {
    //   block [0x831332A0..0x831332F0)
	// 831332A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831332A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831332A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831332AC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831332B0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 831332B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831332B8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831332BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831332C0: 4BFFC651  bl 0x8312f910
	ctx.lr = 0x831332C4;
	sub_8312F910(ctx, base);
	// 831332C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831332C8: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 831332CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831332D0: 4BFFC6D1  bl 0x8312f9a0
	ctx.lr = 0x831332D4;
	sub_8312F9A0(ctx, base);
	// 831332D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831332D8: 48005619  bl 0x831388f0
	ctx.lr = 0x831332DC;
	sub_831388F0(ctx, base);
	// 831332DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831332E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831332E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831332E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831332EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831332F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831332F0 size=28
    let mut pc: u32 = 0x831332F0;
    'dispatch: loop {
        match pc {
            0x831332F0 => {
    //   block [0x831332F0..0x8313330C)
	// 831332F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831332F4: 409A0018  bne cr6, 0x8313330c
	if !ctx.cr[6].eq {
		sub_8313330C(ctx, base);
		return;
	}
	// 831332F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831332FC: 388B16E8  addi r4, r11, 0x16e8
	ctx.r[4].s64 = ctx.r[11].s64 + 5864;
	// 83133300: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133304: 386B17A0  addi r3, r11, 0x17a0
	ctx.r[3].s64 = ctx.r[11].s64 + 6048;
	// 83133308: 4BFFFF98  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8313330C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8313330C size=32
    let mut pc: u32 = 0x8313330C;
    'dispatch: loop {
        match pc {
            0x8313330C => {
    //   block [0x8313330C..0x8313332C)
	// 8313330C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133310: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83133314: 409A0018  bne cr6, 0x8313332c
	if !ctx.cr[6].eq {
		sub_8313332C(ctx, base);
		return;
	}
	// 83133318: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8313331C: 388B16BC  addi r4, r11, 0x16bc
	ctx.r[4].s64 = ctx.r[11].s64 + 5820;
	// 83133320: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83133324: 386B1794  addi r3, r11, 0x1794
	ctx.r[3].s64 = ctx.r[11].s64 + 6036;
	// 83133328: 4BFFFF78  b 0x831332a0
	sub_831332A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8313332C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8313332C size=12
    let mut pc: u32 = 0x8313332C;
    'dispatch: loop {
        match pc {
            0x8313332C => {
    //   block [0x8313332C..0x83133338)
	// 8313332C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83133330: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83133334: 4C810020  blelr
	if !ctx.cr[0].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133338 size=12
    let mut pc: u32 = 0x83133338;
    'dispatch: loop {
        match pc {
            0x83133338 => {
    //   block [0x83133338..0x83133344)
	// 83133338: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8313333C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83133340: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133344(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133344 size=20
    let mut pc: u32 = 0x83133344;
    'dispatch: loop {
        match pc {
            0x83133344 => {
    //   block [0x83133344..0x83133358)
	// 83133344: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 83133348: 409A0020  bne cr6, 0x83133368
	if !ctx.cr[6].eq {
		sub_83133368(ctx, base);
		return;
	}
	// 8313334C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83133350: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83133354: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133358 size=16
    let mut pc: u32 = 0x83133358;
    'dispatch: loop {
        match pc {
            0x83133358 => {
    //   block [0x83133358..0x83133368)
	// 83133358: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8313335C: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83133360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83133364: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83133368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83133368 size=80
    let mut pc: u32 = 0x83133368;
    'dispatch: loop {
        match pc {
            0x83133368 => {
    //   block [0x83133368..0x831333B8)
	// 83133368: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8313336C: 409A0068  bne cr6, 0x831333d4
	if !ctx.cr[6].eq {
		sub_831333D4(ctx, base);
		return;
	}
	// 83133370: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83133374: 7D0B5051  subf. r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83133378: 41810008  bgt 0x83133380
	if ctx.cr[0].gt {
	pc = 0x83133380; continue 'dispatch;
	}
	// 8313337C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83133380: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 83133384: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83133388: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8313338C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83133390: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83133394: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83133398: 41980008  blt cr6, 0x831333a0
	if ctx.cr[6].lt {
	pc = 0x831333A0; continue 'dispatch;
	}
	// 8313339C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 831333A0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831333A4: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 831333A8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 831333AC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831333B0: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831333B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831333B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831333B8 size=12
    let mut pc: u32 = 0x831333B8;
    'dispatch: loop {
        match pc {
            0x831333B8 => {
    //   block [0x831333B8..0x831333C4)
	// 831333B8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831333BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831333C0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831333C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831333C4 size=16
    let mut pc: u32 = 0x831333C4;
    'dispatch: loop {
        match pc {
            0x831333C4 => {
    //   block [0x831333C4..0x831333D4)
	// 831333C4: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831333C8: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 831333CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831333D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831333D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831333D4 size=24
    let mut pc: u32 = 0x831333D4;
    'dispatch: loop {
        match pc {
            0x831333D4 => {
    //   block [0x831333D4..0x831333EC)
	// 831333D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831333D8: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831333DC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831333E0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831333E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 831333E8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831333EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831333EC size=16
    let mut pc: u32 = 0x831333EC;
    'dispatch: loop {
        match pc {
            0x831333EC => {
    //   block [0x831333EC..0x831333FC)
	// 831333EC: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831333F0: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 831333F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831333F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


