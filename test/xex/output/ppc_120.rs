pub fn sub_8270C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C750 size=32
    let mut pc: u32 = 0x8270C750;
    'dispatch: loop {
        match pc {
            0x8270C750 => {
    //   block [0x8270C750..0x8270C770)
	// 8270C750: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270C758: 394B0508  addi r10, r11, 0x508
	ctx.r[10].s64 = ctx.r[11].s64 + 1288;
	// 8270C75C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C760: 396B9080  addi r11, r11, -0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + -28544;
	// 8270C764: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8270C768: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270C76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C770 size=20
    let mut pc: u32 = 0x8270C770;
    'dispatch: loop {
        match pc {
            0x8270C770 => {
    //   block [0x8270C770..0x8270C784)
	// 8270C770: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C778: 396B9100  addi r11, r11, -0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + -28416;
	// 8270C77C: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8270C780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C788 size=32
    let mut pc: u32 = 0x8270C788;
    'dispatch: loop {
        match pc {
            0x8270C788 => {
    //   block [0x8270C788..0x8270C7A8)
	// 8270C788: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270C790: 394B0508  addi r10, r11, 0x508
	ctx.r[10].s64 = ctx.r[11].s64 + 1288;
	// 8270C794: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C798: 396B9100  addi r11, r11, -0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + -28416;
	// 8270C79C: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8270C7A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270C7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C7A8 size=20
    let mut pc: u32 = 0x8270C7A8;
    'dispatch: loop {
        match pc {
            0x8270C7A8 => {
    //   block [0x8270C7A8..0x8270C7BC)
	// 8270C7A8: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C7B0: 396B1B00  addi r11, r11, 0x1b00
	ctx.r[11].s64 = ctx.r[11].s64 + 6912;
	// 8270C7B4: 914B0310  stw r10, 0x310(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(784 as u32), ctx.r[10].u32 ) };
	// 8270C7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C7C0 size=96
    let mut pc: u32 = 0x8270C7C0;
    'dispatch: loop {
        match pc {
            0x8270C7C0 => {
    //   block [0x8270C7C0..0x8270C80C)
	// 8270C7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C7C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C7CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C7D0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C7D4: 3BEB9000  addi r31, r11, -0x7000
	ctx.r[31].s64 = ctx.r[11].s64 + -28672;
	// 8270C7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C7DC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8270C7E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8270C7E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C7E8: 419A0024  beq cr6, 0x8270c80c
	if ctx.cr[6].eq {
	pc = 0x8270C80C; continue 'dispatch;
	}
	// 8270C7EC: 4BC9421D  bl 0x823a0a08
	ctx.lr = 0x8270C7F0;
	sub_823A0A08(ctx, base);
	// 8270C7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C7F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8270C7F8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8270C7FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C800: 419A000C  beq cr6, 0x8270c80c
	if ctx.cr[6].eq {
	pc = 0x8270C80C; continue 'dispatch;
	}
	// 8270C804: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C808: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	pc = 0x8270C80C; continue 'dispatch;
            }
            0x8270C80C => {
    //   block [0x8270C80C..0x8270C820)
	// 8270C80C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270C810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C838 size=84
    let mut pc: u32 = 0x8270C838;
    'dispatch: loop {
        match pc {
            0x8270C838 => {
    //   block [0x8270C838..0x8270C884)
	// 8270C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C83C: 4BE28881  bl 0x825350bc
	ctx.lr = 0x8270C840;
	sub_82535080(ctx, base);
	// 8270C840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C844: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C848: 3BCBF480  addi r30, r11, -0xb80
	ctx.r[30].s64 = ctx.r[11].s64 + -2944;
	// 8270C84C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C850: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8270C854: 419A0030  beq cr6, 0x8270c884
	if ctx.cr[6].eq {
	pc = 0x8270C884; continue 'dispatch;
	}
	// 8270C858: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C85C: 3BEB2200  addi r31, r11, 0x2200
	ctx.r[31].s64 = ctx.r[11].s64 + 8704;
	// 8270C860: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8270C864: 480009F9  bl 0x8270d25c
	ctx.lr = 0x8270C868;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8270C868: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8270C86C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8270C870: 4BC9E671  bl 0x823aaee0
	ctx.lr = 0x8270C874;
	sub_823AAEE0(ctx, base);
	// 8270C874: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8270C878: 480009F5  bl 0x8270d26c
	ctx.lr = 0x8270C87C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8270C87C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C880: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x8270C884; continue 'dispatch;
            }
            0x8270C884 => {
    //   block [0x8270C884..0x8270C88C)
	// 8270C884: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270C888: 4BE28884  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C890 size=52
    let mut pc: u32 = 0x8270C890;
    'dispatch: loop {
        match pc {
            0x8270C890 => {
    //   block [0x8270C890..0x8270C8AC)
	// 8270C890: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C894: 396B9180  addi r11, r11, -0x6e80
	ctx.r[11].s64 = ctx.r[11].s64 + -28288;
	// 8270C898: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8270C89C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C8A0: 419A000C  beq cr6, 0x8270c8ac
	if ctx.cr[6].eq {
	pc = 0x8270C8AC; continue 'dispatch;
	}
	// 8270C8A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C8A8: 914B009C  stw r10, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	pc = 0x8270C8AC; continue 'dispatch;
            }
            0x8270C8AC => {
    //   block [0x8270C8AC..0x8270C8C4)
	// 8270C8AC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C8B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C8B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C8BC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270C8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C8C8 size=32
    let mut pc: u32 = 0x8270C8C8;
    'dispatch: loop {
        match pc {
            0x8270C8C8 => {
    //   block [0x8270C8C8..0x8270C8E8)
	// 8270C8C8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C8CC: 396BF500  addi r11, r11, -0xb00
	ctx.r[11].s64 = ctx.r[11].s64 + -2816;
	// 8270C8D0: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8270C8D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C8D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C8DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C8E0: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8270C8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C8E8 size=44
    let mut pc: u32 = 0x8270C8E8;
    'dispatch: loop {
        match pc {
            0x8270C8E8 => {
    //   block [0x8270C8E8..0x8270C900)
	// 8270C8E8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C8EC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8270C8F0: 396BD280  addi r11, r11, -0x2d80
	ctx.r[11].s64 = ctx.r[11].s64 + -11648;
	// 8270C8F4: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 8270C8F8: 396B2180  addi r11, r11, 0x2180
	ctx.r[11].s64 = ctx.r[11].s64 + 8576;
	// 8270C8FC: 3929053C  addi r9, r9, 0x53c
	ctx.r[9].s64 = ctx.r[9].s64 + 1340;
	pc = 0x8270C900; continue 'dispatch;
            }
            0x8270C900 => {
    //   block [0x8270C900..0x8270C914)
	// 8270C900: 396BFDE8  addi r11, r11, -0x218
	ctx.r[11].s64 = ctx.r[11].s64 + -536;
	// 8270C904: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8270C908: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8270C90C: 4080FFF4  bge 0x8270c900
	if !ctx.cr[0].lt {
	pc = 0x8270C900; continue 'dispatch;
	}
	// 8270C910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C918 size=92
    let mut pc: u32 = 0x8270C918;
    'dispatch: loop {
        match pc {
            0x8270C918 => {
    //   block [0x8270C918..0x8270C934)
	// 8270C918: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C91C: 396B9580  addi r11, r11, -0x6a80
	ctx.r[11].s64 = ctx.r[11].s64 + -27264;
	// 8270C920: 814B03BC  lwz r10, 0x3bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(956 as u32) ) } as u64;
	// 8270C924: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C928: 419A000C  beq cr6, 0x8270c934
	if ctx.cr[6].eq {
	pc = 0x8270C934; continue 'dispatch;
	}
	// 8270C92C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C930: 914B03BC  stw r10, 0x3bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(956 as u32), ctx.r[10].u32 ) };
	pc = 0x8270C934; continue 'dispatch;
            }
            0x8270C934 => {
    //   block [0x8270C934..0x8270C948)
	// 8270C934: 814B039C  lwz r10, 0x39c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(924 as u32) ) } as u64;
	// 8270C938: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C93C: 419A000C  beq cr6, 0x8270c948
	if ctx.cr[6].eq {
	pc = 0x8270C948; continue 'dispatch;
	}
	// 8270C940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C944: 914B039C  stw r10, 0x39c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(924 as u32), ctx.r[10].u32 ) };
	pc = 0x8270C948; continue 'dispatch;
            }
            0x8270C948 => {
    //   block [0x8270C948..0x8270C95C)
	// 8270C948: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8270C94C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C950: 419A000C  beq cr6, 0x8270c95c
	if ctx.cr[6].eq {
	pc = 0x8270C95C; continue 'dispatch;
	}
	// 8270C954: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C958: 914B009C  stw r10, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	pc = 0x8270C95C; continue 'dispatch;
            }
            0x8270C95C => {
    //   block [0x8270C95C..0x8270C974)
	// 8270C95C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C960: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C964: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C96C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270C970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C978 size=32
    let mut pc: u32 = 0x8270C978;
    'dispatch: loop {
        match pc {
            0x8270C978 => {
    //   block [0x8270C978..0x8270C998)
	// 8270C978: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C97C: 396BFE00  addi r11, r11, -0x200
	ctx.r[11].s64 = ctx.r[11].s64 + -512;
	// 8270C980: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8270C984: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C988: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C98C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C990: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8270C994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C998 size=4
    let mut pc: u32 = 0x8270C998;
    'dispatch: loop {
        match pc {
            0x8270C998 => {
    //   block [0x8270C998..0x8270C99C)
	// 8270C998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9A0 size=32
    let mut pc: u32 = 0x8270C9A0;
    'dispatch: loop {
        match pc {
            0x8270C9A0 => {
    //   block [0x8270C9A0..0x8270C9C0)
	// 8270C9A0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C9A4: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 8270C9A8: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8270C9AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C9B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C9B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C9B8: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8270C9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9C0 size=32
    let mut pc: u32 = 0x8270C9C0;
    'dispatch: loop {
        match pc {
            0x8270C9C0 => {
    //   block [0x8270C9C0..0x8270C9E0)
	// 8270C9C0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C9C4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 8270C9C8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8270C9CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C9D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C9D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C9D8: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8270C9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9E0 size=32
    let mut pc: u32 = 0x8270C9E0;
    'dispatch: loop {
        match pc {
            0x8270C9E0 => {
    //   block [0x8270C9E0..0x8270CA00)
	// 8270C9E0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C9E4: 396BFF80  addi r11, r11, -0x80
	ctx.r[11].s64 = ctx.r[11].s64 + -128;
	// 8270C9E8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8270C9EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C9F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270C9F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C9F8: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8270C9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA00 size=32
    let mut pc: u32 = 0x8270CA00;
    'dispatch: loop {
        match pc {
            0x8270CA00 => {
    //   block [0x8270CA00..0x8270CA20)
	// 8270CA00: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CA04: 396BFF00  addi r11, r11, -0x100
	ctx.r[11].s64 = ctx.r[11].s64 + -256;
	// 8270CA08: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8270CA0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270CA10: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270CA14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CA18: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8270CA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA20 size=32
    let mut pc: u32 = 0x8270CA20;
    'dispatch: loop {
        match pc {
            0x8270CA20 => {
    //   block [0x8270CA20..0x8270CA40)
	// 8270CA20: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CA24: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 8270CA28: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8270CA2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270CA30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270CA34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CA38: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8270CA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA40 size=4
    let mut pc: u32 = 0x8270CA40;
    'dispatch: loop {
        match pc {
            0x8270CA40 => {
    //   block [0x8270CA40..0x8270CA44)
	// 8270CA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA48 size=12
    let mut pc: u32 = 0x8270CA48;
    'dispatch: loop {
        match pc {
            0x8270CA48 => {
    //   block [0x8270CA48..0x8270CA54)
	// 8270CA48: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CA4C: 386B9980  addi r3, r11, -0x6680
	ctx.r[3].s64 = ctx.r[11].s64 + -26240;
	// 8270CA50: 4BC97260  b 0x823a3cb0
	sub_823A3CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CA58 size=92
    let mut pc: u32 = 0x8270CA58;
    'dispatch: loop {
        match pc {
            0x8270CA58 => {
    //   block [0x8270CA58..0x8270CA8C)
	// 8270CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270CA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270CA64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CA68: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CA6C: 3BEB0700  addi r31, r11, 0x700
	ctx.r[31].s64 = ctx.r[11].s64 + 1792;
	// 8270CA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270CA74: 4BC97B75  bl 0x823a45e8
	ctx.lr = 0x8270CA78;
	sub_823A45E8(ctx, base);
	// 8270CA78: 817F07C4  lwz r11, 0x7c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1988 as u32) ) } as u64;
	// 8270CA7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CA80: 419A000C  beq cr6, 0x8270ca8c
	if ctx.cr[6].eq {
	pc = 0x8270CA8C; continue 'dispatch;
	}
	// 8270CA84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270CA88: 917F07C4  stw r11, 0x7c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1988 as u32), ctx.r[11].u32 ) };
	pc = 0x8270CA8C; continue 'dispatch;
            }
            0x8270CA8C => {
    //   block [0x8270CA8C..0x8270CAA0)
	// 8270CA8C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8270CA90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CA94: 419A000C  beq cr6, 0x8270caa0
	if ctx.cr[6].eq {
	pc = 0x8270CAA0; continue 'dispatch;
	}
	// 8270CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270CA9C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	pc = 0x8270CAA0; continue 'dispatch;
            }
            0x8270CAA0 => {
    //   block [0x8270CAA0..0x8270CAB4)
	// 8270CAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270CAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270CAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270CAAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270CAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAB8 size=4
    let mut pc: u32 = 0x8270CAB8;
    'dispatch: loop {
        match pc {
            0x8270CAB8 => {
    //   block [0x8270CAB8..0x8270CABC)
	// 8270CAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAC0 size=4
    let mut pc: u32 = 0x8270CAC0;
    'dispatch: loop {
        match pc {
            0x8270CAC0 => {
    //   block [0x8270CAC0..0x8270CAC4)
	// 8270CAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAC8 size=20
    let mut pc: u32 = 0x8270CAC8;
    'dispatch: loop {
        match pc {
            0x8270CAC8 => {
    //   block [0x8270CAC8..0x8270CADC)
	// 8270CAC8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CAD0: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 8270CAD4: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8270CAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAE0 size=4
    let mut pc: u32 = 0x8270CAE0;
    'dispatch: loop {
        match pc {
            0x8270CAE0 => {
    //   block [0x8270CAE0..0x8270CAE4)
	// 8270CAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAE8 size=4
    let mut pc: u32 = 0x8270CAE8;
    'dispatch: loop {
        match pc {
            0x8270CAE8 => {
    //   block [0x8270CAE8..0x8270CAEC)
	// 8270CAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAF0 size=12
    let mut pc: u32 = 0x8270CAF0;
    'dispatch: loop {
        match pc {
            0x8270CAF0 => {
    //   block [0x8270CAF0..0x8270CAFC)
	// 8270CAF0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CAF4: 386B1100  addi r3, r11, 0x1100
	ctx.r[3].s64 = ctx.r[11].s64 + 4352;
	// 8270CAF8: 4BC9B530  b 0x823a8028
	sub_823A8028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CB00 size=4
    let mut pc: u32 = 0x8270CB00;
    'dispatch: loop {
        match pc {
            0x8270CB00 => {
    //   block [0x8270CB00..0x8270CB04)
	// 8270CB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CB08 size=64
    let mut pc: u32 = 0x8270CB08;
    'dispatch: loop {
        match pc {
            0x8270CB08 => {
    //   block [0x8270CB08..0x8270CB38)
	// 8270CB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270CB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CB14: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CB18: 38CB9B00  addi r6, r11, -0x6500
	ctx.r[6].s64 = ctx.r[11].s64 + -25856;
	// 8270CB1C: 38660024  addi r3, r6, 0x24
	ctx.r[3].s64 = ctx.r[6].s64 + 36;
	// 8270CB20: 4BC9B7C9  bl 0x823a82e8
	ctx.lr = 0x8270CB24;
	sub_823A82E8(ctx, base);
	// 8270CB24: 81660020  lwz r11, 0x20(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270CB28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CB2C: 419A000C  beq cr6, 0x8270cb38
	if ctx.cr[6].eq {
	pc = 0x8270CB38; continue 'dispatch;
	}
	// 8270CB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270CB34: 91660020  stw r11, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x8270CB38; continue 'dispatch;
            }
            0x8270CB38 => {
    //   block [0x8270CB38..0x8270CB48)
	// 8270CB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270CB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270CB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270CB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CB68 size=132
    let mut pc: u32 = 0x8270CB68;
    'dispatch: loop {
        match pc {
            0x8270CB68 => {
    //   block [0x8270CB68..0x8270CBD0)
	// 8270CB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CB6C: 4BE28551  bl 0x825350bc
	ctx.lr = 0x8270CB70;
	sub_82535080(ctx, base);
	// 8270CB70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CB74: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CB78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8270CB7C: 3BCB1380  addi r30, r11, 0x1380
	ctx.r[30].s64 = ctx.r[11].s64 + 4992;
	// 8270CB80: 83FE0E30  lwz r31, 0xe30(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3632 as u32) ) } as u64;
	// 8270CB84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8270CB88: 419A0048  beq cr6, 0x8270cbd0
	if ctx.cr[6].eq {
	pc = 0x8270CBD0; continue 'dispatch;
	}
	// 8270CB8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270CB90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8270CB94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8270CB98: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8270CB9C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8270CBA0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8270CBA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270CBA8: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8270CBAC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8270CBB0: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8270CBB4: 4BC93EAD  bl 0x823a0a60
	ctx.lr = 0x8270CBB8;
	sub_823A0A60(ctx, base);
	// 8270CBB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8270CBBC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270CBC0: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 8270CBC4: 4BC93FD5  bl 0x823a0b98
	ctx.lr = 0x8270CBC8;
	sub_823A0B98(ctx, base);
	// 8270CBC8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8270CBCC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	pc = 0x8270CBD0; continue 'dispatch;
            }
            0x8270CBD0 => {
    //   block [0x8270CBD0..0x8270CBE4)
	// 8270CBD0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270CBD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CBD8: 419A000C  beq cr6, 0x8270cbe4
	if ctx.cr[6].eq {
	pc = 0x8270CBE4; continue 'dispatch;
	}
	// 8270CBDC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8270CBE0: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	pc = 0x8270CBE4; continue 'dispatch;
            }
            0x8270CBE4 => {
    //   block [0x8270CBE4..0x8270CBEC)
	// 8270CBE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8270CBE8: 4BE28524  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CBF0 size=20
    let mut pc: u32 = 0x8270CBF0;
    'dispatch: loop {
        match pc {
            0x8270CBF0 => {
    //   block [0x8270CBF0..0x8270CC04)
	// 8270CBF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270CBF4: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8270CBF8: 396B1638  addi r11, r11, 0x1638
	ctx.r[11].s64 = ctx.r[11].s64 + 5688;
	// 8270CBFC: 916AA200  stw r11, -0x5e00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24064 as u32), ctx.r[11].u32 ) };
	// 8270CC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC08 size=12
    let mut pc: u32 = 0x8270CC08;
    'dispatch: loop {
        match pc {
            0x8270CC08 => {
    //   block [0x8270CC08..0x8270CC14)
	// 8270CC08: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CC0C: 386BDC90  addi r3, r11, -0x2370
	ctx.r[3].s64 = ctx.r[11].s64 + -9072;
	// 8270CC10: 4BCC03F0  b 0x823cd000
	sub_823CD000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC18 size=40
    let mut pc: u32 = 0x8270CC18;
    'dispatch: loop {
        match pc {
            0x8270CC18 => {
    //   block [0x8270CC18..0x8270CC40)
	// 8270CC18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CC1C: 394BBD38  addi r10, r11, -0x42c8
	ctx.r[10].s64 = ctx.r[11].s64 + -17096;
	// 8270CC20: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CC24: 396B3810  addi r11, r11, 0x3810
	ctx.r[11].s64 = ctx.r[11].s64 + 14352;
	// 8270CC28: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8270CC2C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270CC30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8270CC34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8270CC38: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270CC3C: 4BE270FC  b 0x82533d38
	sub_82533D38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC40 size=4
    let mut pc: u32 = 0x8270CC40;
    'dispatch: loop {
        match pc {
            0x8270CC40 => {
    //   block [0x8270CC40..0x8270CC44)
	// 8270CC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC48 size=4
    let mut pc: u32 = 0x8270CC48;
    'dispatch: loop {
        match pc {
            0x8270CC48 => {
    //   block [0x8270CC48..0x8270CC4C)
	// 8270CC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC50 size=4
    let mut pc: u32 = 0x8270CC50;
    'dispatch: loop {
        match pc {
            0x8270CC50 => {
    //   block [0x8270CC50..0x8270CC54)
	// 8270CC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CC58 size=84
    let mut pc: u32 = 0x8270CC58;
    'dispatch: loop {
        match pc {
            0x8270CC58 => {
    //   block [0x8270CC58..0x8270CC80)
	// 8270CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270CC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8270CC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270CC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CC6C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CC70: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 8270CC74: 396B5870  addi r11, r11, 0x5870
	ctx.r[11].s64 = ctx.r[11].s64 + 22640;
	// 8270CC78: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 8270CC7C: 3BEB9B5C  addi r31, r11, -0x64a4
	ctx.r[31].s64 = ctx.r[11].s64 + -25764;
	pc = 0x8270CC80; continue 'dispatch;
            }
            0x8270CC80 => {
    //   block [0x8270CC80..0x8270CCAC)
	// 8270CC80: 3BFFECD4  addi r31, r31, -0x132c
	ctx.r[31].s64 = ctx.r[31].s64 + -4908;
	// 8270CC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270CC88: 4BBA62F9  bl 0x822b2f80
	ctx.lr = 0x8270CC8C;
	sub_822B2F80(ctx, base);
	// 8270CC8C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270CC90: 4080FFF0  bge 0x8270cc80
	if !ctx.cr[0].lt {
	pc = 0x8270CC80; continue 'dispatch;
	}
	// 8270CC94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270CC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270CC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270CCA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8270CCA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270CCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCB0 size=16
    let mut pc: u32 = 0x8270CCB0;
    'dispatch: loop {
        match pc {
            0x8270CCB0 => {
    //   block [0x8270CCB0..0x8270CCC0)
	// 8270CCB0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8270CCB4: 396B9198  addi r11, r11, -0x6e68
	ctx.r[11].s64 = ctx.r[11].s64 + -28264;
	// 8270CCB8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8270CCBC: 4BD59664  b 0x82466320
	sub_82466320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCC0 size=20
    let mut pc: u32 = 0x8270CCC0;
    'dispatch: loop {
        match pc {
            0x8270CCC0 => {
    //   block [0x8270CCC0..0x8270CCD4)
	// 8270CCC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CCC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CCC8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CCCC: 916A91F8  stw r11, -0x6e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28168 as u32), ctx.r[11].u32 ) };
	// 8270CCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCD8 size=20
    let mut pc: u32 = 0x8270CCD8;
    'dispatch: loop {
        match pc {
            0x8270CCD8 => {
    //   block [0x8270CCD8..0x8270CCEC)
	// 8270CCD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CCDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CCE0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CCE4: 916A92D8  stw r11, -0x6d28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27944 as u32), ctx.r[11].u32 ) };
	// 8270CCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCF0 size=20
    let mut pc: u32 = 0x8270CCF0;
    'dispatch: loop {
        match pc {
            0x8270CCF0 => {
    //   block [0x8270CCF0..0x8270CD04)
	// 8270CCF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CCF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CCF8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CCFC: 916AEC64  stw r11, -0x139c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5020 as u32), ctx.r[11].u32 ) };
	// 8270CD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD08 size=20
    let mut pc: u32 = 0x8270CD08;
    'dispatch: loop {
        match pc {
            0x8270CD08 => {
    //   block [0x8270CD08..0x8270CD1C)
	// 8270CD08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CD10: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD14: 916A452C  stw r11, 0x452c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(17708 as u32), ctx.r[11].u32 ) };
	// 8270CD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD20 size=20
    let mut pc: u32 = 0x8270CD20;
    'dispatch: loop {
        match pc {
            0x8270CD20 => {
    //   block [0x8270CD20..0x8270CD34)
	// 8270CD20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD28: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD2C: 916A90D4  stw r11, -0x6f2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28460 as u32), ctx.r[11].u32 ) };
	// 8270CD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD38 size=20
    let mut pc: u32 = 0x8270CD38;
    'dispatch: loop {
        match pc {
            0x8270CD38 => {
    //   block [0x8270CD38..0x8270CD4C)
	// 8270CD38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD40: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD44: 916AE3CC  stw r11, -0x1c34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7220 as u32), ctx.r[11].u32 ) };
	// 8270CD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD50 size=20
    let mut pc: u32 = 0x8270CD50;
    'dispatch: loop {
        match pc {
            0x8270CD50 => {
    //   block [0x8270CD50..0x8270CD64)
	// 8270CD50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD58: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD5C: 916A1714  stw r11, 0x1714(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5908 as u32), ctx.r[11].u32 ) };
	// 8270CD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD68 size=20
    let mut pc: u32 = 0x8270CD68;
    'dispatch: loop {
        match pc {
            0x8270CD68 => {
    //   block [0x8270CD68..0x8270CD7C)
	// 8270CD68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD70: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD74: 916A595C  stw r11, 0x595c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22876 as u32), ctx.r[11].u32 ) };
	// 8270CD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD80 size=20
    let mut pc: u32 = 0x8270CD80;
    'dispatch: loop {
        match pc {
            0x8270CD80 => {
    //   block [0x8270CD80..0x8270CD94)
	// 8270CD80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CD88: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD8C: 916A94E4  stw r11, -0x6b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27420 as u32), ctx.r[11].u32 ) };
	// 8270CD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD98 size=20
    let mut pc: u32 = 0x8270CD98;
    'dispatch: loop {
        match pc {
            0x8270CD98 => {
    //   block [0x8270CD98..0x8270CDAC)
	// 8270CD98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CDA0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDA4: 916AE5CC  stw r11, -0x1a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6708 as u32), ctx.r[11].u32 ) };
	// 8270CDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDB0 size=20
    let mut pc: u32 = 0x8270CDB0;
    'dispatch: loop {
        match pc {
            0x8270CDB0 => {
    //   block [0x8270CDB0..0x8270CDC4)
	// 8270CDB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CDB8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDBC: 916A1164  stw r11, 0x1164(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4452 as u32), ctx.r[11].u32 ) };
	// 8270CDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDC8 size=20
    let mut pc: u32 = 0x8270CDC8;
    'dispatch: loop {
        match pc {
            0x8270CDC8 => {
    //   block [0x8270CDC8..0x8270CDDC)
	// 8270CDC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CDD0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDD4: 916A53B0  stw r11, 0x53b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(21424 as u32), ctx.r[11].u32 ) };
	// 8270CDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDE0 size=20
    let mut pc: u32 = 0x8270CDE0;
    'dispatch: loop {
        match pc {
            0x8270CDE0 => {
    //   block [0x8270CDE0..0x8270CDF4)
	// 8270CDE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CDE8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDEC: 916AA6A8  stw r11, -0x5958(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-22872 as u32), ctx.r[11].u32 ) };
	// 8270CDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDF8 size=20
    let mut pc: u32 = 0x8270CDF8;
    'dispatch: loop {
        match pc {
            0x8270CDF8 => {
    //   block [0x8270CDF8..0x8270CE0C)
	// 8270CDF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CE00: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE04: 916AF9A0  stw r11, -0x660(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-1632 as u32), ctx.r[11].u32 ) };
	// 8270CE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE10 size=20
    let mut pc: u32 = 0x8270CE10;
    'dispatch: loop {
        match pc {
            0x8270CE10 => {
    //   block [0x8270CE10..0x8270CE24)
	// 8270CE10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CE18: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE1C: 916A2D48  stw r11, 0x2d48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(11592 as u32), ctx.r[11].u32 ) };
	// 8270CE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE28 size=20
    let mut pc: u32 = 0x8270CE28;
    'dispatch: loop {
        match pc {
            0x8270CE28 => {
    //   block [0x8270CE28..0x8270CE3C)
	// 8270CE28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CE30: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE34: 916A5850  stw r11, 0x5850(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22608 as u32), ctx.r[11].u32 ) };
	// 8270CE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE40 size=20
    let mut pc: u32 = 0x8270CE40;
    'dispatch: loop {
        match pc {
            0x8270CE40 => {
    //   block [0x8270CE40..0x8270CE54)
	// 8270CE40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE48: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE4C: 916AA3F8  stw r11, -0x5c08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23560 as u32), ctx.r[11].u32 ) };
	// 8270CE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE58 size=20
    let mut pc: u32 = 0x8270CE58;
    'dispatch: loop {
        match pc {
            0x8270CE58 => {
    //   block [0x8270CE58..0x8270CE6C)
	// 8270CE58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE60: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE64: 916AD7A0  stw r11, -0x2860(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10336 as u32), ctx.r[11].u32 ) };
	// 8270CE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE70 size=20
    let mut pc: u32 = 0x8270CE70;
    'dispatch: loop {
        match pc {
            0x8270CE70 => {
    //   block [0x8270CE70..0x8270CE84)
	// 8270CE70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE78: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE7C: 916A0B18  stw r11, 0xb18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(2840 as u32), ctx.r[11].u32 ) };
	// 8270CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE88 size=20
    let mut pc: u32 = 0x8270CE88;
    'dispatch: loop {
        match pc {
            0x8270CE88 => {
    //   block [0x8270CE88..0x8270CE9C)
	// 8270CE88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE90: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE94: 916A6620  stw r11, 0x6620(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26144 as u32), ctx.r[11].u32 ) };
	// 8270CE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CEA0 size=20
    let mut pc: u32 = 0x8270CEA0;
    'dispatch: loop {
        match pc {
            0x8270CEA0 => {
    //   block [0x8270CEA0..0x8270CEB4)
	// 8270CEA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 8270CEA8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEAC: 916AC0F8  stw r11, -0x3f08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16136 as u32), ctx.r[11].u32 ) };
	// 8270CEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CEB8 size=20
    let mut pc: u32 = 0x8270CEB8;
    'dispatch: loop {
        match pc {
            0x8270CEB8 => {
    //   block [0x8270CEB8..0x8270CECC)
	// 8270CEB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CEBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 8270CEC0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEC4: 916A00D4  stw r11, 0xd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8270CEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CED0 size=20
    let mut pc: u32 = 0x8270CED0;
    'dispatch: loop {
        match pc {
            0x8270CED0 => {
    //   block [0x8270CED0..0x8270CEE4)
	// 8270CED0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 8270CED8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEDC: 916A41CC  stw r11, 0x41cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16844 as u32), ctx.r[11].u32 ) };
	// 8270CEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CEE8 size=20
    let mut pc: u32 = 0x8270CEE8;
    'dispatch: loop {
        match pc {
            0x8270CEE8 => {
    //   block [0x8270CEE8..0x8270CEFC)
	// 8270CEE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CEEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CEF0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEF4: 916A82C4  stw r11, -0x7d3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32060 as u32), ctx.r[11].u32 ) };
	// 8270CEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF00 size=20
    let mut pc: u32 = 0x8270CF00;
    'dispatch: loop {
        match pc {
            0x8270CF00 => {
    //   block [0x8270CF00..0x8270CF14)
	// 8270CF00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CF08: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF0C: 916AB27C  stw r11, -0x4d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19844 as u32), ctx.r[11].u32 ) };
	// 8270CF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF18 size=20
    let mut pc: u32 = 0x8270CF18;
    'dispatch: loop {
        match pc {
            0x8270CF18 => {
    //   block [0x8270CF18..0x8270CF2C)
	// 8270CF18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CF20: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF24: 916A0B44  stw r11, 0xb44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(2884 as u32), ctx.r[11].u32 ) };
	// 8270CF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF30 size=20
    let mut pc: u32 = 0x8270CF30;
    'dispatch: loop {
        match pc {
            0x8270CF30 => {
    //   block [0x8270CF30..0x8270CF44)
	// 8270CF30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CF38: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF3C: 916A649C  stw r11, 0x649c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25756 as u32), ctx.r[11].u32 ) };
	// 8270CF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF48 size=20
    let mut pc: u32 = 0x8270CF48;
    'dispatch: loop {
        match pc {
            0x8270CF48 => {
    //   block [0x8270CF48..0x8270CF5C)
	// 8270CF48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270CF50: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF54: 916AB794  stw r11, -0x486c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18540 as u32), ctx.r[11].u32 ) };
	// 8270CF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF60 size=20
    let mut pc: u32 = 0x8270CF60;
    'dispatch: loop {
        match pc {
            0x8270CF60 => {
    //   block [0x8270CF60..0x8270CF74)
	// 8270CF60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270CF68: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF6C: 916AF79C  stw r11, -0x864(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-2148 as u32), ctx.r[11].u32 ) };
	// 8270CF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF78 size=20
    let mut pc: u32 = 0x8270CF78;
    'dispatch: loop {
        match pc {
            0x8270CF78 => {
    //   block [0x8270CF78..0x8270CF8C)
	// 8270CF78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF7C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270CF80: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF84: 916AB67C  stw r11, -0x4984(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18820 as u32), ctx.r[11].u32 ) };
	// 8270CF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF90 size=20
    let mut pc: u32 = 0x8270CF90;
    'dispatch: loop {
        match pc {
            0x8270CF90 => {
    //   block [0x8270CF90..0x8270CFA4)
	// 8270CF90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF94: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270CF98: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF9C: 916AD24C  stw r11, -0x2db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11700 as u32), ctx.r[11].u32 ) };
	// 8270CFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFA8 size=20
    let mut pc: u32 = 0x8270CFA8;
    'dispatch: loop {
        match pc {
            0x8270CFA8 => {
    //   block [0x8270CFA8..0x8270CFBC)
	// 8270CFA8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270CFAC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270CFB0: 396B6DF8  addi r11, r11, 0x6df8
	ctx.r[11].s64 = ctx.r[11].s64 + 28152;
	// 8270CFB4: 916AE660  stw r11, -0x19a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6560 as u32), ctx.r[11].u32 ) };
	// 8270CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFC0 size=20
    let mut pc: u32 = 0x8270CFC0;
    'dispatch: loop {
        match pc {
            0x8270CFC0 => {
    //   block [0x8270CFC0..0x8270CFD4)
	// 8270CFC0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8270CFC4: 3D408283  lis r10, -0x7d7d
	ctx.r[10].s64 = -2105344000;
	// 8270CFC8: 396B68EC  addi r11, r11, 0x68ec
	ctx.r[11].s64 = ctx.r[11].s64 + 26860;
	// 8270CFCC: 916A2348  stw r11, 0x2348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9032 as u32), ctx.r[11].u32 ) };
	// 8270CFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFD8 size=20
    let mut pc: u32 = 0x8270CFD8;
    'dispatch: loop {
        match pc {
            0x8270CFD8 => {
    //   block [0x8270CFD8..0x8270CFEC)
	// 8270CFD8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8270CFDC: 3D408283  lis r10, -0x7d7d
	ctx.r[10].s64 = -2105344000;
	// 8270CFE0: 396B68EC  addi r11, r11, 0x68ec
	ctx.r[11].s64 = ctx.r[11].s64 + 26860;
	// 8270CFE4: 916A2368  stw r11, 0x2368(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9064 as u32), ctx.r[11].u32 ) };
	// 8270CFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFEC size=16
    let mut pc: u32 = 0x8270CFEC;
    'dispatch: loop {
        match pc {
            0x8270CFEC => {
    //   block [0x8270CFEC..0x8270CFFC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFFC size=16
    let mut pc: u32 = 0x8270CFFC;
    'dispatch: loop {
        match pc {
            0x8270CFFC => {
    //   block [0x8270CFFC..0x8270D00C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D00C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D00C size=16
    let mut pc: u32 = 0x8270D00C;
    'dispatch: loop {
        match pc {
            0x8270D00C => {
    //   block [0x8270D00C..0x8270D01C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D01C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D01C size=16
    let mut pc: u32 = 0x8270D01C;
    'dispatch: loop {
        match pc {
            0x8270D01C => {
    //   block [0x8270D01C..0x8270D02C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D02C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D02C size=16
    let mut pc: u32 = 0x8270D02C;
    'dispatch: loop {
        match pc {
            0x8270D02C => {
    //   block [0x8270D02C..0x8270D03C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D03C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D03C size=16
    let mut pc: u32 = 0x8270D03C;
    'dispatch: loop {
        match pc {
            0x8270D03C => {
    //   block [0x8270D03C..0x8270D04C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D04C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D04C size=16
    let mut pc: u32 = 0x8270D04C;
    'dispatch: loop {
        match pc {
            0x8270D04C => {
    //   block [0x8270D04C..0x8270D05C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D05C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D05C size=16
    let mut pc: u32 = 0x8270D05C;
    'dispatch: loop {
        match pc {
            0x8270D05C => {
    //   block [0x8270D05C..0x8270D06C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D06C size=16
    let mut pc: u32 = 0x8270D06C;
    'dispatch: loop {
        match pc {
            0x8270D06C => {
    //   block [0x8270D06C..0x8270D07C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D07C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D07C size=16
    let mut pc: u32 = 0x8270D07C;
    'dispatch: loop {
        match pc {
            0x8270D07C => {
    //   block [0x8270D07C..0x8270D08C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D09C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D09C size=16
    let mut pc: u32 = 0x8270D09C;
    'dispatch: loop {
        match pc {
            0x8270D09C => {
    //   block [0x8270D09C..0x8270D0AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0AC size=16
    let mut pc: u32 = 0x8270D0AC;
    'dispatch: loop {
        match pc {
            0x8270D0AC => {
    //   block [0x8270D0AC..0x8270D0BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0BC size=16
    let mut pc: u32 = 0x8270D0BC;
    'dispatch: loop {
        match pc {
            0x8270D0BC => {
    //   block [0x8270D0BC..0x8270D0CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0CC size=16
    let mut pc: u32 = 0x8270D0CC;
    'dispatch: loop {
        match pc {
            0x8270D0CC => {
    //   block [0x8270D0CC..0x8270D0DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0DC size=16
    let mut pc: u32 = 0x8270D0DC;
    'dispatch: loop {
        match pc {
            0x8270D0DC => {
    //   block [0x8270D0DC..0x8270D0EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0EC size=16
    let mut pc: u32 = 0x8270D0EC;
    'dispatch: loop {
        match pc {
            0x8270D0EC => {
    //   block [0x8270D0EC..0x8270D0FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0FC size=16
    let mut pc: u32 = 0x8270D0FC;
    'dispatch: loop {
        match pc {
            0x8270D0FC => {
    //   block [0x8270D0FC..0x8270D10C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D10C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D10C size=16
    let mut pc: u32 = 0x8270D10C;
    'dispatch: loop {
        match pc {
            0x8270D10C => {
    //   block [0x8270D10C..0x8270D11C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D11C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D11C size=16
    let mut pc: u32 = 0x8270D11C;
    'dispatch: loop {
        match pc {
            0x8270D11C => {
    //   block [0x8270D11C..0x8270D12C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D12C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D12C size=16
    let mut pc: u32 = 0x8270D12C;
    'dispatch: loop {
        match pc {
            0x8270D12C => {
    //   block [0x8270D12C..0x8270D13C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D13C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D13C size=16
    let mut pc: u32 = 0x8270D13C;
    'dispatch: loop {
        match pc {
            0x8270D13C => {
    //   block [0x8270D13C..0x8270D14C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D14C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D14C size=16
    let mut pc: u32 = 0x8270D14C;
    'dispatch: loop {
        match pc {
            0x8270D14C => {
    //   block [0x8270D14C..0x8270D15C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D15C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D15C size=16
    let mut pc: u32 = 0x8270D15C;
    'dispatch: loop {
        match pc {
            0x8270D15C => {
    //   block [0x8270D15C..0x8270D16C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D16C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D16C size=16
    let mut pc: u32 = 0x8270D16C;
    'dispatch: loop {
        match pc {
            0x8270D16C => {
    //   block [0x8270D16C..0x8270D17C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D17C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D17C size=16
    let mut pc: u32 = 0x8270D17C;
    'dispatch: loop {
        match pc {
            0x8270D17C => {
    //   block [0x8270D17C..0x8270D18C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D18C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D18C size=16
    let mut pc: u32 = 0x8270D18C;
    'dispatch: loop {
        match pc {
            0x8270D18C => {
    //   block [0x8270D18C..0x8270D19C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D19C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D19C size=16
    let mut pc: u32 = 0x8270D19C;
    'dispatch: loop {
        match pc {
            0x8270D19C => {
    //   block [0x8270D19C..0x8270D1AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1AC size=16
    let mut pc: u32 = 0x8270D1AC;
    'dispatch: loop {
        match pc {
            0x8270D1AC => {
    //   block [0x8270D1AC..0x8270D1BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1BC size=16
    let mut pc: u32 = 0x8270D1BC;
    'dispatch: loop {
        match pc {
            0x8270D1BC => {
    //   block [0x8270D1BC..0x8270D1CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1DC size=16
    let mut pc: u32 = 0x8270D1DC;
    'dispatch: loop {
        match pc {
            0x8270D1DC => {
    //   block [0x8270D1DC..0x8270D1EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1EC size=16
    let mut pc: u32 = 0x8270D1EC;
    'dispatch: loop {
        match pc {
            0x8270D1EC => {
    //   block [0x8270D1EC..0x8270D1FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1FC size=16
    let mut pc: u32 = 0x8270D1FC;
    'dispatch: loop {
        match pc {
            0x8270D1FC => {
    //   block [0x8270D1FC..0x8270D20C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D20C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D20C size=16
    let mut pc: u32 = 0x8270D20C;
    'dispatch: loop {
        match pc {
            0x8270D20C => {
    //   block [0x8270D20C..0x8270D21C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D21C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D21C size=16
    let mut pc: u32 = 0x8270D21C;
    'dispatch: loop {
        match pc {
            0x8270D21C => {
    //   block [0x8270D21C..0x8270D22C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D22C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D22C size=16
    let mut pc: u32 = 0x8270D22C;
    'dispatch: loop {
        match pc {
            0x8270D22C => {
    //   block [0x8270D22C..0x8270D23C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D23C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D23C size=16
    let mut pc: u32 = 0x8270D23C;
    'dispatch: loop {
        match pc {
            0x8270D23C => {
    //   block [0x8270D23C..0x8270D24C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D24C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D24C size=16
    let mut pc: u32 = 0x8270D24C;
    'dispatch: loop {
        match pc {
            0x8270D24C => {
    //   block [0x8270D24C..0x8270D25C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D25C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D25C size=16
    let mut pc: u32 = 0x8270D25C;
    'dispatch: loop {
        match pc {
            0x8270D25C => {
    //   block [0x8270D25C..0x8270D26C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D26C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D26C size=16
    let mut pc: u32 = 0x8270D26C;
    'dispatch: loop {
        match pc {
            0x8270D26C => {
    //   block [0x8270D26C..0x8270D27C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D27C size=16
    let mut pc: u32 = 0x8270D27C;
    'dispatch: loop {
        match pc {
            0x8270D27C => {
    //   block [0x8270D27C..0x8270D28C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D28C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D28C size=16
    let mut pc: u32 = 0x8270D28C;
    'dispatch: loop {
        match pc {
            0x8270D28C => {
    //   block [0x8270D28C..0x8270D29C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D29C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D29C size=16
    let mut pc: u32 = 0x8270D29C;
    'dispatch: loop {
        match pc {
            0x8270D29C => {
    //   block [0x8270D29C..0x8270D2AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2AC size=16
    let mut pc: u32 = 0x8270D2AC;
    'dispatch: loop {
        match pc {
            0x8270D2AC => {
    //   block [0x8270D2AC..0x8270D2BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2BC size=16
    let mut pc: u32 = 0x8270D2BC;
    'dispatch: loop {
        match pc {
            0x8270D2BC => {
    //   block [0x8270D2BC..0x8270D2CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2CC size=16
    let mut pc: u32 = 0x8270D2CC;
    'dispatch: loop {
        match pc {
            0x8270D2CC => {
    //   block [0x8270D2CC..0x8270D2DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2DC size=16
    let mut pc: u32 = 0x8270D2DC;
    'dispatch: loop {
        match pc {
            0x8270D2DC => {
    //   block [0x8270D2DC..0x8270D2EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2EC size=16
    let mut pc: u32 = 0x8270D2EC;
    'dispatch: loop {
        match pc {
            0x8270D2EC => {
    //   block [0x8270D2EC..0x8270D2FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2FC size=16
    let mut pc: u32 = 0x8270D2FC;
    'dispatch: loop {
        match pc {
            0x8270D2FC => {
    //   block [0x8270D2FC..0x8270D30C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D30C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D30C size=16
    let mut pc: u32 = 0x8270D30C;
    'dispatch: loop {
        match pc {
            0x8270D30C => {
    //   block [0x8270D30C..0x8270D31C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D31C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D31C size=16
    let mut pc: u32 = 0x8270D31C;
    'dispatch: loop {
        match pc {
            0x8270D31C => {
    //   block [0x8270D31C..0x8270D32C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D32C size=16
    let mut pc: u32 = 0x8270D32C;
    'dispatch: loop {
        match pc {
            0x8270D32C => {
    //   block [0x8270D32C..0x8270D33C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D33C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D33C size=16
    let mut pc: u32 = 0x8270D33C;
    'dispatch: loop {
        match pc {
            0x8270D33C => {
    //   block [0x8270D33C..0x8270D34C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D34C size=16
    let mut pc: u32 = 0x8270D34C;
    'dispatch: loop {
        match pc {
            0x8270D34C => {
    //   block [0x8270D34C..0x8270D35C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D35C size=16
    let mut pc: u32 = 0x8270D35C;
    'dispatch: loop {
        match pc {
            0x8270D35C => {
    //   block [0x8270D35C..0x8270D36C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D36C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D36C size=16
    let mut pc: u32 = 0x8270D36C;
    'dispatch: loop {
        match pc {
            0x8270D36C => {
    //   block [0x8270D36C..0x8270D37C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D37C size=16
    let mut pc: u32 = 0x8270D37C;
    'dispatch: loop {
        match pc {
            0x8270D37C => {
    //   block [0x8270D37C..0x8270D38C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D38C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D38C size=16
    let mut pc: u32 = 0x8270D38C;
    'dispatch: loop {
        match pc {
            0x8270D38C => {
    //   block [0x8270D38C..0x8270D39C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D39C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D39C size=16
    let mut pc: u32 = 0x8270D39C;
    'dispatch: loop {
        match pc {
            0x8270D39C => {
    //   block [0x8270D39C..0x8270D3AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D3AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D3AC size=16
    let mut pc: u32 = 0x8270D3AC;
    'dispatch: loop {
        match pc {
            0x8270D3AC => {
    //   block [0x8270D3AC..0x8270D3BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D3BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D3BC size=16
    let mut pc: u32 = 0x8270D3BC;
    'dispatch: loop {
        match pc {
            0x8270D3BC => {
    //   block [0x8270D3BC..0x8270D3CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D3CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D3CC size=16
    let mut pc: u32 = 0x8270D3CC;
    'dispatch: loop {
        match pc {
            0x8270D3CC => {
    //   block [0x8270D3CC..0x8270D3DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D3DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D3DC size=16
    let mut pc: u32 = 0x8270D3DC;
    'dispatch: loop {
        match pc {
            0x8270D3DC => {
    //   block [0x8270D3DC..0x8270D3EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D3EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D3EC size=16
    let mut pc: u32 = 0x8270D3EC;
    'dispatch: loop {
        match pc {
            0x8270D3EC => {
    //   block [0x8270D3EC..0x8270D3FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D3FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D3FC size=16
    let mut pc: u32 = 0x8270D3FC;
    'dispatch: loop {
        match pc {
            0x8270D3FC => {
    //   block [0x8270D3FC..0x8270D40C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D40C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D40C size=16
    let mut pc: u32 = 0x8270D40C;
    'dispatch: loop {
        match pc {
            0x8270D40C => {
    //   block [0x8270D40C..0x8270D41C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D41C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D41C size=16
    let mut pc: u32 = 0x8270D41C;
    'dispatch: loop {
        match pc {
            0x8270D41C => {
    //   block [0x8270D41C..0x8270D42C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D42C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D42C size=16
    let mut pc: u32 = 0x8270D42C;
    'dispatch: loop {
        match pc {
            0x8270D42C => {
    //   block [0x8270D42C..0x8270D43C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D43C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D43C size=16
    let mut pc: u32 = 0x8270D43C;
    'dispatch: loop {
        match pc {
            0x8270D43C => {
    //   block [0x8270D43C..0x8270D44C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D44C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D44C size=16
    let mut pc: u32 = 0x8270D44C;
    'dispatch: loop {
        match pc {
            0x8270D44C => {
    //   block [0x8270D44C..0x8270D45C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D45C size=16
    let mut pc: u32 = 0x8270D45C;
    'dispatch: loop {
        match pc {
            0x8270D45C => {
    //   block [0x8270D45C..0x8270D46C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D46C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D46C size=16
    let mut pc: u32 = 0x8270D46C;
    'dispatch: loop {
        match pc {
            0x8270D46C => {
    //   block [0x8270D46C..0x8270D47C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D47C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D47C size=16
    let mut pc: u32 = 0x8270D47C;
    'dispatch: loop {
        match pc {
            0x8270D47C => {
    //   block [0x8270D47C..0x8270D48C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D48C size=16
    let mut pc: u32 = 0x8270D48C;
    'dispatch: loop {
        match pc {
            0x8270D48C => {
    //   block [0x8270D48C..0x8270D49C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D49C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D49C size=16
    let mut pc: u32 = 0x8270D49C;
    'dispatch: loop {
        match pc {
            0x8270D49C => {
    //   block [0x8270D49C..0x8270D4AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D4AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D4AC size=16
    let mut pc: u32 = 0x8270D4AC;
    'dispatch: loop {
        match pc {
            0x8270D4AC => {
    //   block [0x8270D4AC..0x8270D4BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D4BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D4BC size=16
    let mut pc: u32 = 0x8270D4BC;
    'dispatch: loop {
        match pc {
            0x8270D4BC => {
    //   block [0x8270D4BC..0x8270D4CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D4CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D4CC size=16
    let mut pc: u32 = 0x8270D4CC;
    'dispatch: loop {
        match pc {
            0x8270D4CC => {
    //   block [0x8270D4CC..0x8270D4DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D4DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D4DC size=16
    let mut pc: u32 = 0x8270D4DC;
    'dispatch: loop {
        match pc {
            0x8270D4DC => {
    //   block [0x8270D4DC..0x8270D4EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D4FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D4FC size=16
    let mut pc: u32 = 0x8270D4FC;
    'dispatch: loop {
        match pc {
            0x8270D4FC => {
    //   block [0x8270D4FC..0x8270D50C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D50C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D50C size=16
    let mut pc: u32 = 0x8270D50C;
    'dispatch: loop {
        match pc {
            0x8270D50C => {
    //   block [0x8270D50C..0x8270D51C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D51C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D51C size=16
    let mut pc: u32 = 0x8270D51C;
    'dispatch: loop {
        match pc {
            0x8270D51C => {
    //   block [0x8270D51C..0x8270D52C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D52C size=16
    let mut pc: u32 = 0x8270D52C;
    'dispatch: loop {
        match pc {
            0x8270D52C => {
    //   block [0x8270D52C..0x8270D53C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D53C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D53C size=16
    let mut pc: u32 = 0x8270D53C;
    'dispatch: loop {
        match pc {
            0x8270D53C => {
    //   block [0x8270D53C..0x8270D54C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D54C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D54C size=16
    let mut pc: u32 = 0x8270D54C;
    'dispatch: loop {
        match pc {
            0x8270D54C => {
    //   block [0x8270D54C..0x8270D55C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D55C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D55C size=16
    let mut pc: u32 = 0x8270D55C;
    'dispatch: loop {
        match pc {
            0x8270D55C => {
    //   block [0x8270D55C..0x8270D56C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D56C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D56C size=16
    let mut pc: u32 = 0x8270D56C;
    'dispatch: loop {
        match pc {
            0x8270D56C => {
    //   block [0x8270D56C..0x8270D57C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D57C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D57C size=16
    let mut pc: u32 = 0x8270D57C;
    'dispatch: loop {
        match pc {
            0x8270D57C => {
    //   block [0x8270D57C..0x8270D58C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D58C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D58C size=16
    let mut pc: u32 = 0x8270D58C;
    'dispatch: loop {
        match pc {
            0x8270D58C => {
    //   block [0x8270D58C..0x8270D59C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D59C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D59C size=16
    let mut pc: u32 = 0x8270D59C;
    'dispatch: loop {
        match pc {
            0x8270D59C => {
    //   block [0x8270D59C..0x8270D5AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D5AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D5AC size=16
    let mut pc: u32 = 0x8270D5AC;
    'dispatch: loop {
        match pc {
            0x8270D5AC => {
    //   block [0x8270D5AC..0x8270D5BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D5BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D5BC size=16
    let mut pc: u32 = 0x8270D5BC;
    'dispatch: loop {
        match pc {
            0x8270D5BC => {
    //   block [0x8270D5BC..0x8270D5CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D5CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D5CC size=16
    let mut pc: u32 = 0x8270D5CC;
    'dispatch: loop {
        match pc {
            0x8270D5CC => {
    //   block [0x8270D5CC..0x8270D5DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D5DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D5DC size=16
    let mut pc: u32 = 0x8270D5DC;
    'dispatch: loop {
        match pc {
            0x8270D5DC => {
    //   block [0x8270D5DC..0x8270D5EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D5EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D5EC size=16
    let mut pc: u32 = 0x8270D5EC;
    'dispatch: loop {
        match pc {
            0x8270D5EC => {
    //   block [0x8270D5EC..0x8270D5FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D5FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D5FC size=16
    let mut pc: u32 = 0x8270D5FC;
    'dispatch: loop {
        match pc {
            0x8270D5FC => {
    //   block [0x8270D5FC..0x8270D60C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D60C size=16
    let mut pc: u32 = 0x8270D60C;
    'dispatch: loop {
        match pc {
            0x8270D60C => {
    //   block [0x8270D60C..0x8270D61C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D61C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D61C size=16
    let mut pc: u32 = 0x8270D61C;
    'dispatch: loop {
        match pc {
            0x8270D61C => {
    //   block [0x8270D61C..0x8270D62C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D62C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D62C size=16
    let mut pc: u32 = 0x8270D62C;
    'dispatch: loop {
        match pc {
            0x8270D62C => {
    //   block [0x8270D62C..0x8270D63C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D63C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D63C size=16
    let mut pc: u32 = 0x8270D63C;
    'dispatch: loop {
        match pc {
            0x8270D63C => {
    //   block [0x8270D63C..0x8270D64C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D64C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D64C size=16
    let mut pc: u32 = 0x8270D64C;
    'dispatch: loop {
        match pc {
            0x8270D64C => {
    //   block [0x8270D64C..0x8270D65C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D65C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D65C size=16
    let mut pc: u32 = 0x8270D65C;
    'dispatch: loop {
        match pc {
            0x8270D65C => {
    //   block [0x8270D65C..0x8270D66C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D66C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D66C size=16
    let mut pc: u32 = 0x8270D66C;
    'dispatch: loop {
        match pc {
            0x8270D66C => {
    //   block [0x8270D66C..0x8270D67C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D67C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D67C size=16
    let mut pc: u32 = 0x8270D67C;
    'dispatch: loop {
        match pc {
            0x8270D67C => {
    //   block [0x8270D67C..0x8270D68C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D68C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D68C size=16
    let mut pc: u32 = 0x8270D68C;
    'dispatch: loop {
        match pc {
            0x8270D68C => {
    //   block [0x8270D68C..0x8270D69C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D69C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D69C size=16
    let mut pc: u32 = 0x8270D69C;
    'dispatch: loop {
        match pc {
            0x8270D69C => {
    //   block [0x8270D69C..0x8270D6AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D6AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D6AC size=16
    let mut pc: u32 = 0x8270D6AC;
    'dispatch: loop {
        match pc {
            0x8270D6AC => {
    //   block [0x8270D6AC..0x8270D6BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D6BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D6BC size=16
    let mut pc: u32 = 0x8270D6BC;
    'dispatch: loop {
        match pc {
            0x8270D6BC => {
    //   block [0x8270D6BC..0x8270D6CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D6CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D6CC size=16
    let mut pc: u32 = 0x8270D6CC;
    'dispatch: loop {
        match pc {
            0x8270D6CC => {
    //   block [0x8270D6CC..0x8270D6DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D6DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D6DC size=16
    let mut pc: u32 = 0x8270D6DC;
    'dispatch: loop {
        match pc {
            0x8270D6DC => {
    //   block [0x8270D6DC..0x8270D6EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D6EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D6EC size=16
    let mut pc: u32 = 0x8270D6EC;
    'dispatch: loop {
        match pc {
            0x8270D6EC => {
    //   block [0x8270D6EC..0x8270D6FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D6FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D6FC size=16
    let mut pc: u32 = 0x8270D6FC;
    'dispatch: loop {
        match pc {
            0x8270D6FC => {
    //   block [0x8270D6FC..0x8270D70C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D70C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D70C size=16
    let mut pc: u32 = 0x8270D70C;
    'dispatch: loop {
        match pc {
            0x8270D70C => {
    //   block [0x8270D70C..0x8270D71C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D71C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D71C size=16
    let mut pc: u32 = 0x8270D71C;
    'dispatch: loop {
        match pc {
            0x8270D71C => {
    //   block [0x8270D71C..0x8270D72C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D72C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D72C size=16
    let mut pc: u32 = 0x8270D72C;
    'dispatch: loop {
        match pc {
            0x8270D72C => {
    //   block [0x8270D72C..0x8270D73C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D73C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D73C size=16
    let mut pc: u32 = 0x8270D73C;
    'dispatch: loop {
        match pc {
            0x8270D73C => {
    //   block [0x8270D73C..0x8270D74C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D74C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D74C size=16
    let mut pc: u32 = 0x8270D74C;
    'dispatch: loop {
        match pc {
            0x8270D74C => {
    //   block [0x8270D74C..0x8270D75C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D75C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D75C size=16
    let mut pc: u32 = 0x8270D75C;
    'dispatch: loop {
        match pc {
            0x8270D75C => {
    //   block [0x8270D75C..0x8270D76C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D76C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D76C size=16
    let mut pc: u32 = 0x8270D76C;
    'dispatch: loop {
        match pc {
            0x8270D76C => {
    //   block [0x8270D76C..0x8270D77C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D77C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D77C size=16
    let mut pc: u32 = 0x8270D77C;
    'dispatch: loop {
        match pc {
            0x8270D77C => {
    //   block [0x8270D77C..0x8270D78C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D78C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D78C size=16
    let mut pc: u32 = 0x8270D78C;
    'dispatch: loop {
        match pc {
            0x8270D78C => {
    //   block [0x8270D78C..0x8270D79C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D79C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D79C size=16
    let mut pc: u32 = 0x8270D79C;
    'dispatch: loop {
        match pc {
            0x8270D79C => {
    //   block [0x8270D79C..0x8270D7AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D7AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D7AC size=16
    let mut pc: u32 = 0x8270D7AC;
    'dispatch: loop {
        match pc {
            0x8270D7AC => {
    //   block [0x8270D7AC..0x8270D7BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D7BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D7BC size=16
    let mut pc: u32 = 0x8270D7BC;
    'dispatch: loop {
        match pc {
            0x8270D7BC => {
    //   block [0x8270D7BC..0x8270D7CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D7CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D7CC size=16
    let mut pc: u32 = 0x8270D7CC;
    'dispatch: loop {
        match pc {
            0x8270D7CC => {
    //   block [0x8270D7CC..0x8270D7DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D7DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D7DC size=16
    let mut pc: u32 = 0x8270D7DC;
    'dispatch: loop {
        match pc {
            0x8270D7DC => {
    //   block [0x8270D7DC..0x8270D7EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D7EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D7EC size=16
    let mut pc: u32 = 0x8270D7EC;
    'dispatch: loop {
        match pc {
            0x8270D7EC => {
    //   block [0x8270D7EC..0x8270D7FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D7FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D7FC size=16
    let mut pc: u32 = 0x8270D7FC;
    'dispatch: loop {
        match pc {
            0x8270D7FC => {
    //   block [0x8270D7FC..0x8270D80C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D80C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D80C size=16
    let mut pc: u32 = 0x8270D80C;
    'dispatch: loop {
        match pc {
            0x8270D80C => {
    //   block [0x8270D80C..0x8270D81C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D81C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D81C size=16
    let mut pc: u32 = 0x8270D81C;
    'dispatch: loop {
        match pc {
            0x8270D81C => {
    //   block [0x8270D81C..0x8270D82C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D82C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D82C size=16
    let mut pc: u32 = 0x8270D82C;
    'dispatch: loop {
        match pc {
            0x8270D82C => {
    //   block [0x8270D82C..0x8270D83C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D83C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D83C size=16
    let mut pc: u32 = 0x8270D83C;
    'dispatch: loop {
        match pc {
            0x8270D83C => {
    //   block [0x8270D83C..0x8270D84C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D84C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D84C size=16
    let mut pc: u32 = 0x8270D84C;
    'dispatch: loop {
        match pc {
            0x8270D84C => {
    //   block [0x8270D84C..0x8270D85C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D85C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D85C size=16
    let mut pc: u32 = 0x8270D85C;
    'dispatch: loop {
        match pc {
            0x8270D85C => {
    //   block [0x8270D85C..0x8270D86C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D86C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D86C size=16
    let mut pc: u32 = 0x8270D86C;
    'dispatch: loop {
        match pc {
            0x8270D86C => {
    //   block [0x8270D86C..0x8270D87C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D87C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D87C size=16
    let mut pc: u32 = 0x8270D87C;
    'dispatch: loop {
        match pc {
            0x8270D87C => {
    //   block [0x8270D87C..0x8270D88C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D88C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D88C size=16
    let mut pc: u32 = 0x8270D88C;
    'dispatch: loop {
        match pc {
            0x8270D88C => {
    //   block [0x8270D88C..0x8270D89C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D89C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D89C size=16
    let mut pc: u32 = 0x8270D89C;
    'dispatch: loop {
        match pc {
            0x8270D89C => {
    //   block [0x8270D89C..0x8270D8AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D8AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D8AC size=16
    let mut pc: u32 = 0x8270D8AC;
    'dispatch: loop {
        match pc {
            0x8270D8AC => {
    //   block [0x8270D8AC..0x8270D8BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D8BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D8BC size=16
    let mut pc: u32 = 0x8270D8BC;
    'dispatch: loop {
        match pc {
            0x8270D8BC => {
    //   block [0x8270D8BC..0x8270D8CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D8CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D8CC size=16
    let mut pc: u32 = 0x8270D8CC;
    'dispatch: loop {
        match pc {
            0x8270D8CC => {
    //   block [0x8270D8CC..0x8270D8DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D8DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D8DC size=16
    let mut pc: u32 = 0x8270D8DC;
    'dispatch: loop {
        match pc {
            0x8270D8DC => {
    //   block [0x8270D8DC..0x8270D8EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D8EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D8EC size=16
    let mut pc: u32 = 0x8270D8EC;
    'dispatch: loop {
        match pc {
            0x8270D8EC => {
    //   block [0x8270D8EC..0x8270D8FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D8FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D8FC size=16
    let mut pc: u32 = 0x8270D8FC;
    'dispatch: loop {
        match pc {
            0x8270D8FC => {
    //   block [0x8270D8FC..0x8270D90C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D90C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D90C size=16
    let mut pc: u32 = 0x8270D90C;
    'dispatch: loop {
        match pc {
            0x8270D90C => {
    //   block [0x8270D90C..0x8270D91C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D91C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D91C size=16
    let mut pc: u32 = 0x8270D91C;
    'dispatch: loop {
        match pc {
            0x8270D91C => {
    //   block [0x8270D91C..0x8270D92C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D92C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D92C size=16
    let mut pc: u32 = 0x8270D92C;
    'dispatch: loop {
        match pc {
            0x8270D92C => {
    //   block [0x8270D92C..0x8270D93C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D93C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D93C size=16
    let mut pc: u32 = 0x8270D93C;
    'dispatch: loop {
        match pc {
            0x8270D93C => {
    //   block [0x8270D93C..0x8270D94C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D94C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D94C size=16
    let mut pc: u32 = 0x8270D94C;
    'dispatch: loop {
        match pc {
            0x8270D94C => {
    //   block [0x8270D94C..0x8270D95C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D95C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D95C size=16
    let mut pc: u32 = 0x8270D95C;
    'dispatch: loop {
        match pc {
            0x8270D95C => {
    //   block [0x8270D95C..0x8270D96C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D96C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D96C size=16
    let mut pc: u32 = 0x8270D96C;
    'dispatch: loop {
        match pc {
            0x8270D96C => {
    //   block [0x8270D96C..0x8270D97C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D97C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D97C size=16
    let mut pc: u32 = 0x8270D97C;
    'dispatch: loop {
        match pc {
            0x8270D97C => {
    //   block [0x8270D97C..0x8270D98C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D98C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D98C size=16
    let mut pc: u32 = 0x8270D98C;
    'dispatch: loop {
        match pc {
            0x8270D98C => {
    //   block [0x8270D98C..0x8270D99C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D99C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D99C size=16
    let mut pc: u32 = 0x8270D99C;
    'dispatch: loop {
        match pc {
            0x8270D99C => {
    //   block [0x8270D99C..0x8270D9AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D9AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D9AC size=16
    let mut pc: u32 = 0x8270D9AC;
    'dispatch: loop {
        match pc {
            0x8270D9AC => {
    //   block [0x8270D9AC..0x8270D9BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D9BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D9BC size=16
    let mut pc: u32 = 0x8270D9BC;
    'dispatch: loop {
        match pc {
            0x8270D9BC => {
    //   block [0x8270D9BC..0x8270D9CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D9CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D9CC size=16
    let mut pc: u32 = 0x8270D9CC;
    'dispatch: loop {
        match pc {
            0x8270D9CC => {
    //   block [0x8270D9CC..0x8270D9DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D9DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D9DC size=16
    let mut pc: u32 = 0x8270D9DC;
    'dispatch: loop {
        match pc {
            0x8270D9DC => {
    //   block [0x8270D9DC..0x8270D9EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D9EC size=16
    let mut pc: u32 = 0x8270D9EC;
    'dispatch: loop {
        match pc {
            0x8270D9EC => {
    //   block [0x8270D9EC..0x8270D9FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D9FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D9FC size=16
    let mut pc: u32 = 0x8270D9FC;
    'dispatch: loop {
        match pc {
            0x8270D9FC => {
    //   block [0x8270D9FC..0x8270DA0C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA0C size=16
    let mut pc: u32 = 0x8270DA0C;
    'dispatch: loop {
        match pc {
            0x8270DA0C => {
    //   block [0x8270DA0C..0x8270DA1C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA1C size=16
    let mut pc: u32 = 0x8270DA1C;
    'dispatch: loop {
        match pc {
            0x8270DA1C => {
    //   block [0x8270DA1C..0x8270DA2C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA2C size=16
    let mut pc: u32 = 0x8270DA2C;
    'dispatch: loop {
        match pc {
            0x8270DA2C => {
    //   block [0x8270DA2C..0x8270DA3C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA3C size=16
    let mut pc: u32 = 0x8270DA3C;
    'dispatch: loop {
        match pc {
            0x8270DA3C => {
    //   block [0x8270DA3C..0x8270DA4C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA4C size=16
    let mut pc: u32 = 0x8270DA4C;
    'dispatch: loop {
        match pc {
            0x8270DA4C => {
    //   block [0x8270DA4C..0x8270DA5C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA5C size=16
    let mut pc: u32 = 0x8270DA5C;
    'dispatch: loop {
        match pc {
            0x8270DA5C => {
    //   block [0x8270DA5C..0x8270DA6C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA6C size=16
    let mut pc: u32 = 0x8270DA6C;
    'dispatch: loop {
        match pc {
            0x8270DA6C => {
    //   block [0x8270DA6C..0x8270DA7C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA7C size=16
    let mut pc: u32 = 0x8270DA7C;
    'dispatch: loop {
        match pc {
            0x8270DA7C => {
    //   block [0x8270DA7C..0x8270DA8C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA8C size=16
    let mut pc: u32 = 0x8270DA8C;
    'dispatch: loop {
        match pc {
            0x8270DA8C => {
    //   block [0x8270DA8C..0x8270DA9C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DA9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DA9C size=16
    let mut pc: u32 = 0x8270DA9C;
    'dispatch: loop {
        match pc {
            0x8270DA9C => {
    //   block [0x8270DA9C..0x8270DAAC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DAAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DAAC size=16
    let mut pc: u32 = 0x8270DAAC;
    'dispatch: loop {
        match pc {
            0x8270DAAC => {
    //   block [0x8270DAAC..0x8270DABC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DABC size=16
    let mut pc: u32 = 0x8270DABC;
    'dispatch: loop {
        match pc {
            0x8270DABC => {
    //   block [0x8270DABC..0x8270DACC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DACC size=16
    let mut pc: u32 = 0x8270DACC;
    'dispatch: loop {
        match pc {
            0x8270DACC => {
    //   block [0x8270DACC..0x8270DADC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DADC size=16
    let mut pc: u32 = 0x8270DADC;
    'dispatch: loop {
        match pc {
            0x8270DADC => {
    //   block [0x8270DADC..0x8270DAEC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DAEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DAEC size=16
    let mut pc: u32 = 0x8270DAEC;
    'dispatch: loop {
        match pc {
            0x8270DAEC => {
    //   block [0x8270DAEC..0x8270DAFC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DAFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DAFC size=16
    let mut pc: u32 = 0x8270DAFC;
    'dispatch: loop {
        match pc {
            0x8270DAFC => {
    //   block [0x8270DAFC..0x8270DB0C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DB0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DB0C size=16
    let mut pc: u32 = 0x8270DB0C;
    'dispatch: loop {
        match pc {
            0x8270DB0C => {
    //   block [0x8270DB0C..0x8270DB1C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DB1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DB1C size=16
    let mut pc: u32 = 0x8270DB1C;
    'dispatch: loop {
        match pc {
            0x8270DB1C => {
    //   block [0x8270DB1C..0x8270DB2C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DB2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DB2C size=16
    let mut pc: u32 = 0x8270DB2C;
    'dispatch: loop {
        match pc {
            0x8270DB2C => {
    //   block [0x8270DB2C..0x8270DB3C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270DB3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270DB3C size=16
    let mut pc: u32 = 0x8270DB3C;
    'dispatch: loop {
        match pc {
            0x8270DB3C => {
    //   block [0x8270DB3C..0x8270DB4C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


