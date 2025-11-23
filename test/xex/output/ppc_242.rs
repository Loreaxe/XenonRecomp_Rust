pub fn sub_832B6648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6648 size=12
    let mut pc: u32 = 0x832B6648;
    'dispatch: loop {
        match pc {
            0x832B6648 => {
    //   block [0x832B6648..0x832B6654)
	// 832B6648: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B664C: 386B5E78  addi r3, r11, 0x5e78
	ctx.r[3].s64 = ctx.r[11].s64 + 24184;
	// 832B6650: 4AF5E788  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6658 size=12
    let mut pc: u32 = 0x832B6658;
    'dispatch: loop {
        match pc {
            0x832B6658 => {
    //   block [0x832B6658..0x832B6664)
	// 832B6658: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B665C: 386B5E7C  addi r3, r11, 0x5e7c
	ctx.r[3].s64 = ctx.r[11].s64 + 24188;
	// 832B6660: 4AF10198  b 0x821c67f8
	sub_821C67F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6668 size=12
    let mut pc: u32 = 0x832B6668;
    'dispatch: loop {
        match pc {
            0x832B6668 => {
    //   block [0x832B6668..0x832B6674)
	// 832B6668: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B666C: 386B5E80  addi r3, r11, 0x5e80
	ctx.r[3].s64 = ctx.r[11].s64 + 24192;
	// 832B6670: 4AF5E768  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6678 size=12
    let mut pc: u32 = 0x832B6678;
    'dispatch: loop {
        match pc {
            0x832B6678 => {
    //   block [0x832B6678..0x832B6684)
	// 832B6678: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B667C: 386B5E84  addi r3, r11, 0x5e84
	ctx.r[3].s64 = ctx.r[11].s64 + 24196;
	// 832B6680: 4B88D6E8  b 0x82b43d68
	sub_82B43D68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6688 size=4
    let mut pc: u32 = 0x832B6688;
    'dispatch: loop {
        match pc {
            0x832B6688 => {
    //   block [0x832B6688..0x832B668C)
	// 832B6688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6690 size=32
    let mut pc: u32 = 0x832B6690;
    'dispatch: loop {
        match pc {
            0x832B6690 => {
    //   block [0x832B6690..0x832B66B0)
	// 832B6690: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6694: 806B19C0  lwz r3, 0x19c0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6592 as u32) ) } as u64;
	// 832B6698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B669C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B66A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B66A4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B66A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B66AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B66B0 size=4
    let mut pc: u32 = 0x832B66B0;
    'dispatch: loop {
        match pc {
            0x832B66B0 => {
    //   block [0x832B66B0..0x832B66B4)
	// 832B66B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B66B8 size=60
    let mut pc: u32 = 0x832B66B8;
    'dispatch: loop {
        match pc {
            0x832B66B8 => {
    //   block [0x832B66B8..0x832B66F4)
	// 832B66B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B66BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B66C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B66C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B66C8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B66CC: 3BEB3880  addi r31, r11, 0x3880
	ctx.r[31].s64 = ctx.r[11].s64 + 14464;
	// 832B66D0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 832B66D4: 4B041625  bl 0x822f7cf8
	ctx.lr = 0x832B66D8;
	sub_822F7CF8(ctx, base);
	// 832B66D8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832B66DC: 4B05D0C5  bl 0x823137a0
	ctx.lr = 0x832B66E0;
	sub_823137A0(ctx, base);
	// 832B66E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B66E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B66E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B66EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B66F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B66F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B66F8 size=12
    let mut pc: u32 = 0x832B66F8;
    'dispatch: loop {
        match pc {
            0x832B66F8 => {
    //   block [0x832B66F8..0x832B6704)
	// 832B66F8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B66FC: 386B5E9C  addi r3, r11, 0x5e9c
	ctx.r[3].s64 = ctx.r[11].s64 + 24220;
	// 832B6700: 4B8AAFA8  b 0x82b616a8
	sub_82B616A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6708 size=12
    let mut pc: u32 = 0x832B6708;
    'dispatch: loop {
        match pc {
            0x832B6708 => {
    //   block [0x832B6708..0x832B6714)
	// 832B6708: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B670C: 386B5F20  addi r3, r11, 0x5f20
	ctx.r[3].s64 = ctx.r[11].s64 + 24352;
	// 832B6710: 4B1088F8  b 0x823bf008
	sub_823BF008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6718 size=4
    let mut pc: u32 = 0x832B6718;
    'dispatch: loop {
        match pc {
            0x832B6718 => {
    //   block [0x832B6718..0x832B671C)
	// 832B6718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6720 size=12
    let mut pc: u32 = 0x832B6720;
    'dispatch: loop {
        match pc {
            0x832B6720 => {
    //   block [0x832B6720..0x832B672C)
	// 832B6720: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6724: 386B5F48  addi r3, r11, 0x5f48
	ctx.r[3].s64 = ctx.r[11].s64 + 24392;
	// 832B6728: 4B8AC8A8  b 0x82b62fd0
	sub_82B62FD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6730 size=12
    let mut pc: u32 = 0x832B6730;
    'dispatch: loop {
        match pc {
            0x832B6730 => {
    //   block [0x832B6730..0x832B673C)
	// 832B6730: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6734: 386B5F54  addi r3, r11, 0x5f54
	ctx.r[3].s64 = ctx.r[11].s64 + 24404;
	// 832B6738: 4B8ACFE0  b 0x82b63718
	sub_82B63718(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6740 size=12
    let mut pc: u32 = 0x832B6740;
    'dispatch: loop {
        match pc {
            0x832B6740 => {
    //   block [0x832B6740..0x832B674C)
	// 832B6740: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6744: 386BB088  addi r3, r11, -0x4f78
	ctx.r[3].s64 = ctx.r[11].s64 + -20344;
	// 832B6748: 4B76D068  b 0x82a237b0
	sub_82A237B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6750 size=12
    let mut pc: u32 = 0x832B6750;
    'dispatch: loop {
        match pc {
            0x832B6750 => {
    //   block [0x832B6750..0x832B675C)
	// 832B6750: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6754: 386B5F60  addi r3, r11, 0x5f60
	ctx.r[3].s64 = ctx.r[11].s64 + 24416;
	// 832B6758: 4B8AD270  b 0x82b639c8
	sub_82B639C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6760 size=12
    let mut pc: u32 = 0x832B6760;
    'dispatch: loop {
        match pc {
            0x832B6760 => {
    //   block [0x832B6760..0x832B676C)
	// 832B6760: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6764: 386B5F6C  addi r3, r11, 0x5f6c
	ctx.r[3].s64 = ctx.r[11].s64 + 24428;
	// 832B6768: 4AF5E670  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6770 size=92
    let mut pc: u32 = 0x832B6770;
    'dispatch: loop {
        match pc {
            0x832B6770 => {
    //   block [0x832B6770..0x832B67CC)
	// 832B6770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B677C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6784: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B6788: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B678C: 3BDFB0BC  addi r30, r31, -0x4f44
	ctx.r[30].s64 = ctx.r[31].s64 + -20292;
	// 832B6790: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B6794: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6798: 917FB0BC  stw r11, -0x4f44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20292 as u32), ctx.r[11].u32 ) };
	// 832B679C: 4AF4586D  bl 0x821fc008
	ctx.lr = 0x832B67A0;
	sub_821FC008(ctx, base);
	// 832B67A0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B67A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B67A8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B67AC: 917FB0BC  stw r11, -0x4f44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20292 as u32), ctx.r[11].u32 ) };
	// 832B67B0: 4AF45859  bl 0x821fc008
	ctx.lr = 0x832B67B4;
	sub_821FC008(ctx, base);
	// 832B67B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B67B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B67BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B67C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B67C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B67C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B67D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B67D0 size=92
    let mut pc: u32 = 0x832B67D0;
    'dispatch: loop {
        match pc {
            0x832B67D0 => {
    //   block [0x832B67D0..0x832B682C)
	// 832B67D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B67D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B67D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B67DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B67E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B67E4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B67E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832B67EC: 3BDFB0C4  addi r30, r31, -0x4f3c
	ctx.r[30].s64 = ctx.r[31].s64 + -20284;
	// 832B67F0: 396B1460  addi r11, r11, 0x1460
	ctx.r[11].s64 = ctx.r[11].s64 + 5216;
	// 832B67F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B67F8: 917FB0C4  stw r11, -0x4f3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20284 as u32), ctx.r[11].u32 ) };
	// 832B67FC: 4AF4580D  bl 0x821fc008
	ctx.lr = 0x832B6800;
	sub_821FC008(ctx, base);
	// 832B6800: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B6804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6808: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B680C: 917FB0C4  stw r11, -0x4f3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20284 as u32), ctx.r[11].u32 ) };
	// 832B6810: 4AF457F9  bl 0x821fc008
	ctx.lr = 0x832B6814;
	sub_821FC008(ctx, base);
	// 832B6814: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B681C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6820: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6824: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6830 size=92
    let mut pc: u32 = 0x832B6830;
    'dispatch: loop {
        match pc {
            0x832B6830 => {
    //   block [0x832B6830..0x832B688C)
	// 832B6830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6838: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B683C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6844: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B6848: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832B684C: 3BDFB0CC  addi r30, r31, -0x4f34
	ctx.r[30].s64 = ctx.r[31].s64 + -20276;
	// 832B6850: 396BADE4  addi r11, r11, -0x521c
	ctx.r[11].s64 = ctx.r[11].s64 + -21020;
	// 832B6854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6858: 917FB0CC  stw r11, -0x4f34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20276 as u32), ctx.r[11].u32 ) };
	// 832B685C: 4AF457AD  bl 0x821fc008
	ctx.lr = 0x832B6860;
	sub_821FC008(ctx, base);
	// 832B6860: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B6864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6868: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B686C: 917FB0CC  stw r11, -0x4f34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20276 as u32), ctx.r[11].u32 ) };
	// 832B6870: 4AF45799  bl 0x821fc008
	ctx.lr = 0x832B6874;
	sub_821FC008(ctx, base);
	// 832B6874: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B687C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6880: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6884: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6890 size=12
    let mut pc: u32 = 0x832B6890;
    'dispatch: loop {
        match pc {
            0x832B6890 => {
    //   block [0x832B6890..0x832B689C)
	// 832B6890: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6894: 386B5F74  addi r3, r11, 0x5f74
	ctx.r[3].s64 = ctx.r[11].s64 + 24436;
	// 832B6898: 4B8B4150  b 0x82b6a9e8
	sub_82B6A9E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68A0 size=4
    let mut pc: u32 = 0x832B68A0;
    'dispatch: loop {
        match pc {
            0x832B68A0 => {
    //   block [0x832B68A0..0x832B68A4)
	// 832B68A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68A8 size=12
    let mut pc: u32 = 0x832B68A8;
    'dispatch: loop {
        match pc {
            0x832B68A8 => {
    //   block [0x832B68A8..0x832B68B4)
	// 832B68A8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B68AC: 386BB0D4  addi r3, r11, -0x4f2c
	ctx.r[3].s64 = ctx.r[11].s64 + -20268;
	// 832B68B0: 4B934D88  b 0x82beb638
	sub_82BEB638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68B8 size=12
    let mut pc: u32 = 0x832B68B8;
    'dispatch: loop {
        match pc {
            0x832B68B8 => {
    //   block [0x832B68B8..0x832B68C4)
	// 832B68B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68BC: 386B5F98  addi r3, r11, 0x5f98
	ctx.r[3].s64 = ctx.r[11].s64 + 24472;
	// 832B68C0: 4B8B7650  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68C8 size=12
    let mut pc: u32 = 0x832B68C8;
    'dispatch: loop {
        match pc {
            0x832B68C8 => {
    //   block [0x832B68C8..0x832B68D4)
	// 832B68C8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68CC: 386B5FA4  addi r3, r11, 0x5fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 24484;
	// 832B68D0: 4B8B7640  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68D8 size=12
    let mut pc: u32 = 0x832B68D8;
    'dispatch: loop {
        match pc {
            0x832B68D8 => {
    //   block [0x832B68D8..0x832B68E4)
	// 832B68D8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68DC: 386B5FB0  addi r3, r11, 0x5fb0
	ctx.r[3].s64 = ctx.r[11].s64 + 24496;
	// 832B68E0: 4B8B7630  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B68E8 size=12
    let mut pc: u32 = 0x832B68E8;
    'dispatch: loop {
        match pc {
            0x832B68E8 => {
    //   block [0x832B68E8..0x832B68F4)
	// 832B68E8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B68EC: 386B5FBC  addi r3, r11, 0x5fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 24508;
	// 832B68F0: 4B8B7620  b 0x82b6df10
	sub_82B6DF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B68F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B68F8 size=88
    let mut pc: u32 = 0x832B68F8;
    'dispatch: loop {
        match pc {
            0x832B68F8 => {
    //   block [0x832B68F8..0x832B6950)
	// 832B68F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B68FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B6904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B690C: 3FC08332  lis r30, -0x7cce
	ctx.r[30].s64 = -2093875200;
	// 832B6910: 3BFEB0E8  addi r31, r30, -0x4f18
	ctx.r[31].s64 = ctx.r[30].s64 + -20248;
	// 832B6914: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832B6918: 4B8AABF9  bl 0x82b61510
	ctx.lr = 0x832B691C;
	sub_82B61510(ctx, base);
	// 832B691C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832B6920: 4AF65419  bl 0x8221bd38
	ctx.lr = 0x832B6924;
	sub_8221BD38(ctx, base);
	// 832B6924: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832B6928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B692C: 394A0FD0  addi r10, r10, 0xfd0
	ctx.r[10].s64 = ctx.r[10].s64 + 4048;
	// 832B6930: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832B6934: 915EB0E8  stw r10, -0x4f18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-20248 as u32), ctx.r[10].u32 ) };
	// 832B6938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B693C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B694C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6950 size=4
    let mut pc: u32 = 0x832B6950;
    'dispatch: loop {
        match pc {
            0x832B6950 => {
    //   block [0x832B6950..0x832B6954)
	// 832B6950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6958 size=4
    let mut pc: u32 = 0x832B6958;
    'dispatch: loop {
        match pc {
            0x832B6958 => {
    //   block [0x832B6958..0x832B695C)
	// 832B6958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6960 size=4
    let mut pc: u32 = 0x832B6960;
    'dispatch: loop {
        match pc {
            0x832B6960 => {
    //   block [0x832B6960..0x832B6964)
	// 832B6960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6968 size=12
    let mut pc: u32 = 0x832B6968;
    'dispatch: loop {
        match pc {
            0x832B6968 => {
    //   block [0x832B6968..0x832B6974)
	// 832B6968: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B696C: 386B6040  addi r3, r11, 0x6040
	ctx.r[3].s64 = ctx.r[11].s64 + 24640;
	// 832B6970: 4B51C600  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6978 size=12
    let mut pc: u32 = 0x832B6978;
    'dispatch: loop {
        match pc {
            0x832B6978 => {
    //   block [0x832B6978..0x832B6984)
	// 832B6978: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B697C: 386B604C  addi r3, r11, 0x604c
	ctx.r[3].s64 = ctx.r[11].s64 + 24652;
	// 832B6980: 4B51C5F0  b 0x827d2f70
	sub_827D2F70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6988 size=12
    let mut pc: u32 = 0x832B6988;
    'dispatch: loop {
        match pc {
            0x832B6988 => {
    //   block [0x832B6988..0x832B6994)
	// 832B6988: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B698C: 386B6058  addi r3, r11, 0x6058
	ctx.r[3].s64 = ctx.r[11].s64 + 24664;
	// 832B6990: 4B8B93D0  b 0x82b6fd60
	sub_82B6FD60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6998 size=4
    let mut pc: u32 = 0x832B6998;
    'dispatch: loop {
        match pc {
            0x832B6998 => {
    //   block [0x832B6998..0x832B699C)
	// 832B6998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B69A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B69A0 size=88
    let mut pc: u32 = 0x832B69A0;
    'dispatch: loop {
        match pc {
            0x832B69A0 => {
    //   block [0x832B69A0..0x832B69C8)
	// 832B69A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B69A4: 4B9F2A65  bl 0x82ca9408
	ctx.lr = 0x832B69A8;
	sub_82CA93D0(ctx, base);
	// 832B69A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B69AC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B69B0: 3B800003  li r28, 3
	ctx.r[28].s64 = 3;
	// 832B69B4: 396B6088  addi r11, r11, 0x6088
	ctx.r[11].s64 = ctx.r[11].s64 + 24712;
	// 832B69B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832B69BC: 3BEB0034  addi r31, r11, 0x34
	ctx.r[31].s64 = ctx.r[11].s64 + 52;
	// 832B69C0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B69C4: 3BAB2A2C  addi r29, r11, 0x2a2c
	ctx.r[29].s64 = ctx.r[11].s64 + 10796;
	pc = 0x832B69C8; continue 'dispatch;
            }
            0x832B69C8 => {
    //   block [0x832B69C8..0x832B69E8)
	// 832B69C8: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 832B69CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B69D0: 93BFFFFC  stw r29, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[29].u32 ) };
	// 832B69D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B69D8: 419A0010  beq cr6, 0x832b69e8
	if ctx.cr[6].eq {
	pc = 0x832B69E8; continue 'dispatch;
	}
	// 832B69DC: 4AF45945  bl 0x821fc320
	ctx.lr = 0x832B69E0;
	sub_821FC320(ctx, base);
	// 832B69E0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832B69E4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	pc = 0x832B69E8; continue 'dispatch;
            }
            0x832B69E8 => {
    //   block [0x832B69E8..0x832B69F8)
	// 832B69E8: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832B69EC: 4080FFDC  bge 0x832b69c8
	if !ctx.cr[0].lt {
	pc = 0x832B69C8; continue 'dispatch;
	}
	// 832B69F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B69F4: 4B9F2A64  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B69F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B69F8 size=88
    let mut pc: u32 = 0x832B69F8;
    'dispatch: loop {
        match pc {
            0x832B69F8 => {
    //   block [0x832B69F8..0x832B6A3C)
	// 832B69F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B69FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6A08: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6A0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B6A10: 3BEBB12C  addi r31, r11, -0x4ed4
	ctx.r[31].s64 = ctx.r[11].s64 + -20180;
	// 832B6A14: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B6A18: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6A1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B6A20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6A24: 419A0018  beq cr6, 0x832b6a3c
	if ctx.cr[6].eq {
	pc = 0x832B6A3C; continue 'dispatch;
	}
	// 832B6A28: 4AF458F9  bl 0x821fc320
	ctx.lr = 0x832B6A2C;
	sub_821FC320(ctx, base);
	// 832B6A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6A34: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6A38: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x832B6A3C; continue 'dispatch;
            }
            0x832B6A3C => {
    //   block [0x832B6A3C..0x832B6A50)
	// 832B6A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6A50 size=80
    let mut pc: u32 = 0x832B6A50;
    'dispatch: loop {
        match pc {
            0x832B6A50 => {
    //   block [0x832B6A50..0x832B6A74)
	// 832B6A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6A58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B6A5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6A60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6A64: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6A68: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 832B6A6C: 396B60C8  addi r11, r11, 0x60c8
	ctx.r[11].s64 = ctx.r[11].s64 + 24776;
	// 832B6A70: 3BEB0260  addi r31, r11, 0x260
	ctx.r[31].s64 = ctx.r[11].s64 + 608;
	pc = 0x832B6A74; continue 'dispatch;
            }
            0x832B6A74 => {
    //   block [0x832B6A74..0x832B6AA0)
	// 832B6A74: 3BFFFFB4  addi r31, r31, -0x4c
	ctx.r[31].s64 = ctx.r[31].s64 + -76;
	// 832B6A78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B6A7C: 4B8B7F15  bl 0x82b6e990
	ctx.lr = 0x832B6A80;
	sub_82B6E990(ctx, base);
	// 832B6A80: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B6A84: 4080FFF0  bge 0x832b6a74
	if !ctx.cr[0].lt {
	pc = 0x832B6A74; continue 'dispatch;
	}
	// 832B6A88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6A94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6A98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6AA0 size=92
    let mut pc: u32 = 0x832B6AA0;
    'dispatch: loop {
        match pc {
            0x832B6AA0 => {
    //   block [0x832B6AA0..0x832B6AFC)
	// 832B6AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6AA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B6AAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6AB4: 3FE08332  lis r31, -0x7cce
	ctx.r[31].s64 = -2093875200;
	// 832B6AB8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B6ABC: 3BDFB138  addi r30, r31, -0x4ec8
	ctx.r[30].s64 = ctx.r[31].s64 + -20168;
	// 832B6AC0: 396B2A30  addi r11, r11, 0x2a30
	ctx.r[11].s64 = ctx.r[11].s64 + 10800;
	// 832B6AC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6AC8: 917FB138  stw r11, -0x4ec8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20168 as u32), ctx.r[11].u32 ) };
	// 832B6ACC: 4AF4553D  bl 0x821fc008
	ctx.lr = 0x832B6AD0;
	sub_821FC008(ctx, base);
	// 832B6AD0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B6AD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B6AD8: 396B2A40  addi r11, r11, 0x2a40
	ctx.r[11].s64 = ctx.r[11].s64 + 10816;
	// 832B6ADC: 917FB138  stw r11, -0x4ec8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-20168 as u32), ctx.r[11].u32 ) };
	// 832B6AE0: 4AF45529  bl 0x821fc008
	ctx.lr = 0x832B6AE4;
	sub_821FC008(ctx, base);
	// 832B6AE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6AF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6AF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6B00 size=88
    let mut pc: u32 = 0x832B6B00;
    'dispatch: loop {
        match pc {
            0x832B6B00 => {
    //   block [0x832B6B00..0x832B6B44)
	// 832B6B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6B08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6B0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6B10: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6B14: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B6B18: 3BEBB140  addi r31, r11, -0x4ec0
	ctx.r[31].s64 = ctx.r[11].s64 + -20160;
	// 832B6B1C: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B6B20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6B24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B6B28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6B2C: 419A0018  beq cr6, 0x832b6b44
	if ctx.cr[6].eq {
	pc = 0x832B6B44; continue 'dispatch;
	}
	// 832B6B30: 4AF457F1  bl 0x821fc320
	ctx.lr = 0x832B6B34;
	sub_821FC320(ctx, base);
	// 832B6B34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6B3C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6B40: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x832B6B44; continue 'dispatch;
            }
            0x832B6B44 => {
    //   block [0x832B6B44..0x832B6B58)
	// 832B6B44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6B50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6B58 size=88
    let mut pc: u32 = 0x832B6B58;
    'dispatch: loop {
        match pc {
            0x832B6B58 => {
    //   block [0x832B6B58..0x832B6B9C)
	// 832B6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6B60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6B64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6B68: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6B6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832B6B70: 3BEBB14C  addi r31, r11, -0x4eb4
	ctx.r[31].s64 = ctx.r[11].s64 + -20148;
	// 832B6B74: 396A2A2C  addi r11, r10, 0x2a2c
	ctx.r[11].s64 = ctx.r[10].s64 + 10796;
	// 832B6B78: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6B7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B6B80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6B84: 419A0018  beq cr6, 0x832b6b9c
	if ctx.cr[6].eq {
	pc = 0x832B6B9C; continue 'dispatch;
	}
	// 832B6B88: 4AF45799  bl 0x821fc320
	ctx.lr = 0x832B6B8C;
	sub_821FC320(ctx, base);
	// 832B6B8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6B94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6B98: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	pc = 0x832B6B9C; continue 'dispatch;
            }
            0x832B6B9C => {
    //   block [0x832B6B9C..0x832B6BB0)
	// 832B6B9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6BA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6BA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6BA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6BB0 size=12
    let mut pc: u32 = 0x832B6BB0;
    'dispatch: loop {
        match pc {
            0x832B6BB0 => {
    //   block [0x832B6BB0..0x832B6BBC)
	// 832B6BB0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6BB4: 386B6328  addi r3, r11, 0x6328
	ctx.r[3].s64 = ctx.r[11].s64 + 25384;
	// 832B6BB8: 4AF7B100  b 0x82231cb8
	sub_82231CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6BC0 size=12
    let mut pc: u32 = 0x832B6BC0;
    'dispatch: loop {
        match pc {
            0x832B6BC0 => {
    //   block [0x832B6BC0..0x832B6BCC)
	// 832B6BC0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6BC4: 386B632C  addi r3, r11, 0x632c
	ctx.r[3].s64 = ctx.r[11].s64 + 25388;
	// 832B6BC8: 4B8BB0F8  b 0x82b71cc0
	sub_82B71CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6BD0 size=4
    let mut pc: u32 = 0x832B6BD0;
    'dispatch: loop {
        match pc {
            0x832B6BD0 => {
    //   block [0x832B6BD0..0x832B6BD4)
	// 832B6BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6BD8 size=12
    let mut pc: u32 = 0x832B6BD8;
    'dispatch: loop {
        match pc {
            0x832B6BD8 => {
    //   block [0x832B6BD8..0x832B6BE4)
	// 832B6BD8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832B6BDC: 386B6354  addi r3, r11, 0x6354
	ctx.r[3].s64 = ctx.r[11].s64 + 25428;
	// 832B6BE0: 4B8BCC98  b 0x82b73878
	sub_82B73878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6BE8 size=12
    let mut pc: u32 = 0x832B6BE8;
    'dispatch: loop {
        match pc {
            0x832B6BE8 => {
    //   block [0x832B6BE8..0x832B6BF4)
	// 832B6BE8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6BEC: 386BDE90  addi r3, r11, -0x2170
	ctx.r[3].s64 = ctx.r[11].s64 + -8560;
	// 832B6BF0: 4B1C9388  b 0x8247ff78
	sub_8247FF78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6BF8 size=12
    let mut pc: u32 = 0x832B6BF8;
    'dispatch: loop {
        match pc {
            0x832B6BF8 => {
    //   block [0x832B6BF8..0x832B6C04)
	// 832B6BF8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6BFC: 386BDE9C  addi r3, r11, -0x2164
	ctx.r[3].s64 = ctx.r[11].s64 + -8548;
	// 832B6C00: 4B8CE058  b 0x82b84c58
	sub_82B84C58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6C08 size=12
    let mut pc: u32 = 0x832B6C08;
    'dispatch: loop {
        match pc {
            0x832B6C08 => {
    //   block [0x832B6C08..0x832B6C14)
	// 832B6C08: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6C0C: 386BDEAC  addi r3, r11, -0x2154
	ctx.r[3].s64 = ctx.r[11].s64 + -8532;
	// 832B6C10: 4B0B3E80  b 0x8236aa90
	sub_8236AA90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6C18 size=12
    let mut pc: u32 = 0x832B6C18;
    'dispatch: loop {
        match pc {
            0x832B6C18 => {
    //   block [0x832B6C18..0x832B6C24)
	// 832B6C18: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6C1C: 386BDEB8  addi r3, r11, -0x2148
	ctx.r[3].s64 = ctx.r[11].s64 + -8520;
	// 832B6C20: 4B0B3E70  b 0x8236aa90
	sub_8236AA90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6C28 size=12
    let mut pc: u32 = 0x832B6C28;
    'dispatch: loop {
        match pc {
            0x832B6C28 => {
    //   block [0x832B6C28..0x832B6C34)
	// 832B6C28: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6C2C: 386BDEC4  addi r3, r11, -0x213c
	ctx.r[3].s64 = ctx.r[11].s64 + -8508;
	// 832B6C30: 4B8CE1D8  b 0x82b84e08
	sub_82B84E08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6C38 size=84
    let mut pc: u32 = 0x832B6C38;
    'dispatch: loop {
        match pc {
            0x832B6C38 => {
    //   block [0x832B6C38..0x832B6C60)
	// 832B6C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6C48: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6C4C: 3BEBDED4  addi r31, r11, -0x212c
	ctx.r[31].s64 = ctx.r[11].s64 + -8492;
	// 832B6C50: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6C58: 419A0008  beq cr6, 0x832b6c60
	if ctx.cr[6].eq {
	pc = 0x832B6C60; continue 'dispatch;
	}
	// 832B6C5C: 4AF650DD  bl 0x8221bd38
	ctx.lr = 0x832B6C60;
	sub_8221BD38(ctx, base);
	pc = 0x832B6C60; continue 'dispatch;
            }
            0x832B6C60 => {
    //   block [0x832B6C60..0x832B6C8C)
	// 832B6C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6C64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6C68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6C6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6C70: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6C74: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6CA0 size=84
    let mut pc: u32 = 0x832B6CA0;
    'dispatch: loop {
        match pc {
            0x832B6CA0 => {
    //   block [0x832B6CA0..0x832B6CC8)
	// 832B6CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6CA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6CAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6CB0: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6CB4: 3BEBDEE8  addi r31, r11, -0x2118
	ctx.r[31].s64 = ctx.r[11].s64 + -8472;
	// 832B6CB8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6CBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6CC0: 419A0008  beq cr6, 0x832b6cc8
	if ctx.cr[6].eq {
	pc = 0x832B6CC8; continue 'dispatch;
	}
	// 832B6CC4: 4AF65075  bl 0x8221bd38
	ctx.lr = 0x832B6CC8;
	sub_8221BD38(ctx, base);
	pc = 0x832B6CC8; continue 'dispatch;
            }
            0x832B6CC8 => {
    //   block [0x832B6CC8..0x832B6CF4)
	// 832B6CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6CCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6CD0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6CD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6CD8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6CDC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6CF8 size=48
    let mut pc: u32 = 0x832B6CF8;
    'dispatch: loop {
        match pc {
            0x832B6CF8 => {
    //   block [0x832B6CF8..0x832B6D28)
	// 832B6CF8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6CFC: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 832B6D00: 396BB16C  addi r11, r11, -0x4e94
	ctx.r[11].s64 = ctx.r[11].s64 + -20116;
	// 832B6D04: 392A9BF8  addi r9, r10, -0x6408
	ctx.r[9].s64 = ctx.r[10].s64 + -25608;
	// 832B6D08: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6D0C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B6D10: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 832B6D14: 419A0014  beq cr6, 0x832b6d28
	if ctx.cr[6].eq {
		sub_832B6D28(ctx, base);
		return;
	}
	// 832B6D18: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 832B6D1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 832B6D20: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832B6D24: 4B8A3BE4  b 0x82b5a908
	sub_82B5A908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6D28 size=8
    let mut pc: u32 = 0x832B6D28;
    'dispatch: loop {
        match pc {
            0x832B6D28 => {
    //   block [0x832B6D28..0x832B6D30)
	// 832B6D28: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832B6D2C: 4B8A0FB4  b 0x82b57ce0
	sub_82B57CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6D30 size=4
    let mut pc: u32 = 0x832B6D30;
    'dispatch: loop {
        match pc {
            0x832B6D30 => {
    //   block [0x832B6D30..0x832B6D34)
	// 832B6D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6D38 size=84
    let mut pc: u32 = 0x832B6D38;
    'dispatch: loop {
        match pc {
            0x832B6D38 => {
    //   block [0x832B6D38..0x832B6D60)
	// 832B6D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6D48: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6D4C: 3BEBDEF8  addi r31, r11, -0x2108
	ctx.r[31].s64 = ctx.r[11].s64 + -8456;
	// 832B6D50: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6D58: 419A0008  beq cr6, 0x832b6d60
	if ctx.cr[6].eq {
	pc = 0x832B6D60; continue 'dispatch;
	}
	// 832B6D5C: 4AF64FDD  bl 0x8221bd38
	ctx.lr = 0x832B6D60;
	sub_8221BD38(ctx, base);
	pc = 0x832B6D60; continue 'dispatch;
            }
            0x832B6D60 => {
    //   block [0x832B6D60..0x832B6D8C)
	// 832B6D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6D68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6D6C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6D70: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6D74: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6D78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6D90 size=84
    let mut pc: u32 = 0x832B6D90;
    'dispatch: loop {
        match pc {
            0x832B6D90 => {
    //   block [0x832B6D90..0x832B6DB8)
	// 832B6D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6DA0: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6DA4: 3BEBDF08  addi r31, r11, -0x20f8
	ctx.r[31].s64 = ctx.r[11].s64 + -8440;
	// 832B6DA8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6DAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6DB0: 419A0008  beq cr6, 0x832b6db8
	if ctx.cr[6].eq {
	pc = 0x832B6DB8; continue 'dispatch;
	}
	// 832B6DB4: 4AF64F85  bl 0x8221bd38
	ctx.lr = 0x832B6DB8;
	sub_8221BD38(ctx, base);
	pc = 0x832B6DB8; continue 'dispatch;
            }
            0x832B6DB8 => {
    //   block [0x832B6DB8..0x832B6DE4)
	// 832B6DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6DBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6DC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6DC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6DC8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6DCC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6DDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6DE8 size=84
    let mut pc: u32 = 0x832B6DE8;
    'dispatch: loop {
        match pc {
            0x832B6DE8 => {
    //   block [0x832B6DE8..0x832B6E10)
	// 832B6DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6DF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6DF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6DF8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6DFC: 3BEBDF18  addi r31, r11, -0x20e8
	ctx.r[31].s64 = ctx.r[11].s64 + -8424;
	// 832B6E00: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6E04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6E08: 419A0008  beq cr6, 0x832b6e10
	if ctx.cr[6].eq {
	pc = 0x832B6E10; continue 'dispatch;
	}
	// 832B6E0C: 4AF64F2D  bl 0x8221bd38
	ctx.lr = 0x832B6E10;
	sub_8221BD38(ctx, base);
	pc = 0x832B6E10; continue 'dispatch;
            }
            0x832B6E10 => {
    //   block [0x832B6E10..0x832B6E3C)
	// 832B6E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6E18: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6E1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6E20: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6E24: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6E40 size=84
    let mut pc: u32 = 0x832B6E40;
    'dispatch: loop {
        match pc {
            0x832B6E40 => {
    //   block [0x832B6E40..0x832B6E68)
	// 832B6E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6E4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6E50: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6E54: 3BEBDF28  addi r31, r11, -0x20d8
	ctx.r[31].s64 = ctx.r[11].s64 + -8408;
	// 832B6E58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6E5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6E60: 419A0008  beq cr6, 0x832b6e68
	if ctx.cr[6].eq {
	pc = 0x832B6E68; continue 'dispatch;
	}
	// 832B6E64: 4AF64ED5  bl 0x8221bd38
	ctx.lr = 0x832B6E68;
	sub_8221BD38(ctx, base);
	pc = 0x832B6E68; continue 'dispatch;
            }
            0x832B6E68 => {
    //   block [0x832B6E68..0x832B6E94)
	// 832B6E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B6E6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B6E70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B6E74: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B6E78: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B6E7C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B6E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B6E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6EA8 size=88
    let mut pc: u32 = 0x832B6EA8;
    'dispatch: loop {
        match pc {
            0x832B6EA8 => {
    //   block [0x832B6EA8..0x832B6ECC)
	// 832B6EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B6EB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B6EB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B6EB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6EBC: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6EC0: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 832B6EC4: 396BDF40  addi r11, r11, -0x20c0
	ctx.r[11].s64 = ctx.r[11].s64 + -8384;
	// 832B6EC8: 3BEB00C8  addi r31, r11, 0xc8
	ctx.r[31].s64 = ctx.r[11].s64 + 200;
	pc = 0x832B6ECC; continue 'dispatch;
            }
            0x832B6ECC => {
    //   block [0x832B6ECC..0x832B6EE0)
	// 832B6ECC: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832B6ED0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B6ED4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B6ED8: 419A0008  beq cr6, 0x832b6ee0
	if ctx.cr[6].eq {
	pc = 0x832B6EE0; continue 'dispatch;
	}
	// 832B6EDC: 4AF45445  bl 0x821fc320
	ctx.lr = 0x832B6EE0;
	sub_821FC320(ctx, base);
	pc = 0x832B6EE0; continue 'dispatch;
            }
            0x832B6EE0 => {
    //   block [0x832B6EE0..0x832B6F00)
	// 832B6EE0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B6EE4: 4080FFE8  bge 0x832b6ecc
	if !ctx.cr[0].lt {
	pc = 0x832B6ECC; continue 'dispatch;
	}
	// 832B6EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B6EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B6EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B6EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B6EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B6EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B6F10 size=104
    let mut pc: u32 = 0x832B6F10;
    'dispatch: loop {
        match pc {
            0x832B6F10 => {
    //   block [0x832B6F10..0x832B6F38)
	// 832B6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B6F14: 4B9F24F5  bl 0x82ca9408
	ctx.lr = 0x832B6F18;
	sub_82CA93D0(ctx, base);
	// 832B6F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B6F1C: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6F20: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 832B6F24: 396BE010  addi r11, r11, -0x1ff0
	ctx.r[11].s64 = ctx.r[11].s64 + -8176;
	// 832B6F28: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B6F2C: 3BEB0438  addi r31, r11, 0x438
	ctx.r[31].s64 = ctx.r[11].s64 + 1080;
	// 832B6F30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832B6F34: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832B6F38; continue 'dispatch;
            }
            0x832B6F38 => {
    //   block [0x832B6F38..0x832B6F48)
	// 832B6F38: 3BFFFF94  addi r31, r31, -0x6c
	ctx.r[31].s64 = ctx.r[31].s64 + -108;
	// 832B6F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B6F40: 4AF0F829  bl 0x821c6768
	ctx.lr = 0x832B6F44;
	sub_821C6768(ctx, base);
	// 832B6F44: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832B6F48; continue 'dispatch;
            }
            0x832B6F48 => {
    //   block [0x832B6F48..0x832B6F78)
	// 832B6F48: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832B6F4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B6F50: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832B6F54: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832B6F58: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832B6F5C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832B6F60: 4082FFE8  bne 0x832b6f48
	if !ctx.cr[0].eq {
	pc = 0x832B6F48; continue 'dispatch;
	}
	// 832B6F64: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B6F68: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B6F6C: 4080FFCC  bge 0x832b6f38
	if !ctx.cr[0].lt {
	pc = 0x832B6F38; continue 'dispatch;
	}
	// 832B6F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B6F74: 4B9F24E4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6F78 size=48
    let mut pc: u32 = 0x832B6F78;
    'dispatch: loop {
        match pc {
            0x832B6F78 => {
    //   block [0x832B6F78..0x832B6FA8)
	// 832B6F78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832B6F7C: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 832B6F80: 396BB1AC  addi r11, r11, -0x4e54
	ctx.r[11].s64 = ctx.r[11].s64 + -20052;
	// 832B6F84: 392A9BF8  addi r9, r10, -0x6408
	ctx.r[9].s64 = ctx.r[10].s64 + -25608;
	// 832B6F88: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B6F8C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 832B6F90: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 832B6F94: 419A0014  beq cr6, 0x832b6fa8
	if ctx.cr[6].eq {
		sub_832B6FA8(ctx, base);
		return;
	}
	// 832B6F98: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 832B6F9C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 832B6FA0: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832B6FA4: 4B8A3964  b 0x82b5a908
	sub_82B5A908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FA8 size=8
    let mut pc: u32 = 0x832B6FA8;
    'dispatch: loop {
        match pc {
            0x832B6FA8 => {
    //   block [0x832B6FA8..0x832B6FB0)
	// 832B6FA8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832B6FAC: 4B8A0D34  b 0x82b57ce0
	sub_82B57CE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FB0 size=4
    let mut pc: u32 = 0x832B6FB0;
    'dispatch: loop {
        match pc {
            0x832B6FB0 => {
    //   block [0x832B6FB0..0x832B6FB4)
	// 832B6FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FB8 size=12
    let mut pc: u32 = 0x832B6FB8;
    'dispatch: loop {
        match pc {
            0x832B6FB8 => {
    //   block [0x832B6FB8..0x832B6FC4)
	// 832B6FB8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6FBC: 386BE448  addi r3, r11, -0x1bb8
	ctx.r[3].s64 = ctx.r[11].s64 + -7096;
	// 832B6FC0: 4AF5DE18  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FC8 size=12
    let mut pc: u32 = 0x832B6FC8;
    'dispatch: loop {
        match pc {
            0x832B6FC8 => {
    //   block [0x832B6FC8..0x832B6FD4)
	// 832B6FC8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6FCC: 386BE44C  addi r3, r11, -0x1bb4
	ctx.r[3].s64 = ctx.r[11].s64 + -7092;
	// 832B6FD0: 4AF5DE08  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FD8 size=12
    let mut pc: u32 = 0x832B6FD8;
    'dispatch: loop {
        match pc {
            0x832B6FD8 => {
    //   block [0x832B6FD8..0x832B6FE4)
	// 832B6FD8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6FDC: 386BE450  addi r3, r11, -0x1bb0
	ctx.r[3].s64 = ctx.r[11].s64 + -7088;
	// 832B6FE0: 4AF5DDF8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FE8 size=12
    let mut pc: u32 = 0x832B6FE8;
    'dispatch: loop {
        match pc {
            0x832B6FE8 => {
    //   block [0x832B6FE8..0x832B6FF4)
	// 832B6FE8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6FEC: 386BE454  addi r3, r11, -0x1bac
	ctx.r[3].s64 = ctx.r[11].s64 + -7084;
	// 832B6FF0: 4AF5DDE8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B6FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B6FF8 size=12
    let mut pc: u32 = 0x832B6FF8;
    'dispatch: loop {
        match pc {
            0x832B6FF8 => {
    //   block [0x832B6FF8..0x832B7004)
	// 832B6FF8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B6FFC: 386BE458  addi r3, r11, -0x1ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -7080;
	// 832B7000: 4AF5DDD8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7008 size=12
    let mut pc: u32 = 0x832B7008;
    'dispatch: loop {
        match pc {
            0x832B7008 => {
    //   block [0x832B7008..0x832B7014)
	// 832B7008: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B700C: 386BE45C  addi r3, r11, -0x1ba4
	ctx.r[3].s64 = ctx.r[11].s64 + -7076;
	// 832B7010: 4AF5DDC8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7018 size=12
    let mut pc: u32 = 0x832B7018;
    'dispatch: loop {
        match pc {
            0x832B7018 => {
    //   block [0x832B7018..0x832B7024)
	// 832B7018: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B701C: 386BE460  addi r3, r11, -0x1ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -7072;
	// 832B7020: 4AF5DDB8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7028 size=12
    let mut pc: u32 = 0x832B7028;
    'dispatch: loop {
        match pc {
            0x832B7028 => {
    //   block [0x832B7028..0x832B7034)
	// 832B7028: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B702C: 386BE464  addi r3, r11, -0x1b9c
	ctx.r[3].s64 = ctx.r[11].s64 + -7068;
	// 832B7030: 4AF5DDA8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7038 size=12
    let mut pc: u32 = 0x832B7038;
    'dispatch: loop {
        match pc {
            0x832B7038 => {
    //   block [0x832B7038..0x832B7044)
	// 832B7038: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B703C: 386BE468  addi r3, r11, -0x1b98
	ctx.r[3].s64 = ctx.r[11].s64 + -7064;
	// 832B7040: 4AF5DD98  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7048 size=12
    let mut pc: u32 = 0x832B7048;
    'dispatch: loop {
        match pc {
            0x832B7048 => {
    //   block [0x832B7048..0x832B7054)
	// 832B7048: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B704C: 386BE46C  addi r3, r11, -0x1b94
	ctx.r[3].s64 = ctx.r[11].s64 + -7060;
	// 832B7050: 4AF5DD88  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7058 size=12
    let mut pc: u32 = 0x832B7058;
    'dispatch: loop {
        match pc {
            0x832B7058 => {
    //   block [0x832B7058..0x832B7064)
	// 832B7058: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B705C: 386BE470  addi r3, r11, -0x1b90
	ctx.r[3].s64 = ctx.r[11].s64 + -7056;
	// 832B7060: 4AF5DD78  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7068 size=12
    let mut pc: u32 = 0x832B7068;
    'dispatch: loop {
        match pc {
            0x832B7068 => {
    //   block [0x832B7068..0x832B7074)
	// 832B7068: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B706C: 386BE474  addi r3, r11, -0x1b8c
	ctx.r[3].s64 = ctx.r[11].s64 + -7052;
	// 832B7070: 4AF5DD68  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7078 size=12
    let mut pc: u32 = 0x832B7078;
    'dispatch: loop {
        match pc {
            0x832B7078 => {
    //   block [0x832B7078..0x832B7084)
	// 832B7078: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B707C: 386BE478  addi r3, r11, -0x1b88
	ctx.r[3].s64 = ctx.r[11].s64 + -7048;
	// 832B7080: 4AF5DD58  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7088 size=12
    let mut pc: u32 = 0x832B7088;
    'dispatch: loop {
        match pc {
            0x832B7088 => {
    //   block [0x832B7088..0x832B7094)
	// 832B7088: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B708C: 386BE47C  addi r3, r11, -0x1b84
	ctx.r[3].s64 = ctx.r[11].s64 + -7044;
	// 832B7090: 4B8DBBA0  b 0x82b92c30
	sub_82B92C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7098 size=12
    let mut pc: u32 = 0x832B7098;
    'dispatch: loop {
        match pc {
            0x832B7098 => {
    //   block [0x832B7098..0x832B70A4)
	// 832B7098: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B709C: 386BE4A0  addi r3, r11, -0x1b60
	ctx.r[3].s64 = ctx.r[11].s64 + -7008;
	// 832B70A0: 4B8DBB90  b 0x82b92c30
	sub_82B92C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B70A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B70A8 size=12
    let mut pc: u32 = 0x832B70A8;
    'dispatch: loop {
        match pc {
            0x832B70A8 => {
    //   block [0x832B70A8..0x832B70B4)
	// 832B70A8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B70AC: 386BE4C4  addi r3, r11, -0x1b3c
	ctx.r[3].s64 = ctx.r[11].s64 + -6972;
	// 832B70B0: 4B8DBB80  b 0x82b92c30
	sub_82B92C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B70B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B70B8 size=12
    let mut pc: u32 = 0x832B70B8;
    'dispatch: loop {
        match pc {
            0x832B70B8 => {
    //   block [0x832B70B8..0x832B70C4)
	// 832B70B8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B70BC: 386BE4E8  addi r3, r11, -0x1b18
	ctx.r[3].s64 = ctx.r[11].s64 + -6936;
	// 832B70C0: 4B8DBB70  b 0x82b92c30
	sub_82B92C30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B70C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B70C8 size=12
    let mut pc: u32 = 0x832B70C8;
    'dispatch: loop {
        match pc {
            0x832B70C8 => {
    //   block [0x832B70C8..0x832B70D4)
	// 832B70C8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B70CC: 386BED80  addi r3, r11, -0x1280
	ctx.r[3].s64 = ctx.r[11].s64 + -4736;
	// 832B70D0: 4B8F8408  b 0x82baf4d8
	sub_82BAF4D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B70D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B70D8 size=20
    let mut pc: u32 = 0x832B70D8;
    'dispatch: loop {
        match pc {
            0x832B70D8 => {
    //   block [0x832B70D8..0x832B70EC)
	// 832B70D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832B70DC: 816A716C  lwz r11, 0x716c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29036 as u32) ) } as u64;
	// 832B70E0: 396BFFC0  addi r11, r11, -0x40
	ctx.r[11].s64 = ctx.r[11].s64 + -64;
	// 832B70E4: 916A716C  stw r11, 0x716c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(29036 as u32), ctx.r[11].u32 ) };
	// 832B70E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B70F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B70F0 size=4
    let mut pc: u32 = 0x832B70F0;
    'dispatch: loop {
        match pc {
            0x832B70F0 => {
    //   block [0x832B70F0..0x832B70F4)
	// 832B70F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B70F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B70F8 size=12
    let mut pc: u32 = 0x832B70F8;
    'dispatch: loop {
        match pc {
            0x832B70F8 => {
    //   block [0x832B70F8..0x832B7104)
	// 832B70F8: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B70FC: 386BE8C0  addi r3, r11, -0x1740
	ctx.r[3].s64 = ctx.r[11].s64 + -5952;
	// 832B7100: 4B933640  b 0x82bea740
	sub_82BEA740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7108 size=12
    let mut pc: u32 = 0x832B7108;
    'dispatch: loop {
        match pc {
            0x832B7108 => {
    //   block [0x832B7108..0x832B7114)
	// 832B7108: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B710C: 386BE8C8  addi r3, r11, -0x1738
	ctx.r[3].s64 = ctx.r[11].s64 + -5944;
	// 832B7110: 4B933748  b 0x82bea858
	sub_82BEA858(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7118 size=12
    let mut pc: u32 = 0x832B7118;
    'dispatch: loop {
        match pc {
            0x832B7118 => {
    //   block [0x832B7118..0x832B7124)
	// 832B7118: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B711C: 386BE8D0  addi r3, r11, -0x1730
	ctx.r[3].s64 = ctx.r[11].s64 + -5936;
	// 832B7120: 4B933850  b 0x82bea970
	sub_82BEA970(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7128 size=84
    let mut pc: u32 = 0x832B7128;
    'dispatch: loop {
        match pc {
            0x832B7128 => {
    //   block [0x832B7128..0x832B7150)
	// 832B7128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B712C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7130: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7134: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7138: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832B713C: 3BEBE8D8  addi r31, r11, -0x1728
	ctx.r[31].s64 = ctx.r[11].s64 + -5928;
	// 832B7140: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7148: 419A0008  beq cr6, 0x832b7150
	if ctx.cr[6].eq {
	pc = 0x832B7150; continue 'dispatch;
	}
	// 832B714C: 4AF64BED  bl 0x8221bd38
	ctx.lr = 0x832B7150;
	sub_8221BD38(ctx, base);
	pc = 0x832B7150; continue 'dispatch;
            }
            0x832B7150 => {
    //   block [0x832B7150..0x832B717C)
	// 832B7150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7154: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B7158: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B715C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7160: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B7164: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B7168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B716C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B71B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B71B0 size=84
    let mut pc: u32 = 0x832B71B0;
    'dispatch: loop {
        match pc {
            0x832B71B0 => {
    //   block [0x832B71B0..0x832B71D8)
	// 832B71B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B71B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B71B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B71BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B71C0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B71C4: 3BEB49E4  addi r31, r11, 0x49e4
	ctx.r[31].s64 = ctx.r[11].s64 + 18916;
	// 832B71C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B71CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B71D0: 419A0008  beq cr6, 0x832b71d8
	if ctx.cr[6].eq {
	pc = 0x832B71D8; continue 'dispatch;
	}
	// 832B71D4: 4B9414AD  bl 0x82bf8680
	ctx.lr = 0x832B71D8;
	sub_82BF8680(ctx, base);
	pc = 0x832B71D8; continue 'dispatch;
            }
            0x832B71D8 => {
    //   block [0x832B71D8..0x832B7204)
	// 832B71D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B71DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B71E0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B71E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B71E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B71EC: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B71F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B71F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B71F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B71FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7228 size=84
    let mut pc: u32 = 0x832B7228;
    'dispatch: loop {
        match pc {
            0x832B7228 => {
    //   block [0x832B7228..0x832B7250)
	// 832B7228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7234: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7238: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B723C: 3BEB4A0C  addi r31, r11, 0x4a0c
	ctx.r[31].s64 = ctx.r[11].s64 + 18956;
	// 832B7240: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7244: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7248: 419A0008  beq cr6, 0x832b7250
	if ctx.cr[6].eq {
	pc = 0x832B7250; continue 'dispatch;
	}
	// 832B724C: 4B941435  bl 0x82bf8680
	ctx.lr = 0x832B7250;
	sub_82BF8680(ctx, base);
	pc = 0x832B7250; continue 'dispatch;
            }
            0x832B7250 => {
    //   block [0x832B7250..0x832B727C)
	// 832B7250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B7258: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B725C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7260: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B7264: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832B7268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B726C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7280 size=112
    let mut pc: u32 = 0x832B7280;
    'dispatch: loop {
        match pc {
            0x832B7280 => {
    //   block [0x832B7280..0x832B72F0)
	// 832B7280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7288: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B728C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7290: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7294: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B7298: 3BEB4B94  addi r31, r11, 0x4b94
	ctx.r[31].s64 = ctx.r[11].s64 + 19348;
	// 832B729C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B72A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B72A4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B72A8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B72AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B72B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B72B4: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B72B8: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B72BC: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B72C0: 4B9CD6A1  bl 0x82c84960
	ctx.lr = 0x832B72C4;
	sub_82C84960(ctx, base);
	// 832B72C4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B72C8: 4B58E4E9  bl 0x828457b0
	ctx.lr = 0x832B72CC;
	sub_828457B0(ctx, base);
	// 832B72CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B72D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B72D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B72D8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B72DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B72E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B72E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B72E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B72EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B72F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B72F0 size=12
    let mut pc: u32 = 0x832B72F0;
    'dispatch: loop {
        match pc {
            0x832B72F0 => {
    //   block [0x832B72F0..0x832B72FC)
	// 832B72F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B72F4: 386B4BAC  addi r3, r11, 0x4bac
	ctx.r[3].s64 = ctx.r[11].s64 + 19372;
	// 832B72F8: 4AF5DAE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7300 size=12
    let mut pc: u32 = 0x832B7300;
    'dispatch: loop {
        match pc {
            0x832B7300 => {
    //   block [0x832B7300..0x832B730C)
	// 832B7300: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7304: 386B4BBC  addi r3, r11, 0x4bbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19388;
	// 832B7308: 4AF5DAD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7310 size=12
    let mut pc: u32 = 0x832B7310;
    'dispatch: loop {
        match pc {
            0x832B7310 => {
    //   block [0x832B7310..0x832B731C)
	// 832B7310: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7314: 386B4BC0  addi r3, r11, 0x4bc0
	ctx.r[3].s64 = ctx.r[11].s64 + 19392;
	// 832B7318: 4AF5DAC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7320 size=32
    let mut pc: u32 = 0x832B7320;
    'dispatch: loop {
        match pc {
            0x832B7320 => {
    //   block [0x832B7320..0x832B7340)
	// 832B7320: 3D60832D  lis r11, -0x7cd3
	ctx.r[11].s64 = -2094202880;
	// 832B7324: 806B6124  lwz r3, 0x6124(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24868 as u32) ) } as u64;
	// 832B7328: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B732C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B7330: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7334: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B7338: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B733C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7340 size=4
    let mut pc: u32 = 0x832B7340;
    'dispatch: loop {
        match pc {
            0x832B7340 => {
    //   block [0x832B7340..0x832B7344)
	// 832B7340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7348 size=112
    let mut pc: u32 = 0x832B7348;
    'dispatch: loop {
        match pc {
            0x832B7348 => {
    //   block [0x832B7348..0x832B73B8)
	// 832B7348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7350: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7354: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7358: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B735C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B7360: 3BEB4BB0  addi r31, r11, 0x4bb0
	ctx.r[31].s64 = ctx.r[11].s64 + 19376;
	// 832B7364: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B7368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B736C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B7370: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7374: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7378: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B737C: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B7380: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B7384: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B7388: 4B06AB39  bl 0x82321ec0
	ctx.lr = 0x832B738C;
	sub_82321EC0(ctx, base);
	// 832B738C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7390: 4B58E421  bl 0x828457b0
	ctx.lr = 0x832B7394;
	sub_828457B0(ctx, base);
	// 832B7394: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B739C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B73A0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B73A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B73A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B73AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B73B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B73B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B73C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B73C8 size=64
    let mut pc: u32 = 0x832B73C8;
    'dispatch: loop {
        match pc {
            0x832B73C8 => {
    //   block [0x832B73C8..0x832B73F4)
	// 832B73C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B73CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B73D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B73D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B73D8: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 832B73DC: 807F4C2C  lwz r3, 0x4c2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(19500 as u32) ) } as u64;
	// 832B73E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B73E4: 419A0010  beq cr6, 0x832b73f4
	if ctx.cr[6].eq {
	pc = 0x832B73F4; continue 'dispatch;
	}
	// 832B73E8: 4B58E3C9  bl 0x828457b0
	ctx.lr = 0x832B73EC;
	sub_828457B0(ctx, base);
	// 832B73EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B73F0: 917F4C2C  stw r11, 0x4c2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(19500 as u32), ctx.r[11].u32 ) };
	pc = 0x832B73F4; continue 'dispatch;
            }
            0x832B73F4 => {
    //   block [0x832B73F4..0x832B7408)
	// 832B73F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B73F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B73FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7428 size=112
    let mut pc: u32 = 0x832B7428;
    'dispatch: loop {
        match pc {
            0x832B7428 => {
    //   block [0x832B7428..0x832B7498)
	// 832B7428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B742C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7430: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7434: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7438: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B743C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B7440: 3BEB4CBC  addi r31, r11, 0x4cbc
	ctx.r[31].s64 = ctx.r[11].s64 + 19644;
	// 832B7444: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B7448: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B744C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B7450: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7454: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B745C: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B7460: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B7464: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B7468: 4B0BE991  bl 0x82375df8
	ctx.lr = 0x832B746C;
	sub_82375DF8(ctx, base);
	// 832B746C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7470: 4B58E341  bl 0x828457b0
	ctx.lr = 0x832B7474;
	sub_828457B0(ctx, base);
	// 832B7474: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B747C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7480: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B7484: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B748C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7490: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7498 size=112
    let mut pc: u32 = 0x832B7498;
    'dispatch: loop {
        match pc {
            0x832B7498 => {
    //   block [0x832B7498..0x832B7508)
	// 832B7498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B749C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B74A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B74A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B74A8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B74AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B74B0: 3BEB4CC8  addi r31, r11, 0x4cc8
	ctx.r[31].s64 = ctx.r[11].s64 + 19656;
	// 832B74B4: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B74B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B74BC: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B74C0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B74C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B74C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B74CC: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B74D0: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B74D4: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B74D8: 4B9553F9  bl 0x82c0c8d0
	ctx.lr = 0x832B74DC;
	sub_82C0C8D0(ctx, base);
	// 832B74DC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B74E0: 4B58E2D1  bl 0x828457b0
	ctx.lr = 0x832B74E4;
	sub_828457B0(ctx, base);
	// 832B74E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B74E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B74EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B74F0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B74F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B74F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B74FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7508 size=112
    let mut pc: u32 = 0x832B7508;
    'dispatch: loop {
        match pc {
            0x832B7508 => {
    //   block [0x832B7508..0x832B7578)
	// 832B7508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B750C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7510: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7514: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7518: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B751C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B7520: 3BEB4CD4  addi r31, r11, 0x4cd4
	ctx.r[31].s64 = ctx.r[11].s64 + 19668;
	// 832B7524: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B7528: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B752C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B7530: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7534: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7538: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B753C: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B7540: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B7544: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B7548: 4B955471  bl 0x82c0c9b8
	ctx.lr = 0x832B754C;
	sub_82C0C9B8(ctx, base);
	// 832B754C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7550: 4B58E261  bl 0x828457b0
	ctx.lr = 0x832B7554;
	sub_828457B0(ctx, base);
	// 832B7554: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B755C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7560: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B7564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B756C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7578 size=112
    let mut pc: u32 = 0x832B7578;
    'dispatch: loop {
        match pc {
            0x832B7578 => {
    //   block [0x832B7578..0x832B75E8)
	// 832B7578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B757C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7584: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7588: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B758C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B7590: 3BEB4CE0  addi r31, r11, 0x4ce0
	ctx.r[31].s64 = ctx.r[11].s64 + 19680;
	// 832B7594: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B7598: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B759C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B75A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B75A4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B75A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B75AC: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B75B0: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B75B4: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B75B8: 4B9554E9  bl 0x82c0caa0
	ctx.lr = 0x832B75BC;
	sub_82C0CAA0(ctx, base);
	// 832B75BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B75C0: 4B58E1F1  bl 0x828457b0
	ctx.lr = 0x832B75C4;
	sub_828457B0(ctx, base);
	// 832B75C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B75C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B75CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B75D0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B75D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B75D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B75DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B75E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B75E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B75E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B75E8 size=80
    let mut pc: u32 = 0x832B75E8;
    'dispatch: loop {
        match pc {
            0x832B75E8 => {
    //   block [0x832B75E8..0x832B760C)
	// 832B75E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B75EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B75F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B75F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B75F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B75FC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7600: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832B7604: 396B4D30  addi r11, r11, 0x4d30
	ctx.r[11].s64 = ctx.r[11].s64 + 19760;
	// 832B7608: 3BEB008C  addi r31, r11, 0x8c
	ctx.r[31].s64 = ctx.r[11].s64 + 140;
	pc = 0x832B760C; continue 'dispatch;
            }
            0x832B760C => {
    //   block [0x832B760C..0x832B7638)
	// 832B760C: 3BFFFFE4  addi r31, r31, -0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + -28;
	// 832B7610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B7614: 4B8DF865  bl 0x82b96e78
	ctx.lr = 0x832B7618;
	sub_82B96E78(ctx, base);
	// 832B7618: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B761C: 4080FFF0  bge 0x832b760c
	if !ctx.cr[0].lt {
	pc = 0x832B760C; continue 'dispatch;
	}
	// 832B7620: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B762C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B7630: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7638 size=80
    let mut pc: u32 = 0x832B7638;
    'dispatch: loop {
        match pc {
            0x832B7638 => {
    //   block [0x832B7638..0x832B765C)
	// 832B7638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B763C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B7644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B764C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7650: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 832B7654: 396B4DBC  addi r11, r11, 0x4dbc
	ctx.r[11].s64 = ctx.r[11].s64 + 19900;
	// 832B7658: 3BEB00B4  addi r31, r11, 0xb4
	ctx.r[31].s64 = ctx.r[11].s64 + 180;
	pc = 0x832B765C; continue 'dispatch;
            }
            0x832B765C => {
    //   block [0x832B765C..0x832B7688)
	// 832B765C: 3BFFFFDC  addi r31, r31, -0x24
	ctx.r[31].s64 = ctx.r[31].s64 + -36;
	// 832B7660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B7664: 4B886375  bl 0x82b3d9d8
	ctx.lr = 0x832B7668;
	sub_82B3D9D8(ctx, base);
	// 832B7668: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832B766C: 4080FFF0  bge 0x832b765c
	if !ctx.cr[0].lt {
	pc = 0x832B765C; continue 'dispatch;
	}
	// 832B7670: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B767C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B7680: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7688 size=124
    let mut pc: u32 = 0x832B7688;
    'dispatch: loop {
        match pc {
            0x832B7688 => {
    //   block [0x832B7688..0x832B76A8)
	// 832B7688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B768C: 4B9F1D7D  bl 0x82ca9408
	ctx.lr = 0x832B7690;
	sub_82CA93D0(ctx, base);
	// 832B7690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7694: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7698: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 832B769C: 396B4CF4  addi r11, r11, 0x4cf4
	ctx.r[11].s64 = ctx.r[11].s64 + 19700;
	// 832B76A0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832B76A4: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	pc = 0x832B76A8; continue 'dispatch;
            }
            0x832B76A8 => {
    //   block [0x832B76A8..0x832B76D0)
	// 832B76A8: 3BFFFFF4  addi r31, r31, -0xc
	ctx.r[31].s64 = ctx.r[31].s64 + -12;
	// 832B76AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76B0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76B4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832B76B8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76BC: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B76C0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76C4: 7F034840  cmplw cr6, r3, r9
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832B76C8: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 832B76CC: 419A001C  beq cr6, 0x832b76e8
	if ctx.cr[6].eq {
	pc = 0x832B76E8; continue 'dispatch;
	}
	pc = 0x832B76D0; continue 'dispatch;
            }
            0x832B76D0 => {
    //   block [0x832B76D0..0x832B76E8)
	// 832B76D0: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76D4: 4B58E0DD  bl 0x828457b0
	ctx.lr = 0x832B76D8;
	sub_828457B0(ctx, base);
	// 832B76D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832B76E0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832B76E4: 409AFFEC  bne cr6, 0x832b76d0
	if !ctx.cr[6].eq {
	pc = 0x832B76D0; continue 'dispatch;
	}
	pc = 0x832B76E8; continue 'dispatch;
            }
            0x832B76E8 => {
    //   block [0x832B76E8..0x832B7704)
	// 832B76E8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B76EC: 4B58E0C5  bl 0x828457b0
	ctx.lr = 0x832B76F0;
	sub_828457B0(ctx, base);
	// 832B76F0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832B76F4: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832B76F8: 4080FFB0  bge 0x832b76a8
	if !ctx.cr[0].lt {
	pc = 0x832B76A8; continue 'dispatch;
	}
	// 832B76FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832B7700: 4B9F1D58  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7708 size=12
    let mut pc: u32 = 0x832B7708;
    'dispatch: loop {
        match pc {
            0x832B7708 => {
    //   block [0x832B7708..0x832B7714)
	// 832B7708: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B770C: 386B4E74  addi r3, r11, 0x4e74
	ctx.r[3].s64 = ctx.r[11].s64 + 20084;
	// 832B7710: 4B9573A0  b 0x82c0eab0
	sub_82C0EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7718 size=12
    let mut pc: u32 = 0x832B7718;
    'dispatch: loop {
        match pc {
            0x832B7718 => {
    //   block [0x832B7718..0x832B7724)
	// 832B7718: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B771C: 386B4EB0  addi r3, r11, 0x4eb0
	ctx.r[3].s64 = ctx.r[11].s64 + 20144;
	// 832B7720: 4B957390  b 0x82c0eab0
	sub_82C0EAB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7728 size=12
    let mut pc: u32 = 0x832B7728;
    'dispatch: loop {
        match pc {
            0x832B7728 => {
    //   block [0x832B7728..0x832B7734)
	// 832B7728: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B772C: 386B4EF8  addi r3, r11, 0x4ef8
	ctx.r[3].s64 = ctx.r[11].s64 + 20216;
	// 832B7730: 4AF5D6A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7738 size=12
    let mut pc: u32 = 0x832B7738;
    'dispatch: loop {
        match pc {
            0x832B7738 => {
    //   block [0x832B7738..0x832B7744)
	// 832B7738: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B773C: 386B4EFC  addi r3, r11, 0x4efc
	ctx.r[3].s64 = ctx.r[11].s64 + 20220;
	// 832B7740: 4AF5D698  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7748 size=12
    let mut pc: u32 = 0x832B7748;
    'dispatch: loop {
        match pc {
            0x832B7748 => {
    //   block [0x832B7748..0x832B7754)
	// 832B7748: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B774C: 386B4F00  addi r3, r11, 0x4f00
	ctx.r[3].s64 = ctx.r[11].s64 + 20224;
	// 832B7750: 4AF5D688  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7758 size=12
    let mut pc: u32 = 0x832B7758;
    'dispatch: loop {
        match pc {
            0x832B7758 => {
    //   block [0x832B7758..0x832B7764)
	// 832B7758: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B775C: 386B4F04  addi r3, r11, 0x4f04
	ctx.r[3].s64 = ctx.r[11].s64 + 20228;
	// 832B7760: 4AF5D678  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7768 size=12
    let mut pc: u32 = 0x832B7768;
    'dispatch: loop {
        match pc {
            0x832B7768 => {
    //   block [0x832B7768..0x832B7774)
	// 832B7768: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B776C: 386B4F08  addi r3, r11, 0x4f08
	ctx.r[3].s64 = ctx.r[11].s64 + 20232;
	// 832B7770: 4AF5D668  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7778 size=12
    let mut pc: u32 = 0x832B7778;
    'dispatch: loop {
        match pc {
            0x832B7778 => {
    //   block [0x832B7778..0x832B7784)
	// 832B7778: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B777C: 386B4F0C  addi r3, r11, 0x4f0c
	ctx.r[3].s64 = ctx.r[11].s64 + 20236;
	// 832B7780: 4AF5D658  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7788 size=12
    let mut pc: u32 = 0x832B7788;
    'dispatch: loop {
        match pc {
            0x832B7788 => {
    //   block [0x832B7788..0x832B7794)
	// 832B7788: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B778C: 386B4F10  addi r3, r11, 0x4f10
	ctx.r[3].s64 = ctx.r[11].s64 + 20240;
	// 832B7790: 4AF5D648  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7798 size=12
    let mut pc: u32 = 0x832B7798;
    'dispatch: loop {
        match pc {
            0x832B7798 => {
    //   block [0x832B7798..0x832B77A4)
	// 832B7798: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B779C: 386B4F14  addi r3, r11, 0x4f14
	ctx.r[3].s64 = ctx.r[11].s64 + 20244;
	// 832B77A0: 4AF5D638  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B77A8 size=12
    let mut pc: u32 = 0x832B77A8;
    'dispatch: loop {
        match pc {
            0x832B77A8 => {
    //   block [0x832B77A8..0x832B77B4)
	// 832B77A8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B77AC: 386B4F18  addi r3, r11, 0x4f18
	ctx.r[3].s64 = ctx.r[11].s64 + 20248;
	// 832B77B0: 4AF5D628  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B77B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B77B8 size=12
    let mut pc: u32 = 0x832B77B8;
    'dispatch: loop {
        match pc {
            0x832B77B8 => {
    //   block [0x832B77B8..0x832B77C4)
	// 832B77B8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B77BC: 386B4F1C  addi r3, r11, 0x4f1c
	ctx.r[3].s64 = ctx.r[11].s64 + 20252;
	// 832B77C0: 4AF5D618  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B77C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B77C8 size=12
    let mut pc: u32 = 0x832B77C8;
    'dispatch: loop {
        match pc {
            0x832B77C8 => {
    //   block [0x832B77C8..0x832B77D4)
	// 832B77C8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B77CC: 386B4F20  addi r3, r11, 0x4f20
	ctx.r[3].s64 = ctx.r[11].s64 + 20256;
	// 832B77D0: 4AF5D608  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B77D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B77D8 size=12
    let mut pc: u32 = 0x832B77D8;
    'dispatch: loop {
        match pc {
            0x832B77D8 => {
    //   block [0x832B77D8..0x832B77E4)
	// 832B77D8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B77DC: 386B4F24  addi r3, r11, 0x4f24
	ctx.r[3].s64 = ctx.r[11].s64 + 20260;
	// 832B77E0: 4AF5D5F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B77E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B77E8 size=12
    let mut pc: u32 = 0x832B77E8;
    'dispatch: loop {
        match pc {
            0x832B77E8 => {
    //   block [0x832B77E8..0x832B77F4)
	// 832B77E8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B77EC: 386B4F28  addi r3, r11, 0x4f28
	ctx.r[3].s64 = ctx.r[11].s64 + 20264;
	// 832B77F0: 4AF5D5E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B77F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B77F8 size=12
    let mut pc: u32 = 0x832B77F8;
    'dispatch: loop {
        match pc {
            0x832B77F8 => {
    //   block [0x832B77F8..0x832B7804)
	// 832B77F8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B77FC: 386B4F2C  addi r3, r11, 0x4f2c
	ctx.r[3].s64 = ctx.r[11].s64 + 20268;
	// 832B7800: 4AF5D5D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7808 size=12
    let mut pc: u32 = 0x832B7808;
    'dispatch: loop {
        match pc {
            0x832B7808 => {
    //   block [0x832B7808..0x832B7814)
	// 832B7808: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B780C: 386B4F30  addi r3, r11, 0x4f30
	ctx.r[3].s64 = ctx.r[11].s64 + 20272;
	// 832B7810: 4AF5D5C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7818 size=12
    let mut pc: u32 = 0x832B7818;
    'dispatch: loop {
        match pc {
            0x832B7818 => {
    //   block [0x832B7818..0x832B7824)
	// 832B7818: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B781C: 386B4F34  addi r3, r11, 0x4f34
	ctx.r[3].s64 = ctx.r[11].s64 + 20276;
	// 832B7820: 4AF5D5B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7828 size=12
    let mut pc: u32 = 0x832B7828;
    'dispatch: loop {
        match pc {
            0x832B7828 => {
    //   block [0x832B7828..0x832B7834)
	// 832B7828: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B782C: 386B4F38  addi r3, r11, 0x4f38
	ctx.r[3].s64 = ctx.r[11].s64 + 20280;
	// 832B7830: 4AF5D5A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7838 size=12
    let mut pc: u32 = 0x832B7838;
    'dispatch: loop {
        match pc {
            0x832B7838 => {
    //   block [0x832B7838..0x832B7844)
	// 832B7838: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B783C: 386B4F3C  addi r3, r11, 0x4f3c
	ctx.r[3].s64 = ctx.r[11].s64 + 20284;
	// 832B7840: 4AF5D598  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7848 size=12
    let mut pc: u32 = 0x832B7848;
    'dispatch: loop {
        match pc {
            0x832B7848 => {
    //   block [0x832B7848..0x832B7854)
	// 832B7848: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B784C: 386B4F40  addi r3, r11, 0x4f40
	ctx.r[3].s64 = ctx.r[11].s64 + 20288;
	// 832B7850: 4AF5D588  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7858 size=12
    let mut pc: u32 = 0x832B7858;
    'dispatch: loop {
        match pc {
            0x832B7858 => {
    //   block [0x832B7858..0x832B7864)
	// 832B7858: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B785C: 386B4F44  addi r3, r11, 0x4f44
	ctx.r[3].s64 = ctx.r[11].s64 + 20292;
	// 832B7860: 4AF5D578  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7868 size=12
    let mut pc: u32 = 0x832B7868;
    'dispatch: loop {
        match pc {
            0x832B7868 => {
    //   block [0x832B7868..0x832B7874)
	// 832B7868: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B786C: 386B4F48  addi r3, r11, 0x4f48
	ctx.r[3].s64 = ctx.r[11].s64 + 20296;
	// 832B7870: 4AF5D568  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7878 size=12
    let mut pc: u32 = 0x832B7878;
    'dispatch: loop {
        match pc {
            0x832B7878 => {
    //   block [0x832B7878..0x832B7884)
	// 832B7878: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B787C: 386B4F4C  addi r3, r11, 0x4f4c
	ctx.r[3].s64 = ctx.r[11].s64 + 20300;
	// 832B7880: 4AF5D558  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7888 size=12
    let mut pc: u32 = 0x832B7888;
    'dispatch: loop {
        match pc {
            0x832B7888 => {
    //   block [0x832B7888..0x832B7894)
	// 832B7888: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B788C: 386B4F50  addi r3, r11, 0x4f50
	ctx.r[3].s64 = ctx.r[11].s64 + 20304;
	// 832B7890: 4AF5D548  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7898 size=12
    let mut pc: u32 = 0x832B7898;
    'dispatch: loop {
        match pc {
            0x832B7898 => {
    //   block [0x832B7898..0x832B78A4)
	// 832B7898: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B789C: 386B4F54  addi r3, r11, 0x4f54
	ctx.r[3].s64 = ctx.r[11].s64 + 20308;
	// 832B78A0: 4AF5D538  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B78A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B78A8 size=12
    let mut pc: u32 = 0x832B78A8;
    'dispatch: loop {
        match pc {
            0x832B78A8 => {
    //   block [0x832B78A8..0x832B78B4)
	// 832B78A8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B78AC: 386B4F58  addi r3, r11, 0x4f58
	ctx.r[3].s64 = ctx.r[11].s64 + 20312;
	// 832B78B0: 4AF5D528  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B78B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B78B8 size=12
    let mut pc: u32 = 0x832B78B8;
    'dispatch: loop {
        match pc {
            0x832B78B8 => {
    //   block [0x832B78B8..0x832B78C4)
	// 832B78B8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B78BC: 386B4F5C  addi r3, r11, 0x4f5c
	ctx.r[3].s64 = ctx.r[11].s64 + 20316;
	// 832B78C0: 4AF5D518  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B78C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B78C8 size=12
    let mut pc: u32 = 0x832B78C8;
    'dispatch: loop {
        match pc {
            0x832B78C8 => {
    //   block [0x832B78C8..0x832B78D4)
	// 832B78C8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B78CC: 386B4F60  addi r3, r11, 0x4f60
	ctx.r[3].s64 = ctx.r[11].s64 + 20320;
	// 832B78D0: 4AF5D508  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B78D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B78D8 size=12
    let mut pc: u32 = 0x832B78D8;
    'dispatch: loop {
        match pc {
            0x832B78D8 => {
    //   block [0x832B78D8..0x832B78E4)
	// 832B78D8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B78DC: 386B4F64  addi r3, r11, 0x4f64
	ctx.r[3].s64 = ctx.r[11].s64 + 20324;
	// 832B78E0: 4AF5D4F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B78E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B78E8 size=12
    let mut pc: u32 = 0x832B78E8;
    'dispatch: loop {
        match pc {
            0x832B78E8 => {
    //   block [0x832B78E8..0x832B78F4)
	// 832B78E8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B78EC: 386B4F68  addi r3, r11, 0x4f68
	ctx.r[3].s64 = ctx.r[11].s64 + 20328;
	// 832B78F0: 4AF5D4E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B78F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B78F8 size=12
    let mut pc: u32 = 0x832B78F8;
    'dispatch: loop {
        match pc {
            0x832B78F8 => {
    //   block [0x832B78F8..0x832B7904)
	// 832B78F8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B78FC: 386B4F6C  addi r3, r11, 0x4f6c
	ctx.r[3].s64 = ctx.r[11].s64 + 20332;
	// 832B7900: 4AF5D4D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7908 size=12
    let mut pc: u32 = 0x832B7908;
    'dispatch: loop {
        match pc {
            0x832B7908 => {
    //   block [0x832B7908..0x832B7914)
	// 832B7908: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B790C: 386B4F70  addi r3, r11, 0x4f70
	ctx.r[3].s64 = ctx.r[11].s64 + 20336;
	// 832B7910: 4AF5D4C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7918 size=12
    let mut pc: u32 = 0x832B7918;
    'dispatch: loop {
        match pc {
            0x832B7918 => {
    //   block [0x832B7918..0x832B7924)
	// 832B7918: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B791C: 386B4F74  addi r3, r11, 0x4f74
	ctx.r[3].s64 = ctx.r[11].s64 + 20340;
	// 832B7920: 4AF5D4B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7928 size=12
    let mut pc: u32 = 0x832B7928;
    'dispatch: loop {
        match pc {
            0x832B7928 => {
    //   block [0x832B7928..0x832B7934)
	// 832B7928: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B792C: 386B4F78  addi r3, r11, 0x4f78
	ctx.r[3].s64 = ctx.r[11].s64 + 20344;
	// 832B7930: 4AF5D4A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7938 size=12
    let mut pc: u32 = 0x832B7938;
    'dispatch: loop {
        match pc {
            0x832B7938 => {
    //   block [0x832B7938..0x832B7944)
	// 832B7938: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B793C: 386B4F7C  addi r3, r11, 0x4f7c
	ctx.r[3].s64 = ctx.r[11].s64 + 20348;
	// 832B7940: 4AF5D498  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7948 size=12
    let mut pc: u32 = 0x832B7948;
    'dispatch: loop {
        match pc {
            0x832B7948 => {
    //   block [0x832B7948..0x832B7954)
	// 832B7948: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B794C: 386B4F80  addi r3, r11, 0x4f80
	ctx.r[3].s64 = ctx.r[11].s64 + 20352;
	// 832B7950: 4AF5D488  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7958 size=12
    let mut pc: u32 = 0x832B7958;
    'dispatch: loop {
        match pc {
            0x832B7958 => {
    //   block [0x832B7958..0x832B7964)
	// 832B7958: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B795C: 386B4F84  addi r3, r11, 0x4f84
	ctx.r[3].s64 = ctx.r[11].s64 + 20356;
	// 832B7960: 4AF5D478  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7968 size=12
    let mut pc: u32 = 0x832B7968;
    'dispatch: loop {
        match pc {
            0x832B7968 => {
    //   block [0x832B7968..0x832B7974)
	// 832B7968: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B796C: 386B4F94  addi r3, r11, 0x4f94
	ctx.r[3].s64 = ctx.r[11].s64 + 20372;
	// 832B7970: 4AF5D468  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7978 size=12
    let mut pc: u32 = 0x832B7978;
    'dispatch: loop {
        match pc {
            0x832B7978 => {
    //   block [0x832B7978..0x832B7984)
	// 832B7978: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B797C: 386B4F98  addi r3, r11, 0x4f98
	ctx.r[3].s64 = ctx.r[11].s64 + 20376;
	// 832B7980: 4AF5D458  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7988 size=12
    let mut pc: u32 = 0x832B7988;
    'dispatch: loop {
        match pc {
            0x832B7988 => {
    //   block [0x832B7988..0x832B7994)
	// 832B7988: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B798C: 386B4F9C  addi r3, r11, 0x4f9c
	ctx.r[3].s64 = ctx.r[11].s64 + 20380;
	// 832B7990: 4AF5D448  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7998 size=12
    let mut pc: u32 = 0x832B7998;
    'dispatch: loop {
        match pc {
            0x832B7998 => {
    //   block [0x832B7998..0x832B79A4)
	// 832B7998: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B799C: 386B4FA0  addi r3, r11, 0x4fa0
	ctx.r[3].s64 = ctx.r[11].s64 + 20384;
	// 832B79A0: 4AF5D438  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B79A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B79A8 size=12
    let mut pc: u32 = 0x832B79A8;
    'dispatch: loop {
        match pc {
            0x832B79A8 => {
    //   block [0x832B79A8..0x832B79B4)
	// 832B79A8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B79AC: 386B4FA4  addi r3, r11, 0x4fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 20388;
	// 832B79B0: 4AF5D428  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B79B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B79B8 size=12
    let mut pc: u32 = 0x832B79B8;
    'dispatch: loop {
        match pc {
            0x832B79B8 => {
    //   block [0x832B79B8..0x832B79C4)
	// 832B79B8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B79BC: 386B4FA8  addi r3, r11, 0x4fa8
	ctx.r[3].s64 = ctx.r[11].s64 + 20392;
	// 832B79C0: 4AF5D418  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B79C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B79C8 size=12
    let mut pc: u32 = 0x832B79C8;
    'dispatch: loop {
        match pc {
            0x832B79C8 => {
    //   block [0x832B79C8..0x832B79D4)
	// 832B79C8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B79CC: 386B4FAC  addi r3, r11, 0x4fac
	ctx.r[3].s64 = ctx.r[11].s64 + 20396;
	// 832B79D0: 4AF5D408  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B79D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B79D8 size=12
    let mut pc: u32 = 0x832B79D8;
    'dispatch: loop {
        match pc {
            0x832B79D8 => {
    //   block [0x832B79D8..0x832B79E4)
	// 832B79D8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B79DC: 386B4FB0  addi r3, r11, 0x4fb0
	ctx.r[3].s64 = ctx.r[11].s64 + 20400;
	// 832B79E0: 4AF5D3F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B79E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B79E8 size=12
    let mut pc: u32 = 0x832B79E8;
    'dispatch: loop {
        match pc {
            0x832B79E8 => {
    //   block [0x832B79E8..0x832B79F4)
	// 832B79E8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B79EC: 386B4FB4  addi r3, r11, 0x4fb4
	ctx.r[3].s64 = ctx.r[11].s64 + 20404;
	// 832B79F0: 4AF5D3E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B79F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B79F8 size=12
    let mut pc: u32 = 0x832B79F8;
    'dispatch: loop {
        match pc {
            0x832B79F8 => {
    //   block [0x832B79F8..0x832B7A04)
	// 832B79F8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B79FC: 386B4FB8  addi r3, r11, 0x4fb8
	ctx.r[3].s64 = ctx.r[11].s64 + 20408;
	// 832B7A00: 4AF5D3D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A08 size=12
    let mut pc: u32 = 0x832B7A08;
    'dispatch: loop {
        match pc {
            0x832B7A08 => {
    //   block [0x832B7A08..0x832B7A14)
	// 832B7A08: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A0C: 386B4FBC  addi r3, r11, 0x4fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 20412;
	// 832B7A10: 4AF5D3C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A18 size=12
    let mut pc: u32 = 0x832B7A18;
    'dispatch: loop {
        match pc {
            0x832B7A18 => {
    //   block [0x832B7A18..0x832B7A24)
	// 832B7A18: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A1C: 386B4FC0  addi r3, r11, 0x4fc0
	ctx.r[3].s64 = ctx.r[11].s64 + 20416;
	// 832B7A20: 4AF5D3B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A28 size=12
    let mut pc: u32 = 0x832B7A28;
    'dispatch: loop {
        match pc {
            0x832B7A28 => {
    //   block [0x832B7A28..0x832B7A34)
	// 832B7A28: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A2C: 386B4FC4  addi r3, r11, 0x4fc4
	ctx.r[3].s64 = ctx.r[11].s64 + 20420;
	// 832B7A30: 4AF5D3A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A38 size=12
    let mut pc: u32 = 0x832B7A38;
    'dispatch: loop {
        match pc {
            0x832B7A38 => {
    //   block [0x832B7A38..0x832B7A44)
	// 832B7A38: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A3C: 386B4FC8  addi r3, r11, 0x4fc8
	ctx.r[3].s64 = ctx.r[11].s64 + 20424;
	// 832B7A40: 4AF5D398  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A48 size=12
    let mut pc: u32 = 0x832B7A48;
    'dispatch: loop {
        match pc {
            0x832B7A48 => {
    //   block [0x832B7A48..0x832B7A54)
	// 832B7A48: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A4C: 386B4FD8  addi r3, r11, 0x4fd8
	ctx.r[3].s64 = ctx.r[11].s64 + 20440;
	// 832B7A50: 4AF5D388  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A58 size=12
    let mut pc: u32 = 0x832B7A58;
    'dispatch: loop {
        match pc {
            0x832B7A58 => {
    //   block [0x832B7A58..0x832B7A64)
	// 832B7A58: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A5C: 386B4FDC  addi r3, r11, 0x4fdc
	ctx.r[3].s64 = ctx.r[11].s64 + 20444;
	// 832B7A60: 4AF5D378  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A68 size=12
    let mut pc: u32 = 0x832B7A68;
    'dispatch: loop {
        match pc {
            0x832B7A68 => {
    //   block [0x832B7A68..0x832B7A74)
	// 832B7A68: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A6C: 386B4FE0  addi r3, r11, 0x4fe0
	ctx.r[3].s64 = ctx.r[11].s64 + 20448;
	// 832B7A70: 4B891570  b 0x82b48fe0
	sub_82B48FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A78 size=12
    let mut pc: u32 = 0x832B7A78;
    'dispatch: loop {
        match pc {
            0x832B7A78 => {
    //   block [0x832B7A78..0x832B7A84)
	// 832B7A78: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A7C: 386B5008  addi r3, r11, 0x5008
	ctx.r[3].s64 = ctx.r[11].s64 + 20488;
	// 832B7A80: 4AF5D358  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7A88 size=12
    let mut pc: u32 = 0x832B7A88;
    'dispatch: loop {
        match pc {
            0x832B7A88 => {
    //   block [0x832B7A88..0x832B7A94)
	// 832B7A88: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7A8C: 386B500C  addi r3, r11, 0x500c
	ctx.r[3].s64 = ctx.r[11].s64 + 20492;
	// 832B7A90: 4AF5D348  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7A98 size=84
    let mut pc: u32 = 0x832B7A98;
    'dispatch: loop {
        match pc {
            0x832B7A98 => {
    //   block [0x832B7A98..0x832B7AEC)
	// 832B7A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7AA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7AA8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7AAC: 3BEB5010  addi r31, r11, 0x5010
	ctx.r[31].s64 = ctx.r[11].s64 + 20496;
	// 832B7AB0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832B7AB4: 4B89152D  bl 0x82b48fe0
	ctx.lr = 0x832B7AB8;
	sub_82B48FE0(ctx, base);
	// 832B7AB8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832B7ABC: 4B891525  bl 0x82b48fe0
	ctx.lr = 0x832B7AC0;
	sub_82B48FE0(ctx, base);
	// 832B7AC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B7AC4: 4B952C05  bl 0x82c0a6c8
	ctx.lr = 0x832B7AC8;
	sub_82C0A6C8(ctx, base);
	// 832B7AC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7ACC: 4B58DCE5  bl 0x828457b0
	ctx.lr = 0x832B7AD0;
	sub_828457B0(ctx, base);
	// 832B7AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7AD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B7ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7AF0 size=84
    let mut pc: u32 = 0x832B7AF0;
    'dispatch: loop {
        match pc {
            0x832B7AF0 => {
    //   block [0x832B7AF0..0x832B7B44)
	// 832B7AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7B00: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7B04: 3BEB5088  addi r31, r11, 0x5088
	ctx.r[31].s64 = ctx.r[11].s64 + 20616;
	// 832B7B08: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832B7B0C: 4B8914D5  bl 0x82b48fe0
	ctx.lr = 0x832B7B10;
	sub_82B48FE0(ctx, base);
	// 832B7B10: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832B7B14: 4B8914CD  bl 0x82b48fe0
	ctx.lr = 0x832B7B18;
	sub_82B48FE0(ctx, base);
	// 832B7B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B7B1C: 4B952BAD  bl 0x82c0a6c8
	ctx.lr = 0x832B7B20;
	sub_82C0A6C8(ctx, base);
	// 832B7B20: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7B24: 4B58DC8D  bl 0x828457b0
	ctx.lr = 0x832B7B28;
	sub_828457B0(ctx, base);
	// 832B7B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7B2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B7B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7B58 size=112
    let mut pc: u32 = 0x832B7B58;
    'dispatch: loop {
        match pc {
            0x832B7B58 => {
    //   block [0x832B7B58..0x832B7BC8)
	// 832B7B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7B60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7B64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7B68: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7B6C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 832B7B70: 3BEB5184  addi r31, r11, 0x5184
	ctx.r[31].s64 = ctx.r[11].s64 + 20868;
	// 832B7B74: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 832B7B78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832B7B7C: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 832B7B80: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7B84: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7B88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832B7B8C: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832B7B90: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832B7B94: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832B7B98: 4B973E99  bl 0x82c2ba30
	ctx.lr = 0x832B7B9C;
	sub_82C2BA30(ctx, base);
	// 832B7B9C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832B7BA0: 4B58DC11  bl 0x828457b0
	ctx.lr = 0x832B7BA4;
	sub_828457B0(ctx, base);
	// 832B7BA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B7BAC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832B7BB0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832B7BB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7BC8 size=32
    let mut pc: u32 = 0x832B7BC8;
    'dispatch: loop {
        match pc {
            0x832B7BC8 => {
    //   block [0x832B7BC8..0x832B7BE8)
	// 832B7BC8: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7BCC: 806B5974  lwz r3, 0x5974(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22900 as u32) ) } as u64;
	// 832B7BD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7BD4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B7BD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7BDC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B7BE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B7BE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7BE8 size=4
    let mut pc: u32 = 0x832B7BE8;
    'dispatch: loop {
        match pc {
            0x832B7BE8 => {
    //   block [0x832B7BE8..0x832B7BEC)
	// 832B7BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7BF0 size=32
    let mut pc: u32 = 0x832B7BF0;
    'dispatch: loop {
        match pc {
            0x832B7BF0 => {
    //   block [0x832B7BF0..0x832B7C10)
	// 832B7BF0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7BF4: 806B597C  lwz r3, 0x597c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22908 as u32) ) } as u64;
	// 832B7BF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7BFC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B7C00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7C04: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B7C08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B7C0C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C10 size=4
    let mut pc: u32 = 0x832B7C10;
    'dispatch: loop {
        match pc {
            0x832B7C10 => {
    //   block [0x832B7C10..0x832B7C14)
	// 832B7C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C18 size=12
    let mut pc: u32 = 0x832B7C18;
    'dispatch: loop {
        match pc {
            0x832B7C18 => {
    //   block [0x832B7C18..0x832B7C24)
	// 832B7C18: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7C1C: 386B5998  addi r3, r11, 0x5998
	ctx.r[3].s64 = ctx.r[11].s64 + 22936;
	// 832B7C20: 4AF5D1B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C28 size=12
    let mut pc: u32 = 0x832B7C28;
    'dispatch: loop {
        match pc {
            0x832B7C28 => {
    //   block [0x832B7C28..0x832B7C34)
	// 832B7C28: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7C2C: 386B5994  addi r3, r11, 0x5994
	ctx.r[3].s64 = ctx.r[11].s64 + 22932;
	// 832B7C30: 4AF5D1A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C38 size=32
    let mut pc: u32 = 0x832B7C38;
    'dispatch: loop {
        match pc {
            0x832B7C38 => {
    //   block [0x832B7C38..0x832B7C58)
	// 832B7C38: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7C3C: 806B5980  lwz r3, 0x5980(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22912 as u32) ) } as u64;
	// 832B7C40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7C44: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B7C48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7C4C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B7C50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B7C54: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C58 size=4
    let mut pc: u32 = 0x832B7C58;
    'dispatch: loop {
        match pc {
            0x832B7C58 => {
    //   block [0x832B7C58..0x832B7C5C)
	// 832B7C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C60 size=32
    let mut pc: u32 = 0x832B7C60;
    'dispatch: loop {
        match pc {
            0x832B7C60 => {
    //   block [0x832B7C60..0x832B7C80)
	// 832B7C60: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7C64: 806B5988  lwz r3, 0x5988(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22920 as u32) ) } as u64;
	// 832B7C68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7C6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B7C70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7C74: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B7C78: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B7C7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C80 size=4
    let mut pc: u32 = 0x832B7C80;
    'dispatch: loop {
        match pc {
            0x832B7C80 => {
    //   block [0x832B7C80..0x832B7C84)
	// 832B7C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7C88 size=32
    let mut pc: u32 = 0x832B7C88;
    'dispatch: loop {
        match pc {
            0x832B7C88 => {
    //   block [0x832B7C88..0x832B7CA8)
	// 832B7C88: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7C8C: 806B5990  lwz r3, 0x5990(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22928 as u32) ) } as u64;
	// 832B7C90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7C94: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 832B7C98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7C9C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832B7CA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832B7CA4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7CA8 size=4
    let mut pc: u32 = 0x832B7CA8;
    'dispatch: loop {
        match pc {
            0x832B7CA8 => {
    //   block [0x832B7CA8..0x832B7CAC)
	// 832B7CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7CB0 size=12
    let mut pc: u32 = 0x832B7CB0;
    'dispatch: loop {
        match pc {
            0x832B7CB0 => {
    //   block [0x832B7CB0..0x832B7CBC)
	// 832B7CB0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7CB4: 806B599C  lwz r3, 0x599c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22940 as u32) ) } as u64;
	// 832B7CB8: 4B58DAF8  b 0x828457b0
	sub_828457B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7CC0 size=12
    let mut pc: u32 = 0x832B7CC0;
    'dispatch: loop {
        match pc {
            0x832B7CC0 => {
    //   block [0x832B7CC0..0x832B7CCC)
	// 832B7CC0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832B7CC4: 386BE8D4  addi r3, r11, -0x172c
	ctx.r[3].s64 = ctx.r[11].s64 + -5932;
	// 832B7CC8: 4B9C0C70  b 0x82c78938
	sub_82C78938(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7CD0 size=12
    let mut pc: u32 = 0x832B7CD0;
    'dispatch: loop {
        match pc {
            0x832B7CD0 => {
    //   block [0x832B7CD0..0x832B7CDC)
	// 832B7CD0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7CD4: 386B59AC  addi r3, r11, 0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22956;
	// 832B7CD8: 4B8DF1A0  b 0x82b96e78
	sub_82B96E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7CE0 size=84
    let mut pc: u32 = 0x832B7CE0;
    'dispatch: loop {
        match pc {
            0x832B7CE0 => {
    //   block [0x832B7CE0..0x832B7D34)
	// 832B7CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7CE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B7CEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7CF4: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 832B7CF8: 3BDF5A28  addi r30, r31, 0x5a28
	ctx.r[30].s64 = ctx.r[31].s64 + 23080;
	// 832B7CFC: 807F5A28  lwz r3, 0x5a28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23080 as u32) ) } as u64;
	// 832B7D00: 4B9D6591  bl 0x82c8e290
	ctx.lr = 0x832B7D04;
	sub_82C8E290(ctx, base);
	// 832B7D04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B7D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B7D10: 917F5A28  stw r11, 0x5a28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(23080 as u32), ctx.r[11].u32 ) };
	// 832B7D14: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B7D18: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B7D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7D28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B7D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7D38 size=84
    let mut pc: u32 = 0x832B7D38;
    'dispatch: loop {
        match pc {
            0x832B7D38 => {
    //   block [0x832B7D38..0x832B7D8C)
	// 832B7D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832B7D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7D4C: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 832B7D50: 3BDF5A1C  addi r30, r31, 0x5a1c
	ctx.r[30].s64 = ctx.r[31].s64 + 23068;
	// 832B7D54: 807F5A1C  lwz r3, 0x5a1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(23068 as u32) ) } as u64;
	// 832B7D58: 4B9D6539  bl 0x82c8e290
	ctx.lr = 0x832B7D5C;
	sub_82C8E290(ctx, base);
	// 832B7D5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B7D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B7D68: 917F5A1C  stw r11, 0x5a1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(23068 as u32), ctx.r[11].u32 ) };
	// 832B7D6C: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832B7D70: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832B7D74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832B7D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7D80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832B7D84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7D90 size=84
    let mut pc: u32 = 0x832B7D90;
    'dispatch: loop {
        match pc {
            0x832B7D90 => {
    //   block [0x832B7D90..0x832B7DE4)
	// 832B7D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832B7D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7DA0: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 832B7DA4: 3BEBE9A8  addi r31, r11, -0x1658
	ctx.r[31].s64 = ctx.r[11].s64 + -5720;
	// 832B7DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832B7DAC: 4B9D0B8D  bl 0x82c88938
	ctx.lr = 0x832B7DB0;
	sub_82C88938(ctx, base);
	// 832B7DB0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 832B7DB4: 4B9D64DD  bl 0x82c8e290
	ctx.lr = 0x832B7DB8;
	sub_82C8E290(ctx, base);
	// 832B7DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832B7DBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B7DC0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832B7DC4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832B7DC8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 832B7DCC: 913F0018  stw r9, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 832B7DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B7DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7DDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832B7DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832B7E08 size=76
    let mut pc: u32 = 0x832B7E08;
    'dispatch: loop {
        match pc {
            0x832B7E08 => {
    //   block [0x832B7E08..0x832B7E44)
	// 832B7E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832B7E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832B7E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832B7E14: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7E18: 806B7150  lwz r3, 0x7150(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29008 as u32) ) } as u64;
	// 832B7E1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832B7E20: 419A0024  beq cr6, 0x832b7e44
	if ctx.cr[6].eq {
	pc = 0x832B7E44; continue 'dispatch;
	}
	// 832B7E24: 4B3C4545  bl 0x8267c368
	ctx.lr = 0x832B7E28;
	sub_8267C368(ctx, base);
	// 832B7E28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832B7E2C: 41820018  beq 0x832b7e44
	if ctx.cr[0].eq {
	pc = 0x832B7E44; continue 'dispatch;
	}
	// 832B7E30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7E34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832B7E38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 832B7E3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832B7E40: 4E800421  bctrl
	ctx.lr = 0x832B7E44;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832B7E44 => {
    //   block [0x832B7E44..0x832B7E54)
	// 832B7E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832B7E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832B7E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832B7E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7E58 size=12
    let mut pc: u32 = 0x832B7E58;
    'dispatch: loop {
        match pc {
            0x832B7E58 => {
    //   block [0x832B7E58..0x832B7E64)
	// 832B7E58: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7E5C: 386B71D0  addi r3, r11, 0x71d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29136;
	// 832B7E60: 4BA20730  b 0x82cd8590
	sub_82CD8590(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7E68 size=12
    let mut pc: u32 = 0x832B7E68;
    'dispatch: loop {
        match pc {
            0x832B7E68 => {
    //   block [0x832B7E68..0x832B7E74)
	// 832B7E68: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7E6C: 386B721C  addi r3, r11, 0x721c
	ctx.r[3].s64 = ctx.r[11].s64 + 29212;
	// 832B7E70: 4BA20720  b 0x82cd8590
	sub_82CD8590(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7E78 size=12
    let mut pc: u32 = 0x832B7E78;
    'dispatch: loop {
        match pc {
            0x832B7E78 => {
    //   block [0x832B7E78..0x832B7E84)
	// 832B7E78: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 832B7E7C: 386B721D  addi r3, r11, 0x721d
	ctx.r[3].s64 = ctx.r[11].s64 + 29213;
	// 832B7E80: 4BA20FA8  b 0x82cd8e28
	sub_82CD8E28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7E88 size=20
    let mut pc: u32 = 0x832B7E88;
    'dispatch: loop {
        match pc {
            0x832B7E88 => {
    //   block [0x832B7E88..0x832B7E9C)
	// 832B7E88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832B7E8C: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 832B7E90: 396B3DCC  addi r11, r11, 0x3dcc
	ctx.r[11].s64 = ctx.r[11].s64 + 15820;
	// 832B7E94: 916A0440  stw r11, 0x440(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1088 as u32), ctx.r[11].u32 ) };
	// 832B7E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7EA0 size=20
    let mut pc: u32 = 0x832B7EA0;
    'dispatch: loop {
        match pc {
            0x832B7EA0 => {
    //   block [0x832B7EA0..0x832B7EB4)
	// 832B7EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832B7EA4: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 832B7EA8: 396B3DCC  addi r11, r11, 0x3dcc
	ctx.r[11].s64 = ctx.r[11].s64 + 15820;
	// 832B7EAC: 916A0460  stw r11, 0x460(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1120 as u32), ctx.r[11].u32 ) };
	// 832B7EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7EB8 size=20
    let mut pc: u32 = 0x832B7EB8;
    'dispatch: loop {
        match pc {
            0x832B7EB8 => {
    //   block [0x832B7EB8..0x832B7ECC)
	// 832B7EB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 832B7EBC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 832B7EC0: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 832B7EC4: 916A04AC  stw r11, 0x4ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1196 as u32), ctx.r[11].u32 ) };
	// 832B7EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7ED0 size=20
    let mut pc: u32 = 0x832B7ED0;
    'dispatch: loop {
        match pc {
            0x832B7ED0 => {
    //   block [0x832B7ED0..0x832B7EE4)
	// 832B7ED0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 832B7ED4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B7ED8: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 832B7EDC: 916AA2C0  stw r11, -0x5d40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23872 as u32), ctx.r[11].u32 ) };
	// 832B7EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7EE8 size=20
    let mut pc: u32 = 0x832B7EE8;
    'dispatch: loop {
        match pc {
            0x832B7EE8 => {
    //   block [0x832B7EE8..0x832B7EFC)
	// 832B7EE8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 832B7EEC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832B7EF0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 832B7EF4: 916AB440  stw r11, -0x4bc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19392 as u32), ctx.r[11].u32 ) };
	// 832B7EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F00 size=20
    let mut pc: u32 = 0x832B7F00;
    'dispatch: loop {
        match pc {
            0x832B7F00 => {
    //   block [0x832B7F00..0x832B7F14)
	// 832B7F00: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B7F04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F08: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B7F0C: 916AB6AC  stw r11, -0x4954(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18772 as u32), ctx.r[11].u32 ) };
	// 832B7F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F18 size=20
    let mut pc: u32 = 0x832B7F18;
    'dispatch: loop {
        match pc {
            0x832B7F18 => {
    //   block [0x832B7F18..0x832B7F2C)
	// 832B7F18: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7F1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F20: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7F24: 916AB7C0  stw r11, -0x4840(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18496 as u32), ctx.r[11].u32 ) };
	// 832B7F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F30 size=20
    let mut pc: u32 = 0x832B7F30;
    'dispatch: loop {
        match pc {
            0x832B7F30 => {
    //   block [0x832B7F30..0x832B7F44)
	// 832B7F30: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7F34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F38: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7F3C: 916AB8D4  stw r11, -0x472c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18220 as u32), ctx.r[11].u32 ) };
	// 832B7F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F48 size=20
    let mut pc: u32 = 0x832B7F48;
    'dispatch: loop {
        match pc {
            0x832B7F48 => {
    //   block [0x832B7F48..0x832B7F5C)
	// 832B7F48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7F4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F50: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7F54: 916AB9E8  stw r11, -0x4618(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17944 as u32), ctx.r[11].u32 ) };
	// 832B7F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F60 size=20
    let mut pc: u32 = 0x832B7F60;
    'dispatch: loop {
        match pc {
            0x832B7F60 => {
    //   block [0x832B7F60..0x832B7F74)
	// 832B7F60: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7F64: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F68: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7F6C: 916ABAFC  stw r11, -0x4504(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17668 as u32), ctx.r[11].u32 ) };
	// 832B7F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F78 size=20
    let mut pc: u32 = 0x832B7F78;
    'dispatch: loop {
        match pc {
            0x832B7F78 => {
    //   block [0x832B7F78..0x832B7F8C)
	// 832B7F78: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7F7C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F80: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7F84: 916ABC10  stw r11, -0x43f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17392 as u32), ctx.r[11].u32 ) };
	// 832B7F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7F90 size=20
    let mut pc: u32 = 0x832B7F90;
    'dispatch: loop {
        match pc {
            0x832B7F90 => {
    //   block [0x832B7F90..0x832B7FA4)
	// 832B7F90: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B7F94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7F98: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B7F9C: 916ABD28  stw r11, -0x42d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17112 as u32), ctx.r[11].u32 ) };
	// 832B7FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7FA8 size=20
    let mut pc: u32 = 0x832B7FA8;
    'dispatch: loop {
        match pc {
            0x832B7FA8 => {
    //   block [0x832B7FA8..0x832B7FBC)
	// 832B7FA8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7FAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7FB0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7FB4: 916ABE3C  stw r11, -0x41c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16836 as u32), ctx.r[11].u32 ) };
	// 832B7FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7FC0 size=20
    let mut pc: u32 = 0x832B7FC0;
    'dispatch: loop {
        match pc {
            0x832B7FC0 => {
    //   block [0x832B7FC0..0x832B7FD4)
	// 832B7FC0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7FC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7FC8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7FCC: 916ABF50  stw r11, -0x40b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16560 as u32), ctx.r[11].u32 ) };
	// 832B7FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7FD8 size=20
    let mut pc: u32 = 0x832B7FD8;
    'dispatch: loop {
        match pc {
            0x832B7FD8 => {
    //   block [0x832B7FD8..0x832B7FEC)
	// 832B7FD8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7FDC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7FE0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7FE4: 916AC064  stw r11, -0x3f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16284 as u32), ctx.r[11].u32 ) };
	// 832B7FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B7FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B7FF0 size=20
    let mut pc: u32 = 0x832B7FF0;
    'dispatch: loop {
        match pc {
            0x832B7FF0 => {
    //   block [0x832B7FF0..0x832B8004)
	// 832B7FF0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B7FF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B7FF8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B7FFC: 916AC178  stw r11, -0x3e88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16008 as u32), ctx.r[11].u32 ) };
	// 832B8000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8008 size=20
    let mut pc: u32 = 0x832B8008;
    'dispatch: loop {
        match pc {
            0x832B8008 => {
    //   block [0x832B8008..0x832B801C)
	// 832B8008: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B800C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8010: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8014: 916AC28C  stw r11, -0x3d74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15732 as u32), ctx.r[11].u32 ) };
	// 832B8018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8020 size=20
    let mut pc: u32 = 0x832B8020;
    'dispatch: loop {
        match pc {
            0x832B8020 => {
    //   block [0x832B8020..0x832B8034)
	// 832B8020: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8024: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8028: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B802C: 916AC3A0  stw r11, -0x3c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15456 as u32), ctx.r[11].u32 ) };
	// 832B8030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8038 size=20
    let mut pc: u32 = 0x832B8038;
    'dispatch: loop {
        match pc {
            0x832B8038 => {
    //   block [0x832B8038..0x832B804C)
	// 832B8038: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B803C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8040: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8044: 916AC4B4  stw r11, -0x3b4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15180 as u32), ctx.r[11].u32 ) };
	// 832B8048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8050 size=20
    let mut pc: u32 = 0x832B8050;
    'dispatch: loop {
        match pc {
            0x832B8050 => {
    //   block [0x832B8050..0x832B8064)
	// 832B8050: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8054: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8058: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B805C: 916AC5C8  stw r11, -0x3a38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14904 as u32), ctx.r[11].u32 ) };
	// 832B8060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8068 size=20
    let mut pc: u32 = 0x832B8068;
    'dispatch: loop {
        match pc {
            0x832B8068 => {
    //   block [0x832B8068..0x832B807C)
	// 832B8068: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B806C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8070: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8074: 916AC6DC  stw r11, -0x3924(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14628 as u32), ctx.r[11].u32 ) };
	// 832B8078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8080 size=20
    let mut pc: u32 = 0x832B8080;
    'dispatch: loop {
        match pc {
            0x832B8080 => {
    //   block [0x832B8080..0x832B8094)
	// 832B8080: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8084: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8088: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B808C: 916AC7F0  stw r11, -0x3810(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14352 as u32), ctx.r[11].u32 ) };
	// 832B8090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8098 size=20
    let mut pc: u32 = 0x832B8098;
    'dispatch: loop {
        match pc {
            0x832B8098 => {
    //   block [0x832B8098..0x832B80AC)
	// 832B8098: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B809C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B80A0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B80A4: 916AC904  stw r11, -0x36fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14076 as u32), ctx.r[11].u32 ) };
	// 832B80A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B80B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B80B0 size=20
    let mut pc: u32 = 0x832B80B0;
    'dispatch: loop {
        match pc {
            0x832B80B0 => {
    //   block [0x832B80B0..0x832B80C4)
	// 832B80B0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B80B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B80B8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B80BC: 916ACA18  stw r11, -0x35e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13800 as u32), ctx.r[11].u32 ) };
	// 832B80C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B80C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B80C8 size=20
    let mut pc: u32 = 0x832B80C8;
    'dispatch: loop {
        match pc {
            0x832B80C8 => {
    //   block [0x832B80C8..0x832B80DC)
	// 832B80C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B80CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B80D0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B80D4: 916ACB2C  stw r11, -0x34d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13524 as u32), ctx.r[11].u32 ) };
	// 832B80D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B80E0 size=20
    let mut pc: u32 = 0x832B80E0;
    'dispatch: loop {
        match pc {
            0x832B80E0 => {
    //   block [0x832B80E0..0x832B80F4)
	// 832B80E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B80E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B80E8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B80EC: 916ACC40  stw r11, -0x33c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13248 as u32), ctx.r[11].u32 ) };
	// 832B80F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B80F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B80F8 size=20
    let mut pc: u32 = 0x832B80F8;
    'dispatch: loop {
        match pc {
            0x832B80F8 => {
    //   block [0x832B80F8..0x832B810C)
	// 832B80F8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B80FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8100: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8104: 916ACD54  stw r11, -0x32ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12972 as u32), ctx.r[11].u32 ) };
	// 832B8108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8110 size=20
    let mut pc: u32 = 0x832B8110;
    'dispatch: loop {
        match pc {
            0x832B8110 => {
    //   block [0x832B8110..0x832B8124)
	// 832B8110: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8114: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8118: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B811C: 916ACE68  stw r11, -0x3198(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12696 as u32), ctx.r[11].u32 ) };
	// 832B8120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8128 size=20
    let mut pc: u32 = 0x832B8128;
    'dispatch: loop {
        match pc {
            0x832B8128 => {
    //   block [0x832B8128..0x832B813C)
	// 832B8128: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B812C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8130: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8134: 916ACF7C  stw r11, -0x3084(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12420 as u32), ctx.r[11].u32 ) };
	// 832B8138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8140 size=20
    let mut pc: u32 = 0x832B8140;
    'dispatch: loop {
        match pc {
            0x832B8140 => {
    //   block [0x832B8140..0x832B8154)
	// 832B8140: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8144: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8148: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B814C: 916AD098  stw r11, -0x2f68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12136 as u32), ctx.r[11].u32 ) };
	// 832B8150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8158 size=20
    let mut pc: u32 = 0x832B8158;
    'dispatch: loop {
        match pc {
            0x832B8158 => {
    //   block [0x832B8158..0x832B816C)
	// 832B8158: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B815C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8160: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8164: 916AD1AC  stw r11, -0x2e54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11860 as u32), ctx.r[11].u32 ) };
	// 832B8168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8170 size=20
    let mut pc: u32 = 0x832B8170;
    'dispatch: loop {
        match pc {
            0x832B8170 => {
    //   block [0x832B8170..0x832B8184)
	// 832B8170: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8174: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8178: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B817C: 916AD2C0  stw r11, -0x2d40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11584 as u32), ctx.r[11].u32 ) };
	// 832B8180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8188 size=20
    let mut pc: u32 = 0x832B8188;
    'dispatch: loop {
        match pc {
            0x832B8188 => {
    //   block [0x832B8188..0x832B819C)
	// 832B8188: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B818C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8190: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8194: 916AD3D4  stw r11, -0x2c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11308 as u32), ctx.r[11].u32 ) };
	// 832B8198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B81A0 size=20
    let mut pc: u32 = 0x832B81A0;
    'dispatch: loop {
        match pc {
            0x832B81A0 => {
    //   block [0x832B81A0..0x832B81B4)
	// 832B81A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B81A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B81A8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B81AC: 916AD4E8  stw r11, -0x2b18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11032 as u32), ctx.r[11].u32 ) };
	// 832B81B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B81B8 size=20
    let mut pc: u32 = 0x832B81B8;
    'dispatch: loop {
        match pc {
            0x832B81B8 => {
    //   block [0x832B81B8..0x832B81CC)
	// 832B81B8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B81BC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B81C0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B81C4: 916AD5FC  stw r11, -0x2a04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10756 as u32), ctx.r[11].u32 ) };
	// 832B81C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B81D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B81D0 size=20
    let mut pc: u32 = 0x832B81D0;
    'dispatch: loop {
        match pc {
            0x832B81D0 => {
    //   block [0x832B81D0..0x832B81E4)
	// 832B81D0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B81D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B81D8: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B81DC: 916AD710  stw r11, -0x28f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10480 as u32), ctx.r[11].u32 ) };
	// 832B81E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B81E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B81E8 size=20
    let mut pc: u32 = 0x832B81E8;
    'dispatch: loop {
        match pc {
            0x832B81E8 => {
    //   block [0x832B81E8..0x832B81FC)
	// 832B81E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B81EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B81F0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B81F4: 916AD824  stw r11, -0x27dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10204 as u32), ctx.r[11].u32 ) };
	// 832B81F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8200 size=20
    let mut pc: u32 = 0x832B8200;
    'dispatch: loop {
        match pc {
            0x832B8200 => {
    //   block [0x832B8200..0x832B8214)
	// 832B8200: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8204: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8208: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B820C: 916AD938  stw r11, -0x26c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9928 as u32), ctx.r[11].u32 ) };
	// 832B8210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8218 size=20
    let mut pc: u32 = 0x832B8218;
    'dispatch: loop {
        match pc {
            0x832B8218 => {
    //   block [0x832B8218..0x832B822C)
	// 832B8218: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B821C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8220: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8224: 916ADA4C  stw r11, -0x25b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9652 as u32), ctx.r[11].u32 ) };
	// 832B8228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8230 size=20
    let mut pc: u32 = 0x832B8230;
    'dispatch: loop {
        match pc {
            0x832B8230 => {
    //   block [0x832B8230..0x832B8244)
	// 832B8230: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8234: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8238: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B823C: 916ADF68  stw r11, -0x2098(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8344 as u32), ctx.r[11].u32 ) };
	// 832B8240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8248 size=20
    let mut pc: u32 = 0x832B8248;
    'dispatch: loop {
        match pc {
            0x832B8248 => {
    //   block [0x832B8248..0x832B825C)
	// 832B8248: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B824C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8250: 396BDB60  addi r11, r11, -0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + -9376;
	// 832B8254: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8260 size=20
    let mut pc: u32 = 0x832B8260;
    'dispatch: loop {
        match pc {
            0x832B8260 => {
    //   block [0x832B8260..0x832B8274)
	// 832B8260: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8264: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8268: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B826C: 916AE07C  stw r11, -0x1f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8068 as u32), ctx.r[11].u32 ) };
	// 832B8270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8278 size=20
    let mut pc: u32 = 0x832B8278;
    'dispatch: loop {
        match pc {
            0x832B8278 => {
    //   block [0x832B8278..0x832B828C)
	// 832B8278: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B827C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8280: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B8284: 916AE190  stw r11, -0x1e70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7792 as u32), ctx.r[11].u32 ) };
	// 832B8288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8290 size=20
    let mut pc: u32 = 0x832B8290;
    'dispatch: loop {
        match pc {
            0x832B8290 => {
    //   block [0x832B8290..0x832B82A4)
	// 832B8290: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B8294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8298: 396BE2A8  addi r11, r11, -0x1d58
	ctx.r[11].s64 = ctx.r[11].s64 + -7512;
	// 832B829C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B82A8 size=20
    let mut pc: u32 = 0x832B82A8;
    'dispatch: loop {
        match pc {
            0x832B82A8 => {
    //   block [0x832B82A8..0x832B82BC)
	// 832B82A8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B82AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B82B0: 396BE6B0  addi r11, r11, -0x1950
	ctx.r[11].s64 = ctx.r[11].s64 + -6480;
	// 832B82B4: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B82B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B82C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B82C0 size=20
    let mut pc: u32 = 0x832B82C0;
    'dispatch: loop {
        match pc {
            0x832B82C0 => {
    //   block [0x832B82C0..0x832B82D4)
	// 832B82C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B82C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B82C8: 396BEEC0  addi r11, r11, -0x1140
	ctx.r[11].s64 = ctx.r[11].s64 + -4416;
	// 832B82CC: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B82D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B82D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B82D8 size=20
    let mut pc: u32 = 0x832B82D8;
    'dispatch: loop {
        match pc {
            0x832B82D8 => {
    //   block [0x832B82D8..0x832B82EC)
	// 832B82D8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B82DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B82E0: 396BEAB8  addi r11, r11, -0x1548
	ctx.r[11].s64 = ctx.r[11].s64 + -5448;
	// 832B82E4: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B82E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B82F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B82F0 size=20
    let mut pc: u32 = 0x832B82F0;
    'dispatch: loop {
        match pc {
            0x832B82F0 => {
    //   block [0x832B82F0..0x832B8304)
	// 832B82F0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B82F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B82F8: 396BF2C8  addi r11, r11, -0xd38
	ctx.r[11].s64 = ctx.r[11].s64 + -3384;
	// 832B82FC: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8308 size=20
    let mut pc: u32 = 0x832B8308;
    'dispatch: loop {
        match pc {
            0x832B8308 => {
    //   block [0x832B8308..0x832B831C)
	// 832B8308: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B830C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8310: 396BF6D0  addi r11, r11, -0x930
	ctx.r[11].s64 = ctx.r[11].s64 + -2352;
	// 832B8314: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8320 size=20
    let mut pc: u32 = 0x832B8320;
    'dispatch: loop {
        match pc {
            0x832B8320 => {
    //   block [0x832B8320..0x832B8334)
	// 832B8320: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B8324: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8328: 396BFAD8  addi r11, r11, -0x528
	ctx.r[11].s64 = ctx.r[11].s64 + -1320;
	// 832B832C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8338 size=4
    let mut pc: u32 = 0x832B8338;
    'dispatch: loop {
        match pc {
            0x832B8338 => {
    //   block [0x832B8338..0x832B833C)
	// 832B8338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8340 size=20
    let mut pc: u32 = 0x832B8340;
    'dispatch: loop {
        match pc {
            0x832B8340 => {
    //   block [0x832B8340..0x832B8354)
	// 832B8340: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B8344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8348: 396B0308  addi r11, r11, 0x308
	ctx.r[11].s64 = ctx.r[11].s64 + 776;
	// 832B834C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8358 size=20
    let mut pc: u32 = 0x832B8358;
    'dispatch: loop {
        match pc {
            0x832B8358 => {
    //   block [0x832B8358..0x832B836C)
	// 832B8358: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B835C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8360: 396BFF00  addi r11, r11, -0x100
	ctx.r[11].s64 = ctx.r[11].s64 + -256;
	// 832B8364: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8370 size=20
    let mut pc: u32 = 0x832B8370;
    'dispatch: loop {
        match pc {
            0x832B8370 => {
    //   block [0x832B8370..0x832B8384)
	// 832B8370: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8374: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8378: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B837C: 916A071C  stw r11, 0x71c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(1820 as u32), ctx.r[11].u32 ) };
	// 832B8380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8388 size=20
    let mut pc: u32 = 0x832B8388;
    'dispatch: loop {
        match pc {
            0x832B8388 => {
    //   block [0x832B8388..0x832B839C)
	// 832B8388: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B838C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8390: 396B0830  addi r11, r11, 0x830
	ctx.r[11].s64 = ctx.r[11].s64 + 2096;
	// 832B8394: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B83A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B83A0 size=20
    let mut pc: u32 = 0x832B83A0;
    'dispatch: loop {
        match pc {
            0x832B83A0 => {
    //   block [0x832B83A0..0x832B83B4)
	// 832B83A0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B83A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B83A8: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B83AC: 916A1048  stw r11, 0x1048(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4168 as u32), ctx.r[11].u32 ) };
	// 832B83B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B83B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B83B8 size=20
    let mut pc: u32 = 0x832B83B8;
    'dispatch: loop {
        match pc {
            0x832B83B8 => {
    //   block [0x832B83B8..0x832B83CC)
	// 832B83B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B83BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B83C0: 396B0C40  addi r11, r11, 0xc40
	ctx.r[11].s64 = ctx.r[11].s64 + 3136;
	// 832B83C4: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B83C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B83D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B83D0 size=20
    let mut pc: u32 = 0x832B83D0;
    'dispatch: loop {
        match pc {
            0x832B83D0 => {
    //   block [0x832B83D0..0x832B83E4)
	// 832B83D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B83D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B83D8: 396B1168  addi r11, r11, 0x1168
	ctx.r[11].s64 = ctx.r[11].s64 + 4456;
	// 832B83DC: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B83E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B83E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B83E8 size=20
    let mut pc: u32 = 0x832B83E8;
    'dispatch: loop {
        match pc {
            0x832B83E8 => {
    //   block [0x832B83E8..0x832B83FC)
	// 832B83E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B83EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B83F0: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B83F4: 916A1978  stw r11, 0x1978(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6520 as u32), ctx.r[11].u32 ) };
	// 832B83F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8400 size=20
    let mut pc: u32 = 0x832B8400;
    'dispatch: loop {
        match pc {
            0x832B8400 => {
    //   block [0x832B8400..0x832B8414)
	// 832B8400: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B8404: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8408: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B840C: 916A1A8C  stw r11, 0x1a8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6796 as u32), ctx.r[11].u32 ) };
	// 832B8410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8418 size=20
    let mut pc: u32 = 0x832B8418;
    'dispatch: loop {
        match pc {
            0x832B8418 => {
    //   block [0x832B8418..0x832B842C)
	// 832B8418: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B841C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8420: 396B1570  addi r11, r11, 0x1570
	ctx.r[11].s64 = ctx.r[11].s64 + 5488;
	// 832B8424: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8430 size=20
    let mut pc: u32 = 0x832B8430;
    'dispatch: loop {
        match pc {
            0x832B8430 => {
    //   block [0x832B8430..0x832B8444)
	// 832B8430: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832B8434: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8438: 396B2390  addi r11, r11, 0x2390
	ctx.r[11].s64 = ctx.r[11].s64 + 9104;
	// 832B843C: 916A1BA0  stw r11, 0x1ba0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(7072 as u32), ctx.r[11].u32 ) };
	// 832B8440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8448 size=20
    let mut pc: u32 = 0x832B8448;
    'dispatch: loop {
        match pc {
            0x832B8448 => {
    //   block [0x832B8448..0x832B845C)
	// 832B8448: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 832B844C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832B8450: 396BB254  addi r11, r11, -0x4dac
	ctx.r[11].s64 = ctx.r[11].s64 + -19884;
	// 832B8454: 916A20C0  stw r11, 0x20c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8384 as u32), ctx.r[11].u32 ) };
	// 832B8458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832B8460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832B8460 size=20
    let mut pc: u32 = 0x832B8460;
    'dispatch: loop {
        match pc {
            0x832B8460 => {
    //   block [0x832B8460..0x832B8474)
	// 832B8460: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832B8464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832B8468: 396B1CB8  addi r11, r11, 0x1cb8
	ctx.r[11].s64 = ctx.r[11].s64 + 7352;
	// 832B846C: 914B0400  stw r10, 0x400(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1024 as u32), ctx.r[10].u32 ) };
	// 832B8470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


