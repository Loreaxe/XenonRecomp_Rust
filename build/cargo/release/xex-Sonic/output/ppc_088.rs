pub fn sub_827EC8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EC8F0 size=96
    let mut pc: u32 = 0x827EC8F0;
    'dispatch: loop {
        match pc {
            0x827EC8F0 => {
    //   block [0x827EC8F0..0x827EC950)
	// 827EC8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EC8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EC8F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EC8FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EC900: 3BE30204  addi r31, r3, 0x204
	ctx.r[31].s64 = ctx.r[3].s64 + 516;
	// 827EC904: 80630204  lwz r3, 0x204(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(516 as u32) ) } as u64;
	// 827EC908: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EC90C: 419A0030  beq cr6, 0x827ec93c
	if ctx.cr[6].eq {
	pc = 0x827EC93C; continue 'dispatch;
	}
	// 827EC910: 4BFE3531  bl 0x827cfe40
	ctx.lr = 0x827EC914;
	sub_827CFE40(ctx, base);
	// 827EC914: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EC918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EC91C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EC920: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EC924: 419A0018  beq cr6, 0x827ec93c
	if ctx.cr[6].eq {
	pc = 0x827EC93C; continue 'dispatch;
	}
	// 827EC928: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EC92C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EC930: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EC934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EC938: 4E800421  bctrl
	ctx.lr = 0x827EC93C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EC93C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EC940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EC944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EC948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EC94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EC950 size=196
    let mut pc: u32 = 0x827EC950;
    'dispatch: loop {
        match pc {
            0x827EC950 => {
    //   block [0x827EC950..0x827ECA14)
	// 827EC950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EC954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EC958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EC95C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EC960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EC964: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EC968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EC96C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827EC970: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EC974: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EC978: 4BAD3FC1  bl 0x822c0938
	ctx.lr = 0x827EC97C;
	sub_822C0938(ctx, base);
	// 827EC97C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EC980: 41820028  beq 0x827ec9a8
	if ctx.cr[0].eq {
	pc = 0x827EC9A8; continue 'dispatch;
	}
	// 827EC984: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EC988: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827EC98C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827EC990: 392B5DB0  addi r9, r11, 0x5db0
	ctx.r[9].s64 = ctx.r[11].s64 + 23984;
	// 827EC994: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827EC998: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EC99C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827EC9A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827EC9A4: 48000008  b 0x827ec9ac
	pc = 0x827EC9AC; continue 'dispatch;
	// 827EC9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EC9AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EC9B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EC9B4: 409A0044  bne cr6, 0x827ec9f8
	if !ctx.cr[6].eq {
	pc = 0x827EC9F8; continue 'dispatch;
	}
	// 827EC9B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EC9BC: 419A001C  beq cr6, 0x827ec9d8
	if ctx.cr[6].eq {
	pc = 0x827EC9D8; continue 'dispatch;
	}
	// 827EC9C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EC9C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EC9C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EC9CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EC9D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EC9D4: 4E800421  bctrl
	ctx.lr = 0x827EC9D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EC9D8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827EC9DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EC9E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EC9E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827EC9E8: 816BC6A4  lwz r11, -0x395c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14684 as u32) ) } as u64;
	// 827EC9EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827EC9F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827EC9F4: 4BAD360D  bl 0x822c0000
	ctx.lr = 0x827EC9F8;
	sub_822C0000(ctx, base);
	// 827EC9F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EC9FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECA08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ECA0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ECA18 size=96
    let mut pc: u32 = 0x827ECA18;
    'dispatch: loop {
        match pc {
            0x827ECA18 => {
    //   block [0x827ECA18..0x827ECA78)
	// 827ECA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECA20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ECA24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECA28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECA2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ECA30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827ECA34: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 827ECA38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECA3C: 419A0008  beq cr6, 0x827eca44
	if ctx.cr[6].eq {
	pc = 0x827ECA44; continue 'dispatch;
	}
	// 827ECA40: 4BAD3E51  bl 0x822c0890
	ctx.lr = 0x827ECA44;
	sub_822C0890(ctx, base);
	// 827ECA44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECA48: 48670E01  bl 0x82e5d848
	ctx.lr = 0x827ECA4C;
	sub_82E5D848(ctx, base);
	// 827ECA4C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827ECA50: 4182000C  beq 0x827eca5c
	if ctx.cr[0].eq {
	pc = 0x827ECA5C; continue 'dispatch;
	}
	// 827ECA54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECA58: 48605981  bl 0x82df23d8
	ctx.lr = 0x827ECA5C;
	sub_82DF23D8(ctx, base);
	// 827ECA5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECA60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECA6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ECA70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ECA78 size=312
    let mut pc: u32 = 0x827ECA78;
    'dispatch: loop {
        match pc {
            0x827ECA78 => {
    //   block [0x827ECA78..0x827ECBB0)
	// 827ECA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECA80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ECA84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECA88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECA8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ECA90: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ECA94: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827ECA98: 396B5DDC  addi r11, r11, 0x5ddc
	ctx.r[11].s64 = ctx.r[11].s64 + 24028;
	// 827ECA9C: 394A5DC4  addi r10, r10, 0x5dc4
	ctx.r[10].s64 = ctx.r[10].s64 + 24004;
	// 827ECAA0: 807F0204  lwz r3, 0x204(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(516 as u32) ) } as u64;
	// 827ECAA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827ECAA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECAAC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827ECAB0: 419A0018  beq cr6, 0x827ecac8
	if ctx.cr[6].eq {
	pc = 0x827ECAC8; continue 'dispatch;
	}
	// 827ECAB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECAB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827ECABC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECAC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827ECAC4: 4E800421  bctrl
	ctx.lr = 0x827ECAC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827ECAC8: 807F0200  lwz r3, 0x200(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(512 as u32) ) } as u64;
	// 827ECACC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECAD0: 419A0008  beq cr6, 0x827ecad8
	if ctx.cr[6].eq {
	pc = 0x827ECAD8; continue 'dispatch;
	}
	// 827ECAD4: 4BAD3DBD  bl 0x822c0890
	ctx.lr = 0x827ECAD8;
	sub_822C0890(ctx, base);
	// 827ECAD8: 807F01F8  lwz r3, 0x1f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 827ECADC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECAE0: 419A0008  beq cr6, 0x827ecae8
	if ctx.cr[6].eq {
	pc = 0x827ECAE8; continue 'dispatch;
	}
	// 827ECAE4: 4BAD3DAD  bl 0x822c0890
	ctx.lr = 0x827ECAE8;
	sub_822C0890(ctx, base);
	// 827ECAE8: 807F01EC  lwz r3, 0x1ec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(492 as u32) ) } as u64;
	// 827ECAEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECAF0: 419A0008  beq cr6, 0x827ecaf8
	if ctx.cr[6].eq {
	pc = 0x827ECAF8; continue 'dispatch;
	}
	// 827ECAF4: 4BAD3D9D  bl 0x822c0890
	ctx.lr = 0x827ECAF8;
	sub_822C0890(ctx, base);
	// 827ECAF8: 807F01E0  lwz r3, 0x1e0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(480 as u32) ) } as u64;
	// 827ECAFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB00: 419A0008  beq cr6, 0x827ecb08
	if ctx.cr[6].eq {
	pc = 0x827ECB08; continue 'dispatch;
	}
	// 827ECB04: 4BAD3D8D  bl 0x822c0890
	ctx.lr = 0x827ECB08;
	sub_822C0890(ctx, base);
	// 827ECB08: 807F01D8  lwz r3, 0x1d8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(472 as u32) ) } as u64;
	// 827ECB0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB10: 419A0008  beq cr6, 0x827ecb18
	if ctx.cr[6].eq {
	pc = 0x827ECB18; continue 'dispatch;
	}
	// 827ECB14: 4BAD3D7D  bl 0x822c0890
	ctx.lr = 0x827ECB18;
	sub_822C0890(ctx, base);
	// 827ECB18: 807F01D0  lwz r3, 0x1d0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(464 as u32) ) } as u64;
	// 827ECB1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB20: 419A0008  beq cr6, 0x827ecb28
	if ctx.cr[6].eq {
	pc = 0x827ECB28; continue 'dispatch;
	}
	// 827ECB24: 4BAD3D6D  bl 0x822c0890
	ctx.lr = 0x827ECB28;
	sub_822C0890(ctx, base);
	// 827ECB28: 807F01C8  lwz r3, 0x1c8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(456 as u32) ) } as u64;
	// 827ECB2C: 3BDF0164  addi r30, r31, 0x164
	ctx.r[30].s64 = ctx.r[31].s64 + 356;
	// 827ECB30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB34: 419A0008  beq cr6, 0x827ecb3c
	if ctx.cr[6].eq {
	pc = 0x827ECB3C; continue 'dispatch;
	}
	// 827ECB38: 4BAD3D59  bl 0x822c0890
	ctx.lr = 0x827ECB3C;
	sub_822C0890(ctx, base);
	// 827ECB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827ECB40: 48670D09  bl 0x82e5d848
	ctx.lr = 0x827ECB44;
	sub_82E5D848(ctx, base);
	// 827ECB44: 807F0160  lwz r3, 0x160(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 827ECB48: 3BDF00FC  addi r30, r31, 0xfc
	ctx.r[30].s64 = ctx.r[31].s64 + 252;
	// 827ECB4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB50: 419A0008  beq cr6, 0x827ecb58
	if ctx.cr[6].eq {
	pc = 0x827ECB58; continue 'dispatch;
	}
	// 827ECB54: 4BAD3D3D  bl 0x822c0890
	ctx.lr = 0x827ECB58;
	sub_822C0890(ctx, base);
	// 827ECB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827ECB5C: 48670CED  bl 0x82e5d848
	ctx.lr = 0x827ECB60;
	sub_82E5D848(ctx, base);
	// 827ECB60: 807F00F8  lwz r3, 0xf8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 827ECB64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB68: 419A0008  beq cr6, 0x827ecb70
	if ctx.cr[6].eq {
	pc = 0x827ECB70; continue 'dispatch;
	}
	// 827ECB6C: 4BAD3D25  bl 0x822c0890
	ctx.lr = 0x827ECB70;
	sub_822C0890(ctx, base);
	// 827ECB70: 807F00F0  lwz r3, 0xf0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 827ECB74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB78: 419A0008  beq cr6, 0x827ecb80
	if ctx.cr[6].eq {
	pc = 0x827ECB80; continue 'dispatch;
	}
	// 827ECB7C: 4BAD3D15  bl 0x822c0890
	ctx.lr = 0x827ECB80;
	sub_822C0890(ctx, base);
	// 827ECB80: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 827ECB84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECB88: 419A0008  beq cr6, 0x827ecb90
	if ctx.cr[6].eq {
	pc = 0x827ECB90; continue 'dispatch;
	}
	// 827ECB8C: 4BAD3D05  bl 0x822c0890
	ctx.lr = 0x827ECB90;
	sub_822C0890(ctx, base);
	// 827ECB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECB94: 4BB627BD  bl 0x8234f350
	ctx.lr = 0x827ECB98;
	sub_8234F350(ctx, base);
	// 827ECB98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECBA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ECBA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827ECBB0 size=8
    let mut pc: u32 = 0x827ECBB0;
    'dispatch: loop {
        match pc {
            0x827ECBB0 => {
    //   block [0x827ECBB0..0x827ECBB8)
	// 827ECBB0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827ECBB4: 4800093C  b 0x827ed4f0
	sub_827ED4F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ECBB8 size=188
    let mut pc: u32 = 0x827ECBB8;
    'dispatch: loop {
        match pc {
            0x827ECBB8 => {
    //   block [0x827ECBB8..0x827ECC74)
	// 827ECBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECBC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ECBC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECBC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECBCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ECBD0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827ECBD4: 4BD24E05  bl 0x825119d8
	ctx.lr = 0x827ECBD8;
	sub_825119D8(ctx, base);
	// 827ECBD8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ECBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ECBE0: 388B5E18  addi r4, r11, 0x5e18
	ctx.r[4].s64 = ctx.r[11].s64 + 24088;
	// 827ECBE4: 38A00059  li r5, 0x59
	ctx.r[5].s64 = 89;
	// 827ECBE8: 386000A8  li r3, 0xa8
	ctx.r[3].s64 = 168;
	// 827ECBEC: 486057FD  bl 0x82df23e8
	ctx.lr = 0x827ECBF0;
	sub_82DF23E8(ctx, base);
	// 827ECBF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ECBF4: 41820010  beq 0x827ecc04
	if ctx.cr[0].eq {
	pc = 0x827ECC04; continue 'dispatch;
	}
	// 827ECBF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827ECBFC: 4BFE3955  bl 0x827d0550
	ctx.lr = 0x827ECC00;
	sub_827D0550(ctx, base);
	// 827ECC00: 48000008  b 0x827ecc08
	pc = 0x827ECC08; continue 'dispatch;
	// 827ECC04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827ECC08: 817F0204  lwz r11, 0x204(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(516 as u32) ) } as u64;
	// 827ECC0C: 907F0204  stw r3, 0x204(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(516 as u32), ctx.r[3].u32 ) };
	// 827ECC10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECC14: 419A001C  beq cr6, 0x827ecc30
	if ctx.cr[6].eq {
	pc = 0x827ECC30; continue 'dispatch;
	}
	// 827ECC18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECC1C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827ECC20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827ECC24: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECC28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827ECC2C: 4E800421  bctrl
	ctx.lr = 0x827ECC30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827ECC30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ECC34: 83FF01FC  lwz r31, 0x1fc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 827ECC38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827ECC3C: 4BD254DD  bl 0x82512118
	ctx.lr = 0x827ECC40;
	sub_82512118(ctx, base);
	// 827ECC40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ECC44: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827ECC48: 486261E9  bl 0x82e12e30
	ctx.lr = 0x827ECC4C;
	sub_82E12E30(ctx, base);
	// 827ECC4C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ECC50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECC54: 419A0008  beq cr6, 0x827ecc5c
	if ctx.cr[6].eq {
	pc = 0x827ECC5C; continue 'dispatch;
	}
	// 827ECC58: 4BAD3C39  bl 0x822c0890
	ctx.lr = 0x827ECC5C;
	sub_822C0890(ctx, base);
	// 827ECC5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECC68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ECC6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ECC78 size=108
    let mut pc: u32 = 0x827ECC78;
    'dispatch: loop {
        match pc {
            0x827ECC78 => {
    //   block [0x827ECC78..0x827ECCE4)
	// 827ECC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECC80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ECC84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECC88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECC8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ECC90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827ECC94: 807F01CC  lwz r3, 0x1cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 827ECC98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECC9C: 419A0008  beq cr6, 0x827ecca4
	if ctx.cr[6].eq {
	pc = 0x827ECCA4; continue 'dispatch;
	}
	// 827ECCA0: 4BFFD4E9  bl 0x827ea188
	ctx.lr = 0x827ECCA4;
	sub_827EA188(ctx, base);
	// 827ECCA4: 387F00FC  addi r3, r31, 0xfc
	ctx.r[3].s64 = ctx.r[31].s64 + 252;
	// 827ECCA8: 4866F9F1  bl 0x82e5c698
	ctx.lr = 0x827ECCAC;
	sub_82E5C698(ctx, base);
	// 827ECCAC: 387F0164  addi r3, r31, 0x164
	ctx.r[3].s64 = ctx.r[31].s64 + 356;
	// 827ECCB0: 4866F9E9  bl 0x82e5c698
	ctx.lr = 0x827ECCB4;
	sub_82E5C698(ctx, base);
	// 827ECCB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827ECCB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECCBC: 4BD253AD  bl 0x82512068
	ctx.lr = 0x827ECCC0;
	sub_82512068(ctx, base);
	// 827ECCC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827ECCC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECCC8: 4BFFFC29  bl 0x827ec8f0
	ctx.lr = 0x827ECCCC;
	sub_827EC8F0(ctx, base);
	// 827ECCCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECCD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ECCDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827ECCE8 size=108
    let mut pc: u32 = 0x827ECCE8;
    'dispatch: loop {
        match pc {
            0x827ECCE8 => {
    //   block [0x827ECCE8..0x827ECD54)
	// 827ECCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECCF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECCF4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECCF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ECCFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827ECD00: 48606F01  bl 0x82df3c00
	ctx.lr = 0x827ECD04;
	sub_82DF3C00(ctx, base);
	// 827ECD04: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827ECD08: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827ECD0C: 389F00FC  addi r4, r31, 0xfc
	ctx.r[4].s64 = ctx.r[31].s64 + 252;
	// 827ECD10: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ECD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ECD18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827ECD1C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827ECD20: 48671C41  bl 0x82e5e960
	ctx.lr = 0x827ECD24;
	sub_82E5E960(ctx, base);
	// 827ECD24: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827ECD28: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECD2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECD30: 419A000C  beq cr6, 0x827ecd3c
	if ctx.cr[6].eq {
	pc = 0x827ECD3C; continue 'dispatch;
	}
	// 827ECD34: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827ECD38: 4BAD3B59  bl 0x822c0890
	ctx.lr = 0x827ECD3C;
	sub_822C0890(ctx, base);
	// 827ECD3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECD40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECD44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECD48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECD4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827ECD58 size=168
    let mut pc: u32 = 0x827ECD58;
    'dispatch: loop {
        match pc {
            0x827ECD58 => {
    //   block [0x827ECD58..0x827ECE00)
	// 827ECD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECD60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECD64: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECD68: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ECD6C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECD70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECD74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827ECD78: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827ECD7C: 419A0024  beq cr6, 0x827ecda0
	if ctx.cr[6].eq {
	pc = 0x827ECDA0; continue 'dispatch;
	}
	// 827ECD80: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ECD84: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ECD88: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ECD8C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ECD90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ECD94: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ECD98: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ECD9C: 4082FFE8  bne 0x827ecd84
	if !ctx.cr[0].eq {
	pc = 0x827ECD84; continue 'dispatch;
	}
	// 827ECDA0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827ECDA4: 388300FC  addi r4, r3, 0xfc
	ctx.r[4].s64 = ctx.r[3].s64 + 252;
	// 827ECDA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827ECDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ECDB0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827ECDB4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ECDB8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827ECDBC: 486718ED  bl 0x82e5e6a8
	ctx.lr = 0x827ECDC0;
	sub_82E5E6A8(ctx, base);
	// 827ECDC0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827ECDC4: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECDC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECDCC: 419A000C  beq cr6, 0x827ecdd8
	if ctx.cr[6].eq {
	pc = 0x827ECDD8; continue 'dispatch;
	}
	// 827ECDD0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827ECDD4: 4BAD3ABD  bl 0x822c0890
	ctx.lr = 0x827ECDD8;
	sub_822C0890(ctx, base);
	// 827ECDD8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ECDDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECDE0: 419A0008  beq cr6, 0x827ecde8
	if ctx.cr[6].eq {
	pc = 0x827ECDE8; continue 'dispatch;
	}
	// 827ECDE4: 4BAD3AAD  bl 0x822c0890
	ctx.lr = 0x827ECDE8;
	sub_822C0890(ctx, base);
	// 827ECDE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECDEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECDF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ECE00 size=148
    let mut pc: u32 = 0x827ECE00;
    'dispatch: loop {
        match pc {
            0x827ECE00 => {
    //   block [0x827ECE00..0x827ECE94)
	// 827ECE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECE08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ECE0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECE10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECE14: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827ECE18: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827ECE1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827ECE20: 3BEBAC78  addi r31, r11, -0x5388
	ctx.r[31].s64 = ctx.r[11].s64 + -21384;
	// 827ECE24: 816AAC7C  lwz r11, -0x5384(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21380 as u32) ) } as u64;
	// 827ECE28: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827ECE2C: 40820028  bne 0x827ece54
	if !ctx.cr[0].eq {
	pc = 0x827ECE54; continue 'dispatch;
	}
	// 827ECE30: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827ECE34: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827ECE38: 916AAC7C  stw r11, -0x5384(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21380 as u32), ctx.r[11].u32 ) };
	// 827ECE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECE40: 38899BC9  addi r4, r9, -0x6437
	ctx.r[4].s64 = ctx.r[9].s64 + -25655;
	// 827ECE44: 48606BC5  bl 0x82df3a08
	ctx.lr = 0x827ECE48;
	sub_82DF3A08(ctx, base);
	// 827ECE48: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 827ECE4C: 386BF948  addi r3, r11, -0x6b8
	ctx.r[3].s64 = ctx.r[11].s64 + -1720;
	// 827ECE50: 489BB689  bl 0x831a84d8
	ctx.lr = 0x827ECE54;
	sub_831A84D8(ctx, base);
	// 827ECE54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827ECE58: 4BFFF601  bl 0x827ec458
	ctx.lr = 0x827ECE5C;
	sub_827EC458(ctx, base);
	// 827ECE5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ECE60: 41820018  beq 0x827ece78
	if ctx.cr[0].eq {
	pc = 0x827ECE78; continue 'dispatch;
	}
	// 827ECE64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECE68: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 827ECE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827ECE70: 4E800421  bctrl
	ctx.lr = 0x827ECE74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827ECE74: 48000008  b 0x827ece7c
	pc = 0x827ECE7C; continue 'dispatch;
	// 827ECE78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECE7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECE88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ECE8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827ECE98 size=148
    let mut pc: u32 = 0x827ECE98;
    'dispatch: loop {
        match pc {
            0x827ECE98 => {
    //   block [0x827ECE98..0x827ECF2C)
	// 827ECE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECEA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECEA4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ECEA8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECEAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECEB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827ECEB4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827ECEB8: 419A0024  beq cr6, 0x827ecedc
	if ctx.cr[6].eq {
	pc = 0x827ECEDC; continue 'dispatch;
	}
	// 827ECEBC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ECEC0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ECEC4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ECEC8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ECECC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ECED0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ECED4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ECED8: 4082FFE8  bne 0x827ecec0
	if !ctx.cr[0].eq {
	pc = 0x827ECEC0; continue 'dispatch;
	}
	// 827ECEDC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827ECEE0: 388300FC  addi r4, r3, 0xfc
	ctx.r[4].s64 = ctx.r[3].s64 + 252;
	// 827ECEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827ECEE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ECEEC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827ECEF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ECEF4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827ECEF8: 48671889  bl 0x82e5e780
	ctx.lr = 0x827ECEFC;
	sub_82E5E780(ctx, base);
	// 827ECEFC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827ECF00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECF04: 419A0008  beq cr6, 0x827ecf0c
	if ctx.cr[6].eq {
	pc = 0x827ECF0C; continue 'dispatch;
	}
	// 827ECF08: 4BAD3989  bl 0x822c0890
	ctx.lr = 0x827ECF0C;
	sub_822C0890(ctx, base);
	// 827ECF0C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ECF10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECF14: 419A0008  beq cr6, 0x827ecf1c
	if ctx.cr[6].eq {
	pc = 0x827ECF1C; continue 'dispatch;
	}
	// 827ECF18: 4BAD3979  bl 0x822c0890
	ctx.lr = 0x827ECF1C;
	sub_822C0890(ctx, base);
	// 827ECF1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827ECF30 size=168
    let mut pc: u32 = 0x827ECF30;
    'dispatch: loop {
        match pc {
            0x827ECF30 => {
    //   block [0x827ECF30..0x827ECFD8)
	// 827ECF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECF38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ECF3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECF40: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ECF44: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECF48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECF4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827ECF50: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827ECF54: 419A0024  beq cr6, 0x827ecf78
	if ctx.cr[6].eq {
	pc = 0x827ECF78; continue 'dispatch;
	}
	// 827ECF58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ECF5C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ECF60: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ECF64: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ECF68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ECF6C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ECF70: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ECF74: 4082FFE8  bne 0x827ecf5c
	if !ctx.cr[0].eq {
	pc = 0x827ECF5C; continue 'dispatch;
	}
	// 827ECF78: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827ECF7C: 38830164  addi r4, r3, 0x164
	ctx.r[4].s64 = ctx.r[3].s64 + 356;
	// 827ECF80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827ECF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ECF88: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827ECF8C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ECF90: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827ECF94: 48671715  bl 0x82e5e6a8
	ctx.lr = 0x827ECF98;
	sub_82E5E6A8(ctx, base);
	// 827ECF98: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827ECF9C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECFA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECFA4: 419A000C  beq cr6, 0x827ecfb0
	if ctx.cr[6].eq {
	pc = 0x827ECFB0; continue 'dispatch;
	}
	// 827ECFA8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827ECFAC: 4BAD38E5  bl 0x822c0890
	ctx.lr = 0x827ECFB0;
	sub_822C0890(ctx, base);
	// 827ECFB0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ECFB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ECFB8: 419A0008  beq cr6, 0x827ecfc0
	if ctx.cr[6].eq {
	pc = 0x827ECFC0; continue 'dispatch;
	}
	// 827ECFBC: 4BAD38D5  bl 0x822c0890
	ctx.lr = 0x827ECFC0;
	sub_822C0890(ctx, base);
	// 827ECFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ECFC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ECFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ECFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ECFD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ECFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ECFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827ECFD8 size=148
    let mut pc: u32 = 0x827ECFD8;
    'dispatch: loop {
        match pc {
            0x827ECFD8 => {
    //   block [0x827ECFD8..0x827ED06C)
	// 827ECFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ECFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ECFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ECFE4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ECFE8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ECFEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ECFF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827ECFF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827ECFF8: 419A0024  beq cr6, 0x827ed01c
	if ctx.cr[6].eq {
	pc = 0x827ED01C; continue 'dispatch;
	}
	// 827ECFFC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ED000: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ED004: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED008: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ED00C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ED010: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ED014: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED018: 4082FFE8  bne 0x827ed000
	if !ctx.cr[0].eq {
	pc = 0x827ED000; continue 'dispatch;
	}
	// 827ED01C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827ED020: 38830164  addi r4, r3, 0x164
	ctx.r[4].s64 = ctx.r[3].s64 + 356;
	// 827ED024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827ED028: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ED02C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827ED030: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ED034: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827ED038: 48671749  bl 0x82e5e780
	ctx.lr = 0x827ED03C;
	sub_82E5E780(ctx, base);
	// 827ED03C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827ED040: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ED044: 419A0008  beq cr6, 0x827ed04c
	if ctx.cr[6].eq {
	pc = 0x827ED04C; continue 'dispatch;
	}
	// 827ED048: 4BAD3849  bl 0x822c0890
	ctx.lr = 0x827ED04C;
	sub_822C0890(ctx, base);
	// 827ED04C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ED050: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ED054: 419A0008  beq cr6, 0x827ed05c
	if ctx.cr[6].eq {
	pc = 0x827ED05C; continue 'dispatch;
	}
	// 827ED058: 4BAD3839  bl 0x822c0890
	ctx.lr = 0x827ED05C;
	sub_822C0890(ctx, base);
	// 827ED05C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED070 size=320
    let mut pc: u32 = 0x827ED070;
    'dispatch: loop {
        match pc {
            0x827ED070 => {
    //   block [0x827ED070..0x827ED1B0)
	// 827ED070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED074: 489BB0F5  bl 0x831a8168
	ctx.lr = 0x827ED078;
	sub_831A8130(ctx, base);
	// 827ED078: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED07C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827ED080: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED084: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827ED088: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED08C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED090: 3BCB0028  addi r30, r11, 0x28
	ctx.r[30].s64 = ctx.r[11].s64 + 40;
	// 827ED094: 409A0008  bne cr6, 0x827ed09c
	if !ctx.cr[6].eq {
	pc = 0x827ED09C; continue 'dispatch;
	}
	// 827ED098: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827ED09C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED0A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ED0A4: 4BD22425  bl 0x8250f4c8
	ctx.lr = 0x827ED0A8;
	sub_8250F4C8(ctx, base);
	// 827ED0A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED0AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED0B0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 827ED0B4: 409A0008  bne cr6, 0x827ed0bc
	if !ctx.cr[6].eq {
	pc = 0x827ED0BC; continue 'dispatch;
	}
	// 827ED0B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827ED0BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827ED0C0: 4BD1B6E1  bl 0x825087a0
	ctx.lr = 0x827ED0C4;
	sub_825087A0(ctx, base);
	// 827ED0C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ED0C8: 48604BC9  bl 0x82df1c90
	ctx.lr = 0x827ED0CC;
	sub_82DF1C90(ctx, base);
	// 827ED0CC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 827ED0D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827ED0D4: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 827ED0D8: 48606931  bl 0x82df3a08
	ctx.lr = 0x827ED0DC;
	sub_82DF3A08(ctx, base);
	// 827ED0DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED0E0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827ED0E4: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED0E8: 4BD223E1  bl 0x8250f4c8
	ctx.lr = 0x827ED0EC;
	sub_8250F4C8(ctx, base);
	// 827ED0EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED0F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED0F4: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 827ED0F8: 409A0008  bne cr6, 0x827ed100
	if !ctx.cr[6].eq {
	pc = 0x827ED100; continue 'dispatch;
	}
	// 827ED0FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827ED100: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827ED104: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827ED108: 4BD1B679  bl 0x82508780
	ctx.lr = 0x827ED10C;
	sub_82508780(ctx, base);
	// 827ED10C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827ED110: 48604B81  bl 0x82df1c90
	ctx.lr = 0x827ED114;
	sub_82DF1C90(ctx, base);
	// 827ED114: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827ED118: 48606311  bl 0x82df3428
	ctx.lr = 0x827ED11C;
	sub_82DF3428(ctx, base);
	// 827ED11C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ED120: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED128: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827ED12C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827ED130: 419A0024  beq cr6, 0x827ed154
	if ctx.cr[6].eq {
	pc = 0x827ED154; continue 'dispatch;
	}
	// 827ED134: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ED138: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ED13C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED140: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ED144: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ED148: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ED14C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED150: 4082FFE8  bne 0x827ed138
	if !ctx.cr[0].eq {
	pc = 0x827ED138; continue 'dispatch;
	}
	// 827ED154: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED158: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827ED15C: 4BD2236D  bl 0x8250f4c8
	ctx.lr = 0x827ED160;
	sub_8250F4C8(ctx, base);
	// 827ED160: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED164: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED168: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 827ED16C: 409A0008  bne cr6, 0x827ed174
	if !ctx.cr[6].eq {
	pc = 0x827ED174; continue 'dispatch;
	}
	// 827ED170: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827ED174: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED178: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827ED17C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 827ED180: 4BD22399  bl 0x8250f518
	ctx.lr = 0x827ED184;
	sub_8250F518(ctx, base);
	// 827ED184: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827ED188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED18C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827ED190: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 827ED194: 4BD20375  bl 0x8250d508
	ctx.lr = 0x827ED198;
	sub_8250D508(ctx, base);
	// 827ED198: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827ED19C: 48604AF5  bl 0x82df1c90
	ctx.lr = 0x827ED1A0;
	sub_82DF1C90(ctx, base);
	// 827ED1A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827ED1A4: 48604AED  bl 0x82df1c90
	ctx.lr = 0x827ED1A8;
	sub_82DF1C90(ctx, base);
	// 827ED1A8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827ED1AC: 489BB00C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED1B0 size=48
    let mut pc: u32 = 0x827ED1B0;
    'dispatch: loop {
        match pc {
            0x827ED1B0 => {
    //   block [0x827ED1B0..0x827ED1E0)
	// 827ED1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED1B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED1BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED1C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED1C4: 4BD24FB5  bl 0x82512178
	ctx.lr = 0x827ED1C8;
	sub_82512178(ctx, base);
	// 827ED1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED1CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827ED1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED1D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED1E0 size=212
    let mut pc: u32 = 0x827ED1E0;
    'dispatch: loop {
        match pc {
            0x827ED1E0 => {
    //   block [0x827ED1E0..0x827ED2B4)
	// 827ED1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED1E4: 489BAF89  bl 0x831a816c
	ctx.lr = 0x827ED1E8;
	sub_831A8130(ctx, base);
	// 827ED1E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED1EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827ED1F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED1F4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED1F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED1FC: 419A00B0  beq cr6, 0x827ed2ac
	if ctx.cr[6].eq {
	pc = 0x827ED2AC; continue 'dispatch;
	}
	// 827ED200: 817F0200  lwz r11, 0x200(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(512 as u32) ) } as u64;
	// 827ED204: 815F01FC  lwz r10, 0x1fc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 827ED208: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED20C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827ED210: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827ED214: 419A0024  beq cr6, 0x827ed238
	if ctx.cr[6].eq {
	pc = 0x827ED238; continue 'dispatch;
	}
	// 827ED218: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ED21C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ED220: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED224: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ED228: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ED22C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ED230: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED234: 4082FFE8  bne 0x827ed21c
	if !ctx.cr[0].eq {
	pc = 0x827ED21C; continue 'dispatch;
	}
	// 827ED238: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED23C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ED240: 4BD22289  bl 0x8250f4c8
	ctx.lr = 0x827ED244;
	sub_8250F4C8(ctx, base);
	// 827ED244: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED248: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED24C: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 827ED250: 409A0008  bne cr6, 0x827ed258
	if !ctx.cr[6].eq {
	pc = 0x827ED258; continue 'dispatch;
	}
	// 827ED254: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827ED258: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827ED25C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED260: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827ED264: 482F7B65  bl 0x82ae4dc8
	ctx.lr = 0x827ED268;
	sub_82AE4DC8(ctx, base);
	// 827ED268: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED26C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED270: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827ED274: 4BD222A5  bl 0x8250f518
	ctx.lr = 0x827ED278;
	sub_8250F518(ctx, base);
	// 827ED278: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827ED27C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827ED280: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827ED284: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827ED288: 4BD20281  bl 0x8250d508
	ctx.lr = 0x827ED28C;
	sub_8250D508(ctx, base);
	// 827ED28C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ED290: 48604A01  bl 0x82df1c90
	ctx.lr = 0x827ED294;
	sub_82DF1C90(ctx, base);
	// 827ED294: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827ED298: 486049F9  bl 0x82df1c90
	ctx.lr = 0x827ED29C;
	sub_82DF1C90(ctx, base);
	// 827ED29C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ED2A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ED2A4: 419A0008  beq cr6, 0x827ed2ac
	if ctx.cr[6].eq {
	pc = 0x827ED2AC; continue 'dispatch;
	}
	// 827ED2A8: 4BAD35E9  bl 0x822c0890
	ctx.lr = 0x827ED2AC;
	sub_822C0890(ctx, base);
	// 827ED2AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827ED2B0: 489BAF0C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED2B8 size=100
    let mut pc: u32 = 0x827ED2B8;
    'dispatch: loop {
        match pc {
            0x827ED2B8 => {
    //   block [0x827ED2B8..0x827ED31C)
	// 827ED2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED2C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ED2C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED2C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED2CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827ED2D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827ED2D4: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 827ED2D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ED2DC: 41820018  beq 0x827ed2f4
	if ctx.cr[0].eq {
	pc = 0x827ED2F4; continue 'dispatch;
	}
	// 827ED2E0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827ED2E4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827ED2E8: 4182000C  beq 0x827ed2f4
	if ctx.cr[0].eq {
	pc = 0x827ED2F4; continue 'dispatch;
	}
	// 827ED2EC: 389E01D4  addi r4, r30, 0x1d4
	ctx.r[4].s64 = ctx.r[30].s64 + 468;
	// 827ED2F0: 4BFFFEF1  bl 0x827ed1e0
	ctx.lr = 0x827ED2F4;
	sub_827ED1E0(ctx, base);
	// 827ED2F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827ED2F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED2FC: 387E0028  addi r3, r30, 0x28
	ctx.r[3].s64 = ctx.r[30].s64 + 40;
	// 827ED300: 4BD25319  bl 0x82512618
	ctx.lr = 0x827ED304;
	sub_82512618(ctx, base);
	// 827ED304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED320 size=108
    let mut pc: u32 = 0x827ED320;
    'dispatch: loop {
        match pc {
            0x827ED320 => {
    //   block [0x827ED320..0x827ED38C)
	// 827ED320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED324: 489BAE49  bl 0x831a816c
	ctx.lr = 0x827ED328;
	sub_831A8130(ctx, base);
	// 827ED328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED32C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827ED330: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED334: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827ED338: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827ED33C: 41820038  beq 0x827ed374
	if ctx.cr[0].eq {
	pc = 0x827ED374; continue 'dispatch;
	}
	// 827ED340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED344: 489BC645  bl 0x831a9988
	ctx.lr = 0x827ED348;
	sub_831A9988(ctx, base);
	// 827ED348: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 827ED34C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827ED350: 386B5A60  addi r3, r11, 0x5a60
	ctx.r[3].s64 = ctx.r[11].s64 + 23136;
	// 827ED354: 489BADA5  bl 0x831a80f8
	ctx.lr = 0x827ED358;
	sub_831A80F8(ctx, base);
	// 827ED358: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827ED35C: 41820018  beq 0x827ed374
	if ctx.cr[0].eq {
	pc = 0x827ED374; continue 'dispatch;
	}
	// 827ED360: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED364: 387DFFD8  addi r3, r29, -0x28
	ctx.r[3].s64 = ctx.r[29].s64 + -40;
	// 827ED368: 4BFFFF51  bl 0x827ed2b8
	ctx.lr = 0x827ED36C;
	sub_827ED2B8(ctx, base);
	// 827ED36C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827ED370: 48000014  b 0x827ed384
	pc = 0x827ED384; continue 'dispatch;
	// 827ED374: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827ED378: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED37C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827ED380: 4BD25299  bl 0x82512618
	ctx.lr = 0x827ED384;
	sub_82512618(ctx, base);
	// 827ED384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED388: 489BAE34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED390 size=352
    let mut pc: u32 = 0x827ED390;
    'dispatch: loop {
        match pc {
            0x827ED390 => {
    //   block [0x827ED390..0x827ED4F0)
	// 827ED390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED394: 489BADD1  bl 0x831a8164
	ctx.lr = 0x827ED398;
	sub_831A8130(ctx, base);
	// 827ED398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED39C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED3A0: 4BD24EC1  bl 0x82512260
	ctx.lr = 0x827ED3A4;
	sub_82512260(ctx, base);
	// 827ED3A4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ED3A8: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827ED3AC: 396B5DDC  addi r11, r11, 0x5ddc
	ctx.r[11].s64 = ctx.r[11].s64 + 24028;
	// 827ED3B0: 394A5DC4  addi r10, r10, 0x5dc4
	ctx.r[10].s64 = ctx.r[10].s64 + 24004;
	// 827ED3B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827ED3B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827ED3BC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827ED3C0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ED3C4: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 827ED3C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ED3CC: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 827ED3D0: 3B6B5E18  addi r27, r11, 0x5e18
	ctx.r[27].s64 = ctx.r[11].s64 + 24088;
	// 827ED3D4: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 827ED3D8: 38A00037  li r5, 0x37
	ctx.r[5].s64 = 55;
	// 827ED3DC: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 827ED3E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827ED3E4: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 827ED3E8: 48605001  bl 0x82df23e8
	ctx.lr = 0x827ED3EC;
	sub_82DF23E8(ctx, base);
	// 827ED3EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ED3F0: 41820010  beq 0x827ed400
	if ctx.cr[0].eq {
	pc = 0x827ED400; continue 'dispatch;
	}
	// 827ED3F4: 48625C15  bl 0x82e13008
	ctx.lr = 0x827ED3F8;
	sub_82E13008(ctx, base);
	// 827ED3F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED3FC: 48000008  b 0x827ed404
	pc = 0x827ED404; continue 'dispatch;
	// 827ED400: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 827ED404: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 827ED408: 397F00F4  addi r11, r31, 0xf4
	ctx.r[11].s64 = ctx.r[31].s64 + 244;
	// 827ED40C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED410: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 827ED414: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED418: 4BAF3C01  bl 0x822e1018
	ctx.lr = 0x827ED41C;
	sub_822E1018(ctx, base);
	// 827ED41C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827ED420: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED424: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED428: 4BAD2BD9  bl 0x822c0000
	ctx.lr = 0x827ED42C;
	sub_822C0000(ctx, base);
	// 827ED42C: 387F00FC  addi r3, r31, 0xfc
	ctx.r[3].s64 = ctx.r[31].s64 + 252;
	// 827ED430: 486704A1  bl 0x82e5d8d0
	ctx.lr = 0x827ED434;
	sub_82E5D8D0(ctx, base);
	// 827ED434: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ED438: 387F0164  addi r3, r31, 0x164
	ctx.r[3].s64 = ctx.r[31].s64 + 356;
	// 827ED43C: 3BAB5DA0  addi r29, r11, 0x5da0
	ctx.r[29].s64 = ctx.r[11].s64 + 23968;
	// 827ED440: 93BF00FC  stw r29, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[29].u32 ) };
	// 827ED444: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 827ED448: 93DF0160  stw r30, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[30].u32 ) };
	// 827ED44C: 48670485  bl 0x82e5d8d0
	ctx.lr = 0x827ED450;
	sub_82E5D8D0(ctx, base);
	// 827ED450: 93BF0164  stw r29, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[29].u32 ) };
	// 827ED454: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 827ED458: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827ED45C: 93DF01C8  stw r30, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[30].u32 ) };
	// 827ED460: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ED464: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 827ED468: 38A00035  li r5, 0x35
	ctx.r[5].s64 = 53;
	// 827ED46C: 93DF01D0  stw r30, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[30].u32 ) };
	// 827ED470: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 827ED474: 93DF01D4  stw r30, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[30].u32 ) };
	// 827ED478: 93DF01D8  stw r30, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[30].u32 ) };
	// 827ED47C: 93DF01DC  stw r30, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[30].u32 ) };
	// 827ED480: 93DF01E0  stw r30, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[30].u32 ) };
	// 827ED484: 9BDF01E4  stb r30, 0x1e4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[30].u8 ) };
	// 827ED488: 93DF01E8  stw r30, 0x1e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), ctx.r[30].u32 ) };
	// 827ED48C: 93DF01EC  stw r30, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[30].u32 ) };
	// 827ED490: 9BDF01F0  stb r30, 0x1f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[30].u8 ) };
	// 827ED494: 93DF01F4  stw r30, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[30].u32 ) };
	// 827ED498: 93DF01F8  stw r30, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[30].u32 ) };
	// 827ED49C: 48604F4D  bl 0x82df23e8
	ctx.lr = 0x827ED4A0;
	sub_82DF23E8(ctx, base);
	// 827ED4A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ED4A4: 41820010  beq 0x827ed4b4
	if ctx.cr[0].eq {
	pc = 0x827ED4B4; continue 'dispatch;
	}
	// 827ED4A8: 4BD9D7F1  bl 0x8258ac98
	ctx.lr = 0x827ED4AC;
	sub_8258AC98(ctx, base);
	// 827ED4AC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED4B0: 48000008  b 0x827ed4b8
	pc = 0x827ED4B8; continue 'dispatch;
	// 827ED4B4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 827ED4B8: 93BF01FC  stw r29, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[29].u32 ) };
	// 827ED4BC: 397F01FC  addi r11, r31, 0x1fc
	ctx.r[11].s64 = ctx.r[31].s64 + 508;
	// 827ED4C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED4C4: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 827ED4C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED4CC: 4BB194C5  bl 0x82306990
	ctx.lr = 0x827ED4D0;
	sub_82306990(ctx, base);
	// 827ED4D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827ED4D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED4D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED4DC: 4BAD2B25  bl 0x822c0000
	ctx.lr = 0x827ED4E0;
	sub_822C0000(ctx, base);
	// 827ED4E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED4E4: 93DF0204  stw r30, 0x204(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(516 as u32), ctx.r[30].u32 ) };
	// 827ED4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827ED4EC: 489BACC8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED4F0 size=76
    let mut pc: u32 = 0x827ED4F0;
    'dispatch: loop {
        match pc {
            0x827ED4F0 => {
    //   block [0x827ED4F0..0x827ED53C)
	// 827ED4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED4F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ED4FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED508: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827ED50C: 4BFFF56D  bl 0x827eca78
	ctx.lr = 0x827ED510;
	sub_827ECA78(ctx, base);
	// 827ED510: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827ED514: 4182000C  beq 0x827ed520
	if ctx.cr[0].eq {
	pc = 0x827ED520; continue 'dispatch;
	}
	// 827ED518: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED51C: 48604EBD  bl 0x82df23d8
	ctx.lr = 0x827ED520;
	sub_82DF23D8(ctx, base);
	// 827ED520: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED530: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED540 size=352
    let mut pc: u32 = 0x827ED540;
    'dispatch: loop {
        match pc {
            0x827ED540 => {
    //   block [0x827ED540..0x827ED6A0)
	// 827ED540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED544: 489BAC21  bl 0x831a8164
	ctx.lr = 0x827ED548;
	sub_831A8130(ctx, base);
	// 827ED548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED54C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED550: 4BD24DE1  bl 0x82512330
	ctx.lr = 0x827ED554;
	sub_82512330(ctx, base);
	// 827ED554: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ED558: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827ED55C: 396B5DDC  addi r11, r11, 0x5ddc
	ctx.r[11].s64 = ctx.r[11].s64 + 24028;
	// 827ED560: 394A5DC4  addi r10, r10, 0x5dc4
	ctx.r[10].s64 = ctx.r[10].s64 + 24004;
	// 827ED564: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827ED568: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827ED56C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827ED570: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ED574: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 827ED578: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ED57C: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 827ED580: 3B6B5E18  addi r27, r11, 0x5e18
	ctx.r[27].s64 = ctx.r[11].s64 + 24088;
	// 827ED584: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 827ED588: 38A00041  li r5, 0x41
	ctx.r[5].s64 = 65;
	// 827ED58C: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 827ED590: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827ED594: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 827ED598: 48604E51  bl 0x82df23e8
	ctx.lr = 0x827ED59C;
	sub_82DF23E8(ctx, base);
	// 827ED59C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ED5A0: 41820010  beq 0x827ed5b0
	if ctx.cr[0].eq {
	pc = 0x827ED5B0; continue 'dispatch;
	}
	// 827ED5A4: 48625A65  bl 0x82e13008
	ctx.lr = 0x827ED5A8;
	sub_82E13008(ctx, base);
	// 827ED5A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED5AC: 48000008  b 0x827ed5b4
	pc = 0x827ED5B4; continue 'dispatch;
	// 827ED5B0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 827ED5B4: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 827ED5B8: 397F00F4  addi r11, r31, 0xf4
	ctx.r[11].s64 = ctx.r[31].s64 + 244;
	// 827ED5BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED5C0: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 827ED5C4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED5C8: 4BAF3A51  bl 0x822e1018
	ctx.lr = 0x827ED5CC;
	sub_822E1018(ctx, base);
	// 827ED5CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827ED5D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED5D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED5D8: 4BAD2A29  bl 0x822c0000
	ctx.lr = 0x827ED5DC;
	sub_822C0000(ctx, base);
	// 827ED5DC: 387F00FC  addi r3, r31, 0xfc
	ctx.r[3].s64 = ctx.r[31].s64 + 252;
	// 827ED5E0: 486702F1  bl 0x82e5d8d0
	ctx.lr = 0x827ED5E4;
	sub_82E5D8D0(ctx, base);
	// 827ED5E4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827ED5E8: 387F0164  addi r3, r31, 0x164
	ctx.r[3].s64 = ctx.r[31].s64 + 356;
	// 827ED5EC: 3BAB5DA0  addi r29, r11, 0x5da0
	ctx.r[29].s64 = ctx.r[11].s64 + 23968;
	// 827ED5F0: 93BF00FC  stw r29, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[29].u32 ) };
	// 827ED5F4: 93DF015C  stw r30, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[30].u32 ) };
	// 827ED5F8: 93DF0160  stw r30, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[30].u32 ) };
	// 827ED5FC: 486702D5  bl 0x82e5d8d0
	ctx.lr = 0x827ED600;
	sub_82E5D8D0(ctx, base);
	// 827ED600: 93BF0164  stw r29, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[29].u32 ) };
	// 827ED604: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 827ED608: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827ED60C: 93DF01C8  stw r30, 0x1c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(456 as u32), ctx.r[30].u32 ) };
	// 827ED610: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827ED614: 93DF01CC  stw r30, 0x1cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(460 as u32), ctx.r[30].u32 ) };
	// 827ED618: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 827ED61C: 93DF01D0  stw r30, 0x1d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(464 as u32), ctx.r[30].u32 ) };
	// 827ED620: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 827ED624: 93DF01D4  stw r30, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[30].u32 ) };
	// 827ED628: 93DF01D8  stw r30, 0x1d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(472 as u32), ctx.r[30].u32 ) };
	// 827ED62C: 93DF01DC  stw r30, 0x1dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(476 as u32), ctx.r[30].u32 ) };
	// 827ED630: 93DF01E0  stw r30, 0x1e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(480 as u32), ctx.r[30].u32 ) };
	// 827ED634: 9BDF01E4  stb r30, 0x1e4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[30].u8 ) };
	// 827ED638: 93DF01E8  stw r30, 0x1e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(488 as u32), ctx.r[30].u32 ) };
	// 827ED63C: 93DF01EC  stw r30, 0x1ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(492 as u32), ctx.r[30].u32 ) };
	// 827ED640: 9BDF01F0  stb r30, 0x1f0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[30].u8 ) };
	// 827ED644: 93DF01F4  stw r30, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[30].u32 ) };
	// 827ED648: 93DF01F8  stw r30, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[30].u32 ) };
	// 827ED64C: 48604D9D  bl 0x82df23e8
	ctx.lr = 0x827ED650;
	sub_82DF23E8(ctx, base);
	// 827ED650: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ED654: 41820010  beq 0x827ed664
	if ctx.cr[0].eq {
	pc = 0x827ED664; continue 'dispatch;
	}
	// 827ED658: 4BD9D641  bl 0x8258ac98
	ctx.lr = 0x827ED65C;
	sub_8258AC98(ctx, base);
	// 827ED65C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827ED660: 48000008  b 0x827ed668
	pc = 0x827ED668; continue 'dispatch;
	// 827ED664: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 827ED668: 93BF01FC  stw r29, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[29].u32 ) };
	// 827ED66C: 397F01FC  addi r11, r31, 0x1fc
	ctx.r[11].s64 = ctx.r[31].s64 + 508;
	// 827ED670: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED674: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 827ED678: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED67C: 4BB19315  bl 0x82306990
	ctx.lr = 0x827ED680;
	sub_82306990(ctx, base);
	// 827ED680: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827ED684: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827ED688: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827ED68C: 4BAD2975  bl 0x822c0000
	ctx.lr = 0x827ED690;
	sub_822C0000(ctx, base);
	// 827ED690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED694: 93DF0204  stw r30, 0x204(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(516 as u32), ctx.r[30].u32 ) };
	// 827ED698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827ED69C: 489BAB18  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED6A0 size=156
    let mut pc: u32 = 0x827ED6A0;
    'dispatch: loop {
        match pc {
            0x827ED6A0 => {
    //   block [0x827ED6A0..0x827ED73C)
	// 827ED6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED6A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ED6AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED6B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED6B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827ED6BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827ED6C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827ED6C4: 4860653D  bl 0x82df3c00
	ctx.lr = 0x827ED6C8;
	sub_82DF3C00(ctx, base);
	// 827ED6C8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827ED6CC: 389E00FC  addi r4, r30, 0xfc
	ctx.r[4].s64 = ctx.r[30].s64 + 252;
	// 827ED6D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827ED6D4: 4866DBCD  bl 0x82e5b2a0
	ctx.lr = 0x827ED6D8;
	sub_82E5B2A0(ctx, base);
	// 827ED6D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED6DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827ED6E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ED6E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827ED6E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827ED6EC: 419A0024  beq cr6, 0x827ed710
	if ctx.cr[6].eq {
	pc = 0x827ED710; continue 'dispatch;
	}
	// 827ED6F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827ED6F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827ED6F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED6FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827ED700: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827ED704: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827ED708: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827ED70C: 4082FFE8  bne 0x827ed6f4
	if !ctx.cr[0].eq {
	pc = 0x827ED6F4; continue 'dispatch;
	}
	// 827ED710: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827ED714: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ED718: 419A0008  beq cr6, 0x827ed720
	if ctx.cr[6].eq {
	pc = 0x827ED720; continue 'dispatch;
	}
	// 827ED71C: 4BAD3175  bl 0x822c0890
	ctx.lr = 0x827ED720;
	sub_822C0890(ctx, base);
	// 827ED720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827ED728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED740 size=112
    let mut pc: u32 = 0x827ED740;
    'dispatch: loop {
        match pc {
            0x827ED740 => {
    //   block [0x827ED740..0x827ED7B0)
	// 827ED740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED748: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ED74C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED754: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827ED758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED75C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 827ED760: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827ED764: 4BFFF1ED  bl 0x827ec950
	ctx.lr = 0x827ED768;
	sub_827EC950(ctx, base);
	// 827ED768: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827ED76C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827ED770: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827ED774: 4BAD288D  bl 0x822c0000
	ctx.lr = 0x827ED778;
	sub_822C0000(ctx, base);
	// 827ED778: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827ED77C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827ED780: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827ED784: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ED788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827ED78C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827ED790: 419A0008  beq cr6, 0x827ed798
	if ctx.cr[6].eq {
	pc = 0x827ED798; continue 'dispatch;
	}
	// 827ED794: 4BAD30FD  bl 0x822c0890
	ctx.lr = 0x827ED798;
	sub_822C0890(ctx, base);
	// 827ED798: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED7A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED7A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED7B0 size=104
    let mut pc: u32 = 0x827ED7B0;
    'dispatch: loop {
        match pc {
            0x827ED7B0 => {
    //   block [0x827ED7B0..0x827ED818)
	// 827ED7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED7B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED7BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED7C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED7C4: 48865B4D  bl 0x83053310
	ctx.lr = 0x827ED7C8;
	sub_83053310(ctx, base);
	// 827ED7C8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 827ED7CC: 4BAD316D  bl 0x822c0938
	ctx.lr = 0x827ED7D0;
	sub_822C0938(ctx, base);
	// 827ED7D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827ED7D4: 41820010  beq 0x827ed7e4
	if ctx.cr[0].eq {
	pc = 0x827ED7E4; continue 'dispatch;
	}
	// 827ED7D8: 4867B5A9  bl 0x82e68d80
	ctx.lr = 0x827ED7DC;
	sub_82E68D80(ctx, base);
	// 827ED7DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827ED7E0: 48000008  b 0x827ed7e8
	pc = 0x827ED7E8; continue 'dispatch;
	// 827ED7E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827ED7E8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 827ED7EC: 4BFFFF55  bl 0x827ed740
	ctx.lr = 0x827ED7F0;
	sub_827ED740(ctx, base);
	// 827ED7F0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 827ED7F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED7F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827ED7FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827ED800: 4E800421  bctrl
	ctx.lr = 0x827ED804;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827ED804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827ED808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827ED818 size=8
    let mut pc: u32 = 0x827ED818;
    'dispatch: loop {
        match pc {
            0x827ED818 => {
    //   block [0x827ED818..0x827ED820)
	// 827ED818: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827ED81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827ED820 size=12
    let mut pc: u32 = 0x827ED820;
    'dispatch: loop {
        match pc {
            0x827ED820 => {
    //   block [0x827ED820..0x827ED82C)
	// 827ED820: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827ED824: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827ED828: 4881B790  b 0x83008fb8
	sub_83008FB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827ED830 size=8
    let mut pc: u32 = 0x827ED830;
    'dispatch: loop {
        match pc {
            0x827ED830 => {
    //   block [0x827ED830..0x827ED838)
	// 827ED830: 806300D4  lwz r3, 0xd4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 827ED834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED838 size=36
    let mut pc: u32 = 0x827ED838;
    'dispatch: loop {
        match pc {
            0x827ED838 => {
    //   block [0x827ED838..0x827ED85C)
	// 827ED838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED844: 48001F3D  bl 0x827ef780
	ctx.lr = 0x827ED848;
	sub_827EF780(ctx, base);
	// 827ED848: 4BFFCAC9  bl 0x827ea310
	ctx.lr = 0x827ED84C;
	sub_827EA310(ctx, base);
	// 827ED84C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827ED850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED860 size=80
    let mut pc: u32 = 0x827ED860;
    'dispatch: loop {
        match pc {
            0x827ED860 => {
    //   block [0x827ED860..0x827ED8B0)
	// 827ED860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED86C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827ED870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED874: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED878: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827ED87C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827ED880: 48001F01  bl 0x827ef780
	ctx.lr = 0x827ED884;
	sub_827EF780(ctx, base);
	// 827ED884: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827ED888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED88C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827ED890: 4BFFCBB9  bl 0x827ea448
	ctx.lr = 0x827ED894;
	sub_827EA448(ctx, base);
	// 827ED894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED8A4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED8A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED8B0 size=80
    let mut pc: u32 = 0x827ED8B0;
    'dispatch: loop {
        match pc {
            0x827ED8B0 => {
    //   block [0x827ED8B0..0x827ED900)
	// 827ED8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED8BC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827ED8C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED8C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827ED8C8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827ED8CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827ED8D0: 48001EB1  bl 0x827ef780
	ctx.lr = 0x827ED8D4;
	sub_827EF780(ctx, base);
	// 827ED8D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827ED8D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED8DC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827ED8E0: 4BFFCBA9  bl 0x827ea488
	ctx.lr = 0x827ED8E4;
	sub_827EA488(ctx, base);
	// 827ED8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827ED8E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED8F4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED900 size=88
    let mut pc: u32 = 0x827ED900;
    'dispatch: loop {
        match pc {
            0x827ED900 => {
    //   block [0x827ED900..0x827ED958)
	// 827ED900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ED90C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED914: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827ED918: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827ED91C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827ED920: 4BFFE8A1  bl 0x827ec1c0
	ctx.lr = 0x827ED924;
	sub_827EC1C0(ctx, base);
	// 827ED924: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED928: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827ED92C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED930: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 827ED934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827ED938: 4E800421  bctrl
	ctx.lr = 0x827ED93C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827ED93C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827ED940: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED94C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED958 size=72
    let mut pc: u32 = 0x827ED958;
    'dispatch: loop {
        match pc {
            0x827ED958 => {
    //   block [0x827ED958..0x827ED9A0)
	// 827ED958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED960: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827ED964: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED96C: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827ED970: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827ED974: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827ED978: 4BFFE849  bl 0x827ec1c0
	ctx.lr = 0x827ED97C;
	sub_827EC1C0(ctx, base);
	// 827ED97C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED980: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827ED984: 4867B4FD  bl 0x82e68e80
	ctx.lr = 0x827ED988;
	sub_82E68E80(ctx, base);
	// 827ED988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827ED98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED994: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827ED998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED9A0 size=72
    let mut pc: u32 = 0x827ED9A0;
    'dispatch: loop {
        match pc {
            0x827ED9A0 => {
    //   block [0x827ED9A0..0x827ED9E8)
	// 827ED9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED9A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED9AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED9B0: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827ED9B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827ED9B8: 4BFFE809  bl 0x827ec1c0
	ctx.lr = 0x827ED9BC;
	sub_827EC1C0(ctx, base);
	// 827ED9BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827ED9C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827ED9C4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827ED9C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827ED9CC: 4E800421  bctrl
	ctx.lr = 0x827ED9D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827ED9D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827ED9D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827ED9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827ED9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827ED9E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827ED9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827ED9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827ED9E8 size=56
    let mut pc: u32 = 0x827ED9E8;
    'dispatch: loop {
        match pc {
            0x827ED9E8 => {
    //   block [0x827ED9E8..0x827EDA20)
	// 827ED9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827ED9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827ED9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827ED9F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827ED9F8: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827ED9FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EDA00: 4BFFE7C1  bl 0x827ec1c0
	ctx.lr = 0x827EDA04;
	sub_827EC1C0(ctx, base);
	// 827EDA04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EDA08: 4867B5B9  bl 0x82e68fc0
	ctx.lr = 0x827EDA0C;
	sub_82E68FC0(ctx, base);
	// 827EDA0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EDA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EDA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EDA18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EDA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDA20 size=68
    let mut pc: u32 = 0x827EDA20;
    'dispatch: loop {
        match pc {
            0x827EDA20 => {
    //   block [0x827EDA20..0x827EDA64)
	// 827EDA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EDA28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EDA2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDA30: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDA34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EDA38: 4BFFE789  bl 0x827ec1c0
	ctx.lr = 0x827EDA3C;
	sub_827EC1C0(ctx, base);
	// 827EDA3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDA40: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EDA44: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EDA48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EDA4C: 4E800421  bctrl
	ctx.lr = 0x827EDA50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EDA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EDA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EDA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EDA5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EDA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDA68 size=500
    let mut pc: u32 = 0x827EDA68;
    'dispatch: loop {
        match pc {
            0x827EDA68 => {
    //   block [0x827EDA68..0x827EDC5C)
	// 827EDA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDA6C: 489BA6FD  bl 0x831a8168
	ctx.lr = 0x827EDA70;
	sub_831A8130(ctx, base);
	// 827EDA70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDA74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827EDA78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EDA7C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 827EDA80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDA84: 48605F85  bl 0x82df3a08
	ctx.lr = 0x827EDA88;
	sub_82DF3A08(ctx, base);
	// 827EDA88: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 827EDA8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDA90: 388B7688  addi r4, r11, 0x7688
	ctx.r[4].s64 = ctx.r[11].s64 + 30344;
	// 827EDA94: 48605F75  bl 0x82df3a08
	ctx.lr = 0x827EDA98;
	sub_82DF3A08(ctx, base);
	// 827EDA98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EDA9C: 48001CFD  bl 0x827ef798
	ctx.lr = 0x827EDAA0;
	sub_827EF798(ctx, base);
	// 827EDAA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EDAA4: 41820090  beq 0x827edb34
	if ctx.cr[0].eq {
	pc = 0x827EDB34; continue 'dispatch;
	}
	// 827EDAA8: 480047A9  bl 0x827f2250
	ctx.lr = 0x827EDAAC;
	sub_827F2250(ctx, base);
	// 827EDAAC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EDAB0: 41820010  beq 0x827edac0
	if ctx.cr[0].eq {
	pc = 0x827EDAC0; continue 'dispatch;
	}
	// 827EDAB4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EDAB8: 388B5E80  addi r4, r11, 0x5e80
	ctx.r[4].s64 = ctx.r[11].s64 + 24192;
	// 827EDABC: 4800000C  b 0x827edac8
	pc = 0x827EDAC8; continue 'dispatch;
	// 827EDAC0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EDAC4: 388B5E78  addi r4, r11, 0x5e78
	ctx.r[4].s64 = ctx.r[11].s64 + 24184;
	// 827EDAC8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EDACC: 48605F3D  bl 0x82df3a08
	ctx.lr = 0x827EDAD0;
	sub_82DF3A08(ctx, base);
	// 827EDAD0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EDAD4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EDAD8: 388B5E68  addi r4, r11, 0x5e68
	ctx.r[4].s64 = ctx.r[11].s64 + 24168;
	// 827EDADC: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 827EDAE0: 3B810054  addi r28, r1, 0x54
	ctx.r[28].s64 = ctx.r[1].s64 + 84;
	// 827EDAE4: 48605F25  bl 0x82df3a08
	ctx.lr = 0x827EDAE8;
	sub_82DF3A08(ctx, base);
	// 827EDAE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDAEC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EDAF0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827EDAF4: 4860614D  bl 0x82df3c40
	ctx.lr = 0x827EDAF8;
	sub_82DF3C40(ctx, base);
	// 827EDAF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDAFC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EDB00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827EDB04: 4860613D  bl 0x82df3c40
	ctx.lr = 0x827EDB08;
	sub_82DF3C40(ctx, base);
	// 827EDB08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDB0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDB10: 48605DA1  bl 0x82df38b0
	ctx.lr = 0x827EDB14;
	sub_82DF38B0(ctx, base);
	// 827EDB14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EDB18: 48605911  bl 0x82df3428
	ctx.lr = 0x827EDB1C;
	sub_82DF3428(ctx, base);
	// 827EDB1C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EDB20: 48605909  bl 0x82df3428
	ctx.lr = 0x827EDB24;
	sub_82DF3428(ctx, base);
	// 827EDB24: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EDB28: 48605901  bl 0x82df3428
	ctx.lr = 0x827EDB2C;
	sub_82DF3428(ctx, base);
	// 827EDB2C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EDB30: 486058F9  bl 0x82df3428
	ctx.lr = 0x827EDB34;
	sub_82DF3428(ctx, base);
	// 827EDB34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EDB38: 48001C49  bl 0x827ef780
	ctx.lr = 0x827EDB3C;
	sub_827EF780(ctx, base);
	// 827EDB3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EDB40: 41820078  beq 0x827edbb8
	if ctx.cr[0].eq {
	pc = 0x827EDBB8; continue 'dispatch;
	}
	// 827EDB44: 4BFFC765  bl 0x827ea2a8
	ctx.lr = 0x827EDB48;
	sub_827EA2A8(ctx, base);
	// 827EDB48: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDB4C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EDB50: 486060B1  bl 0x82df3c00
	ctx.lr = 0x827EDB54;
	sub_82DF3C00(ctx, base);
	// 827EDB54: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EDB58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EDB5C: 388B5E5C  addi r4, r11, 0x5e5c
	ctx.r[4].s64 = ctx.r[11].s64 + 24156;
	// 827EDB60: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 827EDB64: 3B810054  addi r28, r1, 0x54
	ctx.r[28].s64 = ctx.r[1].s64 + 84;
	// 827EDB68: 48605EA1  bl 0x82df3a08
	ctx.lr = 0x827EDB6C;
	sub_82DF3A08(ctx, base);
	// 827EDB6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDB70: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EDB74: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827EDB78: 486060C9  bl 0x82df3c40
	ctx.lr = 0x827EDB7C;
	sub_82DF3C40(ctx, base);
	// 827EDB7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDB80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EDB84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827EDB88: 486060B9  bl 0x82df3c40
	ctx.lr = 0x827EDB8C;
	sub_82DF3C40(ctx, base);
	// 827EDB8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDB94: 48605D1D  bl 0x82df38b0
	ctx.lr = 0x827EDB98;
	sub_82DF38B0(ctx, base);
	// 827EDB98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EDB9C: 4860588D  bl 0x82df3428
	ctx.lr = 0x827EDBA0;
	sub_82DF3428(ctx, base);
	// 827EDBA0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EDBA4: 48605885  bl 0x82df3428
	ctx.lr = 0x827EDBA8;
	sub_82DF3428(ctx, base);
	// 827EDBA8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EDBAC: 4860587D  bl 0x82df3428
	ctx.lr = 0x827EDBB0;
	sub_82DF3428(ctx, base);
	// 827EDBB0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EDBB4: 48605875  bl 0x82df3428
	ctx.lr = 0x827EDBB8;
	sub_82DF3428(ctx, base);
	// 827EDBB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EDBBC: 48001B95  bl 0x827ef750
	ctx.lr = 0x827EDBC0;
	sub_827EF750(ctx, base);
	// 827EDBC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EDBC4: 41820084  beq 0x827edc48
	if ctx.cr[0].eq {
	pc = 0x827EDC48; continue 'dispatch;
	}
	// 827EDBC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDBCC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 827EDBD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EDBD4: 4E800421  bctrl
	ctx.lr = 0x827EDBD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EDBD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDBDC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EDBE0: 48606021  bl 0x82df3c00
	ctx.lr = 0x827EDBE4;
	sub_82DF3C00(ctx, base);
	// 827EDBE4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EDBE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EDBEC: 388B5E50  addi r4, r11, 0x5e50
	ctx.r[4].s64 = ctx.r[11].s64 + 24144;
	// 827EDBF0: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 827EDBF4: 3BA10054  addi r29, r1, 0x54
	ctx.r[29].s64 = ctx.r[1].s64 + 84;
	// 827EDBF8: 48605E11  bl 0x82df3a08
	ctx.lr = 0x827EDBFC;
	sub_82DF3A08(ctx, base);
	// 827EDBFC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDC00: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EDC04: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827EDC08: 48606039  bl 0x82df3c40
	ctx.lr = 0x827EDC0C;
	sub_82DF3C40(ctx, base);
	// 827EDC0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDC10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EDC14: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EDC18: 48606029  bl 0x82df3c40
	ctx.lr = 0x827EDC1C;
	sub_82DF3C40(ctx, base);
	// 827EDC1C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDC20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDC24: 48605C8D  bl 0x82df38b0
	ctx.lr = 0x827EDC28;
	sub_82DF38B0(ctx, base);
	// 827EDC28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EDC2C: 486057FD  bl 0x82df3428
	ctx.lr = 0x827EDC30;
	sub_82DF3428(ctx, base);
	// 827EDC30: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EDC34: 486057F5  bl 0x82df3428
	ctx.lr = 0x827EDC38;
	sub_82DF3428(ctx, base);
	// 827EDC38: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EDC3C: 486057ED  bl 0x82df3428
	ctx.lr = 0x827EDC40;
	sub_82DF3428(ctx, base);
	// 827EDC40: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EDC44: 486057E5  bl 0x82df3428
	ctx.lr = 0x827EDC48;
	sub_82DF3428(ctx, base);
	// 827EDC48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDC4C: 486057DD  bl 0x82df3428
	ctx.lr = 0x827EDC50;
	sub_82DF3428(ctx, base);
	// 827EDC50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDC54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827EDC58: 489BA560  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDC60 size=148
    let mut pc: u32 = 0x827EDC60;
    'dispatch: loop {
        match pc {
            0x827EDC60 => {
    //   block [0x827EDC60..0x827EDCF4)
	// 827EDC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EDC68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EDC6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EDC70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDC74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDC78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EDC7C: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 827EDC80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EDC84: 409A0038  bne cr6, 0x827edcbc
	if !ctx.cr[6].eq {
	pc = 0x827EDCBC; continue 'dispatch;
	}
	// 827EDC88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDC8C: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDC90: 4BD21839  bl 0x8250f4c8
	ctx.lr = 0x827EDC94;
	sub_8250F4C8(ctx, base);
	// 827EDC94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDC98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EDC9C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 827EDCA0: 409A0008  bne cr6, 0x827edca8
	if !ctx.cr[6].eq {
	pc = 0x827EDCA8; continue 'dispatch;
	}
	// 827EDCA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827EDCA8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827EDCAC: 4BD1AD6D  bl 0x82508a18
	ctx.lr = 0x827EDCB0;
	sub_82508A18(ctx, base);
	// 827EDCB0: 907F00D4  stw r3, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[3].u32 ) };
	// 827EDCB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDCB8: 48603FD9  bl 0x82df1c90
	ctx.lr = 0x827EDCBC;
	sub_82DF1C90(ctx, base);
	// 827EDCBC: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 827EDCC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EDCC4: 419A0018  beq cr6, 0x827edcdc
	if ctx.cr[6].eq {
	pc = 0x827EDCDC; continue 'dispatch;
	}
	// 827EDCC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDCCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EDCD0: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 827EDCD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EDCD8: 4E800421  bctrl
	ctx.lr = 0x827EDCDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EDCDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EDCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EDCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EDCE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EDCEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EDCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDCF8 size=8
    let mut pc: u32 = 0x827EDCF8;
    'dispatch: loop {
        match pc {
            0x827EDCF8 => {
    //   block [0x827EDCF8..0x827EDD00)
	// 827EDCF8: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDCFC: 48225984  b 0x82a13680
	sub_82A13680(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDD00 size=8
    let mut pc: u32 = 0x827EDD00;
    'dispatch: loop {
        match pc {
            0x827EDD00 => {
    //   block [0x827EDD00..0x827EDD08)
	// 827EDD00: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDD04: 4BFFE454  b 0x827ec158
	sub_827EC158(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDD08 size=8
    let mut pc: u32 = 0x827EDD08;
    'dispatch: loop {
        match pc {
            0x827EDD08 => {
    //   block [0x827EDD08..0x827EDD10)
	// 827EDD08: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDD0C: 482D4284  b 0x82ac1f90
	sub_82AC1F90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDD10 size=20
    let mut pc: u32 = 0x827EDD10;
    'dispatch: loop {
        match pc {
            0x827EDD10 => {
    //   block [0x827EDD10..0x827EDD24)
	// 827EDD10: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDD14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDD18: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 827EDD1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EDD20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDD28 size=20
    let mut pc: u32 = 0x827EDD28;
    'dispatch: loop {
        match pc {
            0x827EDD28 => {
    //   block [0x827EDD28..0x827EDD3C)
	// 827EDD28: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDD2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDD30: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 827EDD34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EDD38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDD40 size=92
    let mut pc: u32 = 0x827EDD40;
    'dispatch: loop {
        match pc {
            0x827EDD40 => {
    //   block [0x827EDD40..0x827EDD9C)
	// 827EDD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EDD48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EDD4C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827EDD50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDD54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDD58: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827EDD5C: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 827EDD60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EDD64: 419A001C  beq cr6, 0x827edd80
	if ctx.cr[6].eq {
	pc = 0x827EDD80; continue 'dispatch;
	}
	// 827EDD68: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDD6C: 480022DD  bl 0x827f0048
	ctx.lr = 0x827EDD70;
	sub_827F0048(ctx, base);
	// 827EDD70: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 827EDD74: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827EDD78: 48002751  bl 0x827f04c8
	ctx.lr = 0x827EDD7C;
	sub_827F04C8(ctx, base);
	// 827EDD7C: 48000008  b 0x827edd84
	pc = 0x827EDD84; continue 'dispatch;
	// 827EDD80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827EDD84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EDD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EDD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EDD90: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EDD94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EDD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDDA0 size=12
    let mut pc: u32 = 0x827EDDA0;
    'dispatch: loop {
        match pc {
            0x827EDDA0 => {
    //   block [0x827EDDA0..0x827EDDAC)
	// 827EDDA0: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 827EDDA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EDDA8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDDAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDDAC size=8
    let mut pc: u32 = 0x827EDDAC;
    'dispatch: loop {
        match pc {
            0x827EDDAC => {
    //   block [0x827EDDAC..0x827EDDB4)
	// 827EDDAC: 48001F5C  b 0x827efd08
	sub_827EFD08(ctx, base);
	return;
	// 827EDDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDDB8 size=12
    let mut pc: u32 = 0x827EDDB8;
    'dispatch: loop {
        match pc {
            0x827EDDB8 => {
    //   block [0x827EDDB8..0x827EDDC4)
	// 827EDDB8: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 827EDDBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EDDC0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDDC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDDC4 size=8
    let mut pc: u32 = 0x827EDDC4;
    'dispatch: loop {
        match pc {
            0x827EDDC4 => {
    //   block [0x827EDDC4..0x827EDDCC)
	// 827EDDC4: 48001F74  b 0x827efd38
	sub_827EFD38(ctx, base);
	return;
	// 827EDDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EDDD0 size=148
    let mut pc: u32 = 0x827EDDD0;
    'dispatch: loop {
        match pc {
            0x827EDDD0 => {
    //   block [0x827EDDD0..0x827EDE64)
	// 827EDDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDDD4: 489BA395  bl 0x831a8168
	ctx.lr = 0x827EDDD8;
	sub_831A8130(ctx, base);
	// 827EDDD8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDDDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EDDE0: 80830070  lwz r4, 0x70(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDDE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDDE8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827EDDEC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827EDDF0: 4BD21729  bl 0x8250f518
	ctx.lr = 0x827EDDF4;
	sub_8250F518(ctx, base);
	// 827EDDF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDDF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EDDFC: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827EDE00: 409A0008  bne cr6, 0x827ede08
	if !ctx.cr[6].eq {
	pc = 0x827EDE08; continue 'dispatch;
	}
	// 827EDE04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827EDE08: 4BD3A239  bl 0x82528040
	ctx.lr = 0x827EDE0C;
	sub_82528040(ctx, base);
	// 827EDE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDE10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDE14: 48603E7D  bl 0x82df1c90
	ctx.lr = 0x827EDE18;
	sub_82DF1C90(ctx, base);
	// 827EDE18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EDE1C: 419A003C  beq cr6, 0x827ede58
	if ctx.cr[6].eq {
	pc = 0x827EDE58; continue 'dispatch;
	}
	// 827EDE20: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827EDE24: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 827EDE28: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827EDE2C: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 827EDE30: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 827EDE34: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827EDE38: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 827EDE3C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827EDE40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDE68 size=116
    let mut pc: u32 = 0x827EDE68;
    'dispatch: loop {
        match pc {
            0x827EDE68 => {
    //   block [0x827EDE68..0x827EDEDC)
	// 827EDE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDE6C: 489BA301  bl 0x831a816c
	ctx.lr = 0x827EDE70;
	sub_831A8130(ctx, base);
	// 827EDE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDE74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EDE78: 80830070  lwz r4, 0x70(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EDE7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDE80: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827EDE84: 4BD21695  bl 0x8250f518
	ctx.lr = 0x827EDE88;
	sub_8250F518(ctx, base);
	// 827EDE88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDE8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EDE90: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827EDE94: 409A0008  bne cr6, 0x827ede9c
	if !ctx.cr[6].eq {
	pc = 0x827EDE9C; continue 'dispatch;
	}
	// 827EDE98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827EDE9C: 4BD3A1A5  bl 0x82528040
	ctx.lr = 0x827EDEA0;
	sub_82528040(ctx, base);
	// 827EDEA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDEA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDEA8: 48603DE9  bl 0x82df1c90
	ctx.lr = 0x827EDEAC;
	sub_82DF1C90(ctx, base);
	// 827EDEAC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EDEB0: 419A0020  beq cr6, 0x827eded0
	if ctx.cr[6].eq {
	pc = 0x827EDED0; continue 'dispatch;
	}
	// 827EDEB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827EDEB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827EDEBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EDEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDEC4: 80CB671C  lwz r6, 0x671c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26396 as u32) ) } as u64;
	// 827EDEC8: 4BAFCAE1  bl 0x822ea9a8
	ctx.lr = 0x827EDECC;
	sub_822EA9A8(ctx, base);
	// 827EDECC: 48000008  b 0x827eded4
	pc = 0x827EDED4; continue 'dispatch;
	// 827EDED0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827EDED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827EDED8: 489BA2E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EDEE0 size=112
    let mut pc: u32 = 0x827EDEE0;
    'dispatch: loop {
        match pc {
            0x827EDEE0 => {
    //   block [0x827EDEE0..0x827EDF50)
	// 827EDEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDEE4: 489BA289  bl 0x831a816c
	ctx.lr = 0x827EDEE8;
	sub_831A8130(ctx, base);
	// 827EDEE8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDEEC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827EDEF0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EDEF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDEF8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827EDEFC: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 827EDF00: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EDF04: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 827EDF08: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827EDF0C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827EDF10: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827EDF14: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827EDF18: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827EDF1C: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827EDF20: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827EDF24: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827EDF28: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827EDF2C: 48001885  bl 0x827ef7b0
	ctx.lr = 0x827EDF30;
	sub_827EF7B0(ctx, base);
	// 827EDF30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDF34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDF38: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827EDF3C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827EDF40: 4868F121  bl 0x82e7d060
	ctx.lr = 0x827EDF44;
	sub_82E7D060(ctx, base);
	// 827EDF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDF48: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827EDF4C: 489BA270  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDF50 size=84
    let mut pc: u32 = 0x827EDF50;
    'dispatch: loop {
        match pc {
            0x827EDF50 => {
    //   block [0x827EDF50..0x827EDFA4)
	// 827EDF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EDF58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EDF5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EDF60: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDF68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EDF6C: 38850020  addi r4, r5, 0x20
	ctx.r[4].s64 = ctx.r[5].s64 + 32;
	// 827EDF70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EDF74: 4868DAF5  bl 0x82e7ba68
	ctx.lr = 0x827EDF78;
	sub_82E7BA68(ctx, base);
	// 827EDF78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EDF7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDF80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EDF84: 4868DD45  bl 0x82e7bcc8
	ctx.lr = 0x827EDF88;
	sub_82E7BCC8(ctx, base);
	// 827EDF88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EDF8C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827EDF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EDF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EDF98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EDF9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EDFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDFA8 size=12
    let mut pc: u32 = 0x827EDFA8;
    'dispatch: loop {
        match pc {
            0x827EDFA8 => {
    //   block [0x827EDFA8..0x827EDFB4)
	// 827EDFA8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EDFAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EDFB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDFB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDFB4 size=20
    let mut pc: u32 = 0x827EDFB4;
    'dispatch: loop {
        match pc {
            0x827EDFB4 => {
    //   block [0x827EDFB4..0x827EDFC8)
	// 827EDFB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDFB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EDFBC: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827EDFC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EDFC4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDFC8 size=4
    let mut pc: u32 = 0x827EDFC8;
    'dispatch: loop {
        match pc {
            0x827EDFC8 => {
    //   block [0x827EDFC8..0x827EDFCC)
	// 827EDFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EDFD0 size=16
    let mut pc: u32 = 0x827EDFD0;
    'dispatch: loop {
        match pc {
            0x827EDFD0 => {
    //   block [0x827EDFD0..0x827EDFE0)
	// 827EDFD0: 816300D0  lwz r11, 0xd0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 827EDFD4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827EDFD8: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827EDFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EDFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EDFE0 size=92
    let mut pc: u32 = 0x827EDFE0;
    'dispatch: loop {
        match pc {
            0x827EDFE0 => {
    //   block [0x827EDFE0..0x827EE03C)
	// 827EDFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EDFE4: 489BA189  bl 0x831a816c
	ctx.lr = 0x827EDFE8;
	sub_831A8130(ctx, base);
	// 827EDFE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EDFEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EDFF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EDFF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EDFF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EDFFC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EE000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EE004: 4E800421  bctrl
	ctx.lr = 0x827EE008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EE008: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE00C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827EE010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EE014: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827EE018: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EE01C: 4E800421  bctrl
	ctx.lr = 0x827EE020;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EE020: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EE024: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE028: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827EE02C: 4BB1974D  bl 0x82307778
	ctx.lr = 0x827EE030;
	sub_82307778(ctx, base);
	// 827EE030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE038: 489BA184  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EE040 size=28
    let mut pc: u32 = 0x827EE040;
    'dispatch: loop {
        match pc {
            0x827EE040 => {
    //   block [0x827EE040..0x827EE05C)
	// 827EE040: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE044: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EE048: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 827EE04C: 394B00B0  addi r10, r11, 0xb0
	ctx.r[10].s64 = ctx.r[11].s64 + 176;
	// 827EE050: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 827EE054: 912B00B0  stw r9, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[9].u32 ) };
	// 827EE058: 4BAD6408  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EE060 size=180
    let mut pc: u32 = 0x827EE060;
    'dispatch: loop {
        match pc {
            0x827EE060 => {
    //   block [0x827EE060..0x827EE114)
	// 827EE060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE06C: 3980FFE0  li r12, -0x20
	ctx.r[12].s64 = -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE118 size=80
    let mut pc: u32 = 0x827EE118;
    'dispatch: loop {
        match pc {
            0x827EE118 => {
    //   block [0x827EE118..0x827EE168)
	// 827EE118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE120: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE124: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE128: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE12C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EE130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE134: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827EE138: 4BFFFEA9  bl 0x827edfe0
	ctx.lr = 0x827EE13C;
	sub_827EDFE0(ctx, base);
	// 827EE13C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827EE140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE144: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EE148: 4BFFFE09  bl 0x827edf50
	ctx.lr = 0x827EE14C;
	sub_827EDF50(ctx, base);
	// 827EE14C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE150: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827EE154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE15C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EE168 size=88
    let mut pc: u32 = 0x827EE168;
    'dispatch: loop {
        match pc {
            0x827EE168 => {
    //   block [0x827EE168..0x827EE1C0)
	// 827EE168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE16C: 489BA001  bl 0x831a816c
	ctx.lr = 0x827EE170;
	sub_831A8130(ctx, base);
	// 827EE170: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE174: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827EE178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EE17C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EE180: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827EE184: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EE188: 4868DD11  bl 0x82e7be98
	ctx.lr = 0x827EE18C;
	sub_82E7BE98(ctx, base);
	// 827EE18C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EE190: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827EE194: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE198: 4868DB31  bl 0x82e7bcc8
	ctx.lr = 0x827EE19C;
	sub_82E7BCC8(ctx, base);
	// 827EE19C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827EE1A0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 827EE1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE1A8: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827EE1AC: 13FD50C7  vcmpequd (lvx128) v31, v29, v10
	tmp.u32 = ctx.r[29].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EE1C0 size=104
    let mut pc: u32 = 0x827EE1C0;
    'dispatch: loop {
        match pc {
            0x827EE1C0 => {
    //   block [0x827EE1C0..0x827EE228)
	// 827EE1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE1C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE1CC: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 827EE1D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE1D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EE1D8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827EE1DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EE1E0: 4BFFF681  bl 0x827ed860
	ctx.lr = 0x827EE1E4;
	sub_827ED860(ctx, base);
	// 827EE1E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827EE1E8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 827EE1EC: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827EE1F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE1F4: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EE1F8: EC00F824  fdivs f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[31].f64) as f32) as f64;
	// 827EE1FC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827EE200: 13C05407  vcmpneb. (lvlx128) v30, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE228 size=104
    let mut pc: u32 = 0x827EE228;
    'dispatch: loop {
        match pc {
            0x827EE228 => {
    //   block [0x827EE228..0x827EE290)
	// 827EE228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE230: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE234: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE23C: 83E30070  lwz r31, 0x70(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EE240: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EE244: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827EE248: 4881AD71  bl 0x83008fb8
	ctx.lr = 0x827EE24C;
	sub_83008FB8(ctx, base);
	// 827EE24C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE250: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827EE254: 388B5EDC  addi r4, r11, 0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + 24284;
	// 827EE258: 38A0027B  li r5, 0x27b
	ctx.r[5].s64 = 635;
	// 827EE25C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827EE260: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 827EE264: 48668D85  bl 0x82e56fe8
	ctx.lr = 0x827EE268;
	sub_82E56FE8(ctx, base);
	// 827EE268: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EE26C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EE270: 419A0008  beq cr6, 0x827ee278
	if ctx.cr[6].eq {
	pc = 0x827EE278; continue 'dispatch;
	}
	// 827EE274: 4BAD261D  bl 0x822c0890
	ctx.lr = 0x827EE278;
	sub_822C0890(ctx, base);
	// 827EE278: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE284: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE288: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE290 size=120
    let mut pc: u32 = 0x827EE290;
    'dispatch: loop {
        match pc {
            0x827EE290 => {
    //   block [0x827EE290..0x827EE308)
	// 827EE290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE29C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE2A0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827EE2A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE2A8: 83E30070  lwz r31, 0x70(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EE2AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EE2B0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827EE2B4: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827EE2B8: 4881AD01  bl 0x83008fb8
	ctx.lr = 0x827EE2BC;
	sub_83008FB8(ctx, base);
	// 827EE2BC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE2C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827EE2C4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827EE2C8: 388B5EDC  addi r4, r11, 0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + 24284;
	// 827EE2CC: 38A00283  li r5, 0x283
	ctx.r[5].s64 = 643;
	// 827EE2D0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 827EE2D4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 827EE2D8: 4866A769  bl 0x82e58a40
	ctx.lr = 0x827EE2DC;
	sub_82E58A40(ctx, base);
	// 827EE2DC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EE2E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EE2E4: 419A0008  beq cr6, 0x827ee2ec
	if ctx.cr[6].eq {
	pc = 0x827EE2EC; continue 'dispatch;
	}
	// 827EE2E8: 4BAD25A9  bl 0x822c0890
	ctx.lr = 0x827EE2EC;
	sub_822C0890(ctx, base);
	// 827EE2EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE2F8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827EE2FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EE308 size=304
    let mut pc: u32 = 0x827EE308;
    'dispatch: loop {
        match pc {
            0x827EE308 => {
    //   block [0x827EE308..0x827EE438)
	// 827EE308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE30C: 489B9E59  bl 0x831a8164
	ctx.lr = 0x827EE310;
	sub_831A8130(ctx, base);
	// 827EE310: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE314: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827EE318: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827EE31C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE320: 3BEBAC80  addi r31, r11, -0x5380
	ctx.r[31].s64 = ctx.r[11].s64 + -21376;
	// 827EE324: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827EE328: 816AAC90  lwz r11, -0x5370(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21360 as u32) ) } as u64;
	// 827EE32C: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827EE330: 4082001C  bne 0x827ee34c
	if !ctx.cr[0].eq {
	pc = 0x827EE34C; continue 'dispatch;
	}
	// 827EE334: 3D208205  lis r9, -0x7dfb
	ctx.r[9].s64 = -2113601536;
	// 827EE338: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827EE33C: 3929C960  addi r9, r9, -0x36a0
	ctx.r[9].s64 = ctx.r[9].s64 + -13984;
	// 827EE340: 916AAC90  stw r11, -0x5370(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21360 as u32), ctx.r[11].u32 ) };
	// 827EE344: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE438 size=188
    let mut pc: u32 = 0x827EE438;
    'dispatch: loop {
        match pc {
            0x827EE438 => {
    //   block [0x827EE438..0x827EE4F4)
	// 827EE438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE440: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE444: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE448: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE44C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE454: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827EE458: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EE45C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE460: 4BAD24D9  bl 0x822c0938
	ctx.lr = 0x827EE464;
	sub_822C0938(ctx, base);
	// 827EE464: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EE468: 41820028  beq 0x827ee490
	if ctx.cr[0].eq {
	pc = 0x827EE490; continue 'dispatch;
	}
	// 827EE46C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE470: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827EE474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827EE478: 392B5E90  addi r9, r11, 0x5e90
	ctx.r[9].s64 = ctx.r[11].s64 + 24208;
	// 827EE47C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827EE480: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EE484: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827EE488: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827EE48C: 48000008  b 0x827ee494
	pc = 0x827EE494; continue 'dispatch;
	// 827EE490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE494: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE498: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE49C: 409A003C  bne cr6, 0x827ee4d8
	if !ctx.cr[6].eq {
	pc = 0x827EE4D8; continue 'dispatch;
	}
	// 827EE4A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EE4A4: 419A0014  beq cr6, 0x827ee4b8
	if ctx.cr[6].eq {
	pc = 0x827EE4B8; continue 'dispatch;
	}
	// 827EE4A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE4AC: 480019F5  bl 0x827efea0
	ctx.lr = 0x827EE4B0;
	sub_827EFEA0(ctx, base);
	// 827EE4B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE4B4: 4BAD1DB5  bl 0x822c0268
	ctx.lr = 0x827EE4B8;
	sub_822C0268(ctx, base);
	// 827EE4B8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827EE4BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EE4C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE4C4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827EE4C8: 816BC7A0  lwz r11, -0x3860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14432 as u32) ) } as u64;
	// 827EE4CC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827EE4D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827EE4D4: 4BAD1B2D  bl 0x822c0000
	ctx.lr = 0x827EE4D8;
	sub_822C0000(ctx, base);
	// 827EE4D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EE4DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE4E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE4EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE4F8 size=196
    let mut pc: u32 = 0x827EE4F8;
    'dispatch: loop {
        match pc {
            0x827EE4F8 => {
    //   block [0x827EE4F8..0x827EE5BC)
	// 827EE4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE50C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE514: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827EE518: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EE51C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE520: 4BAD2419  bl 0x822c0938
	ctx.lr = 0x827EE524;
	sub_822C0938(ctx, base);
	// 827EE524: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EE528: 41820028  beq 0x827ee550
	if ctx.cr[0].eq {
	pc = 0x827EE550; continue 'dispatch;
	}
	// 827EE52C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE530: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827EE534: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827EE538: 392B5EA4  addi r9, r11, 0x5ea4
	ctx.r[9].s64 = ctx.r[11].s64 + 24228;
	// 827EE53C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827EE540: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EE544: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827EE548: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827EE54C: 48000008  b 0x827ee554
	pc = 0x827EE554; continue 'dispatch;
	// 827EE550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE554: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE55C: 409A0044  bne cr6, 0x827ee5a0
	if !ctx.cr[6].eq {
	pc = 0x827EE5A0; continue 'dispatch;
	}
	// 827EE560: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EE564: 419A001C  beq cr6, 0x827ee580
	if ctx.cr[6].eq {
	pc = 0x827EE580; continue 'dispatch;
	}
	// 827EE568: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE56C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EE570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE574: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827EE578: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EE57C: 4E800421  bctrl
	ctx.lr = 0x827EE580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EE580: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827EE584: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EE588: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE58C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827EE590: 816BC7A0  lwz r11, -0x3860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14432 as u32) ) } as u64;
	// 827EE594: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827EE598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827EE59C: 4BAD1A65  bl 0x822c0000
	ctx.lr = 0x827EE5A0;
	sub_822C0000(ctx, base);
	// 827EE5A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EE5A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE5B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE5B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE5C0 size=196
    let mut pc: u32 = 0x827EE5C0;
    'dispatch: loop {
        match pc {
            0x827EE5C0 => {
    //   block [0x827EE5C0..0x827EE684)
	// 827EE5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE5C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE5CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE5D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE5D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE5DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827EE5E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EE5E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE5E8: 4BAD2351  bl 0x822c0938
	ctx.lr = 0x827EE5EC;
	sub_822C0938(ctx, base);
	// 827EE5EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EE5F0: 41820028  beq 0x827ee618
	if ctx.cr[0].eq {
	pc = 0x827EE618; continue 'dispatch;
	}
	// 827EE5F4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE5F8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827EE5FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827EE600: 392B5EB8  addi r9, r11, 0x5eb8
	ctx.r[9].s64 = ctx.r[11].s64 + 24248;
	// 827EE604: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827EE608: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EE60C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827EE610: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827EE614: 48000008  b 0x827ee61c
	pc = 0x827EE61C; continue 'dispatch;
	// 827EE618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE61C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE620: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE624: 409A0044  bne cr6, 0x827ee668
	if !ctx.cr[6].eq {
	pc = 0x827EE668; continue 'dispatch;
	}
	// 827EE628: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EE62C: 419A001C  beq cr6, 0x827ee648
	if ctx.cr[6].eq {
	pc = 0x827EE648; continue 'dispatch;
	}
	// 827EE630: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE634: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EE638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE63C: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827EE640: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EE644: 4E800421  bctrl
	ctx.lr = 0x827EE648;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EE648: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827EE64C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EE650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE654: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827EE658: 816BC7A0  lwz r11, -0x3860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14432 as u32) ) } as u64;
	// 827EE65C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827EE660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827EE664: 4BAD199D  bl 0x822c0000
	ctx.lr = 0x827EE668;
	sub_822C0000(ctx, base);
	// 827EE668: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EE66C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE678: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE67C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE688 size=196
    let mut pc: u32 = 0x827EE688;
    'dispatch: loop {
        match pc {
            0x827EE688 => {
    //   block [0x827EE688..0x827EE74C)
	// 827EE688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE69C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE6A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827EE6A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EE6AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE6B0: 4BAD2289  bl 0x822c0938
	ctx.lr = 0x827EE6B4;
	sub_822C0938(ctx, base);
	// 827EE6B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EE6B8: 41820028  beq 0x827ee6e0
	if ctx.cr[0].eq {
	pc = 0x827EE6E0; continue 'dispatch;
	}
	// 827EE6BC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE6C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827EE6C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827EE6C8: 392B5ECC  addi r9, r11, 0x5ecc
	ctx.r[9].s64 = ctx.r[11].s64 + 24268;
	// 827EE6CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827EE6D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EE6D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827EE6D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827EE6DC: 48000008  b 0x827ee6e4
	pc = 0x827EE6E4; continue 'dispatch;
	// 827EE6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE6E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE6E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE6EC: 409A0044  bne cr6, 0x827ee730
	if !ctx.cr[6].eq {
	pc = 0x827EE730; continue 'dispatch;
	}
	// 827EE6F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EE6F4: 419A001C  beq cr6, 0x827ee710
	if ctx.cr[6].eq {
	pc = 0x827EE710; continue 'dispatch;
	}
	// 827EE6F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE6FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EE700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE704: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EE708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EE70C: 4E800421  bctrl
	ctx.lr = 0x827EE710;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EE710: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827EE714: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EE718: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE71C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827EE720: 816BC7A0  lwz r11, -0x3860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14432 as u32) ) } as u64;
	// 827EE724: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827EE728: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827EE72C: 4BAD18D5  bl 0x822c0000
	ctx.lr = 0x827EE730;
	sub_822C0000(ctx, base);
	// 827EE730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EE734: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE73C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE740: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE744: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE750 size=64
    let mut pc: u32 = 0x827EE750;
    'dispatch: loop {
        match pc {
            0x827EE750 => {
    //   block [0x827EE750..0x827EE790)
	// 827EE750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE758: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE75C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE760: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EE764: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EE768: 419A0014  beq cr6, 0x827ee77c
	if ctx.cr[6].eq {
	pc = 0x827EE77C; continue 'dispatch;
	}
	// 827EE76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE770: 48001731  bl 0x827efea0
	ctx.lr = 0x827EE774;
	sub_827EFEA0(ctx, base);
	// 827EE774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE778: 4BAD1AF1  bl 0x822c0268
	ctx.lr = 0x827EE77C;
	sub_822C0268(ctx, base);
	// 827EE77C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EE780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EE790 size=24
    let mut pc: u32 = 0x827EE790;
    'dispatch: loop {
        match pc {
            0x827EE790 => {
    //   block [0x827EE790..0x827EE7A8)
	// 827EE790: 816400B0  lwz r11, 0xb0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(176 as u32) ) } as u64;
	// 827EE794: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE798: 816400B4  lwz r11, 0xb4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(180 as u32) ) } as u64;
	// 827EE79C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE7A0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827EE7A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EE7A8 size=36
    let mut pc: u32 = 0x827EE7A8;
    'dispatch: loop {
        match pc {
            0x827EE7A8 => {
    //   block [0x827EE7A8..0x827EE7CC)
	// 827EE7A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827EE7AC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827EE7B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EE7B4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827EE7B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827EE7BC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827EE7C0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EE7C4: 4082FFE8  bne 0x827ee7ac
	if !ctx.cr[0].eq {
	pc = 0x827EE7AC; continue 'dispatch;
	}
	// 827EE7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE7D0 size=80
    let mut pc: u32 = 0x827EE7D0;
    'dispatch: loop {
        match pc {
            0x827EE7D0 => {
    //   block [0x827EE7D0..0x827EE820)
	// 827EE7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE7D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE7DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE7E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE7E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EE7E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE7EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827EE7F0: 4BFFF7F1  bl 0x827edfe0
	ctx.lr = 0x827EE7F4;
	sub_827EDFE0(ctx, base);
	// 827EE7F4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827EE7F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE7FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EE800: 4BFFF969  bl 0x827ee168
	ctx.lr = 0x827EE804;
	sub_827EE168(ctx, base);
	// 827EE804: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE808: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827EE80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE814: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE820 size=140
    let mut pc: u32 = 0x827EE820;
    'dispatch: loop {
        match pc {
            0x827EE820 => {
    //   block [0x827EE820..0x827EE8AC)
	// 827EE820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EE82C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EE838: 80840070  lwz r4, 0x70(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EE83C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE840: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827EE844: 4BFFDD85  bl 0x827ec5c8
	ctx.lr = 0x827EE848;
	sub_827EC5C8(ctx, base);
	// 827EE848: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827EE84C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827EE850: 419A0024  beq cr6, 0x827ee874
	if ctx.cr[6].eq {
	pc = 0x827EE874; continue 'dispatch;
	}
	// 827EE854: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EE858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE85C: 486267D5  bl 0x82e15030
	ctx.lr = 0x827EE860;
	sub_82E15030(ctx, base);
	// 827EE860: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EE864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EE868: 419A0028  beq cr6, 0x827ee890
	if ctx.cr[6].eq {
	pc = 0x827EE890; continue 'dispatch;
	}
	// 827EE86C: 4BAD2025  bl 0x822c0890
	ctx.lr = 0x827EE870;
	sub_822C0890(ctx, base);
	// 827EE870: 48000020  b 0x827ee890
	pc = 0x827EE890; continue 'dispatch;
	// 827EE874: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EE878: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EE87C: 419A0008  beq cr6, 0x827ee884
	if ctx.cr[6].eq {
	pc = 0x827EE884; continue 'dispatch;
	}
	// 827EE880: 4BAD2011  bl 0x822c0890
	ctx.lr = 0x827EE884;
	sub_822C0890(ctx, base);
	// 827EE884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EE888: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EE88C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827EE890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE8A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EE8A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE8B0 size=104
    let mut pc: u32 = 0x827EE8B0;
    'dispatch: loop {
        match pc {
            0x827EE8B0 => {
    //   block [0x827EE8B0..0x827EE918)
	// 827EE8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EE8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EE8BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE8C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EE8C4: 48000EBD  bl 0x827ef780
	ctx.lr = 0x827EE8C8;
	sub_827EF780(ctx, base);
	// 827EE8C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE8CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE8D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EE8D4: 4E800421  bctrl
	ctx.lr = 0x827EE8D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EE8D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EE8DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EE8E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EE8E4: 483C42E5  bl 0x82bb2bc8
	ctx.lr = 0x827EE8E8;
	sub_82BB2BC8(ctx, base);
	// 827EE8E8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EE8EC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE8F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE8F4: 419A000C  beq cr6, 0x827ee900
	if ctx.cr[6].eq {
	pc = 0x827EE900; continue 'dispatch;
	}
	// 827EE8F8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827EE8FC: 4BAD1F95  bl 0x822c0890
	ctx.lr = 0x827EE900;
	sub_822C0890(ctx, base);
	// 827EE900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EE90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EE910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EE914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EE918 size=208
    let mut pc: u32 = 0x827EE918;
    'dispatch: loop {
        match pc {
            0x827EE918 => {
    //   block [0x827EE918..0x827EE9E8)
	// 827EE918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE91C: 489B9851  bl 0x831a816c
	ctx.lr = 0x827EE920;
	sub_831A8130(ctx, base);
	// 827EE920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE924: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EE928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EE92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827EE930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EE934: 409A0014  bne cr6, 0x827ee948
	if !ctx.cr[6].eq {
	pc = 0x827EE948; continue 'dispatch;
	}
	// 827EE938: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE93C: 38A001BC  li r5, 0x1bc
	ctx.r[5].s64 = 444;
	// 827EE940: 388B5EDC  addi r4, r11, 0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + 24284;
	// 827EE944: 48000054  b 0x827ee998
	pc = 0x827EE998; continue 'dispatch;
	// 827EE948: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827EE94C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EE950: 388B5EDC  addi r4, r11, 0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + 24284;
	// 827EE954: 409A0040  bne cr6, 0x827ee994
	if !ctx.cr[6].eq {
	pc = 0x827EE994; continue 'dispatch;
	}
	// 827EE958: 38A001C0  li r5, 0x1c0
	ctx.r[5].s64 = 448;
	// 827EE95C: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 827EE960: 48603A89  bl 0x82df23e8
	ctx.lr = 0x827EE964;
	sub_82DF23E8(ctx, base);
	// 827EE964: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EE968: 41820010  beq 0x827ee978
	if ctx.cr[0].eq {
	pc = 0x827EE978; continue 'dispatch;
	}
	// 827EE96C: 4800A3BD  bl 0x827f8d28
	ctx.lr = 0x827EE970;
	sub_827F8D28(ctx, base);
	// 827EE970: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE974: 48000008  b 0x827ee97c
	pc = 0x827EE97C; continue 'dispatch;
	// 827EE978: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827EE97C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827EE980: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 827EE984: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EE988: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EE98C: 4BFFFC35  bl 0x827ee5c0
	ctx.lr = 0x827EE990;
	sub_827EE5C0(ctx, base);
	// 827EE990: 4800003C  b 0x827ee9cc
	pc = 0x827EE9CC; continue 'dispatch;
	// 827EE994: 38A001C5  li r5, 0x1c5
	ctx.r[5].s64 = 453;
	// 827EE998: 38600120  li r3, 0x120
	ctx.r[3].s64 = 288;
	// 827EE99C: 48603A4D  bl 0x82df23e8
	ctx.lr = 0x827EE9A0;
	sub_82DF23E8(ctx, base);
	// 827EE9A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EE9A4: 41820010  beq 0x827ee9b4
	if ctx.cr[0].eq {
	pc = 0x827EE9B4; continue 'dispatch;
	}
	// 827EE9A8: 48009D59  bl 0x827f8700
	ctx.lr = 0x827EE9AC;
	sub_827F8700(ctx, base);
	// 827EE9AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EE9B0: 48000008  b 0x827ee9b8
	pc = 0x827EE9B8; continue 'dispatch;
	// 827EE9B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827EE9B8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827EE9BC: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 827EE9C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EE9C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EE9C8: 4BFFFB31  bl 0x827ee4f8
	ctx.lr = 0x827EE9CC;
	sub_827EE4F8(ctx, base);
	// 827EE9CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EE9D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EE9D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EE9D8: 4BAD1629  bl 0x822c0000
	ctx.lr = 0x827EE9DC;
	sub_822C0000(ctx, base);
	// 827EE9DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EE9E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EE9E4: 489B97D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EE9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EE9E8 size=188
    let mut pc: u32 = 0x827EE9E8;
    'dispatch: loop {
        match pc {
            0x827EE9E8 => {
    //   block [0x827EE9E8..0x827EEAA4)
	// 827EE9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EE9EC: 489B9781  bl 0x831a816c
	ctx.lr = 0x827EE9F0;
	sub_831A8130(ctx, base);
	// 827EE9F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EE9F4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827EE9F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827EE9FC: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827EEA00: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 827EEA04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EEA08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EEA0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EEA10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827EEA14: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEAA8 size=80
    let mut pc: u32 = 0x827EEAA8;
    'dispatch: loop {
        match pc {
            0x827EEAA8 => {
    //   block [0x827EEAA8..0x827EEAF8)
	// 827EEAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EEAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EEAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EEAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EEAC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EEAC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EEAC8: 4BFFFF21  bl 0x827ee9e8
	ctx.lr = 0x827EEACC;
	sub_827EE9E8(ctx, base);
	// 827EEACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EEAD0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827EEAD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EEAD8: 4BFFF641  bl 0x827ee118
	ctx.lr = 0x827EEADC;
	sub_827EE118(ctx, base);
	// 827EEADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EEAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827EEAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EEAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EEAEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EEAF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EEAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEAF8 size=140
    let mut pc: u32 = 0x827EEAF8;
    'dispatch: loop {
        match pc {
            0x827EEAF8 => {
    //   block [0x827EEAF8..0x827EEB84)
	// 827EEAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EEB00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EEB04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEB08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EEB0C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EEB10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EEB14: 419A005C  beq cr6, 0x827eeb70
	if ctx.cr[6].eq {
	pc = 0x827EEB70; continue 'dispatch;
	}
	// 827EEB18: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EEB1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827EEB20: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EEB24: 5569083A  rlwinm r9, r11, 1, 0, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 827EEB28: 556B1F38  rlwinm r11, r11, 3, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x1FFFFFFFu64;
	// 827EEB2C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 827EEB30: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827EEB34: 4829CA8D  bl 0x82a8b5c0
	ctx.lr = 0x827EEB38;
	sub_82A8B5C0(ctx, base);
	// 827EEB38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EEB3C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827EEB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827EEB44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 827EEB48: 5529083C  slwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 827EEB4C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827EEB50: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827EEB54: 41990008  bgt cr6, 0x827eeb5c
	if ctx.cr[6].gt {
	pc = 0x827EEB5C; continue 'dispatch;
	}
	// 827EEB58: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827EEB5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EEB60: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EEB64: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827EEB68: 40820008  bne 0x827eeb70
	if !ctx.cr[0].eq {
	pc = 0x827EEB70; continue 'dispatch;
	}
	// 827EEB6C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827EEB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EEB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EEB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EEB7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EEB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEB88 size=84
    let mut pc: u32 = 0x827EEB88;
    'dispatch: loop {
        match pc {
            0x827EEB88 => {
    //   block [0x827EEB88..0x827EEBDC)
	// 827EEB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EEB90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EEB94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EEB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEB9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EEBA0: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 827EEBA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EEBA8: 419A001C  beq cr6, 0x827eebc4
	if ctx.cr[6].eq {
	pc = 0x827EEBC4; continue 'dispatch;
	}
	// 827EEBAC: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 827EEBB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EEBB4: 4BFFFF45  bl 0x827eeaf8
	ctx.lr = 0x827EEBB8;
	sub_827EEAF8(ctx, base);
	// 827EEBB8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 827EEBBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EEBC0: 409AFFF0  bne cr6, 0x827eebb0
	if !ctx.cr[6].eq {
	pc = 0x827EEBB0; continue 'dispatch;
	}
	// 827EEBC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EEBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EEBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EEBD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EEBD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EEBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEBE0 size=144
    let mut pc: u32 = 0x827EEBE0;
    'dispatch: loop {
        match pc {
            0x827EEBE0 => {
    //   block [0x827EEBE0..0x827EEC70)
	// 827EEBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EEBE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EEBEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EEBF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEBF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EEBF8: 83FE00B4  lwz r31, 0xb4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 827EEBFC: 807E00B0  lwz r3, 0xb0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) } as u64;
	// 827EEC00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EEC04: 419A0024  beq cr6, 0x827eec28
	if ctx.cr[6].eq {
	pc = 0x827EEC28; continue 'dispatch;
	}
	// 827EEC08: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 827EEC0C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827EEC10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EEC14: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827EEC18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827EEC1C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827EEC20: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EEC24: 4082FFE8  bne 0x827eec0c
	if !ctx.cr[0].eq {
	pc = 0x827EEC0C; continue 'dispatch;
	}
	// 827EEC28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EEC2C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 827EEC30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EEC34: 4E800421  bctrl
	ctx.lr = 0x827EEC38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EEC38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827EEC3C: 419A000C  beq cr6, 0x827eec48
	if ctx.cr[6].eq {
	pc = 0x827EEC48; continue 'dispatch;
	}
	// 827EEC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EEC44: 4BAD1C4D  bl 0x822c0890
	ctx.lr = 0x827EEC48;
	sub_822C0890(ctx, base);
	// 827EEC48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EEC4C: 4BFFFF3D  bl 0x827eeb88
	ctx.lr = 0x827EEC50;
	sub_827EEB88(ctx, base);
	// 827EEC50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EEC54: 48000B15  bl 0x827ef768
	ctx.lr = 0x827EEC58;
	sub_827EF768(ctx, base);
	// 827EEC58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EEC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EEC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EEC64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EEC68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EEC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEC70 size=124
    let mut pc: u32 = 0x827EEC70;
    'dispatch: loop {
        match pc {
            0x827EEC70 => {
    //   block [0x827EEC70..0x827EECEC)
	// 827EEC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EEC78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EEC7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EEC80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEC84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EEC88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EEC8C: 4BFFFF55  bl 0x827eebe0
	ctx.lr = 0x827EEC90;
	sub_827EEBE0(ctx, base);
	// 827EEC90: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EEC94: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EEC98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EEC9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827EECA0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827EECA4: 419A0024  beq cr6, 0x827eecc8
	if ctx.cr[6].eq {
	pc = 0x827EECC8; continue 'dispatch;
	}
	// 827EECA8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827EECAC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827EECB0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EECB4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827EECB8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827EECBC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827EECC0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EECC4: 4082FFE8  bne 0x827eecac
	if !ctx.cr[0].eq {
	pc = 0x827EECAC; continue 'dispatch;
	}
	// 827EECC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EECCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EECD0: 48000FA9  bl 0x827efc78
	ctx.lr = 0x827EECD4;
	sub_827EFC78(ctx, base);
	// 827EECD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EECE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EECE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EECE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EECF0 size=116
    let mut pc: u32 = 0x827EECF0;
    'dispatch: loop {
        match pc {
            0x827EECF0 => {
    //   block [0x827EECF0..0x827EED64)
	// 827EECF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EECF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EECF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EECFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EED00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EED04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EED08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EED0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EED10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EED14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EED18: 48000E89  bl 0x827efba0
	ctx.lr = 0x827EED1C;
	sub_827EFBA0(ctx, base);
	// 827EED1C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827EED20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EED24: 419A0010  beq cr6, 0x827eed34
	if ctx.cr[6].eq {
	pc = 0x827EED34; continue 'dispatch;
	}
	// 827EED28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EED2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EED30: 4BFFFF41  bl 0x827eec70
	ctx.lr = 0x827EED34;
	sub_827EEC70(ctx, base);
	// 827EED34: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EED38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EED3C: 419A0008  beq cr6, 0x827eed44
	if ctx.cr[6].eq {
	pc = 0x827EED44; continue 'dispatch;
	}
	// 827EED40: 4BAD1B51  bl 0x822c0890
	ctx.lr = 0x827EED44;
	sub_822C0890(ctx, base);
	// 827EED44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EED48: 486046E1  bl 0x82df3428
	ctx.lr = 0x827EED4C;
	sub_82DF3428(ctx, base);
	// 827EED4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EED50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EED54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EED58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EED5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EED60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EED68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EED68 size=104
    let mut pc: u32 = 0x827EED68;
    'dispatch: loop {
        match pc {
            0x827EED68 => {
    //   block [0x827EED68..0x827EEDD0)
	// 827EED68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EED6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EED70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EED74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EED78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EED7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EED80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EED84: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EED88: 4BFFD439  bl 0x827ec1c0
	ctx.lr = 0x827EED8C;
	sub_827EC1C0(ctx, base);
	// 827EED8C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827EED90: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 827EED94: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EED98: 4BFFD429  bl 0x827ec1c0
	ctx.lr = 0x827EED9C;
	sub_827EC1C0(ctx, base);
	// 827EED9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EEDA0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 827EEDA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EEDA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EEDAC: 4E800421  bctrl
	ctx.lr = 0x827EEDB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EEDB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EEDB4: 4BFFFE2D  bl 0x827eebe0
	ctx.lr = 0x827EEDB8;
	sub_827EEBE0(ctx, base);
	// 827EEDB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EEDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EEDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EEDC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EEDC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EEDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEDD0 size=508
    let mut pc: u32 = 0x827EEDD0;
    'dispatch: loop {
        match pc {
            0x827EEDD0 => {
    //   block [0x827EEDD0..0x827EEFCC)
	// 827EEDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEDD4: 489B9399  bl 0x831a816c
	ctx.lr = 0x827EEDD8;
	sub_831A8130(ctx, base);
	// 827EEDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEDDC: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EEDE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EEDE4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827EEDE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EEDEC: 388B5F2C  addi r4, r11, 0x5f2c
	ctx.r[4].s64 = ctx.r[11].s64 + 24364;
	// 827EEDF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827EEDF4: 48604C15  bl 0x82df3a08
	ctx.lr = 0x827EEDF8;
	sub_82DF3A08(ctx, base);
	// 827EEDF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EEDFC: 486043B5  bl 0x82df31b0
	ctx.lr = 0x827EEE00;
	sub_82DF31B0(ctx, base);
	// 827EEE00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EEE04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEE08: 48604C01  bl 0x82df3a08
	ctx.lr = 0x827EEE0C;
	sub_82DF3A08(ctx, base);
	// 827EEE0C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827EEE10: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EEE14: 48604A9D  bl 0x82df38b0
	ctx.lr = 0x827EEE18;
	sub_82DF38B0(ctx, base);
	// 827EEE18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEE1C: 4860460D  bl 0x82df3428
	ctx.lr = 0x827EEE20;
	sub_82DF3428(ctx, base);
	// 827EEE20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EEE24: 486042CD  bl 0x82df30f0
	ctx.lr = 0x827EEE28;
	sub_82DF30F0(ctx, base);
	// 827EEE28: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EEE2C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827EEE30: 38AB5F24  addi r5, r11, 0x5f24
	ctx.r[5].s64 = ctx.r[11].s64 + 24356;
	// 827EEE34: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEE38: 48604EC1  bl 0x82df3cf8
	ctx.lr = 0x827EEE3C;
	sub_82DF3CF8(ctx, base);
	// 827EEE3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EEE40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EEE44: 48604D8D  bl 0x82df3bd0
	ctx.lr = 0x827EEE48;
	sub_82DF3BD0(ctx, base);
	// 827EEE48: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEE4C: 486045DD  bl 0x82df3428
	ctx.lr = 0x827EEE50;
	sub_82DF3428(ctx, base);
	// 827EEE50: 807E0070  lwz r3, 0x70(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EEE54: 4BFFD36D  bl 0x827ec1c0
	ctx.lr = 0x827EEE58;
	sub_827EC1C0(ctx, base);
	// 827EEE58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EEE5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EEE60: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EEE64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EEE68: 4E800421  bctrl
	ctx.lr = 0x827EEE6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EEE6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EEE70: 41820008  beq 0x827eee78
	if ctx.cr[0].eq {
	pc = 0x827EEE78; continue 'dispatch;
	}
	// 827EEE74: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 827EEE78: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EEE7C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827EEE80: 38AB5D94  addi r5, r11, 0x5d94
	ctx.r[5].s64 = ctx.r[11].s64 + 23956;
	// 827EEE84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEE88: 48604E71  bl 0x82df3cf8
	ctx.lr = 0x827EEE8C;
	sub_82DF3CF8(ctx, base);
	// 827EEE8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EEE90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EEE94: 48604D3D  bl 0x82df3bd0
	ctx.lr = 0x827EEE98;
	sub_82DF3BD0(ctx, base);
	// 827EEE98: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEE9C: 4860458D  bl 0x82df3428
	ctx.lr = 0x827EEEA0;
	sub_82DF3428(ctx, base);
	// 827EEEA0: 807E0070  lwz r3, 0x70(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EEEA4: 4BFFD31D  bl 0x827ec1c0
	ctx.lr = 0x827EEEA8;
	sub_827EC1C0(ctx, base);
	// 827EEEA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EEEAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EEEB0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EEEB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EEEB8: 4E800421  bctrl
	ctx.lr = 0x827EEEBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EEEBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EEEC0: 41820008  beq 0x827eeec8
	if ctx.cr[0].eq {
	pc = 0x827EEEC8; continue 'dispatch;
	}
	// 827EEEC4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 827EEEC8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EEECC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827EEED0: 38AB5F1C  addi r5, r11, 0x5f1c
	ctx.r[5].s64 = ctx.r[11].s64 + 24348;
	// 827EEED4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEED8: 48604E21  bl 0x82df3cf8
	ctx.lr = 0x827EEEDC;
	sub_82DF3CF8(ctx, base);
	// 827EEEDC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EEEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EEEE4: 48604CED  bl 0x82df3bd0
	ctx.lr = 0x827EEEE8;
	sub_82DF3BD0(ctx, base);
	// 827EEEE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEEEC: 4860453D  bl 0x82df3428
	ctx.lr = 0x827EEEF0;
	sub_82DF3428(ctx, base);
	// 827EEEF0: 807E0070  lwz r3, 0x70(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EEEF4: 4BFFD2CD  bl 0x827ec1c0
	ctx.lr = 0x827EEEF8;
	sub_827EC1C0(ctx, base);
	// 827EEEF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EEEFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EEF00: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EEF04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EEF08: 4E800421  bctrl
	ctx.lr = 0x827EEF0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EEF0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EEF10: 41820008  beq 0x827eef18
	if ctx.cr[0].eq {
	pc = 0x827EEF18; continue 'dispatch;
	}
	// 827EEF14: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 827EEF18: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 827EEF1C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827EEF20: 409A007C  bne cr6, 0x827eef9c
	if !ctx.cr[6].eq {
	pc = 0x827EEF9C; continue 'dispatch;
	}
	// 827EEF24: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EEF28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827EEF2C: 388B5EDC  addi r4, r11, 0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + 24284;
	// 827EEF30: 38A00304  li r5, 0x304
	ctx.r[5].s64 = 772;
	// 827EEF34: 38600064  li r3, 0x64
	ctx.r[3].s64 = 100;
	// 827EEF38: 486034B1  bl 0x82df23e8
	ctx.lr = 0x827EEF3C;
	sub_82DF23E8(ctx, base);
	// 827EEF3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EEF40: 41820018  beq 0x827eef58
	if ctx.cr[0].eq {
	pc = 0x827EEF58; continue 'dispatch;
	}
	// 827EEF44: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EEF48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827EEF4C: 48006C1D  bl 0x827f5b68
	ctx.lr = 0x827EEF50;
	sub_827F5B68(ctx, base);
	// 827EEF50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EEF54: 48000008  b 0x827eef5c
	pc = 0x827EEF5C; continue 'dispatch;
	// 827EEF58: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827EEF5C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 827EEF60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EEF64: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EEF68: 4BFFF721  bl 0x827ee688
	ctx.lr = 0x827EEF6C;
	sub_827EE688(ctx, base);
	// 827EEF6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EEF70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EEF74: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EEF78: 4BAD1089  bl 0x822c0000
	ctx.lr = 0x827EEF7C;
	sub_822C0000(ctx, base);
	// 827EEF7C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827EEF80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EEF84: 4BFFFCED  bl 0x827eec70
	ctx.lr = 0x827EEF88;
	sub_827EEC70(ctx, base);
	// 827EEF88: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827EEF8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EEF90: 419A0024  beq cr6, 0x827eefb4
	if ctx.cr[6].eq {
	pc = 0x827EEFB4; continue 'dispatch;
	}
	// 827EEF94: 4BAD18FD  bl 0x822c0890
	ctx.lr = 0x827EEF98;
	sub_822C0890(ctx, base);
	// 827EEF98: 4800001C  b 0x827eefb4
	pc = 0x827EEFB4; continue 'dispatch;
	// 827EEF9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827EEFA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EEFA4: 48604C5D  bl 0x82df3c00
	ctx.lr = 0x827EEFA8;
	sub_82DF3C00(ctx, base);
	// 827EEFA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EEFAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EEFB0: 4BFFFD41  bl 0x827eecf0
	ctx.lr = 0x827EEFB4;
	sub_827EECF0(ctx, base);
	// 827EEFB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EEFB8: 48604471  bl 0x82df3428
	ctx.lr = 0x827EEFBC;
	sub_82DF3428(ctx, base);
	// 827EEFBC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EEFC0: 48604469  bl 0x82df3428
	ctx.lr = 0x827EEFC4;
	sub_82DF3428(ctx, base);
	// 827EEFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827EEFC8: 489B91F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EEFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EEFD0 size=136
    let mut pc: u32 = 0x827EEFD0;
    'dispatch: loop {
        match pc {
            0x827EEFD0 => {
    //   block [0x827EEFD0..0x827EF058)
	// 827EEFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EEFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EEFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EEFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EEFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EEFE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EEFE8: 4867A4F9  bl 0x82e694e0
	ctx.lr = 0x827EEFEC;
	sub_82E694E0(ctx, base);
	// 827EEFEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EEFF0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EEFF4: 4867A695  bl 0x82e69688
	ctx.lr = 0x827EEFF8;
	sub_82E69688(ctx, base);
	// 827EEFF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EEFFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF000: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827EF004: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 827EF008: 4867A5E1  bl 0x82e695e8
	ctx.lr = 0x827EF00C;
	sub_82E695E8(ctx, base);
	// 827EF00C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EF010: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF014: 486049F5  bl 0x82df3a08
	ctx.lr = 0x827EF018;
	sub_82DF3A08(ctx, base);
	// 827EF018: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF01C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF020: 4BFFFDB1  bl 0x827eedd0
	ctx.lr = 0x827EF024;
	sub_827EEDD0(ctx, base);
	// 827EF024: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827EF028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF02C: 4867B325  bl 0x82e6a350
	ctx.lr = 0x827EF030;
	sub_82E6A350(ctx, base);
	// 827EF030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF034: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF038: 486043F1  bl 0x82df3428
	ctx.lr = 0x827EF03C;
	sub_82DF3428(ctx, base);
	// 827EF03C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF040: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EF044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF04C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF050: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EF058 size=148
    let mut pc: u32 = 0x827EF058;
    'dispatch: loop {
        match pc {
            0x827EF058 => {
    //   block [0x827EF058..0x827EF0EC)
	// 827EF058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF068: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF06C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF070: 4867A471  bl 0x82e694e0
	ctx.lr = 0x827EF074;
	sub_82E694E0(ctx, base);
	// 827EF074: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF078: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EF07C: 4867A60D  bl 0x82e69688
	ctx.lr = 0x827EF080;
	sub_82E69688(ctx, base);
	// 827EF080: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EF084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF088: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827EF08C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 827EF090: 4867A559  bl 0x82e695e8
	ctx.lr = 0x827EF094;
	sub_82E695E8(ctx, base);
	// 827EF094: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EF098: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF09C: 4BAD682D  bl 0x822c58c8
	ctx.lr = 0x827EF0A0;
	sub_822C58C8(ctx, base);
	// 827EF0A0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 827EF0A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF0A8: C02BD7BC  lfs f1, -0x2844(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10308 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827EF0AC: 4BFFEC95  bl 0x827edd40
	ctx.lr = 0x827EF0B0;
	sub_827EDD40(ctx, base);
	// 827EF0B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EF0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF0B8: 5564063E  clrlwi r4, r11, 0x18
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 827EF0BC: 4867A7AD  bl 0x82e69868
	ctx.lr = 0x827EF0C0;
	sub_82E69868(ctx, base);
	// 827EF0C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827EF0C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EF0C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF0CC: 4BAD5C15  bl 0x822c4ce0
	ctx.lr = 0x827EF0D0;
	sub_822C4CE0(ctx, base);
	// 827EF0D0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827EF0D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827EF0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF0E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF0E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF0F0 size=120
    let mut pc: u32 = 0x827EF0F0;
    'dispatch: loop {
        match pc {
            0x827EF0F0 => {
    //   block [0x827EF0F0..0x827EF168)
	// 827EF0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF0F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF0FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF104: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF108: 4867A3D9  bl 0x82e694e0
	ctx.lr = 0x827EF10C;
	sub_82E694E0(ctx, base);
	// 827EF10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF110: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EF114: 4867A575  bl 0x82e69688
	ctx.lr = 0x827EF118;
	sub_82E69688(ctx, base);
	// 827EF118: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EF11C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827EF120: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 827EF124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF128: 4867A4C1  bl 0x82e695e8
	ctx.lr = 0x827EF12C;
	sub_82E695E8(ctx, base);
	// 827EF12C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EF130: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF134: 486048D5  bl 0x82df3a08
	ctx.lr = 0x827EF138;
	sub_82DF3A08(ctx, base);
	// 827EF138: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF13C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF140: 4BFFFC29  bl 0x827eed68
	ctx.lr = 0x827EF144;
	sub_827EED68(ctx, base);
	// 827EF144: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF148: 486042E1  bl 0x82df3428
	ctx.lr = 0x827EF14C;
	sub_82DF3428(ctx, base);
	// 827EF14C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827EF150: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EF154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF15C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF168 size=240
    let mut pc: u32 = 0x827EF168;
    'dispatch: loop {
        match pc {
            0x827EF168 => {
    //   block [0x827EF168..0x827EF258)
	// 827EF168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF16C: 489B9001  bl 0x831a816c
	ctx.lr = 0x827EF170;
	sub_831A8130(ctx, base);
	// 827EF170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF174: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF17C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF180: 388B5F68  addi r4, r11, 0x5f68
	ctx.r[4].s64 = ctx.r[11].s64 + 24424;
	// 827EF184: 48604885  bl 0x82df3a08
	ctx.lr = 0x827EF188;
	sub_82DF3A08(ctx, base);
	// 827EF188: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF18C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 827EF190: 4BFFD031  bl 0x827ec1c0
	ctx.lr = 0x827EF194;
	sub_827EC1C0(ctx, base);
	// 827EF194: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EF198: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EF19C: 48679D4D  bl 0x82e68ee8
	ctx.lr = 0x827EF1A0;
	sub_82E68EE8(ctx, base);
	// 827EF1A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF1A4: 48604285  bl 0x82df3428
	ctx.lr = 0x827EF1A8;
	sub_82DF3428(ctx, base);
	// 827EF1A8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF1AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EF1B0: 388B5F5C  addi r4, r11, 0x5f5c
	ctx.r[4].s64 = ctx.r[11].s64 + 24412;
	// 827EF1B4: 48604855  bl 0x82df3a08
	ctx.lr = 0x827EF1B8;
	sub_82DF3A08(ctx, base);
	// 827EF1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 827EF1BC: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF1C0: 3BC10054  addi r30, r1, 0x54
	ctx.r[30].s64 = ctx.r[1].s64 + 84;
	// 827EF1C4: 3BABEFD0  addi r29, r11, -0x1030
	ctx.r[29].s64 = ctx.r[11].s64 + -4144;
	// 827EF1C8: 4BFFCFF9  bl 0x827ec1c0
	ctx.lr = 0x827EF1CC;
	sub_827EC1C0(ctx, base);
	// 827EF1CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827EF1D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EF1D4: 48679CAD  bl 0x82e68e80
	ctx.lr = 0x827EF1D8;
	sub_82E68E80(ctx, base);
	// 827EF1D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EF1DC: 4860424D  bl 0x82df3428
	ctx.lr = 0x827EF1E0;
	sub_82DF3428(ctx, base);
	// 827EF1E0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF1E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EF1E8: 388B5F4C  addi r4, r11, 0x5f4c
	ctx.r[4].s64 = ctx.r[11].s64 + 24396;
	// 827EF1EC: 4860481D  bl 0x82df3a08
	ctx.lr = 0x827EF1F0;
	sub_82DF3A08(ctx, base);
	// 827EF1F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 827EF1F4: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF1F8: 3BC10058  addi r30, r1, 0x58
	ctx.r[30].s64 = ctx.r[1].s64 + 88;
	// 827EF1FC: 3BABF058  addi r29, r11, -0xfa8
	ctx.r[29].s64 = ctx.r[11].s64 + -4008;
	// 827EF200: 4BFFCFC1  bl 0x827ec1c0
	ctx.lr = 0x827EF204;
	sub_827EC1C0(ctx, base);
	// 827EF204: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827EF208: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EF20C: 48679C75  bl 0x82e68e80
	ctx.lr = 0x827EF210;
	sub_82E68E80(ctx, base);
	// 827EF210: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EF214: 48604215  bl 0x82df3428
	ctx.lr = 0x827EF218;
	sub_82DF3428(ctx, base);
	// 827EF218: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF21C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EF220: 388B5F38  addi r4, r11, 0x5f38
	ctx.r[4].s64 = ctx.r[11].s64 + 24376;
	// 827EF224: 486047E5  bl 0x82df3a08
	ctx.lr = 0x827EF228;
	sub_82DF3A08(ctx, base);
	// 827EF228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 827EF22C: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF230: 3BE1005C  addi r31, r1, 0x5c
	ctx.r[31].s64 = ctx.r[1].s64 + 92;
	// 827EF234: 3BCBF0F0  addi r30, r11, -0xf10
	ctx.r[30].s64 = ctx.r[11].s64 + -3856;
	// 827EF238: 4BFFCF89  bl 0x827ec1c0
	ctx.lr = 0x827EF23C;
	sub_827EC1C0(ctx, base);
	// 827EF23C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EF240: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EF244: 48679C3D  bl 0x82e68e80
	ctx.lr = 0x827EF248;
	sub_82E68E80(ctx, base);
	// 827EF248: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827EF24C: 486041DD  bl 0x82df3428
	ctx.lr = 0x827EF250;
	sub_82DF3428(ctx, base);
	// 827EF250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827EF254: 489B8F68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EF258 size=236
    let mut pc: u32 = 0x827EF258;
    'dispatch: loop {
        match pc {
            0x827EF258 => {
    //   block [0x827EF258..0x827EF344)
	// 827EF258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF26C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF270: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 827EF274: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF278: 419A0040  beq cr6, 0x827ef2b8
	if ctx.cr[6].eq {
	pc = 0x827EF2B8; continue 'dispatch;
	}
	// 827EF27C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF280: C0240000  lfs f1, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827EF284: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EF288: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 827EF28C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EF290: 4E800421  bctrl
	ctx.lr = 0x827EF294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EF294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF298: 480004E9  bl 0x827ef780
	ctx.lr = 0x827EF29C;
	sub_827EF780(ctx, base);
	// 827EF29C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827EF2A0: 41820018  beq 0x827ef2b8
	if ctx.cr[0].eq {
	pc = 0x827EF2B8; continue 'dispatch;
	}
	// 827EF2A4: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 827EF2A8: 48005139  bl 0x827f43e0
	ctx.lr = 0x827EF2AC;
	sub_827F43E0(ctx, base);
	// 827EF2AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827EF2B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF2B4: 4BFFAFBD  bl 0x827ea270
	ctx.lr = 0x827EF2B8;
	sub_827EA270(ctx, base);
	// 827EF2B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF2BC: 48000495  bl 0x827ef750
	ctx.lr = 0x827EF2C0;
	sub_827EF750(ctx, base);
	// 827EF2C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EF2C4: 40820068  bne 0x827ef32c
	if !ctx.cr[0].eq {
	pc = 0x827EF32C; continue 'dispatch;
	}
	// 827EF2C8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 827EF2CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EF2D0: 409A000C  bne cr6, 0x827ef2dc
	if !ctx.cr[6].eq {
	pc = 0x827EF2DC; continue 'dispatch;
	}
	// 827EF2D4: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 827EF2D8: 4BC643A9  bl 0x82453680
	ctx.lr = 0x827EF2DC;
	sub_82453680(ctx, base);
	// 827EF2DC: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 827EF2E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EF2E4: 419A0048  beq cr6, 0x827ef32c
	if ctx.cr[6].eq {
	pc = 0x827EF32C; continue 'dispatch;
	}
	// 827EF2E8: 813F00CC  lwz r9, 0xcc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 827EF2EC: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 827EF2F0: 815F00C8  lwz r10, 0xc8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 827EF2F4: 552BF87E  srwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827EF2F8: 552907FE  clrlwi r9, r9, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 827EF2FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827EF300: 41990008  bgt cr6, 0x827ef308
	if ctx.cr[6].gt {
	pc = 0x827EF308; continue 'dispatch;
	}
	// 827EF304: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827EF308: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EF30C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 827EF310: 552B1838  slwi r11, r9, 3
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827EF314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF318: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 827EF31C: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827EF320: 48000439  bl 0x827ef758
	ctx.lr = 0x827EF324;
	sub_827EF758(ctx, base);
	// 827EF324: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF328: 4BFFF7D1  bl 0x827eeaf8
	ctx.lr = 0x827EF32C;
	sub_827EEAF8(ctx, base);
	// 827EF32C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EF330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF33C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF348 size=252
    let mut pc: u32 = 0x827EF348;
    'dispatch: loop {
        match pc {
            0x827EF348 => {
    //   block [0x827EF348..0x827EF444)
	// 827EF348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF34C: 489B8E1D  bl 0x831a8168
	ctx.lr = 0x827EF350;
	sub_831A8130(ctx, base);
	// 827EF350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF358: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827EF35C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EF360: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EF364: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 827EF368: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827EF36C: 40820020  bne 0x827ef38c
	if !ctx.cr[0].eq {
	pc = 0x827EF38C; continue 'dispatch;
	}
	// 827EF370: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 827EF374: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827EF378: 556BF87E  srwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827EF37C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827EF380: 4199000C  bgt cr6, 0x827ef38c
	if ctx.cr[6].gt {
	pc = 0x827EF38C; continue 'dispatch;
	}
	// 827EF384: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EF388: 4802BFC9  bl 0x8281b350
	ctx.lr = 0x827EF38C;
	sub_8281B350(ctx, base);
	// 827EF38C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EF390: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EF394: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827EF398: 7FA95A14  add r29, r9, r11
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 827EF39C: 57ABF87E  srwi r11, r29, 1
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827EF3A0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827EF3A4: 41990008  bgt cr6, 0x827ef3ac
	if ctx.cr[6].gt {
	pc = 0x827EF3AC; continue 'dispatch;
	}
	// 827EF3A8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827EF3AC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EF3B0: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 827EF3B4: 7D6AF02E  lwzx r11, r10, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 827EF3B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EF3BC: 409A0028  bne cr6, 0x827ef3e4
	if !ctx.cr[6].eq {
	pc = 0x827EF3E4; continue 'dispatch;
	}
	// 827EF3C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827EF3C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827EF3C8: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 827EF3CC: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 827EF3D0: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 827EF3D4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827EF3D8: 48602CF1  bl 0x82df20c8
	ctx.lr = 0x827EF3DC;
	sub_82DF20C8(ctx, base);
	// 827EF3DC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EF3E0: 7C6BF12E  stwx r3, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 827EF3E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EF3E8: 57AA1F38  rlwinm r10, r29, 3, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x1FFFFFFFu64;
	// 827EF3EC: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 827EF3F0: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EF3F4: 4182003C  beq 0x827ef430
	if ctx.cr[0].eq {
	pc = 0x827EF430; continue 'dispatch;
	}
	// 827EF3F8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF3FC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827EF400: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EF404: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827EF408: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827EF40C: 419A0024  beq cr6, 0x827ef430
	if ctx.cr[6].eq {
	pc = 0x827EF430; continue 'dispatch;
	}
	// 827EF410: 396A0004  addi r11, r10, 4
	ctx.r[11].s64 = ctx.r[10].s64 + 4;
	// 827EF414: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827EF418: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EF41C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827EF420: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827EF424: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827EF428: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827EF42C: 4082FFE8  bne 0x827ef414
	if !ctx.cr[0].eq {
	pc = 0x827EF414; continue 'dispatch;
	}
	// 827EF430: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EF434: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 827EF438: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827EF43C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827EF440: 489B8D78  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF448 size=108
    let mut pc: u32 = 0x827EF448;
    'dispatch: loop {
        match pc {
            0x827EF448 => {
    //   block [0x827EF448..0x827EF4B4)
	// 827EF448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF450: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF458: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF45C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF460: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827EF464: 396B5F84  addi r11, r11, 0x5f84
	ctx.r[11].s64 = ctx.r[11].s64 + 24452;
	// 827EF468: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EF46C: 4BDD0C35  bl 0x825c00a0
	ctx.lr = 0x827EF470;
	sub_825C00A0(ctx, base);
	// 827EF470: 807F00BC  lwz r3, 0xbc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 827EF474: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF478: 419A0008  beq cr6, 0x827ef480
	if ctx.cr[6].eq {
	pc = 0x827EF480; continue 'dispatch;
	}
	// 827EF47C: 4BAD1415  bl 0x822c0890
	ctx.lr = 0x827EF480;
	sub_822C0890(ctx, base);
	// 827EF480: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 827EF484: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF488: 419A0008  beq cr6, 0x827ef490
	if ctx.cr[6].eq {
	pc = 0x827EF490; continue 'dispatch;
	}
	// 827EF48C: 4BAD1405  bl 0x822c0890
	ctx.lr = 0x827EF490;
	sub_822C0890(ctx, base);
	// 827EF490: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 827EF494: 4BAD9825  bl 0x822c8cb8
	ctx.lr = 0x827EF498;
	sub_822C8CB8(ctx, base);
	// 827EF498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF49C: 48000305  bl 0x827ef7a0
	ctx.lr = 0x827EF4A0;
	sub_827EF7A0(ctx, base);
	// 827EF4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EF4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF4AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF4B8 size=408
    let mut pc: u32 = 0x827EF4B8;
    'dispatch: loop {
        match pc {
            0x827EF4B8 => {
    //   block [0x827EF4B8..0x827EF650)
	// 827EF4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF4BC: 489B8C99  bl 0x831a8154
	ctx.lr = 0x827EF4C0;
	sub_831A8130(ctx, base);
	// 827EF4C0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF4C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827EF4C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EF4CC: 7FF9FB78  mr r25, r31
	ctx.r[25].u64 = ctx.r[31].u64;
	// 827EF4D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827EF4D4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 827EF4D8: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 827EF4DC: 48000585  bl 0x827efa60
	ctx.lr = 0x827EF4E0;
	sub_827EFA60(ctx, base);
	// 827EF4E0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF4E4: 3B5E00B0  addi r26, r30, 0xb0
	ctx.r[26].s64 = ctx.r[30].s64 + 176;
	// 827EF4E8: 396B5F84  addi r11, r11, 0x5f84
	ctx.r[11].s64 = ctx.r[11].s64 + 24452;
	// 827EF4EC: 3B9E00B8  addi r28, r30, 0xb8
	ctx.r[28].s64 = ctx.r[30].s64 + 184;
	// 827EF4F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EF4F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EF4F8: 93FE0090  stw r31, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 827EF4FC: 93FE00B0  stw r31, 0xb0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 827EF500: 93FE00B4  stw r31, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 827EF504: 93FE00B8  stw r31, 0xb8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 827EF508: 93FE00BC  stw r31, 0xbc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(188 as u32), ctx.r[31].u32 ) };
	// 827EF50C: 93FE00C4  stw r31, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 827EF510: 93FE00C8  stw r31, 0xc8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(200 as u32), ctx.r[31].u32 ) };
	// 827EF514: 93FE00CC  stw r31, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[31].u32 ) };
	// 827EF518: 93FE00D0  stw r31, 0xd0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(208 as u32), ctx.r[31].u32 ) };
	// 827EF51C: 93FE00D4  stw r31, 0xd4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[31].u32 ) };
	// 827EF520: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF524: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 827EF528: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EF52C: 4E800421  bctrl
	ctx.lr = 0x827EF530;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EF530: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF534: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 827EF538: 388B5EDC  addi r4, r11, 0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + 24284;
	// 827EF53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827EF540: 38A00041  li r5, 0x41
	ctx.r[5].s64 = 65;
	// 827EF544: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 827EF548: 4BAD0E91  bl 0x822c03d8
	ctx.lr = 0x827EF54C;
	sub_822C03D8(ctx, base);
	// 827EF54C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 827EF550: 41820044  beq 0x827ef594
	if ctx.cr[0].eq {
	pc = 0x827EF594; continue 'dispatch;
	}
	// 827EF554: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 827EF558: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 827EF55C: 48819A5D  bl 0x83008fb8
	ctx.lr = 0x827EF560;
	sub_83008FB8(ctx, base);
	// 827EF560: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF568: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 827EF56C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EF570: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 827EF574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EF578: 4E800421  bctrl
	ctx.lr = 0x827EF57C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EF57C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827EF580: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 827EF584: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827EF588: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827EF58C: 4800097D  bl 0x827eff08
	ctx.lr = 0x827EF590;
	sub_827EFF08(ctx, base);
	// 827EF590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF594: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827EF598: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EF59C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EF5A0: 4BFFEE99  bl 0x827ee438
	ctx.lr = 0x827EF5A4;
	sub_827EE438(ctx, base);
	// 827EF5A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EF5A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EF5AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827EF5B0: 4BAD0A51  bl 0x822c0000
	ctx.lr = 0x827EF5B4;
	sub_822C0000(ctx, base);
	// 827EF5B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827EF5B8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827EF5BC: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 827EF5C0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EF5C4: 4BAD4E9D  bl 0x822c4460
	ctx.lr = 0x827EF5C8;
	sub_822C4460(ctx, base);
	// 827EF5C8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EF5CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF5D0: 419A0008  beq cr6, 0x827ef5d8
	if ctx.cr[6].eq {
	pc = 0x827EF5D8; continue 'dispatch;
	}
	// 827EF5D4: 4BAD12BD  bl 0x822c0890
	ctx.lr = 0x827EF5D8;
	sub_822C0890(ctx, base);
	// 827EF5D8: 572B07FF  clrlwi. r11, r25, 0x1f
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EF5DC: 41820014  beq 0x827ef5f0
	if ctx.cr[0].eq {
	pc = 0x827EF5F0; continue 'dispatch;
	}
	// 827EF5E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827EF5E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF5E8: 419A0008  beq cr6, 0x827ef5f0
	if ctx.cr[6].eq {
	pc = 0x827EF5F0; continue 'dispatch;
	}
	// 827EF5EC: 4BAD12A5  bl 0x822c0890
	ctx.lr = 0x827EF5F0;
	sub_822C0890(ctx, base);
	// 827EF5F0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 827EF5F4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EF5F8: 4BFFF321  bl 0x827ee918
	ctx.lr = 0x827EF5FC;
	sub_827EE918(ctx, base);
	// 827EF5FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EF600: 387A0004  addi r3, r26, 4
	ctx.r[3].s64 = ctx.r[26].s64 + 4;
	// 827EF604: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827EF608: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF60C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EF610: 4BAD4E51  bl 0x822c4460
	ctx.lr = 0x827EF614;
	sub_822C4460(ctx, base);
	// 827EF614: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827EF618: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF61C: 419A0008  beq cr6, 0x827ef624
	if ctx.cr[6].eq {
	pc = 0x827EF624; continue 'dispatch;
	}
	// 827EF620: 4BAD1271  bl 0x822c0890
	ctx.lr = 0x827EF624;
	sub_822C0890(ctx, base);
	// 827EF624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EF628: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EF62C: 4BFFCA7D  bl 0x827ec0a8
	ctx.lr = 0x827EF630;
	sub_827EC0A8(ctx, base);
	// 827EF630: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 827EF634: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EF638: 4BFFCAD9  bl 0x827ec110
	ctx.lr = 0x827EF63C;
	sub_827EC110(ctx, base);
	// 827EF63C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF640: 4BFFFB29  bl 0x827ef168
	ctx.lr = 0x827EF644;
	sub_827EF168(ctx, base);
	// 827EF644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF648: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 827EF64C: 489B8B58  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF650 size=76
    let mut pc: u32 = 0x827EF650;
    'dispatch: loop {
        match pc {
            0x827EF650 => {
    //   block [0x827EF650..0x827EF69C)
	// 827EF650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF65C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF664: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF668: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EF66C: 4BFFFDDD  bl 0x827ef448
	ctx.lr = 0x827EF670;
	sub_827EF448(ctx, base);
	// 827EF670: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827EF674: 4182000C  beq 0x827ef680
	if ctx.cr[0].eq {
	pc = 0x827EF680; continue 'dispatch;
	}
	// 827EF678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF67C: 4BAD0BED  bl 0x822c0268
	ctx.lr = 0x827EF680;
	sub_822C0268(ctx, base);
	// 827EF680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF684: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EF688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF690: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF6A0 size=12
    let mut pc: u32 = 0x827EF6A0;
    'dispatch: loop {
        match pc {
            0x827EF6A0 => {
    //   block [0x827EF6A0..0x827EF6AC)
	// 827EF6A0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF6A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EF6A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF6AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF6AC size=8
    let mut pc: u32 = 0x827EF6AC;
    'dispatch: loop {
        match pc {
            0x827EF6AC => {
    //   block [0x827EF6AC..0x827EF6B4)
	// 827EF6AC: 386300C0  addi r3, r3, 0xc0
	ctx.r[3].s64 = ctx.r[3].s64 + 192;
	// 827EF6B0: 4BFFFC98  b 0x827ef348
	sub_827EF348(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF6B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF6B4 size=4
    let mut pc: u32 = 0x827EF6B4;
    'dispatch: loop {
        match pc {
            0x827EF6B4 => {
    //   block [0x827EF6B4..0x827EF6B8)
	// 827EF6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF6B8 size=116
    let mut pc: u32 = 0x827EF6B8;
    'dispatch: loop {
        match pc {
            0x827EF6B8 => {
    //   block [0x827EF6B8..0x827EF72C)
	// 827EF6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF6C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF6C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF6C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF6CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF6D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EF6D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827EF6D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EF6DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF6E0: 480004C1  bl 0x827efba0
	ctx.lr = 0x827EF6E4;
	sub_827EFBA0(ctx, base);
	// 827EF6E4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827EF6E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EF6EC: 419A0010  beq cr6, 0x827ef6fc
	if ctx.cr[6].eq {
	pc = 0x827EF6FC; continue 'dispatch;
	}
	// 827EF6F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF6F4: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827EF6F8: 4BFFFC51  bl 0x827ef348
	ctx.lr = 0x827EF6FC;
	sub_827EF348(ctx, base);
	// 827EF6FC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EF700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EF704: 419A0008  beq cr6, 0x827ef70c
	if ctx.cr[6].eq {
	pc = 0x827EF70C; continue 'dispatch;
	}
	// 827EF708: 4BAD1189  bl 0x822c0890
	ctx.lr = 0x827EF70C;
	sub_822C0890(ctx, base);
	// 827EF70C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF710: 48603D19  bl 0x82df3428
	ctx.lr = 0x827EF714;
	sub_82DF3428(ctx, base);
	// 827EF714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EF718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF730 size=8
    let mut pc: u32 = 0x827EF730;
    'dispatch: loop {
        match pc {
            0x827EF730 => {
    //   block [0x827EF730..0x827EF738)
	// 827EF730: 38630080  addi r3, r3, 0x80
	ctx.r[3].s64 = ctx.r[3].s64 + 128;
	// 827EF734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF738 size=8
    let mut pc: u32 = 0x827EF738;
    'dispatch: loop {
        match pc {
            0x827EF738 => {
    //   block [0x827EF738..0x827EF740)
	// 827EF738: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 827EF73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF740 size=8
    let mut pc: u32 = 0x827EF740;
    'dispatch: loop {
        match pc {
            0x827EF740 => {
    //   block [0x827EF740..0x827EF748)
	// 827EF740: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF744: 4BFFD5A4  b 0x827ecce8
	sub_827ECCE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF748 size=8
    let mut pc: u32 = 0x827EF748;
    'dispatch: loop {
        match pc {
            0x827EF748 => {
    //   block [0x827EF748..0x827EF750)
	// 827EF748: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF74C: 4BFFD6B4  b 0x827ece00
	sub_827ECE00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF750 size=8
    let mut pc: u32 = 0x827EF750;
    'dispatch: loop {
        match pc {
            0x827EF750 => {
    //   block [0x827EF750..0x827EF758)
	// 827EF750: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF754: 4BFFCD04  b 0x827ec458
	sub_827EC458(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF758 size=8
    let mut pc: u32 = 0x827EF758;
    'dispatch: loop {
        match pc {
            0x827EF758 => {
    //   block [0x827EF758..0x827EF760)
	// 827EF758: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF75C: 4BFFD73C  b 0x827ece98
	sub_827ECE98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF760 size=8
    let mut pc: u32 = 0x827EF760;
    'dispatch: loop {
        match pc {
            0x827EF760 => {
    //   block [0x827EF760..0x827EF768)
	// 827EF760: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF764: 4BFFCD44  b 0x827ec4a8
	sub_827EC4A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF768 size=8
    let mut pc: u32 = 0x827EF768;
    'dispatch: loop {
        match pc {
            0x827EF768 => {
    //   block [0x827EF768..0x827EF770)
	// 827EF768: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF76C: 4BFFC99C  b 0x827ec108
	sub_827EC108(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF770 size=8
    let mut pc: u32 = 0x827EF770;
    'dispatch: loop {
        match pc {
            0x827EF770 => {
    //   block [0x827EF770..0x827EF778)
	// 827EF770: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF774: 4BFFCD94  b 0x827ec508
	sub_827EC508(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF778 size=8
    let mut pc: u32 = 0x827EF778;
    'dispatch: loop {
        match pc {
            0x827EF778 => {
    //   block [0x827EF778..0x827EF780)
	// 827EF778: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF77C: 4BFFCA4C  b 0x827ec1c8
	sub_827EC1C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF780 size=8
    let mut pc: u32 = 0x827EF780;
    'dispatch: loop {
        match pc {
            0x827EF780 => {
    //   block [0x827EF780..0x827EF788)
	// 827EF780: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF784: 4BFFCA4C  b 0x827ec1d0
	sub_827EC1D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF788 size=12
    let mut pc: u32 = 0x827EF788;
    'dispatch: loop {
        match pc {
            0x827EF788 => {
    //   block [0x827EF788..0x827EF794)
	// 827EF788: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827EF790: 4BFFCA70  b 0x827ec200
	sub_827EC200(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF798 size=8
    let mut pc: u32 = 0x827EF798;
    'dispatch: loop {
        match pc {
            0x827EF798 => {
    //   block [0x827EF798..0x827EF7A0)
	// 827EF798: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF79C: 4BFFCA54  b 0x827ec1f0
	sub_827EC1F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EF7A0 size=16
    let mut pc: u32 = 0x827EF7A0;
    'dispatch: loop {
        match pc {
            0x827EF7A0 => {
    //   block [0x827EF7A0..0x827EF7B0)
	// 827EF7A0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EF7A4: 396B5FB0  addi r11, r11, 0x5fb0
	ctx.r[11].s64 = ctx.r[11].s64 + 24496;
	// 827EF7A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EF7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EF7B0 size=156
    let mut pc: u32 = 0x827EF7B0;
    'dispatch: loop {
        match pc {
            0x827EF7B0 => {
    //   block [0x827EF7B0..0x827EF84C)
	// 827EF7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF7B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF7BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF7C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF7C4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827EF7C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827EF7CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EF7D0: 3BEBACA0  addi r31, r11, -0x5360
	ctx.r[31].s64 = ctx.r[11].s64 + -21344;
	// 827EF7D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827EF7D8: 816AACB0  lwz r11, -0x5350(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21328 as u32) ) } as u64;
	// 827EF7DC: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827EF7E0: 4082002C  bne 0x827ef80c
	if !ctx.cr[0].eq {
	pc = 0x827EF80C; continue 'dispatch;
	}
	// 827EF7E4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827EF7E8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827EF7EC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827EF7F0: 916AACB0  stw r11, -0x5350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21328 as u32), ctx.r[11].u32 ) };
	// 827EF7F4: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EF7F8: C1A808A8  lfs f13, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827EF7FC: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827EF800: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827EF804: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827EF808: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827EF80C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF810: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF814: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EF818: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EF81C: 4E800421  bctrl
	ctx.lr = 0x827EF820;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EF820: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EF824: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF82C: 4868C51D  bl 0x82e7bd48
	ctx.lr = 0x827EF830;
	sub_82E7BD48(ctx, base);
	// 827EF830: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF834: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827EF838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EF850 size=156
    let mut pc: u32 = 0x827EF850;
    'dispatch: loop {
        match pc {
            0x827EF850 => {
    //   block [0x827EF850..0x827EF8EC)
	// 827EF850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF85C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF860: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF864: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827EF868: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827EF86C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EF870: 3BEBACC0  addi r31, r11, -0x5340
	ctx.r[31].s64 = ctx.r[11].s64 + -21312;
	// 827EF874: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827EF878: 816AACD0  lwz r11, -0x5330(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21296 as u32) ) } as u64;
	// 827EF87C: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827EF880: 4082002C  bne 0x827ef8ac
	if !ctx.cr[0].eq {
	pc = 0x827EF8AC; continue 'dispatch;
	}
	// 827EF884: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827EF888: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827EF88C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827EF890: 916AACD0  stw r11, -0x5330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21296 as u32), ctx.r[11].u32 ) };
	// 827EF894: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EF898: C1A808A8  lfs f13, 0x8a8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827EF89C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827EF8A0: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827EF8A4: D1BF0008  stfs f13, 8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827EF8A8: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827EF8AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF8B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF8B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EF8B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EF8BC: 4E800421  bctrl
	ctx.lr = 0x827EF8C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EF8C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EF8C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF8C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF8CC: 4868C47D  bl 0x82e7bd48
	ctx.lr = 0x827EF8D0;
	sub_82E7BD48(ctx, base);
	// 827EF8D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF8D4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827EF8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF8E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF8E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EF8F0 size=156
    let mut pc: u32 = 0x827EF8F0;
    'dispatch: loop {
        match pc {
            0x827EF8F0 => {
    //   block [0x827EF8F0..0x827EF98C)
	// 827EF8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF8F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EF8FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF900: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF904: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 827EF908: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 827EF90C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EF910: 3BEBACE0  addi r31, r11, -0x5320
	ctx.r[31].s64 = ctx.r[11].s64 + -21280;
	// 827EF914: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827EF918: 816AACF0  lwz r11, -0x5310(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21264 as u32) ) } as u64;
	// 827EF91C: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 827EF920: 4082002C  bne 0x827ef94c
	if !ctx.cr[0].eq {
	pc = 0x827EF94C; continue 'dispatch;
	}
	// 827EF924: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827EF928: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827EF92C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 827EF930: 916AACF0  stw r11, -0x5310(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-21264 as u32), ctx.r[11].u32 ) };
	// 827EF934: C1A99534  lfs f13, -0x6acc(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827EF938: C00808A4  lfs f0, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EF93C: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827EF940: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827EF944: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827EF948: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827EF94C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF950: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF954: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EF958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EF95C: 4E800421  bctrl
	ctx.lr = 0x827EF960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EF960: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827EF964: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827EF968: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF96C: 4868C3DD  bl 0x82e7bd48
	ctx.lr = 0x827EF970;
	sub_82E7BD48(ctx, base);
	// 827EF970: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827EF974: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827EF978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF97C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF980: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EF984: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827EF990 size=16
    let mut pc: u32 = 0x827EF990;
    'dispatch: loop {
        match pc {
            0x827EF990 => {
    //   block [0x827EF990..0x827EF9A0)
	// 827EF990: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 827EF994: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EF9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EF9A0 size=92
    let mut pc: u32 = 0x827EF9A0;
    'dispatch: loop {
        match pc {
            0x827EF9A0 => {
    //   block [0x827EF9A0..0x827EF9FC)
	// 827EF9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EF9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EF9A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EF9AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EF9B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EF9B4: 80840070  lwz r4, 0x70(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EF9B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF9BC: 4BD1FB0D  bl 0x8250f4c8
	ctx.lr = 0x827EF9C0;
	sub_8250F4C8(ctx, base);
	// 827EF9C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EF9C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EF9C8: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 827EF9CC: 409A0008  bne cr6, 0x827ef9d4
	if !ctx.cr[6].eq {
	pc = 0x827EF9D4; continue 'dispatch;
	}
	// 827EF9D0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827EF9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF9D8: 4BD18FD9  bl 0x825089b0
	ctx.lr = 0x827EF9DC;
	sub_825089B0(ctx, base);
	// 827EF9DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EF9E0: 486022B1  bl 0x82df1c90
	ctx.lr = 0x827EF9E4;
	sub_82DF1C90(ctx, base);
	// 827EF9E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EF9E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EF9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EF9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EF9F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EF9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFA00 size=92
    let mut pc: u32 = 0x827EFA00;
    'dispatch: loop {
        match pc {
            0x827EFA00 => {
    //   block [0x827EFA00..0x827EFA5C)
	// 827EFA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFA0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFA10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EFA14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EFA18: 808B0070  lwz r4, 0x70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFA1C: 4BD1FAFD  bl 0x8250f518
	ctx.lr = 0x827EFA20;
	sub_8250F518(ctx, base);
	// 827EFA20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EFA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EFA28: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 827EFA2C: 409A0008  bne cr6, 0x827efa34
	if !ctx.cr[6].eq {
	pc = 0x827EFA34; continue 'dispatch;
	}
	// 827EFA30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827EFA34: 4BD38655  bl 0x82528088
	ctx.lr = 0x827EFA38;
	sub_82528088(ctx, base);
	// 827EFA38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFA3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EFA40: 48602251  bl 0x82df1c90
	ctx.lr = 0x827EFA44;
	sub_82DF1C90(ctx, base);
	// 827EFA44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFA48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EFA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFA54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EFA60 size=172
    let mut pc: u32 = 0x827EFA60;
    'dispatch: loop {
        match pc {
            0x827EFA60 => {
    //   block [0x827EFA60..0x827EFB0C)
	// 827EFA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFA68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EFA6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFA70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFA74: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827EFA78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFA7C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827EFA80: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827EFA84: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 827EFA88: 394A5FB0  addi r10, r10, 0x5fb0
	ctx.r[10].s64 = ctx.r[10].s64 + 24496;
	// 827EFA8C: 909F0070  stw r4, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 827EFA90: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827EFA94: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFB10 size=68
    let mut pc: u32 = 0x827EFB10;
    'dispatch: loop {
        match pc {
            0x827EFB10 => {
    //   block [0x827EFB10..0x827EFB54)
	// 827EFB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFB18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFB1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFB20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFB24: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EFB28: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827EFB2C: 396B5FB0  addi r11, r11, 0x5fb0
	ctx.r[11].s64 = ctx.r[11].s64 + 24496;
	// 827EFB30: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827EFB34: 41820008  beq 0x827efb3c
	if ctx.cr[0].eq {
	pc = 0x827EFB3C; continue 'dispatch;
	}
	// 827EFB38: 4BAD0731  bl 0x822c0268
	ctx.lr = 0x827EFB3C;
	sub_822C0268(ctx, base);
	// 827EFB3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFB40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFB4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFB58 size=68
    let mut pc: u32 = 0x827EFB58;
    'dispatch: loop {
        match pc {
            0x827EFB58 => {
    //   block [0x827EFB58..0x827EFB9C)
	// 827EFB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFB60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFB64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFB68: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFB6C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 827EFB70: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827EFB74: 48668ECD  bl 0x82e58a40
	ctx.lr = 0x827EFB78;
	sub_82E58A40(ctx, base);
	// 827EFB78: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EFB7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFB80: 419A0008  beq cr6, 0x827efb88
	if ctx.cr[6].eq {
	pc = 0x827EFB88; continue 'dispatch;
	}
	// 827EFB84: 4BAD0D0D  bl 0x822c0890
	ctx.lr = 0x827EFB88;
	sub_822C0890(ctx, base);
	// 827EFB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFB94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFBA0 size=52
    let mut pc: u32 = 0x827EFBA0;
    'dispatch: loop {
        match pc {
            0x827EFBA0 => {
    //   block [0x827EFBA0..0x827EFBD4)
	// 827EFBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFBA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFBAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFBB0: 80840070  lwz r4, 0x70(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFBB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFBB8: 4BFFDAE9  bl 0x827ed6a0
	ctx.lr = 0x827EFBBC;
	sub_827ED6A0(ctx, base);
	// 827EFBBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFBCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFBD8 size=80
    let mut pc: u32 = 0x827EFBD8;
    'dispatch: loop {
        match pc {
            0x827EFBD8 => {
    //   block [0x827EFBD8..0x827EFC28)
	// 827EFBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFBE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFBE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFBE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827EFBEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827EFBF0: 808B0070  lwz r4, 0x70(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFBF4: 4BFFC9D5  bl 0x827ec5c8
	ctx.lr = 0x827EFBF8;
	sub_827EC5C8(ctx, base);
	// 827EFBF8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827EFBFC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EFC00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EFC04: 419A000C  beq cr6, 0x827efc10
	if ctx.cr[6].eq {
	pc = 0x827EFC10; continue 'dispatch;
	}
	// 827EFC08: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827EFC0C: 4BAD0C85  bl 0x822c0890
	ctx.lr = 0x827EFC10;
	sub_822C0890(ctx, base);
	// 827EFC10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFC14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827EFC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFC20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFC28 size=80
    let mut pc: u32 = 0x827EFC28;
    'dispatch: loop {
        match pc {
            0x827EFC28 => {
    //   block [0x827EFC28..0x827EFC78)
	// 827EFC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFC30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFC34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFC38: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFC3C: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 827EFC40: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827EFC44: 486673A5  bl 0x82e56fe8
	ctx.lr = 0x827EFC48;
	sub_82E56FE8(ctx, base);
	// 827EFC48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EFC4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFC50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EFC54: 419A000C  beq cr6, 0x827efc60
	if ctx.cr[6].eq {
	pc = 0x827EFC60; continue 'dispatch;
	}
	// 827EFC58: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827EFC5C: 4BAD0C35  bl 0x822c0890
	ctx.lr = 0x827EFC60;
	sub_822C0890(ctx, base);
	// 827EFC60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFC64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFC70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFC78 size=76
    let mut pc: u32 = 0x827EFC78;
    'dispatch: loop {
        match pc {
            0x827EFC78 => {
    //   block [0x827EFC78..0x827EFCC4)
	// 827EFC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFC80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFC84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFC88: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFC8C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827EFC90: 4BFFD0C9  bl 0x827ecd58
	ctx.lr = 0x827EFC94;
	sub_827ECD58(ctx, base);
	// 827EFC94: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EFC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFC9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827EFCA0: 419A000C  beq cr6, 0x827efcac
	if ctx.cr[6].eq {
	pc = 0x827EFCAC; continue 'dispatch;
	}
	// 827EFCA4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827EFCA8: 4BAD0BE9  bl 0x822c0890
	ctx.lr = 0x827EFCAC;
	sub_822C0890(ctx, base);
	// 827EFCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFCB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFCB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFCB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFCBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFCC8 size=64
    let mut pc: u32 = 0x827EFCC8;
    'dispatch: loop {
        match pc {
            0x827EFCC8 => {
    //   block [0x827EFCC8..0x827EFD08)
	// 827EFCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFCD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFCD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFCD8: 80840070  lwz r4, 0x70(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(112 as u32) ) } as u64;
	// 827EFCDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFCE0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EFCE4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 827EFCE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827EFCEC: 4E800421  bctrl
	ctx.lr = 0x827EFCF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827EFCF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFCF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFD00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EFD08 size=32
    let mut pc: u32 = 0x827EFD08;
    'dispatch: loop {
        match pc {
            0x827EFD08 => {
    //   block [0x827EFD08..0x827EFD28)
	// 827EFD08: 89630028  lbz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 827EFD0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EFD10: 4082000C  bne 0x827efd1c
	if !ctx.cr[0].eq {
	pc = 0x827EFD1C; continue 'dispatch;
	}
	// 827EFD14: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827EFD18: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 827EFD1C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EFD20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFD24: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EFD28 size=8
    let mut pc: u32 = 0x827EFD28;
    'dispatch: loop {
        match pc {
            0x827EFD28 => {
    //   block [0x827EFD28..0x827EFD30)
	// 827EFD28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827EFD2C: 4BCA22B4  b 0x82491fe0
	sub_82491FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EFD30 size=4
    let mut pc: u32 = 0x827EFD30;
    'dispatch: loop {
        match pc {
            0x827EFD30 => {
    //   block [0x827EFD30..0x827EFD34)
	// 827EFD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EFD38 size=32
    let mut pc: u32 = 0x827EFD38;
    'dispatch: loop {
        match pc {
            0x827EFD38 => {
    //   block [0x827EFD38..0x827EFD58)
	// 827EFD38: 89630028  lbz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 827EFD3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EFD40: 4182000C  beq 0x827efd4c
	if ctx.cr[0].eq {
	pc = 0x827EFD4C; continue 'dispatch;
	}
	// 827EFD44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827EFD48: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 827EFD4C: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 827EFD50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFD54: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EFD58 size=8
    let mut pc: u32 = 0x827EFD58;
    'dispatch: loop {
        match pc {
            0x827EFD58 => {
    //   block [0x827EFD58..0x827EFD60)
	// 827EFD58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827EFD5C: 4BCA2284  b 0x82491fe0
	sub_82491FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827EFD60 size=4
    let mut pc: u32 = 0x827EFD60;
    'dispatch: loop {
        match pc {
            0x827EFD60 => {
    //   block [0x827EFD60..0x827EFD64)
	// 827EFD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827EFD68 size=312
    let mut pc: u32 = 0x827EFD68;
    'dispatch: loop {
        match pc {
            0x827EFD68 => {
    //   block [0x827EFD68..0x827EFEA0)
	// 827EFD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFD70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827EFD74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFD78: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 827EFD7C: 489B8CF9  bl 0x831a8a74
	ctx.lr = 0x827EFD80;
	sub_831A8A40(ctx, base);
	// 827EFD80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFD84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827EFD88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827EFD8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFD90: C01E0018  lfs f0, 0x18(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EFD94: C3EBCEE4  lfs f31, -0x311c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827EFD98: C3DE000C  lfs f30, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827EFD9C: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 827EFDA0: C3BE0010  lfs f29, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827EFDA4: 489B9025  bl 0x831a8dc8
	ctx.lr = 0x827EFDA8;
	sub_831A8DC8(ctx, base);
	// 827EFDA8: C01E0014  lfs f0, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EFDAC: FF800818  frsp f28, f1
	ctx.f[28].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827EFDB0: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 827EFDB4: 489B9015  bl 0x831a8dc8
	ctx.lr = 0x827EFDB8;
	sub_831A8DC8(ctx, base);
	// 827EFDB8: C01E0014  lfs f0, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EFDBC: FF600818  frsp f27, f1
	ctx.f[27].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827EFDC0: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 827EFDC4: 489B90E5  bl 0x831a8ea8
	ctx.lr = 0x827EFDC8;
	sub_831A8EA8(ctx, base);
	// 827EFDC8: EDBB07B2  fmuls f13, f27, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[27].f64 * ctx.f[30].f64) as f32) as f64);
	// 827EFDCC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827EFDD0: FD800818  frsp f12, f1
	ctx.f[12].f64 = (ctx.f[1].f64 as f32) as f64;
	// 827EFDD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827EFDD8: ED7C07B2  fmuls f11, f28, f30
	ctx.f[11].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 827EFDDC: D17F0000  stfs f11, 0(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827EFDE0: ED5B0772  fmuls f10, f27, f29
	ctx.f[10].f64 = (((ctx.f[27].f64 * ctx.f[29].f64) as f32) as f64);
	// 827EFDE4: ED3C0772  fmuls f9, f28, f29
	ctx.f[9].f64 = (((ctx.f[28].f64 * ctx.f[29].f64) as f32) as f64);
	// 827EFDE8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827EFDEC: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827EFDF0: FD006850  fneg f8, f13
	ctx.f[8].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 827EFDF4: D11F0004  stfs f8, 4(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827EFDF8: ECEC07B2  fmuls f7, f12, f30
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[30].f64) as f32) as f64);
	// 827EFDFC: D0FF0008  stfs f7, 8(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827EFE00: FCC05850  fneg f6, f11
	ctx.f[6].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 827EFE04: D11F0014  stfs f8, 0x14(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 827EFE08: D0DF0010  stfs f6, 0x10(r31)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 827EFE0C: ED8C0772  fmuls f12, f12, f29
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[29].f64) as f32) as f64);
	// 827EFE10: D0FF0018  stfs f7, 0x18(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827EFE14: FD005050  fneg f8, f10
	ctx.f[8].u64 = ctx.f[10].u64 ^ 0x8000_0000_0000_0000u64;
	// 827EFE18: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 827EFE1C: FCA04850  fneg f5, f9
	ctx.f[5].u64 = ctx.f[9].u64 ^ 0x8000_0000_0000_0000u64;
	// 827EFE20: D17F0020  stfs f11, 0x20(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827EFE24: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827EFE28: D0FF0028  stfs f7, 0x28(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 827EFE2C: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 827EFE30: D0DF0030  stfs f6, 0x30(r31)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 827EFE34: D1BF0034  stfs f13, 0x34(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 827EFE38: D0FF0038  stfs f7, 0x38(r31)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 827EFE3C: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 827EFE40: D13F0040  stfs f9, 0x40(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 827EFE44: D11F0044  stfs f8, 0x44(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 827EFE48: D19F0048  stfs f12, 0x48(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 827EFE4C: D01F004C  stfs f0, 0x4c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 827EFE50: D0BF0050  stfs f5, 0x50(r31)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827EFE54: D11F0054  stfs f8, 0x54(r31)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827EFE58: D19F0058  stfs f12, 0x58(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827EFE5C: D01F005C  stfs f0, 0x5c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827EFE60: D13F0060  stfs f9, 0x60(r31)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827EFE64: D15F0064  stfs f10, 0x64(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827EFE68: D19F0068  stfs f12, 0x68(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827EFE6C: D01F006C  stfs f0, 0x6c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827EFE70: D0BF0070  stfs f5, 0x70(r31)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 827EFE74: D15F0074  stfs f10, 0x74(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 827EFE78: D19F0078  stfs f12, 0x78(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 827EFE7C: D01F007C  stfs f0, 0x7c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 827EFE80: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827EFE84: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 827EFE88: 489B8C39  bl 0x831a8ac0
	ctx.lr = 0x827EFE8C;
	sub_831A8A8C(ctx, base);
	// 827EFE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFE94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827EFE98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFEA0 size=104
    let mut pc: u32 = 0x827EFEA0;
    'dispatch: loop {
        match pc {
            0x827EFEA0 => {
    //   block [0x827EFEA0..0x827EFF08)
	// 827EFEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827EFEA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827EFEAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFEB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFEB4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 827EFEB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFEBC: 419A0008  beq cr6, 0x827efec4
	if ctx.cr[6].eq {
	pc = 0x827EFEC4; continue 'dispatch;
	}
	// 827EFEC0: 4BAD09D1  bl 0x822c0890
	ctx.lr = 0x827EFEC4;
	sub_822C0890(ctx, base);
	// 827EFEC4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827EFEC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFECC: 419A0008  beq cr6, 0x827efed4
	if ctx.cr[6].eq {
	pc = 0x827EFED4; continue 'dispatch;
	}
	// 827EFED0: 4BAD09C1  bl 0x822c0890
	ctx.lr = 0x827EFED4;
	sub_822C0890(ctx, base);
	// 827EFED4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827EFED8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFEDC: 419A0008  beq cr6, 0x827efee4
	if ctx.cr[6].eq {
	pc = 0x827EFEE4; continue 'dispatch;
	}
	// 827EFEE0: 4BAD09B1  bl 0x822c0890
	ctx.lr = 0x827EFEE4;
	sub_822C0890(ctx, base);
	// 827EFEE4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827EFEE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827EFEEC: 419A0008  beq cr6, 0x827efef4
	if ctx.cr[6].eq {
	pc = 0x827EFEF4; continue 'dispatch;
	}
	// 827EFEF0: 4BAD09A1  bl 0x822c0890
	ctx.lr = 0x827EFEF4;
	sub_822C0890(ctx, base);
	// 827EFEF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827EFEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827EFEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827EFF00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827EFF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827EFF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827EFF08 size=320
    let mut pc: u32 = 0x827EFF08;
    'dispatch: loop {
        match pc {
            0x827EFF08 => {
    //   block [0x827EFF08..0x827F0048)
	// 827EFF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827EFF0C: 489B8251  bl 0x831a815c
	ctx.lr = 0x827EFF10;
	sub_831A8130(ctx, base);
	// 827EFF10: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827EFF14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827EFF18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827EFF1C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827EFF20: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 827EFF24: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 827EFF28: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 827EFF2C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827EFF30: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 827EFF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827EFF38: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 827EFF3C: 388B5FCC  addi r4, r11, 0x5fcc
	ctx.r[4].s64 = ctx.r[11].s64 + 24524;
	// 827EFF40: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 827EFF44: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 827EFF48: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 827EFF4C: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 827EFF50: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 827EFF54: 3B7F0004  addi r27, r31, 4
	ctx.r[27].s64 = ctx.r[31].s64 + 4;
	// 827EFF58: 48602491  bl 0x82df23e8
	ctx.lr = 0x827EFF5C;
	sub_82DF23E8(ctx, base);
	// 827EFF5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827EFF60: 41820014  beq 0x827eff74
	if ctx.cr[0].eq {
	pc = 0x827EFF74; continue 'dispatch;
	}
	// 827EFF64: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827EFF68: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 827EFF6C: 48623185  bl 0x82e130f0
	ctx.lr = 0x827EFF70;
	sub_82E130F0(ctx, base);
	// 827EFF70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EFF74: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 827EFF78: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 827EFF7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EFF80: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 827EFF84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EFF88: 4BAF1091  bl 0x822e1018
	ctx.lr = 0x827EFF8C;
	sub_822E1018(ctx, base);
	// 827EFF8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827EFF90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827EFF94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827EFF98: 4BAD0069  bl 0x822c0000
	ctx.lr = 0x827EFF9C;
	sub_822C0000(ctx, base);
	// 827EFF9C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827EFFA0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827EFFA4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827EFFA8: 80AB6840  lwz r5, 0x6840(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26688 as u32) ) } as u64;
	// 827EFFAC: 808A6784  lwz r4, 0x6784(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26500 as u32) ) } as u64;
	// 827EFFB0: 4BB077F1  bl 0x822f77a0
	ctx.lr = 0x827EFFB4;
	sub_822F77A0(ctx, base);
	// 827EFFB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827EFFB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827EFFBC: 4BAF4F05  bl 0x822e4ec0
	ctx.lr = 0x827EFFC0;
	sub_822E4EC0(ctx, base);
	// 827EFFC0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827EFFC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827EFFC8: E89E0000  ld r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827EFFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827EFFD0: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827EFFD4: 4BC9C2B5  bl 0x8248c288
	ctx.lr = 0x827EFFD8;
	sub_8248C288(ctx, base);
	// 827EFFD8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827EFFDC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827EFFE0: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 827EFFE4: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 827EFFE8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827EFFEC: 997F0028  stb r11, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 827EFFF0: 419A000C  beq cr6, 0x827efffc
	if ctx.cr[6].eq {
	pc = 0x827EFFFC; continue 'dispatch;
	}
	// 827EFFF4: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 827EFFF8: 48622E39  bl 0x82e12e30
	ctx.lr = 0x827EFFFC;
	sub_82E12E30(ctx, base);
	// 827EFFFC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827F0000: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F0004: 4BFFFD65  bl 0x827efd68
	ctx.lr = 0x827F0008;
	sub_827EFD68(ctx, base);
	// 827F0008: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F000C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0010: 4BAF69C1  bl 0x822e69d0
	ctx.lr = 0x827F0014;
	sub_822E69D0(ctx, base);
	// 827F0014: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F0018: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827F001C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F0020: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0024: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F0028: 4BAD4439  bl 0x822c4460
	ctx.lr = 0x827F002C;
	sub_822C4460(ctx, base);
	// 827F002C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F0030: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F0034: 419A0008  beq cr6, 0x827f003c
	if ctx.cr[6].eq {
	pc = 0x827F003C; continue 'dispatch;
	}
	// 827F0038: 4BAD0859  bl 0x822c0890
	ctx.lr = 0x827F003C;
	sub_822C0890(ctx, base);
	// 827F003C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0040: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 827F0044: 489B8168  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F0048 size=436
    let mut pc: u32 = 0x827F0048;
    'dispatch: loop {
        match pc {
            0x827F0048 => {
    //   block [0x827F0048..0x827F01FC)
	// 827F0048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F004C: 489B810D  bl 0x831a8158
	ctx.lr = 0x827F0050;
	sub_831A8130(ctx, base);
	// 827F0050: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0054: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 827F0058: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F005C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F0060: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 827F0064: 3B9B0010  addi r28, r27, 0x10
	ctx.r[28].s64 = ctx.r[27].s64 + 16;
	// 827F0068: 815B0010  lwz r10, 0x10(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F006C: 3B4B5FCC  addi r26, r11, 0x5fcc
	ctx.r[26].s64 = ctx.r[11].s64 + 24524;
	// 827F0070: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 827F0074: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F0078: 409A0110  bne cr6, 0x827f0188
	if !ctx.cr[6].eq {
	pc = 0x827F0188; continue 'dispatch;
	}
	// 827F007C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F0080: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827F0084: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 827F0088: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F008C: 80CB67C0  lwz r6, 0x67c0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26560 as u32) ) } as u64;
	// 827F0090: 80AA6840  lwz r5, 0x6840(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26688 as u32) ) } as u64;
	// 827F0094: 80896784  lwz r4, 0x6784(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26500 as u32) ) } as u64;
	// 827F0098: 4BB07781  bl 0x822f7818
	ctx.lr = 0x827F009C;
	sub_822F7818(ctx, base);
	// 827F009C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F00A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F00A4: 4BAF4E1D  bl 0x822e4ec0
	ctx.lr = 0x827F00A8;
	sub_822E4EC0(ctx, base);
	// 827F00A8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F00AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F00B0: E89E0000  ld r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827F00B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F00B8: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827F00BC: 4BC9C1CD  bl 0x8248c288
	ctx.lr = 0x827F00C0;
	sub_8248C288(ctx, base);
	// 827F00C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F00C4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827F00C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F00CC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 827F00D0: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 827F00D4: 4BAD0305  bl 0x822c03d8
	ctx.lr = 0x827F00D8;
	sub_822C03D8(ctx, base);
	// 827F00D8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827F00DC: 41820054  beq 0x827f0130
	if ctx.cr[0].eq {
	pc = 0x827F0130; continue 'dispatch;
	}
	// 827F00E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F00E4: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F00E8: 4BAF6579  bl 0x822e6660
	ctx.lr = 0x827F00EC;
	sub_822E6660(ctx, base);
	// 827F00EC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F00F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F00F4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 827F00F8: 419A0008  beq cr6, 0x827f0100
	if ctx.cr[6].eq {
	pc = 0x827F0100; continue 'dispatch;
	}
	// 827F00FC: 4BAF814D  bl 0x822e8248
	ctx.lr = 0x827F0100;
	sub_822E8248(ctx, base);
	// 827F0100: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 827F0104: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F0108: 3BE00007  li r31, 7
	ctx.r[31].s64 = 7;
	// 827F010C: 3B010050  addi r24, r1, 0x50
	ctx.r[24].s64 = ctx.r[1].s64 + 80;
	// 827F0110: 4BD1F409  bl 0x8250f518
	ctx.lr = 0x827F0114;
	sub_8250F518(ctx, base);
	// 827F0114: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F0118: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F011C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 827F0120: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827F0124: 4BCA20CD  bl 0x824921f0
	ctx.lr = 0x827F0128;
	sub_824921F0(ctx, base);
	// 827F0128: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F012C: 48000008  b 0x827f0134
	pc = 0x827F0134; continue 'dispatch;
	// 827F0130: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F0134: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F0138: 4BAF4CB1  bl 0x822e4de8
	ctx.lr = 0x827F013C;
	sub_822E4DE8(ctx, base);
	// 827F013C: 57EB077B  rlwinm. r11, r31, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F0140: 41820010  beq 0x827f0150
	if ctx.cr[0].eq {
	pc = 0x827F0150; continue 'dispatch;
	}
	// 827F0144: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F0148: 57FF07B8  rlwinm r31, r31, 0, 0x1e, 0x1c
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 827F014C: 48601B45  bl 0x82df1c90
	ctx.lr = 0x827F0150;
	sub_82DF1C90(ctx, base);
	// 827F0150: 57EB07BD  rlwinm. r11, r31, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F0154: 41820018  beq 0x827f016c
	if ctx.cr[0].eq {
	pc = 0x827F016C; continue 'dispatch;
	}
	// 827F0158: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F015C: 57FF07FA  rlwinm r31, r31, 0, 0x1f, 0x1d
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 827F0160: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F0164: 419A0008  beq cr6, 0x827f016c
	if ctx.cr[6].eq {
	pc = 0x827F016C; continue 'dispatch;
	}
	// 827F0168: 4BAF8101  bl 0x822e8268
	ctx.lr = 0x827F016C;
	sub_822E8268(ctx, base);
	// 827F016C: 57EB07FF  clrlwi. r11, r31, 0x1f
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F0170: 41820018  beq 0x827f0188
	if ctx.cr[0].eq {
	pc = 0x827F0188; continue 'dispatch;
	}
	// 827F0174: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F0178: 57FF003C  rlwinm r31, r31, 0, 0, 0x1e
	ctx.r[31].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	// 827F017C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F0180: 419A0008  beq cr6, 0x827f0188
	if ctx.cr[6].eq {
	pc = 0x827F0188; continue 'dispatch;
	}
	// 827F0184: 4BAF80E5  bl 0x822e8268
	ctx.lr = 0x827F0188;
	sub_822E8268(ctx, base);
	// 827F0188: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F018C: 3BBB0008  addi r29, r27, 8
	ctx.r[29].s64 = ctx.r[27].s64 + 8;
	// 827F0190: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0194: 409A0060  bne cr6, 0x827f01f4
	if !ctx.cr[6].eq {
	pc = 0x827F01F4; continue 'dispatch;
	}
	// 827F0198: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827F019C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F01A0: 38A00084  li r5, 0x84
	ctx.r[5].s64 = 132;
	// 827F01A4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 827F01A8: 4BAD0231  bl 0x822c03d8
	ctx.lr = 0x827F01AC;
	sub_822C03D8(ctx, base);
	// 827F01AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 827F01B0: 41820028  beq 0x827f01d8
	if ctx.cr[0].eq {
	pc = 0x827F01D8; continue 'dispatch;
	}
	// 827F01B4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 827F01B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F01BC: 63FF0008  ori r31, r31, 8
	ctx.r[31].u64 = ctx.r[31].u64 | 8;
	// 827F01C0: 4BD1F359  bl 0x8250f518
	ctx.lr = 0x827F01C4;
	sub_8250F518(ctx, base);
	// 827F01C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F01C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F01CC: 4BCA0F25  bl 0x824910f0
	ctx.lr = 0x827F01D0;
	sub_824910F0(ctx, base);
	// 827F01D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F01D4: 48000008  b 0x827f01dc
	pc = 0x827F01DC; continue 'dispatch;
	// 827F01D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F01DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F01E0: 4BB1A501  bl 0x8230a6e0
	ctx.lr = 0x827F01E4;
	sub_8230A6E0(ctx, base);
	// 827F01E4: 57EB0739  rlwinm. r11, r31, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F01E8: 4182000C  beq 0x827f01f4
	if ctx.cr[0].eq {
	pc = 0x827F01F4; continue 'dispatch;
	}
	// 827F01EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F01F0: 48601AA1  bl 0x82df1c90
	ctx.lr = 0x827F01F4;
	sub_82DF1C90(ctx, base);
	// 827F01F4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 827F01F8: 489B7FB0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F0200 size=240
    let mut pc: u32 = 0x827F0200;
    'dispatch: loop {
        match pc {
            0x827F0200 => {
    //   block [0x827F0200..0x827F02F0)
	// 827F0200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F0204: 489B7F61  bl 0x831a8164
	ctx.lr = 0x827F0208;
	sub_831A8130(ctx, base);
	// 827F0208: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F020C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F0210: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F0214: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F0218: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F021C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827F0220: 4BAFFFB9  bl 0x822f01d8
	ctx.lr = 0x827F0224;
	sub_822F01D8(ctx, base);
	// 827F0224: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0228: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 827F022C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0230: 419A0020  beq cr6, 0x827f0250
	if ctx.cr[6].eq {
	pc = 0x827F0250; continue 'dispatch;
	}
	// 827F0234: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827F0238: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F023C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 827F0240: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F0244: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827F0248: 80EA66CC  lwz r7, 0x66cc(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26316 as u32) ) } as u64;
	// 827F024C: 4BAFB475  bl 0x822eb6c0
	ctx.lr = 0x827F0250;
	sub_822EB6C0(ctx, base);
	// 827F0250: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F0254: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0258: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F025C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F0260: 419A007C  beq cr6, 0x827f02dc
	if ctx.cr[6].eq {
	pc = 0x827F02DC; continue 'dispatch;
	}
	// 827F0264: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 827F0268: 83EB0024  lwz r31, 0x24(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 827F026C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F0270: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F0274: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F0278: 419A0028  beq cr6, 0x827f02a0
	if ctx.cr[6].eq {
	pc = 0x827F02A0; continue 'dispatch;
	}
	// 827F027C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0280: 4BFE9D51  bl 0x827d9fd0
	ctx.lr = 0x827F0284;
	sub_827D9FD0(ctx, base);
	// 827F0284: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F0288: 4182000C  beq 0x827f0294
	if ctx.cr[0].eq {
	pc = 0x827F0294; continue 'dispatch;
	}
	// 827F028C: 48818D2D  bl 0x83008fb8
	ctx.lr = 0x827F0290;
	sub_83008FB8(ctx, base);
	// 827F0290: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F0294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0298: 4BD20D49  bl 0x82510fe0
	ctx.lr = 0x827F029C;
	sub_82510FE0(ctx, base);
	// 827F029C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F02A0: 807D66D4  lwz r3, 0x66d4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(26324 as u32) ) } as u64;
	// 827F02A4: 4BC9B875  bl 0x8248bb18
	ctx.lr = 0x827F02A8;
	sub_8248BB18(ctx, base);
	// 827F02A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F02AC: 40820018  bne 0x827f02c4
	if !ctx.cr[0].eq {
	pc = 0x827F02C4; continue 'dispatch;
	}
	// 827F02B0: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 827F02B4: 419A0010  beq cr6, 0x827f02c4
	if ctx.cr[6].eq {
	pc = 0x827F02C4; continue 'dispatch;
	}
	// 827F02B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F02BC: 419A0008  beq cr6, 0x827f02c4
	if ctx.cr[6].eq {
	pc = 0x827F02C4; continue 'dispatch;
	}
	// 827F02C0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 827F02C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F02C8: 4BCF8F31  bl 0x824e91f8
	ctx.lr = 0x827F02CC;
	sub_824E91F8(ctx, base);
	// 827F02CC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F02D0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F02D4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F02D8: 409AFF90  bne cr6, 0x827f0268
	if !ctx.cr[6].eq {
	pc = 0x827F0268; continue 'dispatch;
	}
	// 827F02DC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F02E0: 4BB00739  bl 0x822f0a18
	ctx.lr = 0x827F02E4;
	sub_822F0A18(ctx, base);
	// 827F02E4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 827F02E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F02EC: 489B7EC8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F02F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F02F0 size=468
    let mut pc: u32 = 0x827F02F0;
    'dispatch: loop {
        match pc {
            0x827F02F0 => {
    //   block [0x827F02F0..0x827F04C4)
	// 827F02F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F02F4: 489B7E69  bl 0x831a815c
	ctx.lr = 0x827F02F8;
	sub_831A8130(ctx, base);
	// 827F02F8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 827F02FC: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0300: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F0304: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F0308: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 827F030C: 807C0018  lwz r3, 0x18(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F0310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0314: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F0318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F031C: 4E800421  bctrl
	ctx.lr = 0x827F0320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F0320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F0324: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0328: C01F003C  lfs f0, 0x3c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F032C: C1BF0038  lfs f13, 0x38(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F0330: C19F0034  lfs f12, 0x34(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F0334: C17F0030  lfs f11, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827F0338: D1610060  stfs f11, 0x60(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827F033C: D1810064  stfs f12, 0x64(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827F0340: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827F0344: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827F0348: 4BC76191  bl 0x824664d8
	ctx.lr = 0x827F034C;
	sub_824664D8(ctx, base);
	// 827F034C: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F0350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F0354: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 827F0358: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F035C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 827F0360: 419A0024  beq cr6, 0x827f0384
	if ctx.cr[6].eq {
	pc = 0x827F0384; continue 'dispatch;
	}
	// 827F0364: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F0368: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 827F036C: 4BCA1C25  bl 0x82491f90
	ctx.lr = 0x827F0370;
	sub_82491F90(ctx, base);
	// 827F0370: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0374: 80BC0024  lwz r5, 0x24(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 827F0378: 807C0010  lwz r3, 0x10(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F037C: 4BCA1BD5  bl 0x82491f50
	ctx.lr = 0x827F0380;
	sub_82491F50(ctx, base);
	// 827F0380: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F0384: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0388: 7F1B1840  cmplw cr6, r27, r3
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[3].u32, &mut ctx.xer);
	// 827F038C: 419A0104  beq cr6, 0x827f0490
	if ctx.cr[6].eq {
	pc = 0x827F0490; continue 'dispatch;
	}
	// 827F0390: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F0394: 3B4B6910  addi r26, r11, 0x6910
	ctx.r[26].s64 = ctx.r[11].s64 + 26896;
	// 827F0398: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 827F039C: 13E0D0C7  vcmpequd (lvx128) v31, v0, v26
	tmp.u32 = ctx.r[26].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F03A0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F03A4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F04C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F04C8 size=44
    let mut pc: u32 = 0x827F04C8;
    'dispatch: loop {
        match pc {
            0x827F04C8 => {
    //   block [0x827F04C8..0x827F04F4)
	// 827F04C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F04CC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F04D0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 827F04D4: 409A000C  bne cr6, 0x827f04e0
	if !ctx.cr[6].eq {
	pc = 0x827F04E0; continue 'dispatch;
	}
	// 827F04D8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F04DC: C02B5FC8  lfs f1, 0x5fc8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24520 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F04E0: 89630028  lbz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 827F04E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F04E8: 4182000C  beq 0x827f04f4
	if ctx.cr[0].eq {
		sub_827F04F4(ctx, base);
		return;
	}
	// 827F04EC: 80830020  lwz r4, 0x20(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F04F0: 4BFFFE00  b 0x827f02f0
	sub_827F02F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F04F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F04F4 size=8
    let mut pc: u32 = 0x827F04F4;
    'dispatch: loop {
        match pc {
            0x827F04F4 => {
    //   block [0x827F04F4..0x827F04FC)
	// 827F04F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F0500 size=72
    let mut pc: u32 = 0x827F0500;
    'dispatch: loop {
        match pc {
            0x827F0500 => {
    //   block [0x827F0500..0x827F0548)
	// 827F0500: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F0504: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F0508: 3D208207  lis r9, -0x7df9
	ctx.r[9].s64 = -2113470464;
	// 827F050C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 827F0510: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 827F0514: C18BA1C4  lfs f12, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F0518: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F051C: C1696000  lfs f11, 0x6000(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24576 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827F0520: C1A89524  lfs f13, -0x6adc(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F0524: C1479450  lfs f10, -0x6bb0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 827F0528: D1830000  stfs f12, 0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827F052C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F0530: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F0534: D163000C  stfs f11, 0xc(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827F0538: D1A30010  stfs f13, 0x10(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 827F053C: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 827F0540: D1430018  stfs f10, 0x18(r3)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827F0544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F0548 size=72
    let mut pc: u32 = 0x827F0548;
    'dispatch: loop {
        match pc {
            0x827F0548 => {
    //   block [0x827F0548..0x827F0590)
	// 827F0548: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827F054C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F0550: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 827F0554: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 827F0558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F055C: C18A9450  lfs f12, -0x6bb0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F0560: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F0564: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F0568: C1686218  lfs f11, 0x6218(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(25112 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 827F056C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F0570: C1A7093C  lfs f13, 0x93c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2364 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F0574: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F0578: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827F057C: D1630010  stfs f11, 0x10(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 827F0580: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 827F0584: D1A30018  stfs f13, 0x18(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827F0588: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 827F058C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F0590 size=60
    let mut pc: u32 = 0x827F0590;
    'dispatch: loop {
        match pc {
            0x827F0590 => {
    //   block [0x827F0590..0x827F05CC)
	// 827F0590: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F0594: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F0598: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827F059C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 827F05A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827F05A4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F05A8: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 827F05AC: C1AA093C  lfs f13, 0x93c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2364 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F05B0: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 827F05B4: C189D72C  lfs f12, -0x28d4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-10452 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F05B8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827F05BC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F05C0: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F05C4: D183000C  stfs f12, 0xc(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827F05C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F05D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F05D0 size=596
    let mut pc: u32 = 0x827F05D0;
    'dispatch: loop {
        match pc {
            0x827F05D0 => {
    //   block [0x827F05D0..0x827F0824)
	// 827F05D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F05D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F05D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F05DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F05E0: DBA1FFD0  stfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[29].u64 ) };
	// 827F05E4: DBC1FFD8  stfd f30, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 827F05E8: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827F05EC: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F05F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F05F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F05F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F05FC: 419A0204  beq cr6, 0x827f0800
	if ctx.cr[6].eq {
	pc = 0x827F0800; continue 'dispatch;
	}
	// 827F0600: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0604: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0608: 388B6054  addi r4, r11, 0x6054
	ctx.r[4].s64 = ctx.r[11].s64 + 24660;
	// 827F060C: 486033FD  bl 0x82df3a08
	ctx.lr = 0x827F0610;
	sub_82DF3A08(ctx, base);
	// 827F0610: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F0614: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827F0618: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F061C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F0620: 38610160  addi r3, r1, 0x160
	ctx.r[3].s64 = ctx.r[1].s64 + 352;
	// 827F0624: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F0628: C3AA6218  lfs f29, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827F062C: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0630: C3C908A4  lfs f30, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827F0634: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F0638: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F063C: 4BDB2B75  bl 0x825a31b0
	ctx.lr = 0x827F0640;
	sub_825A31B0(ctx, base);
	// 827F0640: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0644: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F064C: 4BDB1125  bl 0x825a1770
	ctx.lr = 0x827F0650;
	sub_825A1770(ctx, base);
	// 827F0650: 38610198  addi r3, r1, 0x198
	ctx.r[3].s64 = ctx.r[1].s64 + 408;
	// 827F0654: 48602DD5  bl 0x82df3428
	ctx.lr = 0x827F0658;
	sub_82DF3428(ctx, base);
	// 827F0658: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 827F065C: 4BAD865D  bl 0x822c8cb8
	ctx.lr = 0x827F0660;
	sub_822C8CB8(ctx, base);
	// 827F0660: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0664: 48602DC5  bl 0x82df3428
	ctx.lr = 0x827F0668;
	sub_82DF3428(ctx, base);
	// 827F0668: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F066C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0670: 388B6044  addi r4, r11, 0x6044
	ctx.r[4].s64 = ctx.r[11].s64 + 24644;
	// 827F0674: 48603395  bl 0x82df3a08
	ctx.lr = 0x827F0678;
	sub_82DF3A08(ctx, base);
	// 827F0678: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 827F067C: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827F0680: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0684: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0688: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F068C: 4BDB2B25  bl 0x825a31b0
	ctx.lr = 0x827F0690;
	sub_825A31B0(ctx, base);
	// 827F0690: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0694: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F069C: 4BDB10D5  bl 0x825a1770
	ctx.lr = 0x827F06A0;
	sub_825A1770(ctx, base);
	// 827F06A0: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 827F06A4: 48602D85  bl 0x82df3428
	ctx.lr = 0x827F06A8;
	sub_82DF3428(ctx, base);
	// 827F06A8: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 827F06AC: 4BAD860D  bl 0x822c8cb8
	ctx.lr = 0x827F06B0;
	sub_822C8CB8(ctx, base);
	// 827F06B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F06B4: 48602D75  bl 0x82df3428
	ctx.lr = 0x827F06B8;
	sub_82DF3428(ctx, base);
	// 827F06B8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F06BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F06C0: 388B6034  addi r4, r11, 0x6034
	ctx.r[4].s64 = ctx.r[11].s64 + 24628;
	// 827F06C4: 48603345  bl 0x82df3a08
	ctx.lr = 0x827F06C8;
	sub_82DF3A08(ctx, base);
	// 827F06C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F06CC: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 827F06D0: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F06D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F06D8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F06DC: C3AB95A0  lfs f29, -0x6a60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27232 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827F06E0: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F06E4: 4BDB2ACD  bl 0x825a31b0
	ctx.lr = 0x827F06E8;
	sub_825A31B0(ctx, base);
	// 827F06E8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F06EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F06F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F06F4: 4BDB107D  bl 0x825a1770
	ctx.lr = 0x827F06F8;
	sub_825A1770(ctx, base);
	// 827F06F8: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 827F06FC: 48602D2D  bl 0x82df3428
	ctx.lr = 0x827F0700;
	sub_82DF3428(ctx, base);
	// 827F0700: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827F0704: 4BAD85B5  bl 0x822c8cb8
	ctx.lr = 0x827F0708;
	sub_822C8CB8(ctx, base);
	// 827F0708: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F070C: 48602D1D  bl 0x82df3428
	ctx.lr = 0x827F0710;
	sub_82DF3428(ctx, base);
	// 827F0710: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0718: 388B6020  addi r4, r11, 0x6020
	ctx.r[4].s64 = ctx.r[11].s64 + 24608;
	// 827F071C: 486032ED  bl 0x82df3a08
	ctx.lr = 0x827F0720;
	sub_82DF3A08(ctx, base);
	// 827F0720: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 827F0724: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827F0728: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F072C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F0730: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0734: 4BDB2A7D  bl 0x825a31b0
	ctx.lr = 0x827F0738;
	sub_825A31B0(ctx, base);
	// 827F0738: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F073C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0744: 4BDB102D  bl 0x825a1770
	ctx.lr = 0x827F0748;
	sub_825A1770(ctx, base);
	// 827F0748: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 827F074C: 48602CDD  bl 0x82df3428
	ctx.lr = 0x827F0750;
	sub_82DF3428(ctx, base);
	// 827F0750: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827F0754: 4BAD8565  bl 0x822c8cb8
	ctx.lr = 0x827F0758;
	sub_822C8CB8(ctx, base);
	// 827F0758: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F075C: 48602CCD  bl 0x82df3428
	ctx.lr = 0x827F0760;
	sub_82DF3428(ctx, base);
	// 827F0760: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0764: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0768: 388B6010  addi r4, r11, 0x6010
	ctx.r[4].s64 = ctx.r[11].s64 + 24592;
	// 827F076C: 4860329D  bl 0x82df3a08
	ctx.lr = 0x827F0770;
	sub_82DF3A08(ctx, base);
	// 827F0770: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F0774: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 827F0778: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F077C: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 827F0780: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827F0784: 4BDB2C25  bl 0x825a33a8
	ctx.lr = 0x827F0788;
	sub_825A33A8(ctx, base);
	// 827F0788: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F078C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0794: 4BDB1745  bl 0x825a1ed8
	ctx.lr = 0x827F0798;
	sub_825A1ED8(ctx, base);
	// 827F0798: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 827F079C: 48602C8D  bl 0x82df3428
	ctx.lr = 0x827F07A0;
	sub_82DF3428(ctx, base);
	// 827F07A0: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 827F07A4: 4BAD8515  bl 0x822c8cb8
	ctx.lr = 0x827F07A8;
	sub_822C8CB8(ctx, base);
	// 827F07A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F07AC: 48602C7D  bl 0x82df3428
	ctx.lr = 0x827F07B0;
	sub_82DF3428(ctx, base);
	// 827F07B0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F07B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F07B8: 388B6004  addi r4, r11, 0x6004
	ctx.r[4].s64 = ctx.r[11].s64 + 24580;
	// 827F07BC: 4860324D  bl 0x82df3a08
	ctx.lr = 0x827F07C0;
	sub_82DF3A08(ctx, base);
	// 827F07C0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F07C4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 827F07C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F07CC: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 827F07D0: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 827F07D4: 4BDB2BD5  bl 0x825a33a8
	ctx.lr = 0x827F07D8;
	sub_825A33A8(ctx, base);
	// 827F07D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F07DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F07E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F07E4: 4BDB16F5  bl 0x825a1ed8
	ctx.lr = 0x827F07E8;
	sub_825A1ED8(ctx, base);
	// 827F07E8: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 827F07EC: 48602C3D  bl 0x82df3428
	ctx.lr = 0x827F07F0;
	sub_82DF3428(ctx, base);
	// 827F07F0: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 827F07F4: 4BAD84C5  bl 0x822c8cb8
	ctx.lr = 0x827F07F8;
	sub_822C8CB8(ctx, base);
	// 827F07F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F07FC: 48602C2D  bl 0x82df3428
	ctx.lr = 0x827F0800;
	sub_82DF3428(ctx, base);
	// 827F0800: 38210210  addi r1, r1, 0x210
	ctx.r[1].s64 = ctx.r[1].s64 + 528;
	// 827F0804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F0808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F080C: CBA1FFD0  lfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827F0810: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827F0814: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827F0818: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F081C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F0820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F0828 size=672
    let mut pc: u32 = 0x827F0828;
    'dispatch: loop {
        match pc {
            0x827F0828 => {
    //   block [0x827F0828..0x827F0AC8)
	// 827F0828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F082C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F0830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F0834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F0838: DBA1FFD0  stfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[29].u64 ) };
	// 827F083C: DBC1FFD8  stfd f30, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 827F0840: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827F0844: 9421FDB0  stwu r1, -0x250(r1)
	ea = ctx.r[1].u32.wrapping_add(-592 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F084C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F0850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F0854: 419A0250  beq cr6, 0x827f0aa4
	if ctx.cr[6].eq {
	pc = 0x827F0AA4; continue 'dispatch;
	}
	// 827F0858: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F085C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0860: 388BEB64  addi r4, r11, -0x149c
	ctx.r[4].s64 = ctx.r[11].s64 + -5276;
	// 827F0864: 486031A5  bl 0x82df3a08
	ctx.lr = 0x827F0868;
	sub_82DF3A08(ctx, base);
	// 827F0868: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F086C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827F0870: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F0874: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F0878: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827F087C: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F0880: C3AA6218  lfs f29, 0x6218(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827F0884: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0888: C3C908A4  lfs f30, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827F088C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F0890: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0894: 4BDB291D  bl 0x825a31b0
	ctx.lr = 0x827F0898;
	sub_825A31B0(ctx, base);
	// 827F0898: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F089C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F08A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F08A4: 4BDB0ECD  bl 0x825a1770
	ctx.lr = 0x827F08A8;
	sub_825A1770(ctx, base);
	// 827F08A8: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 827F08AC: 48602B7D  bl 0x82df3428
	ctx.lr = 0x827F08B0;
	sub_82DF3428(ctx, base);
	// 827F08B0: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 827F08B4: 4BAD8405  bl 0x822c8cb8
	ctx.lr = 0x827F08B8;
	sub_822C8CB8(ctx, base);
	// 827F08B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F08BC: 48602B6D  bl 0x82df3428
	ctx.lr = 0x827F08C0;
	sub_82DF3428(ctx, base);
	// 827F08C0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F08C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F08C8: 388B60A0  addi r4, r11, 0x60a0
	ctx.r[4].s64 = ctx.r[11].s64 + 24736;
	// 827F08CC: 4860313D  bl 0x82df3a08
	ctx.lr = 0x827F08D0;
	sub_82DF3A08(ctx, base);
	// 827F08D0: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 827F08D4: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 827F08D8: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F08DC: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F08E0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F08E4: 4BDB28CD  bl 0x825a31b0
	ctx.lr = 0x827F08E8;
	sub_825A31B0(ctx, base);
	// 827F08E8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F08EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F08F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F08F4: 4BDB0E7D  bl 0x825a1770
	ctx.lr = 0x827F08F8;
	sub_825A1770(ctx, base);
	// 827F08F8: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 827F08FC: 48602B2D  bl 0x82df3428
	ctx.lr = 0x827F0900;
	sub_82DF3428(ctx, base);
	// 827F0900: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 827F0904: 4BAD83B5  bl 0x822c8cb8
	ctx.lr = 0x827F0908;
	sub_822C8CB8(ctx, base);
	// 827F0908: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F090C: 48602B1D  bl 0x82df3428
	ctx.lr = 0x827F0910;
	sub_82DF3428(ctx, base);
	// 827F0910: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0914: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0918: 388B6098  addi r4, r11, 0x6098
	ctx.r[4].s64 = ctx.r[11].s64 + 24728;
	// 827F091C: 486030ED  bl 0x82df3a08
	ctx.lr = 0x827F0920;
	sub_82DF3A08(ctx, base);
	// 827F0920: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 827F0924: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F0928: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F092C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F0930: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0934: 4BDB287D  bl 0x825a31b0
	ctx.lr = 0x827F0938;
	sub_825A31B0(ctx, base);
	// 827F0938: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F093C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0944: 4BDB0E2D  bl 0x825a1770
	ctx.lr = 0x827F0948;
	sub_825A1770(ctx, base);
	// 827F0948: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 827F094C: 48602ADD  bl 0x82df3428
	ctx.lr = 0x827F0950;
	sub_82DF3428(ctx, base);
	// 827F0950: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827F0954: 4BAD8365  bl 0x822c8cb8
	ctx.lr = 0x827F0958;
	sub_822C8CB8(ctx, base);
	// 827F0958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F095C: 48602ACD  bl 0x82df3428
	ctx.lr = 0x827F0960;
	sub_82DF3428(ctx, base);
	// 827F0960: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0968: 388B6088  addi r4, r11, 0x6088
	ctx.r[4].s64 = ctx.r[11].s64 + 24712;
	// 827F096C: 4860309D  bl 0x82df3a08
	ctx.lr = 0x827F0970;
	sub_82DF3A08(ctx, base);
	// 827F0970: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 827F0974: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827F0978: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F097C: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F0980: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0984: 4BDB282D  bl 0x825a31b0
	ctx.lr = 0x827F0988;
	sub_825A31B0(ctx, base);
	// 827F0988: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F098C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0994: 4BDB0DDD  bl 0x825a1770
	ctx.lr = 0x827F0998;
	sub_825A1770(ctx, base);
	// 827F0998: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 827F099C: 48602A8D  bl 0x82df3428
	ctx.lr = 0x827F09A0;
	sub_82DF3428(ctx, base);
	// 827F09A0: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827F09A4: 4BAD8315  bl 0x822c8cb8
	ctx.lr = 0x827F09A8;
	sub_822C8CB8(ctx, base);
	// 827F09A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F09AC: 48602A7D  bl 0x82df3428
	ctx.lr = 0x827F09B0;
	sub_82DF3428(ctx, base);
	// 827F09B0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F09B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F09B8: 388B607C  addi r4, r11, 0x607c
	ctx.r[4].s64 = ctx.r[11].s64 + 24700;
	// 827F09BC: 4860304D  bl 0x82df3a08
	ctx.lr = 0x827F09C0;
	sub_82DF3A08(ctx, base);
	// 827F09C0: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 827F09C4: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827F09C8: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F09CC: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F09D0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F09D4: 4BDB27DD  bl 0x825a31b0
	ctx.lr = 0x827F09D8;
	sub_825A31B0(ctx, base);
	// 827F09D8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F09DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F09E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F09E4: 4BDB0D8D  bl 0x825a1770
	ctx.lr = 0x827F09E8;
	sub_825A1770(ctx, base);
	// 827F09E8: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 827F09EC: 48602A3D  bl 0x82df3428
	ctx.lr = 0x827F09F0;
	sub_82DF3428(ctx, base);
	// 827F09F0: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 827F09F4: 4BAD82C5  bl 0x822c8cb8
	ctx.lr = 0x827F09F8;
	sub_822C8CB8(ctx, base);
	// 827F09F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F09FC: 48602A2D  bl 0x82df3428
	ctx.lr = 0x827F0A00;
	sub_82DF3428(ctx, base);
	// 827F0A00: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0A04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0A08: 388B6070  addi r4, r11, 0x6070
	ctx.r[4].s64 = ctx.r[11].s64 + 24688;
	// 827F0A0C: 48602FFD  bl 0x82df3a08
	ctx.lr = 0x827F0A10;
	sub_82DF3A08(ctx, base);
	// 827F0A10: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 827F0A14: 38610160  addi r3, r1, 0x160
	ctx.r[3].s64 = ctx.r[1].s64 + 352;
	// 827F0A18: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0A1C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0A20: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	// 827F0A24: 4BDB278D  bl 0x825a31b0
	ctx.lr = 0x827F0A28;
	sub_825A31B0(ctx, base);
	// 827F0A28: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0A2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0A34: 4BDB0D3D  bl 0x825a1770
	ctx.lr = 0x827F0A38;
	sub_825A1770(ctx, base);
	// 827F0A38: 38610198  addi r3, r1, 0x198
	ctx.r[3].s64 = ctx.r[1].s64 + 408;
	// 827F0A3C: 486029ED  bl 0x82df3428
	ctx.lr = 0x827F0A40;
	sub_82DF3428(ctx, base);
	// 827F0A40: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 827F0A44: 4BAD8275  bl 0x822c8cb8
	ctx.lr = 0x827F0A48;
	sub_822C8CB8(ctx, base);
	// 827F0A48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0A4C: 486029DD  bl 0x82df3428
	ctx.lr = 0x827F0A50;
	sub_82DF3428(ctx, base);
	// 827F0A50: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0A54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0A58: 388B6064  addi r4, r11, 0x6064
	ctx.r[4].s64 = ctx.r[11].s64 + 24676;
	// 827F0A5C: 48602FAD  bl 0x82df3a08
	ctx.lr = 0x827F0A60;
	sub_82DF3A08(ctx, base);
	// 827F0A60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F0A64: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 827F0A68: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0A6C: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 827F0A70: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 827F0A74: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 827F0A78: 4BDB2739  bl 0x825a31b0
	ctx.lr = 0x827F0A7C;
	sub_825A31B0(ctx, base);
	// 827F0A7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0A80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0A88: 4BDB0CE9  bl 0x825a1770
	ctx.lr = 0x827F0A8C;
	sub_825A1770(ctx, base);
	// 827F0A8C: 38610218  addi r3, r1, 0x218
	ctx.r[3].s64 = ctx.r[1].s64 + 536;
	// 827F0A90: 48602999  bl 0x82df3428
	ctx.lr = 0x827F0A94;
	sub_82DF3428(ctx, base);
	// 827F0A94: 386101F8  addi r3, r1, 0x1f8
	ctx.r[3].s64 = ctx.r[1].s64 + 504;
	// 827F0A98: 4BAD8221  bl 0x822c8cb8
	ctx.lr = 0x827F0A9C;
	sub_822C8CB8(ctx, base);
	// 827F0A9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0AA0: 48602989  bl 0x82df3428
	ctx.lr = 0x827F0AA4;
	sub_82DF3428(ctx, base);
	// 827F0AA4: 38210250  addi r1, r1, 0x250
	ctx.r[1].s64 = ctx.r[1].s64 + 592;
	// 827F0AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F0AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F0AB0: CBA1FFD0  lfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827F0AB4: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827F0AB8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827F0ABC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F0AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F0AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F0AC8 size=748
    let mut pc: u32 = 0x827F0AC8;
    'dispatch: loop {
        match pc {
            0x827F0AC8 => {
    //   block [0x827F0AC8..0x827F0DB4)
	// 827F0AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F0ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F0AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F0AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F0AD8: DBA1FFD0  stfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[29].u64 ) };
	// 827F0ADC: DBC1FFD8  stfd f30, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 827F0AE0: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827F0AE4: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0AE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F0AEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F0AF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F0AF4: 419A029C  beq cr6, 0x827f0d90
	if ctx.cr[6].eq {
	pc = 0x827F0D90; continue 'dispatch;
	}
	// 827F0AF8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0AFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0B00: 388B6110  addi r4, r11, 0x6110
	ctx.r[4].s64 = ctx.r[11].s64 + 24848;
	// 827F0B04: 48602F05  bl 0x82df3a08
	ctx.lr = 0x827F0B08;
	sub_82DF3A08(ctx, base);
	// 827F0B08: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F0B0C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 827F0B10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F0B14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F0B18: 38610160  addi r3, r1, 0x160
	ctx.r[3].s64 = ctx.r[1].s64 + 352;
	// 827F0B1C: 4BDB288D  bl 0x825a33a8
	ctx.lr = 0x827F0B20;
	sub_825A33A8(ctx, base);
	// 827F0B20: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0B24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0B2C: 4BDB13AD  bl 0x825a1ed8
	ctx.lr = 0x827F0B30;
	sub_825A1ED8(ctx, base);
	// 827F0B30: 38610198  addi r3, r1, 0x198
	ctx.r[3].s64 = ctx.r[1].s64 + 408;
	// 827F0B34: 486028F5  bl 0x82df3428
	ctx.lr = 0x827F0B38;
	sub_82DF3428(ctx, base);
	// 827F0B38: 38610178  addi r3, r1, 0x178
	ctx.r[3].s64 = ctx.r[1].s64 + 376;
	// 827F0B3C: 4BAD817D  bl 0x822c8cb8
	ctx.lr = 0x827F0B40;
	sub_822C8CB8(ctx, base);
	// 827F0B40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0B44: 486028E5  bl 0x82df3428
	ctx.lr = 0x827F0B48;
	sub_82DF3428(ctx, base);
	// 827F0B48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F0B4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0B50: 388BFDE4  addi r4, r11, -0x21c
	ctx.r[4].s64 = ctx.r[11].s64 + -540;
	// 827F0B54: 48602EB5  bl 0x82df3a08
	ctx.lr = 0x827F0B58;
	sub_82DF3A08(ctx, base);
	// 827F0B58: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F0B5C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 827F0B60: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F0B64: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 827F0B68: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827F0B6C: 4BDB283D  bl 0x825a33a8
	ctx.lr = 0x827F0B70;
	sub_825A33A8(ctx, base);
	// 827F0B70: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0B74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0B7C: 4BDB135D  bl 0x825a1ed8
	ctx.lr = 0x827F0B80;
	sub_825A1ED8(ctx, base);
	// 827F0B80: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 827F0B84: 486028A5  bl 0x82df3428
	ctx.lr = 0x827F0B88;
	sub_82DF3428(ctx, base);
	// 827F0B88: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 827F0B8C: 4BAD812D  bl 0x822c8cb8
	ctx.lr = 0x827F0B90;
	sub_822C8CB8(ctx, base);
	// 827F0B90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0B94: 48602895  bl 0x82df3428
	ctx.lr = 0x827F0B98;
	sub_82DF3428(ctx, base);
	// 827F0B98: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0B9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0BA0: 388B6104  addi r4, r11, 0x6104
	ctx.r[4].s64 = ctx.r[11].s64 + 24836;
	// 827F0BA4: 48602E65  bl 0x82df3a08
	ctx.lr = 0x827F0BA8;
	sub_82DF3A08(ctx, base);
	// 827F0BA8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F0BAC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 827F0BB0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F0BB4: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 827F0BB8: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 827F0BBC: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F0BC0: C3CA9A8C  lfs f30, -0x6574(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 827F0BC4: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0BC8: C3A908A4  lfs f29, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 827F0BCC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F0BD0: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F0BD4: 4BDB25DD  bl 0x825a31b0
	ctx.lr = 0x827F0BD8;
	sub_825A31B0(ctx, base);
	// 827F0BD8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0BDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0BE4: 4BDB0B8D  bl 0x825a1770
	ctx.lr = 0x827F0BE8;
	sub_825A1770(ctx, base);
	// 827F0BE8: 38610218  addi r3, r1, 0x218
	ctx.r[3].s64 = ctx.r[1].s64 + 536;
	// 827F0BEC: 4860283D  bl 0x82df3428
	ctx.lr = 0x827F0BF0;
	sub_82DF3428(ctx, base);
	// 827F0BF0: 386101F8  addi r3, r1, 0x1f8
	ctx.r[3].s64 = ctx.r[1].s64 + 504;
	// 827F0BF4: 4BAD80C5  bl 0x822c8cb8
	ctx.lr = 0x827F0BF8;
	sub_822C8CB8(ctx, base);
	// 827F0BF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0BFC: 4860282D  bl 0x82df3428
	ctx.lr = 0x827F0C00;
	sub_82DF3428(ctx, base);
	// 827F0C00: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0C04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0C08: 388B60F4  addi r4, r11, 0x60f4
	ctx.r[4].s64 = ctx.r[11].s64 + 24820;
	// 827F0C0C: 48602DFD  bl 0x82df3a08
	ctx.lr = 0x827F0C10;
	sub_82DF3A08(ctx, base);
	// 827F0C10: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 827F0C14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F0C18: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0C1C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F0C20: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F0C24: 4BDB258D  bl 0x825a31b0
	ctx.lr = 0x827F0C28;
	sub_825A31B0(ctx, base);
	// 827F0C28: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0C2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0C34: 4BDB0B3D  bl 0x825a1770
	ctx.lr = 0x827F0C38;
	sub_825A1770(ctx, base);
	// 827F0C38: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 827F0C3C: 486027ED  bl 0x82df3428
	ctx.lr = 0x827F0C40;
	sub_82DF3428(ctx, base);
	// 827F0C40: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827F0C44: 4BAD8075  bl 0x822c8cb8
	ctx.lr = 0x827F0C48;
	sub_822C8CB8(ctx, base);
	// 827F0C48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0C4C: 486027DD  bl 0x82df3428
	ctx.lr = 0x827F0C50;
	sub_82DF3428(ctx, base);
	// 827F0C50: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0C54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0C58: 388B60E4  addi r4, r11, 0x60e4
	ctx.r[4].s64 = ctx.r[11].s64 + 24804;
	// 827F0C5C: 48602DAD  bl 0x82df3a08
	ctx.lr = 0x827F0C60;
	sub_82DF3A08(ctx, base);
	// 827F0C60: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 827F0C64: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 827F0C68: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0C6C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F0C70: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F0C74: 4BDB253D  bl 0x825a31b0
	ctx.lr = 0x827F0C78;
	sub_825A31B0(ctx, base);
	// 827F0C78: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0C7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0C80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0C84: 4BDB0AED  bl 0x825a1770
	ctx.lr = 0x827F0C88;
	sub_825A1770(ctx, base);
	// 827F0C88: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 827F0C8C: 4860279D  bl 0x82df3428
	ctx.lr = 0x827F0C90;
	sub_82DF3428(ctx, base);
	// 827F0C90: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 827F0C94: 4BAD8025  bl 0x822c8cb8
	ctx.lr = 0x827F0C98;
	sub_822C8CB8(ctx, base);
	// 827F0C98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0C9C: 4860278D  bl 0x82df3428
	ctx.lr = 0x827F0CA0;
	sub_82DF3428(ctx, base);
	// 827F0CA0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0CA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0CA8: 388B60D0  addi r4, r11, 0x60d0
	ctx.r[4].s64 = ctx.r[11].s64 + 24784;
	// 827F0CAC: 48602D5D  bl 0x82df3a08
	ctx.lr = 0x827F0CB0;
	sub_82DF3A08(ctx, base);
	// 827F0CB0: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 827F0CB4: 38610120  addi r3, r1, 0x120
	ctx.r[3].s64 = ctx.r[1].s64 + 288;
	// 827F0CB8: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0CBC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F0CC0: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F0CC4: 4BDB24ED  bl 0x825a31b0
	ctx.lr = 0x827F0CC8;
	sub_825A31B0(ctx, base);
	// 827F0CC8: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0CCC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0CD4: 4BDB0A9D  bl 0x825a1770
	ctx.lr = 0x827F0CD8;
	sub_825A1770(ctx, base);
	// 827F0CD8: 38610158  addi r3, r1, 0x158
	ctx.r[3].s64 = ctx.r[1].s64 + 344;
	// 827F0CDC: 4860274D  bl 0x82df3428
	ctx.lr = 0x827F0CE0;
	sub_82DF3428(ctx, base);
	// 827F0CE0: 38610138  addi r3, r1, 0x138
	ctx.r[3].s64 = ctx.r[1].s64 + 312;
	// 827F0CE4: 4BAD7FD5  bl 0x822c8cb8
	ctx.lr = 0x827F0CE8;
	sub_822C8CB8(ctx, base);
	// 827F0CE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0CEC: 4860273D  bl 0x82df3428
	ctx.lr = 0x827F0CF0;
	sub_82DF3428(ctx, base);
	// 827F0CF0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0CF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0CF8: 388B60BC  addi r4, r11, 0x60bc
	ctx.r[4].s64 = ctx.r[11].s64 + 24764;
	// 827F0CFC: 48602D0D  bl 0x82df3a08
	ctx.lr = 0x827F0D00;
	sub_82DF3A08(ctx, base);
	// 827F0D00: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 827F0D04: 386101A0  addi r3, r1, 0x1a0
	ctx.r[3].s64 = ctx.r[1].s64 + 416;
	// 827F0D08: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0D0C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F0D10: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F0D14: 4BDB249D  bl 0x825a31b0
	ctx.lr = 0x827F0D18;
	sub_825A31B0(ctx, base);
	// 827F0D18: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0D1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0D24: 4BDB0A4D  bl 0x825a1770
	ctx.lr = 0x827F0D28;
	sub_825A1770(ctx, base);
	// 827F0D28: 386101D8  addi r3, r1, 0x1d8
	ctx.r[3].s64 = ctx.r[1].s64 + 472;
	// 827F0D2C: 486026FD  bl 0x82df3428
	ctx.lr = 0x827F0D30;
	sub_82DF3428(ctx, base);
	// 827F0D30: 386101B8  addi r3, r1, 0x1b8
	ctx.r[3].s64 = ctx.r[1].s64 + 440;
	// 827F0D34: 4BAD7F85  bl 0x822c8cb8
	ctx.lr = 0x827F0D38;
	sub_822C8CB8(ctx, base);
	// 827F0D38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0D3C: 486026ED  bl 0x82df3428
	ctx.lr = 0x827F0D40;
	sub_82DF3428(ctx, base);
	// 827F0D40: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F0D44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0D48: 388B60A8  addi r4, r11, 0x60a8
	ctx.r[4].s64 = ctx.r[11].s64 + 24744;
	// 827F0D4C: 48602CBD  bl 0x82df3a08
	ctx.lr = 0x827F0D50;
	sub_82DF3A08(ctx, base);
	// 827F0D50: 389E001C  addi r4, r30, 0x1c
	ctx.r[4].s64 = ctx.r[30].s64 + 28;
	// 827F0D54: 38610220  addi r3, r1, 0x220
	ctx.r[3].s64 = ctx.r[1].s64 + 544;
	// 827F0D58: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 827F0D5C: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 827F0D60: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 827F0D64: 4BDB244D  bl 0x825a31b0
	ctx.lr = 0x827F0D68;
	sub_825A31B0(ctx, base);
	// 827F0D68: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 827F0D6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F0D74: 4BDB09FD  bl 0x825a1770
	ctx.lr = 0x827F0D78;
	sub_825A1770(ctx, base);
	// 827F0D78: 38610258  addi r3, r1, 0x258
	ctx.r[3].s64 = ctx.r[1].s64 + 600;
	// 827F0D7C: 486026AD  bl 0x82df3428
	ctx.lr = 0x827F0D80;
	sub_82DF3428(ctx, base);
	// 827F0D80: 38610238  addi r3, r1, 0x238
	ctx.r[3].s64 = ctx.r[1].s64 + 568;
	// 827F0D84: 4BAD7F35  bl 0x822c8cb8
	ctx.lr = 0x827F0D88;
	sub_822C8CB8(ctx, base);
	// 827F0D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0D8C: 4860269D  bl 0x82df3428
	ctx.lr = 0x827F0D90;
	sub_82DF3428(ctx, base);
	// 827F0D90: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 827F0D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F0D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F0D9C: CBA1FFD0  lfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827F0DA0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 827F0DA4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827F0DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F0DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F0DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F0DB8 size=100
    let mut pc: u32 = 0x827F0DB8;
    'dispatch: loop {
        match pc {
            0x827F0DB8 => {
    //   block [0x827F0DB8..0x827F0E1C)
	// 827F0DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F0DBC: 489B73AD  bl 0x831a8168
	ctx.lr = 0x827F0DC0;
	sub_831A8130(ctx, base);
	// 827F0DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0DC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F0DC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F0DCC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 827F0DD0: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0DD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0DD8: 409A003C  bne cr6, 0x827f0e14
	if !ctx.cr[6].eq {
	pc = 0x827F0E14; continue 'dispatch;
	}
	// 827F0DDC: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 827F0DE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F0DE4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0DE8: 4BFFFFD1  bl 0x827f0db8
	ctx.lr = 0x827F0DEC;
	sub_827F0DB8(ctx, base);
	// 827F0DEC: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 827F0DF0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0DF4: 48602635  bl 0x82df3428
	ctx.lr = 0x827F0DF8;
	sub_82DF3428(ctx, base);
	// 827F0DF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F0DFC: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F0E00: 48601389  bl 0x82df2188
	ctx.lr = 0x827F0E04;
	sub_82DF2188(ctx, base);
	// 827F0E04: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0E08: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 827F0E0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0E10: 419AFFD0  beq cr6, 0x827f0de0
	if ctx.cr[6].eq {
	pc = 0x827F0DE0; continue 'dispatch;
	}
	// 827F0E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F0E18: 489B73A0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F0E20 size=84
    let mut pc: u32 = 0x827F0E20;
    'dispatch: loop {
        match pc {
            0x827F0E20 => {
    //   block [0x827F0E20..0x827F0E74)
	// 827F0E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F0E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F0E28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F0E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0E30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F0E34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0E38: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0E3C: 4BFFFF7D  bl 0x827f0db8
	ctx.lr = 0x827F0E40;
	sub_827F0DB8(ctx, base);
	// 827F0E40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0E44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F0E48: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F0E4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F0E50: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0E54: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F0E58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0E5C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F0E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F0E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F0E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F0E6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F0E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F0E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F0E78 size=1024
    let mut pc: u32 = 0x827F0E78;
    'dispatch: loop {
        match pc {
            0x827F0E78 => {
    //   block [0x827F0E78..0x827F1278)
	// 827F0E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F0E7C: 489B72DD  bl 0x831a8158
	ctx.lr = 0x827F0E80;
	sub_831A8130(ctx, base);
	// 827F0E80: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F0E84: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827F0E88: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 827F0E8C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 827F0E90: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 827F0E94: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0E9C: 419A0048  beq cr6, 0x827f0ee4
	if ctx.cr[6].eq {
	pc = 0x827F0EE4; continue 'dispatch;
	}
	// 827F0EA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F0EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0EA8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 827F0EAC: 4BAD4A1D  bl 0x822c58c8
	ctx.lr = 0x827F0EB0;
	sub_822C58C8(ctx, base);
	// 827F0EB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F0EB4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F0EB8: 4BAD8FF9  bl 0x822c9eb0
	ctx.lr = 0x827F0EBC;
	sub_822C9EB0(ctx, base);
	// 827F0EBC: 4BAD33F5  bl 0x822c42b0
	ctx.lr = 0x827F0EC0;
	sub_822C42B0(ctx, base);
	// 827F0EC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F0EC4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F0EC8: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 827F0ECC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827F0ED0: 4BAD45A1  bl 0x822c5470
	ctx.lr = 0x827F0ED4;
	sub_822C5470(ctx, base);
	// 827F0ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F0ED8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F0EDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F0EE0: 4BAD3E01  bl 0x822c4ce0
	ctx.lr = 0x827F0EE4;
	sub_822C4CE0(ctx, base);
	// 827F0EE4: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 827F0EE8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 827F0EEC: 4BBB079D  bl 0x823a1688
	ctx.lr = 0x827F0EF0;
	sub_823A1688(ctx, base);
	// 827F0EF0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0EF4: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0EF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F0EFC: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 827F0F00: 419A000C  beq cr6, 0x827f0f0c
	if ctx.cr[6].eq {
	pc = 0x827F0F0C; continue 'dispatch;
	}
	// 827F0F04: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0F08: 48000028  b 0x827f0f30
	pc = 0x827F0F30; continue 'dispatch;
	// 827F0F0C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0F10: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0F14: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F0F18: 419A000C  beq cr6, 0x827f0f24
	if ctx.cr[6].eq {
	pc = 0x827F0F24; continue 'dispatch;
	}
	// 827F0F1C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 827F0F20: 48000010  b 0x827f0f30
	pc = 0x827F0F30; continue 'dispatch;
	// 827F0F24: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0F28: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F0F2C: 409A00DC  bne cr6, 0x827f1008
	if !ctx.cr[6].eq {
	pc = 0x827F1008; continue 'dispatch;
	}
	// 827F0F30: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0F34: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0F38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0F3C: 409A0008  bne cr6, 0x827f0f44
	if !ctx.cr[6].eq {
	pc = 0x827F0F44; continue 'dispatch;
	}
	// 827F0F40: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827F0F44: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0F48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0F4C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F0F50: 409A000C  bne cr6, 0x827f0f5c
	if !ctx.cr[6].eq {
	pc = 0x827F0F5C; continue 'dispatch;
	}
	// 827F0F54: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827F0F58: 4800001C  b 0x827f0f74
	pc = 0x827F0F74; continue 'dispatch;
	// 827F0F5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0F60: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F0F64: 409A000C  bne cr6, 0x827f0f70
	if !ctx.cr[6].eq {
	pc = 0x827F0F70; continue 'dispatch;
	}
	// 827F0F68: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827F0F6C: 48000008  b 0x827f0f74
	pc = 0x827F0F74; continue 'dispatch;
	// 827F0F70: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827F0F74: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0F78: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0F7C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F0F80: 409A003C  bne cr6, 0x827f0fbc
	if !ctx.cr[6].eq {
	pc = 0x827F0FBC; continue 'dispatch;
	}
	// 827F0F84: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0F88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0F8C: 419A000C  beq cr6, 0x827f0f98
	if ctx.cr[6].eq {
	pc = 0x827F0F98; continue 'dispatch;
	}
	// 827F0F90: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827F0F94: 48000024  b 0x827f0fb8
	pc = 0x827F0FB8; continue 'dispatch;
	// 827F0F98: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0F9C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827F0FA0: 4800000C  b 0x827f0fac
	pc = 0x827F0FAC; continue 'dispatch;
	// 827F0FA4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827F0FA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F0FAC: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0FB0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827F0FB4: 419AFFF0  beq cr6, 0x827f0fa4
	if ctx.cr[6].eq {
	pc = 0x827F0FA4; continue 'dispatch;
	}
	// 827F0FB8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F0FBC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F0FC0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0FC4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F0FC8: 409A00D4  bne cr6, 0x827f109c
	if !ctx.cr[6].eq {
	pc = 0x827F109C; continue 'dispatch;
	}
	// 827F0FCC: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0FD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F0FD4: 419A000C  beq cr6, 0x827f0fe0
	if ctx.cr[6].eq {
	pc = 0x827F0FE0; continue 'dispatch;
	}
	// 827F0FD8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827F0FDC: 48000024  b 0x827f1000
	pc = 0x827F1000; continue 'dispatch;
	// 827F0FE0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0FE4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827F0FE8: 4800000C  b 0x827f0ff4
	pc = 0x827F0FF4; continue 'dispatch;
	// 827F0FEC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827F0FF0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F0FF4: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F0FF8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827F0FFC: 419AFFF0  beq cr6, 0x827f0fec
	if ctx.cr[6].eq {
	pc = 0x827F0FEC; continue 'dispatch;
	}
	// 827F1000: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F1004: 48000098  b 0x827f109c
	pc = 0x827F109C; continue 'dispatch;
	// 827F1008: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827F100C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1010: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1014: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1018: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F101C: 409A000C  bne cr6, 0x827f1028
	if !ctx.cr[6].eq {
	pc = 0x827F1028; continue 'dispatch;
	}
	// 827F1020: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 827F1024: 4800002C  b 0x827f1050
	pc = 0x827F1050; continue 'dispatch;
	// 827F1028: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F102C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1034: 409A0008  bne cr6, 0x827f103c
	if !ctx.cr[6].eq {
	pc = 0x827F103C; continue 'dispatch;
	}
	// 827F1038: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827F103C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827F1040: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1044: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F1048: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F104C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827F1050: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1054: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1058: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F105C: 409A000C  bne cr6, 0x827f1068
	if !ctx.cr[6].eq {
	pc = 0x827F1068; continue 'dispatch;
	}
	// 827F1060: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827F1064: 48000020  b 0x827f1084
	pc = 0x827F1084; continue 'dispatch;
	// 827F1068: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F106C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1070: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F1074: 409A000C  bne cr6, 0x827f1080
	if !ctx.cr[6].eq {
	pc = 0x827F1080; continue 'dispatch;
	}
	// 827F1078: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827F107C: 48000008  b 0x827f1084
	pc = 0x827F1084; continue 'dispatch;
	// 827F1080: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 827F1084: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1088: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F108C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1090: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1094: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 827F1098: 995B0018  stb r10, 0x18(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827F109C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F10A0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F10A4: 409A0198  bne cr6, 0x827f123c
	if !ctx.cr[6].eq {
	pc = 0x827F123C; continue 'dispatch;
	}
	// 827F10A8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F10AC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 827F10B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F10B4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F10B8: 419A0180  beq cr6, 0x827f1238
	if ctx.cr[6].eq {
	pc = 0x827F1238; continue 'dispatch;
	}
	// 827F10BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F10C0: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F10C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F10C8: 409A0170  bne cr6, 0x827f1238
	if !ctx.cr[6].eq {
	pc = 0x827F1238; continue 'dispatch;
	}
	// 827F10CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F10D0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F10D4: 409A00A8  bne cr6, 0x827f117c
	if !ctx.cr[6].eq {
	pc = 0x827F117C; continue 'dispatch;
	}
	// 827F10D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F10DC: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F10E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F10E4: 409A001C  bne cr6, 0x827f1100
	if !ctx.cr[6].eq {
	pc = 0x827F1100; continue 'dispatch;
	}
	// 827F10E8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F10EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F10F0: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F10F4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F10F8: 4BC52E71  bl 0x82443f68
	ctx.lr = 0x827F10FC;
	sub_82443F68(ctx, base);
	// 827F10FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1100: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F1104: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F1108: 409A00C8  bne cr6, 0x827f11d0
	if !ctx.cr[6].eq {
	pc = 0x827F11D0; continue 'dispatch;
	}
	// 827F110C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1110: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1114: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F1118: 409A0014  bne cr6, 0x827f112c
	if !ctx.cr[6].eq {
	pc = 0x827F112C; continue 'dispatch;
	}
	// 827F111C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1120: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1124: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F1128: 419A00A4  beq cr6, 0x827f11cc
	if ctx.cr[6].eq {
	pc = 0x827F11CC; continue 'dispatch;
	}
	// 827F112C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1130: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1134: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F1138: 409A0020  bne cr6, 0x827f1158
	if !ctx.cr[6].eq {
	pc = 0x827F1158; continue 'dispatch;
	}
	// 827F113C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1140: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827F1144: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1148: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F114C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F1150: 4BC52DB1  bl 0x82443f00
	ctx.lr = 0x827F1154;
	sub_82443F00(ctx, base);
	// 827F1154: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1158: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F115C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1160: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1164: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827F1168: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F116C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1170: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F1174: 4BC52DF5  bl 0x82443f68
	ctx.lr = 0x827F1178;
	sub_82443F68(ctx, base);
	// 827F1178: 480000C0  b 0x827f1238
	pc = 0x827F1238; continue 'dispatch;
	// 827F117C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1180: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F1184: 409A001C  bne cr6, 0x827f11a0
	if !ctx.cr[6].eq {
	pc = 0x827F11A0; continue 'dispatch;
	}
	// 827F1188: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F118C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1190: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F1194: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1198: 4BC52D69  bl 0x82443f00
	ctx.lr = 0x827F119C;
	sub_82443F00(ctx, base);
	// 827F119C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F11A0: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F11A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F11A8: 409A0028  bne cr6, 0x827f11d0
	if !ctx.cr[6].eq {
	pc = 0x827F11D0; continue 'dispatch;
	}
	// 827F11AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F11B0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F11B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F11B8: 409A0034  bne cr6, 0x827f11ec
	if !ctx.cr[6].eq {
	pc = 0x827F11EC; continue 'dispatch;
	}
	// 827F11BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F11C0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F11C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F11C8: 409A0024  bne cr6, 0x827f11ec
	if !ctx.cr[6].eq {
	pc = 0x827F11EC; continue 'dispatch;
	}
	// 827F11CC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F11D0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F11D4: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 827F11D8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F11DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F11E0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F11E4: 409AFEDC  bne cr6, 0x827f10c0
	if !ctx.cr[6].eq {
	pc = 0x827F10C0; continue 'dispatch;
	}
	// 827F11E8: 48000050  b 0x827f1238
	pc = 0x827F1238; continue 'dispatch;
	// 827F11EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F11F0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F11F4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F11F8: 409A0020  bne cr6, 0x827f1218
	if !ctx.cr[6].eq {
	pc = 0x827F1218; continue 'dispatch;
	}
	// 827F11FC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1200: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827F1204: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1208: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F120C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F1210: 4BC52D59  bl 0x82443f68
	ctx.lr = 0x827F1214;
	sub_82443F68(ctx, base);
	// 827F1214: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1218: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F121C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1220: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1224: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827F1228: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F122C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1230: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F1234: 4BC52CCD  bl 0x82443f00
	ctx.lr = 0x827F1238;
	sub_82443F00(ctx, base);
	// 827F1238: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F123C: 387B000C  addi r3, r27, 0xc
	ctx.r[3].s64 = ctx.r[27].s64 + 12;
	// 827F1240: 486021E9  bl 0x82df3428
	ctx.lr = 0x827F1244;
	sub_82DF3428(ctx, base);
	// 827F1244: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F1248: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827F124C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F1250: 48600F39  bl 0x82df2188
	ctx.lr = 0x827F1254;
	sub_82DF2188(ctx, base);
	// 827F1254: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F125C: 419A000C  beq cr6, 0x827f1268
	if ctx.cr[6].eq {
	pc = 0x827F1268; continue 'dispatch;
	}
	// 827F1260: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827F1264: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F1268: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827F126C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 827F1270: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827F1274: 489B6F34  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1278 size=1036
    let mut pc: u32 = 0x827F1278;
    'dispatch: loop {
        match pc {
            0x827F1278 => {
    //   block [0x827F1278..0x827F1684)
	// 827F1278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F127C: 489B6EDD  bl 0x831a8158
	ctx.lr = 0x827F1280;
	sub_831A8130(ctx, base);
	// 827F1280: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1284: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827F1288: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 827F128C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 827F1290: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 827F1294: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F1298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F129C: 419A0048  beq cr6, 0x827f12e4
	if ctx.cr[6].eq {
	pc = 0x827F12E4; continue 'dispatch;
	}
	// 827F12A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F12A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F12A8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 827F12AC: 4BAD461D  bl 0x822c58c8
	ctx.lr = 0x827F12B0;
	sub_822C58C8(ctx, base);
	// 827F12B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F12B4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F12B8: 4BAD8BF9  bl 0x822c9eb0
	ctx.lr = 0x827F12BC;
	sub_822C9EB0(ctx, base);
	// 827F12BC: 4BAD2FF5  bl 0x822c42b0
	ctx.lr = 0x827F12C0;
	sub_822C42B0(ctx, base);
	// 827F12C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F12C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F12C8: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 827F12CC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 827F12D0: 4BAD41A1  bl 0x822c5470
	ctx.lr = 0x827F12D4;
	sub_822C5470(ctx, base);
	// 827F12D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F12D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F12DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F12E0: 4BAD3A01  bl 0x822c4ce0
	ctx.lr = 0x827F12E4;
	sub_822C4CE0(ctx, base);
	// 827F12E4: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 827F12E8: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 827F12EC: 4BBB039D  bl 0x823a1688
	ctx.lr = 0x827F12F0;
	sub_823A1688(ctx, base);
	// 827F12F0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F12F4: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F12F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F12FC: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 827F1300: 419A000C  beq cr6, 0x827f130c
	if ctx.cr[6].eq {
	pc = 0x827F130C; continue 'dispatch;
	}
	// 827F1304: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1308: 48000028  b 0x827f1330
	pc = 0x827F1330; continue 'dispatch;
	// 827F130C: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1310: 894A0019  lbz r10, 0x19(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F1314: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F1318: 419A000C  beq cr6, 0x827f1324
	if ctx.cr[6].eq {
	pc = 0x827F1324; continue 'dispatch;
	}
	// 827F131C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 827F1320: 48000010  b 0x827f1330
	pc = 0x827F1330; continue 'dispatch;
	// 827F1324: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1328: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F132C: 409A00DC  bne cr6, 0x827f1408
	if !ctx.cr[6].eq {
	pc = 0x827F1408; continue 'dispatch;
	}
	// 827F1330: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F1334: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F133C: 409A0008  bne cr6, 0x827f1344
	if !ctx.cr[6].eq {
	pc = 0x827F1344; continue 'dispatch;
	}
	// 827F1340: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827F1344: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1348: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F134C: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F1350: 409A000C  bne cr6, 0x827f135c
	if !ctx.cr[6].eq {
	pc = 0x827F135C; continue 'dispatch;
	}
	// 827F1354: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 827F1358: 4800001C  b 0x827f1374
	pc = 0x827F1374; continue 'dispatch;
	// 827F135C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1360: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F1364: 409A000C  bne cr6, 0x827f1370
	if !ctx.cr[6].eq {
	pc = 0x827F1370; continue 'dispatch;
	}
	// 827F1368: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827F136C: 48000008  b 0x827f1374
	pc = 0x827F1374; continue 'dispatch;
	// 827F1370: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 827F1374: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1378: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F137C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F1380: 409A003C  bne cr6, 0x827f13bc
	if !ctx.cr[6].eq {
	pc = 0x827F13BC; continue 'dispatch;
	}
	// 827F1384: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F1388: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F138C: 419A000C  beq cr6, 0x827f1398
	if ctx.cr[6].eq {
	pc = 0x827F1398; continue 'dispatch;
	}
	// 827F1390: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827F1394: 48000024  b 0x827f13b8
	pc = 0x827F13B8; continue 'dispatch;
	// 827F1398: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F139C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827F13A0: 4800000C  b 0x827f13ac
	pc = 0x827F13AC; continue 'dispatch;
	// 827F13A4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827F13A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F13AC: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F13B0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827F13B4: 419AFFF0  beq cr6, 0x827f13a4
	if ctx.cr[6].eq {
	pc = 0x827F13A4; continue 'dispatch;
	}
	// 827F13B8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F13BC: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F13C0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F13C4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F13C8: 409A00D4  bne cr6, 0x827f149c
	if !ctx.cr[6].eq {
	pc = 0x827F149C; continue 'dispatch;
	}
	// 827F13CC: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F13D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F13D4: 419A000C  beq cr6, 0x827f13e0
	if ctx.cr[6].eq {
	pc = 0x827F13E0; continue 'dispatch;
	}
	// 827F13D8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 827F13DC: 48000024  b 0x827f1400
	pc = 0x827F1400; continue 'dispatch;
	// 827F13E0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F13E4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 827F13E8: 4800000C  b 0x827f13f4
	pc = 0x827F13F4; continue 'dispatch;
	// 827F13EC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 827F13F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F13F4: 890B0019  lbz r8, 0x19(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F13F8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 827F13FC: 419AFFF0  beq cr6, 0x827f13ec
	if ctx.cr[6].eq {
	pc = 0x827F13EC; continue 'dispatch;
	}
	// 827F1400: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F1404: 48000098  b 0x827f149c
	pc = 0x827F149C; continue 'dispatch;
	// 827F1408: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827F140C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1410: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1414: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1418: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F141C: 409A000C  bne cr6, 0x827f1428
	if !ctx.cr[6].eq {
	pc = 0x827F1428; continue 'dispatch;
	}
	// 827F1420: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 827F1424: 4800002C  b 0x827f1450
	pc = 0x827F1450; continue 'dispatch;
	// 827F1428: 897C0019  lbz r11, 0x19(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F142C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1430: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1434: 409A0008  bne cr6, 0x827f143c
	if !ctx.cr[6].eq {
	pc = 0x827F143C; continue 'dispatch;
	}
	// 827F1438: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827F143C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 827F1440: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1444: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F1448: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F144C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827F1450: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1454: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1458: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F145C: 409A000C  bne cr6, 0x827f1468
	if !ctx.cr[6].eq {
	pc = 0x827F1468; continue 'dispatch;
	}
	// 827F1460: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 827F1464: 48000020  b 0x827f1484
	pc = 0x827F1484; continue 'dispatch;
	// 827F1468: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F146C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1470: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 827F1474: 409A000C  bne cr6, 0x827f1480
	if !ctx.cr[6].eq {
	pc = 0x827F1480; continue 'dispatch;
	}
	// 827F1478: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827F147C: 48000008  b 0x827f1484
	pc = 0x827F1484; continue 'dispatch;
	// 827F1480: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 827F1484: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1488: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F148C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1490: 89590018  lbz r10, 0x18(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1494: 99790018  stb r11, 0x18(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 827F1498: 995B0018  stb r10, 0x18(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827F149C: 897B0018  lbz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F14A0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F14A4: 409A0198  bne cr6, 0x827f163c
	if !ctx.cr[6].eq {
	pc = 0x827F163C; continue 'dispatch;
	}
	// 827F14A8: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F14AC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 827F14B0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F14B4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F14B8: 419A0180  beq cr6, 0x827f1638
	if ctx.cr[6].eq {
	pc = 0x827F1638; continue 'dispatch;
	}
	// 827F14BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F14C0: 897C0018  lbz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F14C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F14C8: 409A0170  bne cr6, 0x827f1638
	if !ctx.cr[6].eq {
	pc = 0x827F1638; continue 'dispatch;
	}
	// 827F14CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F14D0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F14D4: 409A00A8  bne cr6, 0x827f157c
	if !ctx.cr[6].eq {
	pc = 0x827F157C; continue 'dispatch;
	}
	// 827F14D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F14DC: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F14E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F14E4: 409A001C  bne cr6, 0x827f1500
	if !ctx.cr[6].eq {
	pc = 0x827F1500; continue 'dispatch;
	}
	// 827F14E8: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F14EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F14F0: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F14F4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F14F8: 4BC52A71  bl 0x82443f68
	ctx.lr = 0x827F14FC;
	sub_82443F68(ctx, base);
	// 827F14FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1500: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F1504: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F1508: 409A00C8  bne cr6, 0x827f15d0
	if !ctx.cr[6].eq {
	pc = 0x827F15D0; continue 'dispatch;
	}
	// 827F150C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1510: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1514: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F1518: 409A0014  bne cr6, 0x827f152c
	if !ctx.cr[6].eq {
	pc = 0x827F152C; continue 'dispatch;
	}
	// 827F151C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1520: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1524: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F1528: 419A00A4  beq cr6, 0x827f15cc
	if ctx.cr[6].eq {
	pc = 0x827F15CC; continue 'dispatch;
	}
	// 827F152C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1530: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1534: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F1538: 409A0020  bne cr6, 0x827f1558
	if !ctx.cr[6].eq {
	pc = 0x827F1558; continue 'dispatch;
	}
	// 827F153C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1540: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827F1544: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1548: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F154C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F1550: 4BC529B1  bl 0x82443f00
	ctx.lr = 0x827F1554;
	sub_82443F00(ctx, base);
	// 827F1554: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1558: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F155C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1560: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1564: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827F1568: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F156C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1570: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F1574: 4BC529F5  bl 0x82443f68
	ctx.lr = 0x827F1578;
	sub_82443F68(ctx, base);
	// 827F1578: 480000C0  b 0x827f1638
	pc = 0x827F1638; continue 'dispatch;
	// 827F157C: 894B0018  lbz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1580: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F1584: 409A001C  bne cr6, 0x827f15a0
	if !ctx.cr[6].eq {
	pc = 0x827F15A0; continue 'dispatch;
	}
	// 827F1588: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F158C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1590: 9BBF0018  stb r29, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F1594: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1598: 4BC52969  bl 0x82443f00
	ctx.lr = 0x827F159C;
	sub_82443F00(ctx, base);
	// 827F159C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F15A0: 894B0019  lbz r10, 0x19(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(25 as u32) ) } as u64;
	// 827F15A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F15A8: 409A0028  bne cr6, 0x827f15d0
	if !ctx.cr[6].eq {
	pc = 0x827F15D0; continue 'dispatch;
	}
	// 827F15AC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F15B0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F15B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F15B8: 409A0034  bne cr6, 0x827f15ec
	if !ctx.cr[6].eq {
	pc = 0x827F15EC; continue 'dispatch;
	}
	// 827F15BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F15C0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F15C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F15C8: 409A0024  bne cr6, 0x827f15ec
	if !ctx.cr[6].eq {
	pc = 0x827F15EC; continue 'dispatch;
	}
	// 827F15CC: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F15D0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F15D4: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 827F15D8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F15DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F15E0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F15E4: 409AFEDC  bne cr6, 0x827f14c0
	if !ctx.cr[6].eq {
	pc = 0x827F14C0; continue 'dispatch;
	}
	// 827F15E8: 48000050  b 0x827f1638
	pc = 0x827F1638; continue 'dispatch;
	// 827F15EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F15F0: 894A0018  lbz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F15F4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 827F15F8: 409A0020  bne cr6, 0x827f1618
	if !ctx.cr[6].eq {
	pc = 0x827F1618; continue 'dispatch;
	}
	// 827F15FC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1600: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 827F1604: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1608: 9BCA0018  stb r30, 0x18(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F160C: 9BAB0018  stb r29, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[29].u8 ) };
	// 827F1610: 4BC52959  bl 0x82443f68
	ctx.lr = 0x827F1614;
	sub_82443F68(ctx, base);
	// 827F1614: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1618: 895F0018  lbz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F161C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1620: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 827F1624: 994B0018  stb r10, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u8 ) };
	// 827F1628: 9BDF0018  stb r30, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F162C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1630: 9BCB0018  stb r30, 0x18(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F1634: 4BC528CD  bl 0x82443f00
	ctx.lr = 0x827F1638;
	sub_82443F00(ctx, base);
	// 827F1638: 9BDC0018  stb r30, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u8 ) };
	// 827F163C: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 827F1640: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F1644: 48601DE5  bl 0x82df3428
	ctx.lr = 0x827F1648;
	sub_82DF3428(ctx, base);
	// 827F1648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F164C: 48601DDD  bl 0x82df3428
	ctx.lr = 0x827F1650;
	sub_82DF3428(ctx, base);
	// 827F1650: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F1654: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 827F1658: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F165C: 48600B2D  bl 0x82df2188
	ctx.lr = 0x827F1660;
	sub_82DF2188(ctx, base);
	// 827F1660: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1664: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1668: 419A000C  beq cr6, 0x827f1674
	if ctx.cr[6].eq {
	pc = 0x827F1674; continue 'dispatch;
	}
	// 827F166C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827F1670: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F1674: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 827F1678: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 827F167C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 827F1680: 489B6B28  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1688 size=132
    let mut pc: u32 = 0x827F1688;
    'dispatch: loop {
        match pc {
            0x827F1688 => {
    //   block [0x827F1688..0x827F170C)
	// 827F1688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F168C: 489B6ADD  bl 0x831a8168
	ctx.lr = 0x827F1690;
	sub_831A8130(ctx, base);
	// 827F1690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1694: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F1698: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 827F169C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F16A0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827F16A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F16A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F16AC: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F16B0: 409A0044  bne cr6, 0x827f16f4
	if !ctx.cr[6].eq {
	pc = 0x827F16F4; continue 'dispatch;
	}
	// 827F16B4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F16B8: 409A003C  bne cr6, 0x827f16f4
	if !ctx.cr[6].eq {
	pc = 0x827F16F4; continue 'dispatch;
	}
	// 827F16BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F16C0: 4BFFF761  bl 0x827f0e20
	ctx.lr = 0x827F16C4;
	sub_827F0E20(ctx, base);
	// 827F16C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F16C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F16CC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F16D0: 48000030  b 0x827f1700
	pc = 0x827F1700; continue 'dispatch;
	// 827F16D4: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 827F16D8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F16DC: 4BBAFFAD  bl 0x823a1688
	ctx.lr = 0x827F16E0;
	sub_823A1688(ctx, base);
	// 827F16E0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827F16E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F16E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F16EC: 4BFFF78D  bl 0x827f0e78
	ctx.lr = 0x827F16F0;
	sub_827F0E78(ctx, base);
	// 827F16F0: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 827F16F4: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827F16F8: 409AFFDC  bne cr6, 0x827f16d4
	if !ctx.cr[6].eq {
	pc = 0x827F16D4; continue 'dispatch;
	}
	// 827F16FC: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 827F1700: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F1704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F1708: 489B6AB0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1710 size=132
    let mut pc: u32 = 0x827F1710;
    'dispatch: loop {
        match pc {
            0x827F1710 => {
    //   block [0x827F1710..0x827F1794)
	// 827F1710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1714: 489B6A55  bl 0x831a8168
	ctx.lr = 0x827F1718;
	sub_831A8130(ctx, base);
	// 827F1718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F171C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F1720: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 827F1724: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F1728: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827F172C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1730: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1734: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F1738: 409A0044  bne cr6, 0x827f177c
	if !ctx.cr[6].eq {
	pc = 0x827F177C; continue 'dispatch;
	}
	// 827F173C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F1740: 409A003C  bne cr6, 0x827f177c
	if !ctx.cr[6].eq {
	pc = 0x827F177C; continue 'dispatch;
	}
	// 827F1744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1748: 4BADA9D1  bl 0x822cc118
	ctx.lr = 0x827F174C;
	sub_822CC118(ctx, base);
	// 827F174C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1750: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1754: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1758: 48000030  b 0x827f1788
	pc = 0x827F1788; continue 'dispatch;
	// 827F175C: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 827F1760: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F1764: 4BBAFF25  bl 0x823a1688
	ctx.lr = 0x827F1768;
	sub_823A1688(ctx, base);
	// 827F1768: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 827F176C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1770: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F1774: 4BFFFB05  bl 0x827f1278
	ctx.lr = 0x827F1778;
	sub_827F1278(ctx, base);
	// 827F1778: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 827F177C: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 827F1780: 409AFFDC  bne cr6, 0x827f175c
	if !ctx.cr[6].eq {
	pc = 0x827F175C; continue 'dispatch;
	}
	// 827F1784: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 827F1788: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F178C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F1790: 489B6A28  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1798 size=88
    let mut pc: u32 = 0x827F1798;
    'dispatch: loop {
        match pc {
            0x827F1798 => {
    //   block [0x827F1798..0x827F17F0)
	// 827F1798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F179C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F17A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F17A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F17A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F17AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F17B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F17B4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F17B8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F17BC: 4BFFFECD  bl 0x827f1688
	ctx.lr = 0x827F17C0;
	sub_827F1688(ctx, base);
	// 827F17C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F17C4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F17C8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 827F17CC: 486009BD  bl 0x82df2188
	ctx.lr = 0x827F17D0;
	sub_82DF2188(ctx, base);
	// 827F17D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F17D4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 827F17D8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F17DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F17E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F17E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F17E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F17EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F17F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F17F0 size=204
    let mut pc: u32 = 0x827F17F0;
    'dispatch: loop {
        match pc {
            0x827F17F0 => {
    //   block [0x827F17F0..0x827F18BC)
	// 827F17F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F17F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F17F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F17FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1804: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F1808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F180C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F1810: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 827F1814: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 827F1818: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F181C: 3BC99BC9  addi r30, r9, -0x6437
	ctx.r[30].s64 = ctx.r[9].s64 + -25655;
	// 827F1820: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F1824: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F1828: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F182C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F1830: 486021D9  bl 0x82df3a08
	ctx.lr = 0x827F1834;
	sub_82DF3A08(ctx, base);
	// 827F1834: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F1838: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 827F183C: 486021CD  bl 0x82df3a08
	ctx.lr = 0x827F1840;
	sub_82DF3A08(ctx, base);
	// 827F1840: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 827F1844: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F1848: 486021C1  bl 0x82df3a08
	ctx.lr = 0x827F184C;
	sub_82DF3A08(ctx, base);
	// 827F184C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F1850: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827F1854: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 827F1858: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 827F185C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F1860: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F1864: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827F1868: 80AA6750  lwz r5, 0x6750(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26448 as u32) ) } as u64;
	// 827F186C: 80896784  lwz r4, 0x6784(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26500 as u32) ) } as u64;
	// 827F1870: 80C867C0  lwz r6, 0x67c0(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(26560 as u32) ) } as u64;
	// 827F1874: 4BB05FA5  bl 0x822f7818
	ctx.lr = 0x827F1878;
	sub_822F7818(ctx, base);
	// 827F1878: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F187C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F1880: 4BAF3641  bl 0x822e4ec0
	ctx.lr = 0x827F1884;
	sub_822E4EC0(ctx, base);
	// 827F1884: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F1888: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F188C: E89E0000  ld r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 827F1890: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F1894: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827F1898: 4BC9A9F1  bl 0x8248c288
	ctx.lr = 0x827F189C;
	sub_8248C288(ctx, base);
	// 827F189C: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 827F18A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F18A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F18A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F18AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F18B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F18B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F18B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F18C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F18C0 size=180
    let mut pc: u32 = 0x827F18C0;
    'dispatch: loop {
        match pc {
            0x827F18C0 => {
    //   block [0x827F18C0..0x827F1974)
	// 827F18C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F18C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F18C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F18CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F18D0: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 827F18D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F18D8: 809F67C0  lwz r4, 0x67c0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26560 as u32) ) } as u64;
	// 827F18DC: 409A0024  bne cr6, 0x827f1900
	if !ctx.cr[6].eq {
	pc = 0x827F1900; continue 'dispatch;
	}
	// 827F18E0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F18E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F18E8: 80AB6750  lwz r5, 0x6750(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26448 as u32) ) } as u64;
	// 827F18EC: 4BB05EB5  bl 0x822f77a0
	ctx.lr = 0x827F18F0;
	sub_822F77A0(ctx, base);
	// 827F18F0: 80FF67C0  lwz r7, 0x67c0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26560 as u32) ) } as u64;
	// 827F18F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F18F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F18FC: 48000030  b 0x827f192c
	pc = 0x827F192C; continue 'dispatch;
	// 827F1900: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 827F1904: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F1908: 409A000C  bne cr6, 0x827f1914
	if !ctx.cr[6].eq {
	pc = 0x827F1914; continue 'dispatch;
	}
	// 827F190C: 4BAF3625  bl 0x822e4f30
	ctx.lr = 0x827F1910;
	sub_822E4F30(ctx, base);
	// 827F1910: 48000010  b 0x827f1920
	pc = 0x827F1920; continue 'dispatch;
	// 827F1914: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F1918: 80AB6750  lwz r5, 0x6750(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26448 as u32) ) } as u64;
	// 827F191C: 4BB05E85  bl 0x822f77a0
	ctx.lr = 0x827F1920;
	sub_822F77A0(ctx, base);
	// 827F1920: 80FF67C0  lwz r7, 0x67c0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(26560 as u32) ) } as u64;
	// 827F1924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F192C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 827F1930: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 827F1934: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 827F1938: 80CB6758  lwz r6, 0x6758(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26456 as u32) ) } as u64;
	// 827F193C: 80AA6754  lwz r5, 0x6754(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26452 as u32) ) } as u64;
	// 827F1940: 80896768  lwz r4, 0x6768(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26472 as u32) ) } as u64;
	// 827F1944: 4BAFD485  bl 0x822eedc8
	ctx.lr = 0x827F1948;
	sub_822EEDC8(ctx, base);
	// 827F1948: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F194C: E89F0000  ld r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 827F1950: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F1954: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 827F1958: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F195C: 4BC9A92D  bl 0x8248c288
	ctx.lr = 0x827F1960;
	sub_8248C288(ctx, base);
	// 827F1960: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F1964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F1968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F196C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F1970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1978 size=116
    let mut pc: u32 = 0x827F1978;
    'dispatch: loop {
        match pc {
            0x827F1978 => {
    //   block [0x827F1978..0x827F19EC)
	// 827F1978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F1984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F198C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F1990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1994: 3BCB9BC9  addi r30, r11, -0x6437
	ctx.r[30].s64 = ctx.r[11].s64 + -25655;
	// 827F1998: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F199C: 4860206D  bl 0x82df3a08
	ctx.lr = 0x827F19A0;
	sub_82DF3A08(ctx, base);
	// 827F19A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F19A4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F19A8: 48602061  bl 0x82df3a08
	ctx.lr = 0x827F19AC;
	sub_82DF3A08(ctx, base);
	// 827F19AC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 827F19B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F19B4: 48602055  bl 0x82df3a08
	ctx.lr = 0x827F19B8;
	sub_82DF3A08(ctx, base);
	// 827F19B8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 827F19BC: 4BB17A6D  bl 0x82309428
	ctx.lr = 0x827F19C0;
	sub_82309428(ctx, base);
	// 827F19C0: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 827F19C4: 4BFAA235  bl 0x8279bbf8
	ctx.lr = 0x827F19C8;
	sub_8279BBF8(ctx, base);
	// 827F19C8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 827F19CC: 4BFAA22D  bl 0x8279bbf8
	ctx.lr = 0x827F19D0;
	sub_8279BBF8(ctx, base);
	// 827F19D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F19D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F19D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F19DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F19E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F19E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F19E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F19F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F19F0 size=4
    let mut pc: u32 = 0x827F19F0;
    'dispatch: loop {
        match pc {
            0x827F19F0 => {
    //   block [0x827F19F0..0x827F19F4)
	// 827F19F0: 4BD1FBA8  b 0x82511598
	sub_82511598(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F19F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F19F8 size=72
    let mut pc: u32 = 0x827F19F8;
    'dispatch: loop {
        match pc {
            0x827F19F8 => {
    //   block [0x827F19F8..0x827F1A40)
	// 827F19F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F19FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1A00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1A04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1A0C: 4BD1F6E5  bl 0x825110f0
	ctx.lr = 0x827F1A10;
	sub_825110F0(ctx, base);
	// 827F1A10: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F1A14: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F1A18: 396B6130  addi r11, r11, 0x6130
	ctx.r[11].s64 = ctx.r[11].s64 + 24880;
	// 827F1A1C: 394A611C  addi r10, r10, 0x611c
	ctx.r[10].s64 = ctx.r[10].s64 + 24860;
	// 827F1A20: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1A24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1A28: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F1A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F1A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F1A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F1A38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F1A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F1A40 size=28
    let mut pc: u32 = 0x827F1A40;
    'dispatch: loop {
        match pc {
            0x827F1A40 => {
    //   block [0x827F1A40..0x827F1A5C)
	// 827F1A40: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F1A44: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F1A48: 396B6130  addi r11, r11, 0x6130
	ctx.r[11].s64 = ctx.r[11].s64 + 24880;
	// 827F1A4C: 394A611C  addi r10, r10, 0x611c
	ctx.r[10].s64 = ctx.r[10].s64 + 24860;
	// 827F1A50: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1A54: 91430028  stw r10, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F1A58: 4BD1F740  b 0x82511198
	sub_82511198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F1A60 size=28
    let mut pc: u32 = 0x827F1A60;
    'dispatch: loop {
        match pc {
            0x827F1A60 => {
    //   block [0x827F1A60..0x827F1A7C)
	// 827F1A60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F1A64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1A68: 388B0028  addi r4, r11, 0x28
	ctx.r[4].s64 = ctx.r[11].s64 + 40;
	// 827F1A6C: 409A0008  bne cr6, 0x827f1a74
	if !ctx.cr[6].eq {
	pc = 0x827F1A74; continue 'dispatch;
	}
	// 827F1A70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F1A74: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 827F1A78: 4BD16D28  b 0x825087a0
	sub_825087A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F1A80 size=8
    let mut pc: u32 = 0x827F1A80;
    'dispatch: loop {
        match pc {
            0x827F1A80 => {
    //   block [0x827F1A80..0x827F1A88)
	// 827F1A80: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827F1A84: 48000004  b 0x827f1a88
	sub_827F1A88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1A88 size=100
    let mut pc: u32 = 0x827F1A88;
    'dispatch: loop {
        match pc {
            0x827F1A88 => {
    //   block [0x827F1A88..0x827F1AEC)
	// 827F1A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F1A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1AA0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F1AA4: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F1AA8: 396B6130  addi r11, r11, 0x6130
	ctx.r[11].s64 = ctx.r[11].s64 + 24880;
	// 827F1AAC: 394A611C  addi r10, r10, 0x611c
	ctx.r[10].s64 = ctx.r[10].s64 + 24860;
	// 827F1AB0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1AB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F1AB8: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F1ABC: 4BD1F6DD  bl 0x82511198
	ctx.lr = 0x827F1AC0;
	sub_82511198(ctx, base);
	// 827F1AC0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F1AC4: 4182000C  beq 0x827f1ad0
	if ctx.cr[0].eq {
	pc = 0x827F1AD0; continue 'dispatch;
	}
	// 827F1AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1ACC: 4860090D  bl 0x82df23d8
	ctx.lr = 0x827F1AD0;
	sub_82DF23D8(ctx, base);
	// 827F1AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1AD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F1AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F1ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F1AE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F1AE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F1AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1AF0 size=160
    let mut pc: u32 = 0x827F1AF0;
    'dispatch: loop {
        match pc {
            0x827F1AF0 => {
    //   block [0x827F1AF0..0x827F1B90)
	// 827F1AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1AF4: 489B6679  bl 0x831a816c
	ctx.lr = 0x827F1AF8;
	sub_831A8130(ctx, base);
	// 827F1AF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1AFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F1B00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F1B04: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1B08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1B0C: 409A0018  bne cr6, 0x827f1b24
	if !ctx.cr[6].eq {
	pc = 0x827F1B24; continue 'dispatch;
	}
	// 827F1B10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F1B14: 4BAE1AC5  bl 0x822d35d8
	ctx.lr = 0x827F1B18;
	sub_822D35D8(ctx, base);
	// 827F1B18: 4BACE4E9  bl 0x822c0000
	ctx.lr = 0x827F1B1C;
	sub_822C0000(ctx, base);
	// 827F1B1C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F1B20: 4BAE0CF1  bl 0x822d2810
	ctx.lr = 0x827F1B24;
	sub_822D2810(ctx, base);
	// 827F1B24: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1B28: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 827F1B2C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1B34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F1B38: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F1B3C: 419A0024  beq cr6, 0x827f1b60
	if ctx.cr[6].eq {
	pc = 0x827F1B60; continue 'dispatch;
	}
	// 827F1B40: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F1B44: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F1B48: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1B4C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F1B50: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F1B54: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F1B58: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1B5C: 4082FFE8  bne 0x827f1b44
	if !ctx.cr[0].eq {
	pc = 0x827F1B44; continue 'dispatch;
	}
	// 827F1B60: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1B64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F1B68: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 827F1B6C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1B70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F1B74: 4E800421  bctrl
	ctx.lr = 0x827F1B78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F1B78: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1B7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1B80: 419A0008  beq cr6, 0x827f1b88
	if ctx.cr[6].eq {
	pc = 0x827F1B88; continue 'dispatch;
	}
	// 827F1B84: 4BACED0D  bl 0x822c0890
	ctx.lr = 0x827F1B88;
	sub_822C0890(ctx, base);
	// 827F1B88: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827F1B8C: 489B6630  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1B90 size=548
    let mut pc: u32 = 0x827F1B90;
    'dispatch: loop {
        match pc {
            0x827F1B90 => {
    //   block [0x827F1B90..0x827F1DB4)
	// 827F1B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1B94: 489B65D5  bl 0x831a8168
	ctx.lr = 0x827F1B98;
	sub_831A8130(ctx, base);
	// 827F1B98: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1B9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F1BA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F1BA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F1BA8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 827F1BAC: 4BDE734D  bl 0x825d8ef8
	ctx.lr = 0x827F1BB0;
	sub_825D8EF8(ctx, base);
	// 827F1BB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F1BB4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F1BB8: 48601E51  bl 0x82df3a08
	ctx.lr = 0x827F1BBC;
	sub_82DF3A08(ctx, base);
	// 827F1BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F1BC0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 827F1BC4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1BC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F1BCC: 4BC2D19D  bl 0x8241ed68
	ctx.lr = 0x827F1BD0;
	sub_8241ED68(ctx, base);
	// 827F1BD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F1BD4: 48601855  bl 0x82df3428
	ctx.lr = 0x827F1BD8;
	sub_82DF3428(ctx, base);
	// 827F1BD8: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 827F1BDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F1BE0: 409A0014  bne cr6, 0x827f1bf4
	if !ctx.cr[6].eq {
	pc = 0x827F1BF4; continue 'dispatch;
	}
	// 827F1BE4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F1BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1BEC: 419A01B0  beq cr6, 0x827f1d9c
	if ctx.cr[6].eq {
	pc = 0x827F1D9C; continue 'dispatch;
	}
	// 827F1BF0: 480001A8  b 0x827f1d98
	pc = 0x827F1D98; continue 'dispatch;
	// 827F1BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1BF8: 4860CBE9  bl 0x82dfe7e0
	ctx.lr = 0x827F1BFC;
	sub_82DFE7E0(ctx, base);
	// 827F1BFC: 83A10064  lwz r29, 0x64(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F1C00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F1C04: 4082005C  bne 0x827f1c60
	if !ctx.cr[0].eq {
	pc = 0x827F1C60; continue 'dispatch;
	}
	// 827F1C08: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 827F1C0C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F1C10: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 827F1C14: 419A0024  beq cr6, 0x827f1c38
	if ctx.cr[6].eq {
	pc = 0x827F1C38; continue 'dispatch;
	}
	// 827F1C18: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 827F1C1C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F1C20: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1C24: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F1C28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F1C2C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F1C30: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1C34: 4082FFE8  bne 0x827f1c1c
	if !ctx.cr[0].eq {
	pc = 0x827F1C1C; continue 'dispatch;
	}
	// 827F1C38: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F1C3C: 4BCF8E4D  bl 0x824eaa88
	ctx.lr = 0x827F1C40;
	sub_824EAA88(ctx, base);
	// 827F1C40: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F1C44: 38C00047  li r6, 0x47
	ctx.r[6].s64 = 71;
	// 827F1C48: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1C4C: 38AB6158  addi r5, r11, 0x6158
	ctx.r[5].s64 = ctx.r[11].s64 + 24920;
	// 827F1C50: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827F1C54: 483C852D  bl 0x82bba180
	ctx.lr = 0x827F1C58;
	sub_82BBA180(ctx, base);
	// 827F1C58: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F1C5C: 48600035  bl 0x82df1c90
	ctx.lr = 0x827F1C60;
	sub_82DF1C90(ctx, base);
	// 827F1C60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F1C64: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 827F1C68: 486A8CA1  bl 0x82e9a908
	ctx.lr = 0x827F1C6C;
	sub_82E9A908(ctx, base);
	// 827F1C6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F1C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1C74: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1C78: 488164B1  bl 0x83008128
	ctx.lr = 0x827F1C7C;
	sub_83008128(ctx, base);
	// 827F1C7C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 827F1C80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F1C84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F1C88: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 827F1C8C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 827F1C90: 4BDE7499  bl 0x825d9128
	ctx.lr = 0x827F1C94;
	sub_825D9128(ctx, base);
	// 827F1C94: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 827F1C98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1C9C: 419A0008  beq cr6, 0x827f1ca4
	if ctx.cr[6].eq {
	pc = 0x827F1CA4; continue 'dispatch;
	}
	// 827F1CA0: 4BACEBF1  bl 0x822c0890
	ctx.lr = 0x827F1CA4;
	sub_822C0890(ctx, base);
	// 827F1CA4: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 827F1CA8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827F1CAC: 419A00D0  beq cr6, 0x827f1d7c
	if ctx.cr[6].eq {
	pc = 0x827F1D7C; continue 'dispatch;
	}
	// 827F1CB0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F1CB4: 4BDE6B55  bl 0x825d8808
	ctx.lr = 0x827F1CB8;
	sub_825D8808(ctx, base);
	// 827F1CB8: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F1CBC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827F1CC0: 419A00AC  beq cr6, 0x827f1d6c
	if ctx.cr[6].eq {
	pc = 0x827F1D6C; continue 'dispatch;
	}
	// 827F1CC4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 827F1CC8: 4BDE7CD9  bl 0x825d99a0
	ctx.lr = 0x827F1CCC;
	sub_825D99A0(ctx, base);
	// 827F1CCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F1CD0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827F1CD4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F1CD8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1CDC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827F1CE0: 4BAD2781  bl 0x822c4460
	ctx.lr = 0x827F1CE4;
	sub_822C4460(ctx, base);
	// 827F1CE4: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 827F1CE8: 4800006C  b 0x827f1d54
	pc = 0x827F1D54; continue 'dispatch;
	// 827F1CEC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F1CF0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 827F1CF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827F1CFC: 419A0024  beq cr6, 0x827f1d20
	if ctx.cr[6].eq {
	pc = 0x827F1D20; continue 'dispatch;
	}
	// 827F1D00: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F1D04: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F1D08: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1D0C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F1D10: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F1D14: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F1D18: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1D1C: 4082FFE8  bne 0x827f1d04
	if !ctx.cr[0].eq {
	pc = 0x827F1D04; continue 'dispatch;
	}
	// 827F1D20: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827F1D24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F1D28: 4BFFFDC9  bl 0x827f1af0
	ctx.lr = 0x827F1D2C;
	sub_827F1AF0(ctx, base);
	// 827F1D2C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 827F1D30: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F1D34: 4BDE7D85  bl 0x825d9ab8
	ctx.lr = 0x827F1D38;
	sub_825D9AB8(ctx, base);
	// 827F1D38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F1D3C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 827F1D40: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 827F1D44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1D48: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827F1D4C: 4BAD2715  bl 0x822c4460
	ctx.lr = 0x827F1D50;
	sub_822C4460(ctx, base);
	// 827F1D50: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 827F1D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1D58: 419A0008  beq cr6, 0x827f1d60
	if ctx.cr[6].eq {
	pc = 0x827F1D60; continue 'dispatch;
	}
	// 827F1D5C: 4BACEB35  bl 0x822c0890
	ctx.lr = 0x827F1D60;
	sub_822C0890(ctx, base);
	// 827F1D60: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F1D64: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F1D68: 409AFF84  bne cr6, 0x827f1cec
	if !ctx.cr[6].eq {
	pc = 0x827F1CEC; continue 'dispatch;
	}
	// 827F1D6C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F1D70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1D74: 419A0008  beq cr6, 0x827f1d7c
	if ctx.cr[6].eq {
	pc = 0x827F1D7C; continue 'dispatch;
	}
	// 827F1D78: 4BACEB19  bl 0x822c0890
	ctx.lr = 0x827F1D7C;
	sub_822C0890(ctx, base);
	// 827F1D7C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 827F1D80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1D84: 419A0008  beq cr6, 0x827f1d8c
	if ctx.cr[6].eq {
	pc = 0x827F1D8C; continue 'dispatch;
	}
	// 827F1D88: 4BACEB09  bl 0x822c0890
	ctx.lr = 0x827F1D8C;
	sub_822C0890(ctx, base);
	// 827F1D8C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F1D90: 419A000C  beq cr6, 0x827f1d9c
	if ctx.cr[6].eq {
	pc = 0x827F1D9C; continue 'dispatch;
	}
	// 827F1D94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F1D98: 4BACEAF9  bl 0x822c0890
	ctx.lr = 0x827F1D9C;
	sub_822C0890(ctx, base);
	// 827F1D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F1DA0: 4BDE7211  bl 0x825d8fb0
	ctx.lr = 0x827F1DA4;
	sub_825D8FB0(ctx, base);
	// 827F1DA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 827F1DA8: 4BAD6F11  bl 0x822c8cb8
	ctx.lr = 0x827F1DAC;
	sub_822C8CB8(ctx, base);
	// 827F1DAC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 827F1DB0: 489B6408  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F1DB8 size=128
    let mut pc: u32 = 0x827F1DB8;
    'dispatch: loop {
        match pc {
            0x827F1DB8 => {
    //   block [0x827F1DB8..0x827F1E38)
	// 827F1DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1DBC: 489B63B1  bl 0x831a816c
	ctx.lr = 0x827F1DC0;
	sub_831A8130(ctx, base);
	// 827F1DC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1DC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1DC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F1DCC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F1DD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1DD4: 419A005C  beq cr6, 0x827f1e30
	if ctx.cr[6].eq {
	pc = 0x827F1E30; continue 'dispatch;
	}
	// 827F1DD8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1DDC: 48000048  b 0x827f1e24
	pc = 0x827F1E24; continue 'dispatch;
	// 827F1DE0: C1BD0000  lfs f13, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F1DE4: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F1DE8: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 827F1DEC: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F1DF0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1DF4: C1AB0018  lfs f13, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F1DF8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 827F1DFC: 4198000C  blt cr6, 0x827f1e08
	if ctx.cr[6].lt {
	pc = 0x827F1E08; continue 'dispatch;
	}
	// 827F1E00: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 827F1E04: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F1E08: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1E0C: C01F0024  lfs f0, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F1E10: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F1E14: C1A4001C  lfs f13, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F1E18: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827F1E1C: 4864CA1D  bl 0x82e3e838
	ctx.lr = 0x827F1E20;
	sub_82E3E838(ctx, base);
	// 827F1E20: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 827F1E24: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F1E28: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F1E2C: 409AFFB4  bne cr6, 0x827f1de0
	if !ctx.cr[6].eq {
	pc = 0x827F1DE0; continue 'dispatch;
	}
	// 827F1E30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F1E34: 489B6388  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1E38 size=144
    let mut pc: u32 = 0x827F1E38;
    'dispatch: loop {
        match pc {
            0x827F1E38 => {
    //   block [0x827F1E38..0x827F1EC8)
	// 827F1E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1E48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1E4C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F1E50: 396B6198  addi r11, r11, 0x6198
	ctx.r[11].s64 = ctx.r[11].s64 + 24984;
	// 827F1E54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F1E58: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F1E5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1E60: 419A003C  beq cr6, 0x827f1e9c
	if ctx.cr[6].eq {
	pc = 0x827F1E9C; continue 'dispatch;
	}
	// 827F1E64: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 827F1E68: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 827F1E6C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1E70: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 827F1E74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827F1E78: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F1E7C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1E80: 4082FFE8  bne 0x827f1e68
	if !ctx.cr[0].eq {
	pc = 0x827F1E68; continue 'dispatch;
	}
	// 827F1E84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F1E88: 409A0014  bne cr6, 0x827f1e9c
	if !ctx.cr[6].eq {
	pc = 0x827F1E9C; continue 'dispatch;
	}
	// 827F1E8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1E90: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F1E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F1E98: 4E800421  bctrl
	ctx.lr = 0x827F1E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F1E9C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F1EA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F1EA4: 419A0008  beq cr6, 0x827f1eac
	if ctx.cr[6].eq {
	pc = 0x827F1EAC; continue 'dispatch;
	}
	// 827F1EA8: 4BACE9E9  bl 0x822c0890
	ctx.lr = 0x827F1EAC;
	sub_822C0890(ctx, base);
	// 827F1EAC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F1EB0: 4BCBF3F1  bl 0x824b12a0
	ctx.lr = 0x827F1EB4;
	sub_824B12A0(ctx, base);
	// 827F1EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F1EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F1EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F1EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F1EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F1EC8 size=160
    let mut pc: u32 = 0x827F1EC8;
    'dispatch: loop {
        match pc {
            0x827F1EC8 => {
    //   block [0x827F1EC8..0x827F1F68)
	// 827F1EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1ED0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1ED4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1ED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1EDC: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F1EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F1EE4: 392A6198  addi r9, r10, 0x6198
	ctx.r[9].s64 = ctx.r[10].s64 + 24984;
	// 827F1EE8: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 827F1EEC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F1EF0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F1EF4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827F1EF8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 827F1EFC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 827F1F00: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 827F1F04: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 827F1F08: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1F0C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 827F1F10: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F1F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F1F18: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 827F1F1C: 419A0024  beq cr6, 0x827f1f40
	if ctx.cr[6].eq {
	pc = 0x827F1F40; continue 'dispatch;
	}
	// 827F1F20: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 827F1F24: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F1F28: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1F2C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F1F30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F1F34: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F1F38: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F1F3C: 4082FFE8  bne 0x827f1f24
	if !ctx.cr[0].eq {
	pc = 0x827F1F24; continue 'dispatch;
	}
	// 827F1F40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F1F44: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F1F48: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 827F1F4C: 4BAD4C5D  bl 0x822c6ba8
	ctx.lr = 0x827F1F50;
	sub_822C6BA8(ctx, base);
	// 827F1F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F1F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F1F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F1F60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F1F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1F68 size=76
    let mut pc: u32 = 0x827F1F68;
    'dispatch: loop {
        match pc {
            0x827F1F68 => {
    //   block [0x827F1F68..0x827F1FB4)
	// 827F1F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1F70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F1F74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1F78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1F7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1F80: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F1F84: 4BFFFEB5  bl 0x827f1e38
	ctx.lr = 0x827F1F88;
	sub_827F1E38(ctx, base);
	// 827F1F88: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F1F8C: 4182000C  beq 0x827f1f98
	if ctx.cr[0].eq {
	pc = 0x827F1F98; continue 'dispatch;
	}
	// 827F1F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1F94: 4BACE2D5  bl 0x822c0268
	ctx.lr = 0x827F1F98;
	sub_822C0268(ctx, base);
	// 827F1F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F1F9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F1FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F1FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F1FA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F1FAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F1FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F1FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F1FB8 size=152
    let mut pc: u32 = 0x827F1FB8;
    'dispatch: loop {
        match pc {
            0x827F1FB8 => {
    //   block [0x827F1FB8..0x827F2050)
	// 827F1FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F1FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F1FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F1FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F1FCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F1FD0: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F1FD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F1FD8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 827F1FDC: 4864DF2D  bl 0x82e3ff08
	ctx.lr = 0x827F1FE0;
	sub_82E3FF08(ctx, base);
	// 827F1FE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F1FE4: 486011CD  bl 0x82df31b0
	ctx.lr = 0x827F1FE8;
	sub_82DF31B0(ctx, base);
	// 827F1FE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F1FEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F1FF0: 48601A19  bl 0x82df3a08
	ctx.lr = 0x827F1FF4;
	sub_82DF3A08(ctx, base);
	// 827F1FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F1FF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F1FFC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F2000: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F2004: 4864EA65  bl 0x82e40a68
	ctx.lr = 0x827F2008;
	sub_82E40A68(ctx, base);
	// 827F2008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F200C: 4860141D  bl 0x82df3428
	ctx.lr = 0x827F2010;
	sub_82DF3428(ctx, base);
	// 827F2010: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F2014: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2018: 419A0010  beq cr6, 0x827f2028
	if ctx.cr[6].eq {
	pc = 0x827F2028; continue 'dispatch;
	}
	// 827F201C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 827F2020: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F2024: 483C10E5  bl 0x82bb3108
	ctx.lr = 0x827F2028;
	sub_82BB3108(ctx, base);
	// 827F2028: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F202C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F2030: 419A0008  beq cr6, 0x827f2038
	if ctx.cr[6].eq {
	pc = 0x827F2038; continue 'dispatch;
	}
	// 827F2034: 4BACE85D  bl 0x822c0890
	ctx.lr = 0x827F2038;
	sub_822C0890(ctx, base);
	// 827F2038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F203C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F2040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F2044: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F2048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F204C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2050 size=356
    let mut pc: u32 = 0x827F2050;
    'dispatch: loop {
        match pc {
            0x827F2050 => {
    //   block [0x827F2050..0x827F21B4)
	// 827F2050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2054: 489B6115  bl 0x831a8168
	ctx.lr = 0x827F2058;
	sub_831A8130(ctx, base);
	// 827F2058: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827F205C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2060: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F2064: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F2068: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827F206C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F2070: 388B619C  addi r4, r11, 0x619c
	ctx.r[4].s64 = ctx.r[11].s64 + 24988;
	// 827F2074: 38A00042  li r5, 0x42
	ctx.r[5].s64 = 66;
	// 827F2078: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 827F207C: 4860036D  bl 0x82df23e8
	ctx.lr = 0x827F2080;
	sub_82DF23E8(ctx, base);
	// 827F2080: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F2084: 41820010  beq 0x827f2094
	if ctx.cr[0].eq {
	pc = 0x827F2094; continue 'dispatch;
	}
	// 827F2088: 4BE0AEE9  bl 0x825fcf70
	ctx.lr = 0x827F208C;
	sub_825FCF70(ctx, base);
	// 827F208C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F2090: 48000008  b 0x827f2098
	pc = 0x827F2098; continue 'dispatch;
	// 827F2094: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F2098: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 827F209C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F20A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F20A4: 4BE08C55  bl 0x825facf8
	ctx.lr = 0x827F20A8;
	sub_825FACF8(ctx, base);
	// 827F20A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F20AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F20B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 827F20B4: 4BACDF4D  bl 0x822c0000
	ctx.lr = 0x827F20B8;
	sub_822C0000(ctx, base);
	// 827F20B8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F20BC: 3BFD0014  addi r31, r29, 0x14
	ctx.r[31].s64 = ctx.r[29].s64 + 20;
	// 827F20C0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 827F20C4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 827F20C8: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 827F20CC: 4BAD2395  bl 0x822c4460
	ctx.lr = 0x827F20D0;
	sub_822C4460(ctx, base);
	// 827F20D0: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F20D4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F20D8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F20DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F20E0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F20E4: 697E0001  xori r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u64 ^ 1;
	// 827F20E8: 419A0008  beq cr6, 0x827f20f0
	if ctx.cr[6].eq {
	pc = 0x827F20F0; continue 'dispatch;
	}
	// 827F20EC: 4BACE7A5  bl 0x822c0890
	ctx.lr = 0x827F20F0;
	sub_822C0890(ctx, base);
	// 827F20F0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F20F4: 418200B4  beq 0x827f21a8
	if ctx.cr[0].eq {
	pc = 0x827F21A8; continue 'dispatch;
	}
	// 827F20F8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 827F20FC: 419A00AC  beq cr6, 0x827f21a8
	if ctx.cr[6].eq {
	pc = 0x827F21A8; continue 'dispatch;
	}
	// 827F2100: 389D001C  addi r4, r29, 0x1c
	ctx.r[4].s64 = ctx.r[29].s64 + 28;
	// 827F2104: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F2108: 481D6651  bl 0x829c8758
	ctx.lr = 0x827F210C;
	sub_829C8758(ctx, base);
	// 827F210C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F2110: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F2114: 419A0044  beq cr6, 0x827f2158
	if ctx.cr[6].eq {
	pc = 0x827F2158; continue 'dispatch;
	}
	// 827F2118: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F211C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F2120: 48624721  bl 0x82e16840
	ctx.lr = 0x827F2124;
	sub_82E16840(ctx, base);
	// 827F2124: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F2128: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F212C: 48625235  bl 0x82e17360
	ctx.lr = 0x827F2130;
	sub_82E17360(ctx, base);
	// 827F2130: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F2134: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F2138: 388B51D0  addi r4, r11, 0x51d0
	ctx.r[4].s64 = ctx.r[11].s64 + 20944;
	// 827F213C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2140: 486073F1  bl 0x82df9530
	ctx.lr = 0x827F2144;
	sub_82DF9530(ctx, base);
	// 827F2144: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F2148: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 827F214C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F2150: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2154: 486255F5  bl 0x82e17748
	ctx.lr = 0x827F2158;
	sub_82E17748(ctx, base);
	// 827F2158: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F215C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F2160: 419A0008  beq cr6, 0x827f2168
	if ctx.cr[6].eq {
	pc = 0x827F2168; continue 'dispatch;
	}
	// 827F2164: 4BACE72D  bl 0x822c0890
	ctx.lr = 0x827F2168;
	sub_822C0890(ctx, base);
	// 827F2168: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F216C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F2170: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F2174: 419A0034  beq cr6, 0x827f21a8
	if ctx.cr[6].eq {
	pc = 0x827F21A8; continue 'dispatch;
	}
	// 827F2178: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F217C: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F2180: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2184: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827F2188: 419A0010  beq cr6, 0x827f2198
	if ctx.cr[6].eq {
	pc = 0x827F2198; continue 'dispatch;
	}
	// 827F218C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2190: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F2194: 4864C6A5  bl 0x82e3e838
	ctx.lr = 0x827F2198;
	sub_82E3E838(ctx, base);
	// 827F2198: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F219C: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 827F21A0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F21A4: 409AFFDC  bne cr6, 0x827f2180
	if !ctx.cr[6].eq {
	pc = 0x827F2180; continue 'dispatch;
	}
	// 827F21A8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 827F21AC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827F21B0: 489B6008  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F21B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F21B8 size=8
    let mut pc: u32 = 0x827F21B8;
    'dispatch: loop {
        match pc {
            0x827F21B8 => {
    //   block [0x827F21B8..0x827F21C0)
	// 827F21B8: C02300C0  lfs f1, 0xc0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F21BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F21C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F21C0 size=72
    let mut pc: u32 = 0x827F21C0;
    'dispatch: loop {
        match pc {
            0x827F21C0 => {
    //   block [0x827F21C0..0x827F2208)
	// 827F21C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F21C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F21C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F21CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F21D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F21D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F21D8: 48678AF1  bl 0x82e6acc8
	ctx.lr = 0x827F21DC;
	sub_82E6ACC8(ctx, base);
	// 827F21DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F21E0: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 827F21E4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F21E8: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 827F21EC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F21F0: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 827F21F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F21F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F21FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F2200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F2204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F2208 size=68
    let mut pc: u32 = 0x827F2208;
    'dispatch: loop {
        match pc {
            0x827F2208 => {
    //   block [0x827F2208..0x827F224C)
	// 827F2208: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F220C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 827F2210: 916300D0  stw r11, 0xd0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 827F2214: C00ACEE4  lfs f0, -0x311c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F2218: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F221C: 916300CC  stw r11, 0xcc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 827F2220: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F2224: D1A300E0  stfs f13, 0xe0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 827F2228: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F222C: D1A300DC  stfs f13, 0xdc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 827F2230: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F2234: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827F2238: D1A300D4  stfs f13, 0xd4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 827F223C: C1A4000C  lfs f13, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F2240: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 827F2244: D00300D8  stfs f0, 0xd8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 827F2248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F2250 size=8
    let mut pc: u32 = 0x827F2250;
    'dispatch: loop {
        match pc {
            0x827F2250 => {
    //   block [0x827F2250..0x827F2258)
	// 827F2250: 886300E8  lbz r3, 0xe8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(232 as u32) ) } as u64;
	// 827F2254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F2258 size=8
    let mut pc: u32 = 0x827F2258;
    'dispatch: loop {
        match pc {
            0x827F2258 => {
    //   block [0x827F2258..0x827F2260)
	// 827F2258: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 827F225C: 480000C4  b 0x827f2320
	sub_827F2320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2260 size=96
    let mut pc: u32 = 0x827F2260;
    'dispatch: loop {
        match pc {
            0x827F2260 => {
    //   block [0x827F2260..0x827F22C0)
	// 827F2260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F226C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F2274: 48678A35  bl 0x82e6aca8
	ctx.lr = 0x827F2278;
	sub_82E6ACA8(ctx, base);
	// 827F2278: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F227C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F2280: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 827F2284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827F2288: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 827F228C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2290: C00B621C  lfs f0, 0x621c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F2294: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 827F2298: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F229C: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 827F22A0: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 827F22A4: D1BF001C  stfs f13, 0x1c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 827F22A8: D1BF0020  stfs f13, 0x20(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 827F22AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F22B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F22B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F22B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F22BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F22C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F22C0 size=92
    let mut pc: u32 = 0x827F22C0;
    'dispatch: loop {
        match pc {
            0x827F22C0 => {
    //   block [0x827F22C0..0x827F231C)
	// 827F22C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F22C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F22C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F22CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F22D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F22D4: 4BD1EE1D  bl 0x825110f0
	ctx.lr = 0x827F22D8;
	sub_825110F0(ctx, base);
	// 827F22D8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F22DC: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F22E0: 396B61F4  addi r11, r11, 0x61f4
	ctx.r[11].s64 = ctx.r[11].s64 + 25076;
	// 827F22E4: 394A61E0  addi r10, r10, 0x61e0
	ctx.r[10].s64 = ctx.r[10].s64 + 25056;
	// 827F22E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F22EC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 827F22F0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F22F4: 4BFFFF6D  bl 0x827f2260
	ctx.lr = 0x827F22F8;
	sub_827F2260(ctx, base);
	// 827F22F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F22FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2300: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 827F2304: 997F00E8  stb r11, 0xe8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u8 ) };
	// 827F2308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F230C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F2310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F2314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F2318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F2320 size=100
    let mut pc: u32 = 0x827F2320;
    'dispatch: loop {
        match pc {
            0x827F2320 => {
    //   block [0x827F2320..0x827F2384)
	// 827F2320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F232C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F2330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F2338: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F233C: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F2340: 396B61F4  addi r11, r11, 0x61f4
	ctx.r[11].s64 = ctx.r[11].s64 + 25076;
	// 827F2344: 394A61E0  addi r10, r10, 0x61e0
	ctx.r[10].s64 = ctx.r[10].s64 + 25056;
	// 827F2348: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F234C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F2350: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 827F2354: 4BD1EE45  bl 0x82511198
	ctx.lr = 0x827F2358;
	sub_82511198(ctx, base);
	// 827F2358: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F235C: 4182000C  beq 0x827f2368
	if ctx.cr[0].eq {
	pc = 0x827F2368; continue 'dispatch;
	}
	// 827F2360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2364: 48600075  bl 0x82df23d8
	ctx.lr = 0x827F2368;
	sub_82DF23D8(ctx, base);
	// 827F2368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F236C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F2370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F2374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F2378: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F237C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F2380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2388 size=144
    let mut pc: u32 = 0x827F2388;
    'dispatch: loop {
        match pc {
            0x827F2388 => {
    //   block [0x827F2388..0x827F2418)
	// 827F2388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F238C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F2394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F2398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F239C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F23A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827F23A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F23A8: 486743B9  bl 0x82e66760
	ctx.lr = 0x827F23AC;
	sub_82E66760(ctx, base);
	// 827F23AC: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827F23B0: 4182004C  beq 0x827f23fc
	if ctx.cr[0].eq {
	pc = 0x827F23FC; continue 'dispatch;
	}
	// 827F23B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F23B8: 486927C1  bl 0x82e84b78
	ctx.lr = 0x827F23BC;
	sub_82E84B78(ctx, base);
	// 827F23BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F23C0: 41820034  beq 0x827f23f4
	if ctx.cr[0].eq {
	pc = 0x827F23F4; continue 'dispatch;
	}
	// 827F23C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F23C8: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F23CC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 827F23D0: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827F23D4: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F23D8: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F23DC: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 827F23E0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F23E4: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 827F23E8: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 827F23EC: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F2418 size=124
    let mut pc: u32 = 0x827F2418;
    'dispatch: loop {
        match pc {
            0x827F2418 => {
    //   block [0x827F2418..0x827F2494)
	// 827F2418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F2424: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2428: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F242C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F2430: 4BD1D099  bl 0x8250f4c8
	ctx.lr = 0x827F2434;
	sub_8250F4C8(ctx, base);
	// 827F2434: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F243C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 827F2440: 409A0008  bne cr6, 0x827f2448
	if !ctx.cr[6].eq {
	pc = 0x827F2448; continue 'dispatch;
	}
	// 827F2444: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F2448: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F244C: 4BD185BD  bl 0x8250aa08
	ctx.lr = 0x827F2450;
	sub_8250AA08(ctx, base);
	// 827F2450: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F2454: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F2458: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F245C: 485FF835  bl 0x82df1c90
	ctx.lr = 0x827F2460;
	sub_82DF1C90(ctx, base);
	// 827F2460: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F2464: 485FF82D  bl 0x82df1c90
	ctx.lr = 0x827F2468;
	sub_82DF1C90(ctx, base);
	// 827F2468: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F246C: 419A0010  beq cr6, 0x827f247c
	if ctx.cr[6].eq {
	pc = 0x827F247C; continue 'dispatch;
	}
	// 827F2470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2474: 48674505  bl 0x82e66978
	ctx.lr = 0x827F2478;
	sub_82E66978(ctx, base);
	// 827F2478: 48000008  b 0x827f2480
	pc = 0x827F2480; continue 'dispatch;
	// 827F247C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F2480: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F2484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F2488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F248C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F2490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2498 size=304
    let mut pc: u32 = 0x827F2498;
    'dispatch: loop {
        match pc {
            0x827F2498 => {
    //   block [0x827F2498..0x827F25C8)
	// 827F2498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F249C: 489B5CC5  bl 0x831a8160
	ctx.lr = 0x827F24A0;
	sub_831A8130(ctx, base);
	// 827F24A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F24A4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 827F24A8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 827F24AC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 827F24B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F24B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F24B8: 4BD1D011  bl 0x8250f4c8
	ctx.lr = 0x827F24BC;
	sub_8250F4C8(ctx, base);
	// 827F24BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F24C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F24C4: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 827F24C8: 409A0008  bne cr6, 0x827f24d0
	if !ctx.cr[6].eq {
	pc = 0x827F24D0; continue 'dispatch;
	}
	// 827F24CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F24D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F24D4: 4BD18535  bl 0x8250aa08
	ctx.lr = 0x827F24D8;
	sub_8250AA08(ctx, base);
	// 827F24D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F24DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F24E0: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F24E4: 485FF7AD  bl 0x82df1c90
	ctx.lr = 0x827F24E8;
	sub_82DF1C90(ctx, base);
	// 827F24E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F24EC: 485FF7A5  bl 0x82df1c90
	ctx.lr = 0x827F24F0;
	sub_82DF1C90(ctx, base);
	// 827F24F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F24F4: 419A00C8  beq cr6, 0x827f25bc
	if ctx.cr[6].eq {
	pc = 0x827F25BC; continue 'dispatch;
	}
	// 827F24F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F24FC: 4867445D  bl 0x82e66958
	ctx.lr = 0x827F2500;
	sub_82E66958(ctx, base);
	// 827F2500: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F2504: 4868606D  bl 0x82e78570
	ctx.lr = 0x827F2508;
	sub_82E78570(ctx, base);
	// 827F2508: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 827F250C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F2510: 615FFFFF  ori r31, r10, 0xffff
	ctx.r[31].u64 = ctx.r[10].u64 | 65535;
	// 827F2514: 3F808207  lis r28, -0x7df9
	ctx.r[28].s64 = -2113470464;
	// 827F2518: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F251C: 3BABC9A0  addi r29, r11, -0x3660
	ctx.r[29].s64 = ctx.r[11].s64 + -13920;
	// 827F2520: 4182002C  beq 0x827f254c
	if ctx.cr[0].eq {
	pc = 0x827F254C; continue 'dispatch;
	}
	// 827F2524: 13C0D8C7  vcmpequd (lvx128) v30, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F2528: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 827F252C: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F2530: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F25C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F25C8 size=196
    let mut pc: u32 = 0x827F25C8;
    'dispatch: loop {
        match pc {
            0x827F25C8 => {
    //   block [0x827F25C8..0x827F268C)
	// 827F25C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F25CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F25D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F25D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F25D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F25DC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 827F25E0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F25E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F25E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F25EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2690 size=196
    let mut pc: u32 = 0x827F2690;
    'dispatch: loop {
        match pc {
            0x827F2690 => {
    //   block [0x827F2690..0x827F2754)
	// 827F2690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F269C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F26A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F26A4: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 827F26A8: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F26AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F26B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F26B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F2758 size=16
    let mut pc: u32 = 0x827F2758;
    'dispatch: loop {
        match pc {
            0x827F2758 => {
    //   block [0x827F2758..0x827F2768)
	// 827F2758: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F275C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2760: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F2764: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F2768 size=60
    let mut pc: u32 = 0x827F2768;
    'dispatch: loop {
        match pc {
            0x827F2768 => {
    //   block [0x827F2768..0x827F27A4)
	// 827F2768: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 827F276C: 7F0B5840  cmplw cr6, r11, r11
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F2770: 419A002C  beq cr6, 0x827f279c
	if ctx.cr[6].eq {
	pc = 0x827F279C; continue 'dispatch;
	}
	// 827F2774: 7D0A5850  subf r8, r10, r11
	ctx.r[8].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 827F2778: 7CE85214  add r7, r8, r10
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 827F277C: 13E048C7  vcmpequd (lvx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F27A8 size=908
    let mut pc: u32 = 0x827F27A8;
    'dispatch: loop {
        match pc {
            0x827F27A8 => {
    //   block [0x827F27A8..0x827F2B34)
	// 827F27A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F27AC: 489B59B1  bl 0x831a815c
	ctx.lr = 0x827F27B0;
	sub_831A8130(ctx, base);
	// 827F27B0: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2B38 size=164
    let mut pc: u32 = 0x827F2B38;
    'dispatch: loop {
        match pc {
            0x827F2B38 => {
    //   block [0x827F2B38..0x827F2BDC)
	// 827F2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2B40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F2B44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F2B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2B4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F2B50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F2B54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2B58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2B5C: 409A000C  bne cr6, 0x827f2b68
	if !ctx.cr[6].eq {
	pc = 0x827F2B68; continue 'dispatch;
	}
	// 827F2B60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827F2B64: 48000010  b 0x827f2b74
	pc = 0x827F2B74; continue 'dispatch;
	// 827F2B68: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2B6C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F2B70: 7D642E70  srawi r4, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 827F2B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F2B78: 4BB08CE9  bl 0x822fb860
	ctx.lr = 0x827F2B7C;
	sub_822FB860(ctx, base);
	// 827F2B7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F2B80: 41820040  beq 0x827f2bc0
	if ctx.cr[0].eq {
	pc = 0x827F2BC0; continue 'dispatch;
	}
	// 827F2B84: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2B88: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2B8C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2B90: 48000024  b 0x827f2bb4
	pc = 0x827F2BB4; continue 'dispatch;
	// 827F2B94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2B98: 419A0014  beq cr6, 0x827f2bac
	if ctx.cr[6].eq {
	pc = 0x827F2BAC; continue 'dispatch;
	}
	// 827F2B9C: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2BE0 size=516
    let mut pc: u32 = 0x827F2BE0;
    'dispatch: loop {
        match pc {
            0x827F2BE0 => {
    //   block [0x827F2BE0..0x827F2DE4)
	// 827F2BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2BE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F2BEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F2BF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2BF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F2BF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F2BFC: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 827F2C00: 419A01C8  beq cr6, 0x827f2dc8
	if ctx.cr[6].eq {
	pc = 0x827F2DC8; continue 'dispatch;
	}
	// 827F2C04: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2C08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2C0C: 419A0014  beq cr6, 0x827f2c20
	if ctx.cr[6].eq {
	pc = 0x827F2C20; continue 'dispatch;
	}
	// 827F2C10: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2C14: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 827F2C18: 7D482E71  srawi. r8, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 827F2C1C: 40820010  bne 0x827f2c2c
	if !ctx.cr[0].eq {
	pc = 0x827F2C2C; continue 'dispatch;
	}
	// 827F2C20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F2C24: 4BFFFB35  bl 0x827f2758
	ctx.lr = 0x827F2C28;
	sub_827F2758(ctx, base);
	// 827F2C28: 480001A0  b 0x827f2dc8
	pc = 0x827F2DC8; continue 'dispatch;
	// 827F2C2C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2C30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827F2C34: 409A000C  bne cr6, 0x827f2c40
	if !ctx.cr[6].eq {
	pc = 0x827F2C40; continue 'dispatch;
	}
	// 827F2C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F2C3C: 48000010  b 0x827f2c4c
	pc = 0x827F2C4C; continue 'dispatch;
	// 827F2C40: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2C44: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 827F2C48: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 827F2C4C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F2C50: 4099002C  ble cr6, 0x827f2c7c
	if !ctx.cr[6].gt {
	pc = 0x827F2C7C; continue 'dispatch;
	}
	// 827F2C54: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 827F2C58: 409A0054  bne cr6, 0x827f2cac
	if !ctx.cr[6].eq {
	pc = 0x827F2CAC; continue 'dispatch;
	}
	// 827F2C5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F2C60: 48000058  b 0x827f2cb8
	pc = 0x827F2CB8; continue 'dispatch;
	// 827F2C64: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F2DE8 size=108
    let mut pc: u32 = 0x827F2DE8;
    'dispatch: loop {
        match pc {
            0x827F2DE8 => {
    //   block [0x827F2DE8..0x827F2E54)
	// 827F2DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2DEC: 489B5381  bl 0x831a816c
	ctx.lr = 0x827F2DF0;
	sub_831A8130(ctx, base);
	// 827F2DF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2DF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F2DF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F2DFC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 827F2E00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2E04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2E08: 419A0014  beq cr6, 0x827f2e1c
	if ctx.cr[6].eq {
	pc = 0x827F2E1C; continue 'dispatch;
	}
	// 827F2E0C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2E10: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F2E14: 7D4A2E71  srawi. r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827F2E18: 4082000C  bne 0x827f2e24
	if !ctx.cr[0].eq {
	pc = 0x827F2E24; continue 'dispatch;
	}
	// 827F2E1C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 827F2E20: 4800000C  b 0x827f2e2c
	pc = 0x827F2E2C; continue 'dispatch;
	// 827F2E24: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 827F2E28: 7D7D2E70  srawi r29, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 827F2E2C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 827F2E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2E34: 4BFFF975  bl 0x827f27a8
	ctx.lr = 0x827F2E38;
	sub_827F27A8(ctx, base);
	// 827F2E38: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2E3C: 57AB2834  slwi r11, r29, 5
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 827F2E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F2E44: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 827F2E48: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F2E4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F2E50: 489B536C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F2E58 size=148
    let mut pc: u32 = 0x827F2E58;
    'dispatch: loop {
        match pc {
            0x827F2E58 => {
    //   block [0x827F2E58..0x827F2EEC)
	// 827F2E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F2E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2E64: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2E68: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 827F2E6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2E70: 409A000C  bne cr6, 0x827f2e7c
	if !ctx.cr[6].eq {
	pc = 0x827F2E7C; continue 'dispatch;
	}
	// 827F2E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827F2E78: 48000010  b 0x827f2e88
	pc = 0x827F2E88; continue 'dispatch;
	// 827F2E7C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2E80: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F2E84: 7D4A2E70  srawi r10, r10, 5
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 5) as i64;
	// 827F2E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2E8C: 419A0040  beq cr6, 0x827f2ecc
	if ctx.cr[6].eq {
	pc = 0x827F2ECC; continue 'dispatch;
	}
	// 827F2E90: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F2E94: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 827F2E98: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 827F2E9C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 827F2EA0: 4098002C  bge cr6, 0x827f2ecc
	if !ctx.cr[6].lt {
	pc = 0x827F2ECC; continue 'dispatch;
	}
	// 827F2EA4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2EA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2EAC: 419A0014  beq cr6, 0x827f2ec0
	if ctx.cr[6].eq {
	pc = 0x827F2EC0; continue 'dispatch;
	}
	// 827F2EB0: 13E030C7  vcmpequd (lvx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F2EF0 size=168
    let mut pc: u32 = 0x827F2EF0;
    'dispatch: loop {
        match pc {
            0x827F2EF0 => {
    //   block [0x827F2EF0..0x827F2F98)
	// 827F2EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2EF4: 489B5275  bl 0x831a8168
	ctx.lr = 0x827F2EF8;
	sub_831A8130(ctx, base);
	// 827F2EF8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2EFC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F2F00: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F2F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2F08: 488D4BD9  bl 0x830c7ae0
	ctx.lr = 0x827F2F0C;
	sub_830C7AE0(ctx, base);
	// 827F2F0C: 83A3000C  lwz r29, 0xc(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F2F10: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 827F2F14: 419A0058  beq cr6, 0x827f2f6c
	if ctx.cr[6].eq {
	pc = 0x827F2F6C; continue 'dispatch;
	}
	// 827F2F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2F1C: 4867A73D  bl 0x82e6d658
	ctx.lr = 0x827F2F20;
	sub_82E6D658(ctx, base);
	// 827F2F20: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 827F2F24: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F2F28: 419A0044  beq cr6, 0x827f2f6c
	if ctx.cr[6].eq {
	pc = 0x827F2F6C; continue 'dispatch;
	}
	// 827F2F2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2F30: 486A1889  bl 0x82e947b8
	ctx.lr = 0x827F2F34;
	sub_82E947B8(ctx, base);
	// 827F2F34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F2F38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F2F3C: 486A187D  bl 0x82e947b8
	ctx.lr = 0x827F2F40;
	sub_82E947B8(ctx, base);
	// 827F2F40: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 827F2F44: 409A0028  bne cr6, 0x827f2f6c
	if !ctx.cr[6].eq {
	pc = 0x827F2F6C; continue 'dispatch;
	}
	// 827F2F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F2F4C: 48691C2D  bl 0x82e84b78
	ctx.lr = 0x827F2F50;
	sub_82E84B78(ctx, base);
	// 827F2F50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F2F54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F2F58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F2F5C: 48677D85  bl 0x82e6ace0
	ctx.lr = 0x827F2F60;
	sub_82E6ACE0(ctx, base);
	// 827F2F60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F2F64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F2F68: 4BFFFEF1  bl 0x827f2e58
	ctx.lr = 0x827F2F6C;
	sub_827F2E58(ctx, base);
	// 827F2F6C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F2F70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2F74: 419A0010  beq cr6, 0x827f2f84
	if ctx.cr[6].eq {
	pc = 0x827F2F84; continue 'dispatch;
	}
	// 827F2F78: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F2F7C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F2F80: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 827F2F84: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F2F88: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F2F8C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 827F2F90: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F2F94: 489B5224  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F2F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F2F98 size=224
    let mut pc: u32 = 0x827F2F98;
    'dispatch: loop {
        match pc {
            0x827F2F98 => {
    //   block [0x827F2F98..0x827F3078)
	// 827F2F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F2F9C: 489B51D1  bl 0x831a816c
	ctx.lr = 0x827F2FA0;
	sub_831A8130(ctx, base);
	// 827F2FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F2FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F2FA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F2FAC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F2FB0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F2FB4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 827F2FB8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 827F2FBC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 827F2FC0: 4BD1C509  bl 0x8250f4c8
	ctx.lr = 0x827F2FC4;
	sub_8250F4C8(ctx, base);
	// 827F2FC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2FC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F2FCC: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 827F2FD0: 409A0008  bne cr6, 0x827f2fd8
	if !ctx.cr[6].eq {
	pc = 0x827F2FD8; continue 'dispatch;
	}
	// 827F2FD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F2FD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F2FDC: 4BD17A2D  bl 0x8250aa08
	ctx.lr = 0x827F2FE0;
	sub_8250AA08(ctx, base);
	// 827F2FE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F2FE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F2FE8: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F2FEC: 485FECA5  bl 0x82df1c90
	ctx.lr = 0x827F2FF0;
	sub_82DF1C90(ctx, base);
	// 827F2FF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F2FF4: 485FEC9D  bl 0x82df1c90
	ctx.lr = 0x827F2FF8;
	sub_82DF1C90(ctx, base);
	// 827F2FF8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F2FFC: 419A0070  beq cr6, 0x827f306c
	if ctx.cr[6].eq {
	pc = 0x827F306C; continue 'dispatch;
	}
	// 827F3000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F3004: 48685585  bl 0x82e78588
	ctx.lr = 0x827F3008;
	sub_82E78588(ctx, base);
	// 827F3008: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F300C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F3010: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F3014: 4BFFFEDD  bl 0x827f2ef0
	ctx.lr = 0x827F3018;
	sub_827F2EF0(ctx, base);
	// 827F3018: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F301C: 4182000C  beq 0x827f3028
	if ctx.cr[0].eq {
	pc = 0x827F3028; continue 'dispatch;
	}
	// 827F3020: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827F3024: 997D00E8  stb r11, 0xe8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(232 as u32), ctx.r[11].u8 ) };
	// 827F3028: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F302C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F3030: 419A0014  beq cr6, 0x827f3044
	if ctx.cr[6].eq {
	pc = 0x827F3044; continue 'dispatch;
	}
	// 827F3034: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F3038: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F303C: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F3040: 4082002C  bne 0x827f306c
	if !ctx.cr[0].eq {
	pc = 0x827F306C; continue 'dispatch;
	}
	// 827F3044: 389D00C0  addi r4, r29, 0xc0
	ctx.r[4].s64 = ctx.r[29].s64 + 192;
	// 827F3048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F304C: 48673F8D  bl 0x82e66fd8
	ctx.lr = 0x827F3050;
	sub_82E66FD8(ctx, base);
	// 827F3050: 987D00E8  stb r3, 0xe8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(232 as u32), ctx.r[3].u8 ) };
	// 827F3054: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F3058: 486738F9  bl 0x82e66950
	ctx.lr = 0x827F305C;
	sub_82E66950(ctx, base);
	// 827F305C: 48685505  bl 0x82e78560
	ctx.lr = 0x827F3060;
	sub_82E78560(ctx, base);
	// 827F3060: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F3064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3068: 4BFFFB79  bl 0x827f2be0
	ctx.lr = 0x827F306C;
	sub_827F2BE0(ctx, base);
	// 827F306C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F3074: 489B5148  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3078 size=8
    let mut pc: u32 = 0x827F3078;
    'dispatch: loop {
        match pc {
            0x827F3078 => {
    //   block [0x827F3078..0x827F3080)
	// 827F3078: 88630024  lbz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 827F307C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3080 size=12
    let mut pc: u32 = 0x827F3080;
    'dispatch: loop {
        match pc {
            0x827F3080 => {
    //   block [0x827F3080..0x827F308C)
	// 827F3080: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F3084: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F3088: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F308C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F308C size=8
    let mut pc: u32 = 0x827F308C;
    'dispatch: loop {
        match pc {
            0x827F308C => {
    //   block [0x827F308C..0x827F3094)
	// 827F308C: 4BACD804  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 827F3090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F3098 size=204
    let mut pc: u32 = 0x827F3098;
    'dispatch: loop {
        match pc {
            0x827F3098 => {
    //   block [0x827F3098..0x827F3164)
	// 827F3098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F309C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F30A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F30A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F30A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F30AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F30B0: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 827F30B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F30B8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 827F30BC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F30C0: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F30C4: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 827F30C8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3168 size=196
    let mut pc: u32 = 0x827F3168;
    'dispatch: loop {
        match pc {
            0x827F3168 => {
    //   block [0x827F3168..0x827F322C)
	// 827F3168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F3170: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F3174: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F3178: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F317C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F3180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F3184: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 827F3188: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F318C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F3190: 4BACD7A9  bl 0x822c0938
	ctx.lr = 0x827F3194;
	sub_822C0938(ctx, base);
	// 827F3194: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F3198: 41820028  beq 0x827f31c0
	if ctx.cr[0].eq {
	pc = 0x827F31C0; continue 'dispatch;
	}
	// 827F319C: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F31A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 827F31A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827F31A8: 392B622C  addi r9, r11, 0x622c
	ctx.r[9].s64 = ctx.r[11].s64 + 25132;
	// 827F31AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827F31B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F31B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 827F31B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827F31BC: 48000008  b 0x827f31c4
	pc = 0x827F31C4; continue 'dispatch;
	// 827F31C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F31C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F31C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F31CC: 409A0044  bne cr6, 0x827f3210
	if !ctx.cr[6].eq {
	pc = 0x827F3210; continue 'dispatch;
	}
	// 827F31D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F31D4: 419A001C  beq cr6, 0x827f31f0
	if ctx.cr[6].eq {
	pc = 0x827F31F0; continue 'dispatch;
	}
	// 827F31D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F31DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 827F31E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F31E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F31E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F31EC: 4E800421  bctrl
	ctx.lr = 0x827F31F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F31F0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 827F31F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 827F31F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F31FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 827F3200: 816BC9D8  lwz r11, -0x3628(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13864 as u32) ) } as u64;
	// 827F3204: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827F3208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827F320C: 4BACCDF5  bl 0x822c0000
	ctx.lr = 0x827F3210;
	sub_822C0000(ctx, base);
	// 827F3210: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F3214: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F3218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F321C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F3220: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F3224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F3228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3230 size=124
    let mut pc: u32 = 0x827F3230;
    'dispatch: loop {
        match pc {
            0x827F3230 => {
    //   block [0x827F3230..0x827F32AC)
	// 827F3230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3234: 489B4F39  bl 0x831a816c
	ctx.lr = 0x827F3238;
	sub_831A8130(ctx, base);
	// 827F3238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F323C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 827F3240: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 827F3244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F3248: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 827F324C: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 827F3250: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 827F3254: 485FF195  bl 0x82df23e8
	ctx.lr = 0x827F3258;
	sub_82DF23E8(ctx, base);
	// 827F3258: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 827F325C: 4182001C  beq 0x827f3278
	if ctx.cr[0].eq {
	pc = 0x827F3278; continue 'dispatch;
	}
	// 827F3260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3264: 4866641D  bl 0x82e59680
	ctx.lr = 0x827F3268;
	sub_82E59680(ctx, base);
	// 827F3268: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F326C: 396B6224  addi r11, r11, 0x6224
	ctx.r[11].s64 = ctx.r[11].s64 + 25124;
	// 827F3270: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F3274: 48000008  b 0x827f327c
	pc = 0x827F327C; continue 'dispatch;
	// 827F3278: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F327C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F3280: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 827F3284: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F3288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F328C: 4BFFFEDD  bl 0x827f3168
	ctx.lr = 0x827F3290;
	sub_827F3168(ctx, base);
	// 827F3290: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 827F3294: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F3298: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F329C: 4BACCD65  bl 0x822c0000
	ctx.lr = 0x827F32A0;
	sub_822C0000(ctx, base);
	// 827F32A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F32A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F32A8: 489B4F14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F32B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F32B0 size=252
    let mut pc: u32 = 0x827F32B0;
    'dispatch: loop {
        match pc {
            0x827F32B0 => {
    //   block [0x827F32B0..0x827F33AC)
	// 827F32B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F32B4: 489B4EB5  bl 0x831a8168
	ctx.lr = 0x827F32B8;
	sub_831A8130(ctx, base);
	// 827F32B8: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827F32BC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F32C0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F32C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F32C8: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F32CC: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F32D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F32D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827F32D8: 419A00C8  beq cr6, 0x827f33a0
	if ctx.cr[6].eq {
	pc = 0x827F33A0; continue 'dispatch;
	}
	// 827F32DC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F32E0: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F32E4: 3BCA6240  addi r30, r10, 0x6240
	ctx.r[30].s64 = ctx.r[10].s64 + 25152;
	// 827F32E8: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 827F32EC: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 827F32F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F32F4: 419A0094  beq cr6, 0x827f3388
	if ctx.cr[6].eq {
	pc = 0x827F3388; continue 'dispatch;
	}
	// 827F32F8: 4BFE6CD9  bl 0x827d9fd0
	ctx.lr = 0x827F32FC;
	sub_827D9FD0(ctx, base);
	// 827F32FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F3300: 41820088  beq 0x827f3388
	if ctx.cr[0].eq {
	pc = 0x827F3388; continue 'dispatch;
	}
	// 827F3304: 48815CB5  bl 0x83008fb8
	ctx.lr = 0x827F3308;
	sub_83008FB8(ctx, base);
	// 827F3308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F330C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 827F3310: 4BFFFF21  bl 0x827f3230
	ctx.lr = 0x827F3314;
	sub_827F3230(ctx, base);
	// 827F3314: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F3318: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 827F331C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F3320: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F3324: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827F3328: 419A0024  beq cr6, 0x827f334c
	if ctx.cr[6].eq {
	pc = 0x827F334C; continue 'dispatch;
	}
	// 827F332C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 827F3330: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 827F3334: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F3338: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 827F333C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 827F3340: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 827F3344: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 827F3348: 4082FFE8  bne 0x827f3330
	if !ctx.cr[0].eq {
	pc = 0x827F3330; continue 'dispatch;
	}
	// 827F334C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F3350: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F3354: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 827F3358: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 827F335C: 38A00096  li r5, 0x96
	ctx.r[5].s64 = 150;
	// 827F3360: 387D0028  addi r3, r29, 0x28
	ctx.r[3].s64 = ctx.r[29].s64 + 40;
	// 827F3364: 486656DD  bl 0x82e58a40
	ctx.lr = 0x827F3368;
	sub_82E58A40(ctx, base);
	// 827F3368: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F336C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F3370: 419A0008  beq cr6, 0x827f3378
	if ctx.cr[6].eq {
	pc = 0x827F3378; continue 'dispatch;
	}
	// 827F3374: 4BACD51D  bl 0x822c0890
	ctx.lr = 0x827F3378;
	sub_822C0890(ctx, base);
	// 827F3378: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 827F337C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F3380: 419A0008  beq cr6, 0x827f3388
	if ctx.cr[6].eq {
	pc = 0x827F3388; continue 'dispatch;
	}
	// 827F3384: 4BACD50D  bl 0x822c0890
	ctx.lr = 0x827F3388;
	sub_822C0890(ctx, base);
	// 827F3388: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F338C: 4BCF5E6D  bl 0x824e91f8
	ctx.lr = 0x827F3390;
	sub_824E91F8(ctx, base);
	// 827F3390: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F3394: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827F3398: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 827F339C: 409AFF50  bne cr6, 0x827f32ec
	if !ctx.cr[6].eq {
	pc = 0x827F32EC; continue 'dispatch;
	}
	// 827F33A0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 827F33A4: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 827F33A8: 489B4E10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F33B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F33B0 size=464
    let mut pc: u32 = 0x827F33B0;
    'dispatch: loop {
        match pc {
            0x827F33B0 => {
    //   block [0x827F33B0..0x827F3580)
	// 827F33B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F33B4: 489B4DB1  bl 0x831a8164
	ctx.lr = 0x827F33B8;
	sub_831A8130(ctx, base);
	// 827F33B8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F33BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F33C0: FDA00850  fneg f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 827F33C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F33C8: D0210060  stfs f1, 0x60(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 827F33CC: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 827F33D0: D0210080  stfs f1, 0x80(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 827F33D4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827F33D8: D0210084  stfs f1, 0x84(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 827F33DC: 394A6910  addi r10, r10, 0x6910
	ctx.r[10].s64 = ctx.r[10].s64 + 26896;
	// 827F33E0: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 827F33E4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 827F33E8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 827F33EC: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 827F33F0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 827F33F4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F33F8: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 827F33FC: C00808A4  lfs f0, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F3400: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 827F3404: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 827F3408: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 827F340C: D1A10090  stfs f13, 0x90(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 827F3410: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 827F3414: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 827F3418: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 827F341C: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 827F3420: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 827F3424: D0010088  stfs f0, 0x88(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 827F3428: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 827F342C: D0210094  stfs f1, 0x94(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 827F3430: D0010098  stfs f0, 0x98(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 827F3434: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 827F3438: D02100A0  stfs f1, 0xa0(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 827F343C: D1A100A4  stfs f13, 0xa4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 827F3440: D02100A8  stfs f1, 0xa8(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 827F3444: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 827F3448: D1A100B0  stfs f13, 0xb0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 827F344C: D1A100B4  stfs f13, 0xb4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 827F3450: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3580 size=136
    let mut pc: u32 = 0x827F3580;
    'dispatch: loop {
        match pc {
            0x827F3580 => {
    //   block [0x827F3580..0x827F3608)
	// 827F3580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3584: 489B4BE9  bl 0x831a816c
	ctx.lr = 0x827F3588;
	sub_831A8130(ctx, base);
	// 827F3588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F358C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3590: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F3594: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F3598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F359C: 419A0064  beq cr6, 0x827f3600
	if ctx.cr[6].eq {
	pc = 0x827F3600; continue 'dispatch;
	}
	// 827F35A0: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 827F35A4: 48815A15  bl 0x83008fb8
	ctx.lr = 0x827F35A8;
	sub_83008FB8(ctx, base);
	// 827F35A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F35AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F35B0: 4BAFCC29  bl 0x822f01d8
	ctx.lr = 0x827F35B4;
	sub_822F01D8(ctx, base);
	// 827F35B4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F35B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F35BC: 419A002C  beq cr6, 0x827f35e8
	if ctx.cr[6].eq {
	pc = 0x827F35E8; continue 'dispatch;
	}
	// 827F35C0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 827F35C4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827F35C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827F35CC: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 827F35D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F35D4: 4BC9E98D  bl 0x82491f60
	ctx.lr = 0x827F35D8;
	sub_82491F60(ctx, base);
	// 827F35D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 827F35DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F35E0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F35E4: 4BFFFCCD  bl 0x827f32b0
	ctx.lr = 0x827F35E8;
	sub_827F32B0(ctx, base);
	// 827F35E8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F35EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F35F0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F35F4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F35F8: 997F0024  stb r11, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u8 ) };
	// 827F35FC: 4BAFD41D  bl 0x822f0a18
	ctx.lr = 0x827F3600;
	sub_822F0A18(ctx, base);
	// 827F3600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F3604: 489B4BB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3608 size=20
    let mut pc: u32 = 0x827F3608;
    'dispatch: loop {
        match pc {
            0x827F3608 => {
    //   block [0x827F3608..0x827F361C)
	// 827F3608: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F360C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F3610: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F3614: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 827F3618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3620 size=36
    let mut pc: u32 = 0x827F3620;
    'dispatch: loop {
        match pc {
            0x827F3620 => {
    //   block [0x827F3620..0x827F3644)
	// 827F3620: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F3624: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F3628: 419A0010  beq cr6, 0x827f3638
	if ctx.cr[6].eq {
	pc = 0x827F3638; continue 'dispatch;
	}
	// 827F362C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 827F3630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827F3634: 409A0008  bne cr6, 0x827f363c
	if !ctx.cr[6].eq {
	pc = 0x827F363C; continue 'dispatch;
	}
	// 827F3638: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827F363C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 827F3640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3648 size=20
    let mut pc: u32 = 0x827F3648;
    'dispatch: loop {
        match pc {
            0x827F3648 => {
    //   block [0x827F3648..0x827F365C)
	// 827F3648: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F364C: 396BFFFD  addi r11, r11, -3
	ctx.r[11].s64 = ctx.r[11].s64 + -3;
	// 827F3650: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F3654: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F3658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3660 size=20
    let mut pc: u32 = 0x827F3660;
    'dispatch: loop {
        match pc {
            0x827F3660 => {
    //   block [0x827F3660..0x827F3674)
	// 827F3660: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F3664: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 827F3668: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 827F366C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F3670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3678 size=8
    let mut pc: u32 = 0x827F3678;
    'dispatch: loop {
        match pc {
            0x827F3678 => {
    //   block [0x827F3678..0x827F3680)
	// 827F3678: 38630050  addi r3, r3, 0x50
	ctx.r[3].s64 = ctx.r[3].s64 + 80;
	// 827F367C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F3680 size=8
    let mut pc: u32 = 0x827F3680;
    'dispatch: loop {
        match pc {
            0x827F3680 => {
    //   block [0x827F3680..0x827F3688)
	// 827F3680: C0230070  lfs f1, 0x70(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F3684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3688 size=16
    let mut pc: u32 = 0x827F3688;
    'dispatch: loop {
        match pc {
            0x827F3688 => {
    //   block [0x827F3688..0x827F3698)
	// 827F3688: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F368C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 827F3690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F3694: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3698 size=8
    let mut pc: u32 = 0x827F3698;
    'dispatch: loop {
        match pc {
            0x827F3698 => {
    //   block [0x827F3698..0x827F36A0)
	// 827F3698: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 827F369C: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F36A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F36A0 size=16
    let mut pc: u32 = 0x827F36A0;
    'dispatch: loop {
        match pc {
            0x827F36A0 => {
    //   block [0x827F36A0..0x827F36B0)
	// 827F36A0: 38830050  addi r4, r3, 0x50
	ctx.r[4].s64 = ctx.r[3].s64 + 80;
	// 827F36A4: C02A0000  lfs f1, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F36A8: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F36AC: 4BFFFED4  b 0x827f3580
	sub_827F3580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F36B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F36B0 size=4
    let mut pc: u32 = 0x827F36B0;
    'dispatch: loop {
        match pc {
            0x827F36B0 => {
    //   block [0x827F36B0..0x827F36B4)
	// 827F36B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F36B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F36B8 size=72
    let mut pc: u32 = 0x827F36B8;
    'dispatch: loop {
        match pc {
            0x827F36B8 => {
    //   block [0x827F36B8..0x827F3700)
	// 827F36B8: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 827F36BC: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F36C0: 3941FFE0  addi r10, r1, -0x20
	ctx.r[10].s64 = ctx.r[1].s64 + -32;
	// 827F36C4: 8923001C  lbz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 827F36C8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 827F36CC: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F36D0: 13C358C7  vcmpequd (lvx128) v30, v3, v11
	tmp.u32 = ctx.r[3].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F36D4: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F3700 size=188
    let mut pc: u32 = 0x827F3700;
    'dispatch: loop {
        match pc {
            0x827F3700 => {
    //   block [0x827F3700..0x827F37BC)
	// 827F3700: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F3704: 40820010  bne 0x827f3714
	if !ctx.cr[0].eq {
	pc = 0x827F3714; continue 'dispatch;
	}
	// 827F3708: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
	// 827F370C: D1A1FFE4  stfs f13, -0x1c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 827F3710: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F3714: 13C028C7  vcmpequd (lvx128) v30, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F37C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F37C0 size=56
    let mut pc: u32 = 0x827F37C0;
    'dispatch: loop {
        match pc {
            0x827F37C0 => {
    //   block [0x827F37C0..0x827F37F8)
	// 827F37C0: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 827F37C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F37C8: 419A0030  beq cr6, 0x827f37f8
	if ctx.cr[6].eq {
		sub_827F37F8(ctx, base);
		return;
	}
	// 827F37CC: 81430040  lwz r10, 0x40(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F37D0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F37D4: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F37D8: 41820020  beq 0x827f37f8
	if ctx.cr[0].eq {
		sub_827F37F8(ctx, base);
		return;
	}
	// 827F37DC: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 827F37E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F37E4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F37F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F37F8 size=8
    let mut pc: u32 = 0x827F37F8;
    'dispatch: loop {
        match pc {
            0x827F37F8 => {
    //   block [0x827F37F8..0x827F3800)
	// 827F37F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F37FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F3800 size=944
    let mut pc: u32 = 0x827F3800;
    'dispatch: loop {
        match pc {
            0x827F3800 => {
    //   block [0x827F3800..0x827F3BB0)
	// 827F3800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3804: 489B495D  bl 0x831a8160
	ctx.lr = 0x827F3808;
	sub_831A8130(ctx, base);
	// 827F3808: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 827F380C: 489B5269  bl 0x831a8a74
	ctx.lr = 0x827F3810;
	sub_831A8A40(ctx, base);
	// 827F3810: 3980FF70  li r12, -0x90
	ctx.r[12].s64 = -144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3BB0 size=120
    let mut pc: u32 = 0x827F3BB0;
    'dispatch: loop {
        match pc {
            0x827F3BB0 => {
    //   block [0x827F3BB0..0x827F3C28)
	// 827F3BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F3BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F3BBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3BC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F3BC4: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 827F3BC8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 827F3BCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F3BD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F3BD4: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 827F3BD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F3BDC: 4E800421  bctrl
	ctx.lr = 0x827F3BE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F3BE0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F3BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F3BE8: 409A000C  bne cr6, 0x827f3bf4
	if !ctx.cr[6].eq {
	pc = 0x827F3BF4; continue 'dispatch;
	}
	// 827F3BEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F3BF0: 48000010  b 0x827f3c00
	pc = 0x827F3C00; continue 'dispatch;
	// 827F3BF4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F3BF8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F3BFC: 7D7F2E70  srawi r31, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 827F3C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F3C04: 4BC763FD  bl 0x8246a000
	ctx.lr = 0x827F3C08;
	sub_8246A000(ctx, base);
	// 827F3C08: 7FEB0034  cntlzw r11, r31
	ctx.r[11].u64 = if ctx.r[31].u32 == 0 { 32 } else { ctx.r[31].u32.leading_zeros() as u64 };
	// 827F3C0C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 827F3C10: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 827F3C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F3C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F3C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F3C20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F3C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F3C28 size=284
    let mut pc: u32 = 0x827F3C28;
    'dispatch: loop {
        match pc {
            0x827F3C28 => {
    //   block [0x827F3C28..0x827F3D44)
	// 827F3C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3C2C: 489B453D  bl 0x831a8168
	ctx.lr = 0x827F3C30;
	sub_831A8130(ctx, base);
	// 827F3C30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3C34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F3C38: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F3C3C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 827F3C40: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 827F3C44: 4867789D  bl 0x82e6b4e0
	ctx.lr = 0x827F3C48;
	sub_82E6B4E0(ctx, base);
	// 827F3C48: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 827F3C4C: 393D0038  addi r9, r29, 0x38
	ctx.r[9].s64 = ctx.r[29].s64 + 56;
	// 827F3C50: C00BE830  lfs f0, -0x17d0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F3C54: 81690004  lwz r11, 4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F3C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F3C5C: 419A00DC  beq cr6, 0x827f3d38
	if ctx.cr[6].eq {
	pc = 0x827F3D38; continue 'dispatch;
	}
	// 827F3C60: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F3C64: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F3C68: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F3C6C: 418200CC  beq 0x827f3d38
	if ctx.cr[0].eq {
	pc = 0x827F3D38; continue 'dispatch;
	}
	// 827F3C70: 57EB063F  clrlwi. r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F3C74: 418200C4  beq 0x827f3d38
	if ctx.cr[0].eq {
	pc = 0x827F3D38; continue 'dispatch;
	}
	// 827F3C78: 895D001C  lbz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 827F3C7C: 817D003C  lwz r11, 0x3c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 827F3C80: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F3C84: 40820018  bne 0x827f3c9c
	if !ctx.cr[0].eq {
	pc = 0x827F3C9C; continue 'dispatch;
	}
	// 827F3C88: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F3C8C: 554807BD  rlwinm. r8, r10, 0, 0x1e, 0x1e
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 827F3C90: 40820018  bne 0x827f3ca8
	if !ctx.cr[0].eq {
	pc = 0x827F3CA8; continue 'dispatch;
	}
	// 827F3C94: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827F3C98: 4800000C  b 0x827f3ca4
	pc = 0x827F3CA4; continue 'dispatch;
	// 827F3C9C: 815D0060  lwz r10, 0x60(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(96 as u32) ) } as u64;
	// 827F3CA0: 554A077B  rlwinm. r10, r10, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827F3CA4: 41820008  beq 0x827f3cac
	if ctx.cr[0].eq {
	pc = 0x827F3CAC; continue 'dispatch;
	}
	// 827F3CA8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 827F3CAC: 57EA063F  clrlwi. r10, r31, 0x18
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827F3CB0: 41820040  beq 0x827f3cf0
	if ctx.cr[0].eq {
	pc = 0x827F3CF0; continue 'dispatch;
	}
	// 827F3CB4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F3CB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 827F3CBC: 409A0034  bne cr6, 0x827f3cf0
	if !ctx.cr[6].eq {
	pc = 0x827F3CF0; continue 'dispatch;
	}
	// 827F3CC0: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F3CC4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 827F3CC8: 13C0E0C7  vcmpequd (lvx128) v30, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3D48 size=176
    let mut pc: u32 = 0x827F3D48;
    'dispatch: loop {
        match pc {
            0x827F3D48 => {
    //   block [0x827F3D48..0x827F3DF8)
	// 827F3D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F3D50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F3D54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F3D58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3D5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3D60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F3D64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F3D68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827F3D6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F3D70: 4BFFFEB9  bl 0x827f3c28
	ctx.lr = 0x827F3D74;
	sub_827F3C28(ctx, base);
	// 827F3D74: 38BF0050  addi r5, r31, 0x50
	ctx.r[5].s64 = ctx.r[31].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F3DF8 size=100
    let mut pc: u32 = 0x827F3DF8;
    'dispatch: loop {
        match pc {
            0x827F3DF8 => {
    //   block [0x827F3DF8..0x827F3E5C)
	// 827F3DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F3E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F3E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F3E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3E10: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F3E14: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F3E18: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 827F3E1C: 4BFFE93D  bl 0x827f2758
	ctx.lr = 0x827F3E20;
	sub_827F2758(ctx, base);
	// 827F3E20: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F3E24: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 827F3E28: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827F3E2C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F3E30: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F3E34: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3E60 size=96
    let mut pc: u32 = 0x827F3E60;
    'dispatch: loop {
        match pc {
            0x827F3E60 => {
    //   block [0x827F3E60..0x827F3EC0)
	// 827F3E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F3E68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F3E6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3E74: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F3E78: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F3E7C: 396B6294  addi r11, r11, 0x6294
	ctx.r[11].s64 = ctx.r[11].s64 + 25236;
	// 827F3E80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F3E84: 4BC7617D  bl 0x8246a000
	ctx.lr = 0x827F3E88;
	sub_8246A000(ctx, base);
	// 827F3E88: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 827F3E8C: 4BDCC215  bl 0x825c00a0
	ctx.lr = 0x827F3E90;
	sub_825C00A0(ctx, base);
	// 827F3E90: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F3E94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 827F3E98: 419A0014  beq cr6, 0x827f3eac
	if ctx.cr[6].eq {
	pc = 0x827F3EAC; continue 'dispatch;
	}
	// 827F3E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3EA0: 4BFFF1E1  bl 0x827f3080
	ctx.lr = 0x827F3EA4;
	sub_827F3080(ctx, base);
	// 827F3EA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3EA8: 4BACC3C1  bl 0x822c0268
	ctx.lr = 0x827F3EAC;
	sub_822C0268(ctx, base);
	// 827F3EAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F3EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F3EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F3EB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F3EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3EC0 size=76
    let mut pc: u32 = 0x827F3EC0;
    'dispatch: loop {
        match pc {
            0x827F3EC0 => {
    //   block [0x827F3EC0..0x827F3F0C)
	// 827F3EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F3EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F3ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F3ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3ED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3ED8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F3EDC: 4BFFFF85  bl 0x827f3e60
	ctx.lr = 0x827F3EE0;
	sub_827F3E60(ctx, base);
	// 827F3EE0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F3EE4: 4182000C  beq 0x827f3ef0
	if ctx.cr[0].eq {
	pc = 0x827F3EF0; continue 'dispatch;
	}
	// 827F3EE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3EEC: 4BACC37D  bl 0x822c0268
	ctx.lr = 0x827F3EF0;
	sub_822C0268(ctx, base);
	// 827F3EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3EF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F3EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F3EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F3F00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F3F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F3F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F3F10 size=160
    let mut pc: u32 = 0x827F3F10;
    'dispatch: loop {
        match pc {
            0x827F3F10 => {
    //   block [0x827F3F10..0x827F3FB0)
	// 827F3F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3F14: 489B4259  bl 0x831a816c
	ctx.lr = 0x827F3F18;
	sub_831A8130(ctx, base);
	// 827F3F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3F1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3F20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F3F24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827F3F28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F3F2C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F3F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F3F34: 4E800421  bctrl
	ctx.lr = 0x827F3F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F3F38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F3F3C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 827F3F40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 827F3F44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F3F48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F3F4C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 827F3F50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F3F54: 4E800421  bctrl
	ctx.lr = 0x827F3F58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F3F58: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 827F3F5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F3F60: 419A0048  beq cr6, 0x827f3fa8
	if ctx.cr[6].eq {
	pc = 0x827F3FA8; continue 'dispatch;
	}
	// 827F3F64: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 827F3F68: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F3F6C: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F3F70: 41820038  beq 0x827f3fa8
	if ctx.cr[0].eq {
	pc = 0x827F3FA8; continue 'dispatch;
	}
	// 827F3F74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F3F78: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 827F3F7C: 4BFFEC65  bl 0x827f2be0
	ctx.lr = 0x827F3F80;
	sub_827F2BE0(ctx, base);
	// 827F3F80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F3F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F3F88: 4BFFFDC1  bl 0x827f3d48
	ctx.lr = 0x827F3F8C;
	sub_827F3D48(ctx, base);
	// 827F3F8C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 827F3F90: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 827F3F94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F3F98: 4BC76069  bl 0x8246a000
	ctx.lr = 0x827F3F9C;
	sub_8246A000(ctx, base);
	// 827F3F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F3FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F3FA4: 489B4218  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 827F3FA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F3FAC: 4BFFFFE8  b 0x827f3f94
	pc = 0x827F3F94; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F3FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F3FB0 size=232
    let mut pc: u32 = 0x827F3FB0;
    'dispatch: loop {
        match pc {
            0x827F3FB0 => {
    //   block [0x827F3FB0..0x827F4098)
	// 827F3FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F3FB4: 489B41B5  bl 0x831a8168
	ctx.lr = 0x827F3FB8;
	sub_831A8130(ctx, base);
	// 827F3FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F3FBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F3FC0: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F3FC4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 827F3FC8: 396B6294  addi r11, r11, 0x6294
	ctx.r[11].s64 = ctx.r[11].s64 + 25236;
	// 827F3FCC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 827F3FD0: 90BF0014  stw r5, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 827F3FD4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 827F3FD8: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 827F3FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827F3FE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F3FE4: 388A6910  addi r4, r10, 0x6910
	ctx.r[4].s64 = ctx.r[10].s64 + 26896;
	// 827F3FE8: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 827F3FEC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 827F3FF0: 98DF001C  stb r6, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[6].u8 ) };
	// 827F3FF4: 3BBF0038  addi r29, r31, 0x38
	ctx.r[29].s64 = ctx.r[31].s64 + 56;
	// 827F3FF8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 827F3FFC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 827F4000: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 827F4004: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 827F4008: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 827F400C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 827F4010: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 827F4014: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 827F4018: 486774E9  bl 0x82e6b500
	ctx.lr = 0x827F401C;
	sub_82E6B500(ctx, base);
	// 827F401C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F4020: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F4024: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F4028: D01F0070  stfs f0, 0x70(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 827F402C: 4BFFE72D  bl 0x827f2758
	ctx.lr = 0x827F4030;
	sub_827F2758(ctx, base);
	// 827F4030: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F4034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827F4038: 388B62D0  addi r4, r11, 0x62d0
	ctx.r[4].s64 = ctx.r[11].s64 + 25296;
	// 827F403C: 38A00042  li r5, 0x42
	ctx.r[5].s64 = 66;
	// 827F4040: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 827F4044: 4BACC395  bl 0x822c03d8
	ctx.lr = 0x827F4048;
	sub_822C03D8(ctx, base);
	// 827F4048: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 827F404C: 4182001C  beq 0x827f4068
	if ctx.cr[0].eq {
	pc = 0x827F4068; continue 'dispatch;
	}
	// 827F4050: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F4054: 4BFFE165  bl 0x827f21b8
	ctx.lr = 0x827F4058;
	sub_827F21B8(ctx, base);
	// 827F4058: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 827F405C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 827F4060: 4BFFF351  bl 0x827f33b0
	ctx.lr = 0x827F4064;
	sub_827F33B0(ctx, base);
	// 827F4064: 48000008  b 0x827f406c
	pc = 0x827F406C; continue 'dispatch;
	// 827F4068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F406C: 83DF0018  lwz r30, 0x18(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F4070: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 827F4074: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 827F4078: 419A0014  beq cr6, 0x827f408c
	if ctx.cr[6].eq {
	pc = 0x827F408C; continue 'dispatch;
	}
	// 827F407C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F4080: 4BFFF001  bl 0x827f3080
	ctx.lr = 0x827F4084;
	sub_827F3080(ctx, base);
	// 827F4084: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F4088: 4BACC1E1  bl 0x822c0268
	ctx.lr = 0x827F408C;
	sub_822C0268(ctx, base);
	// 827F408C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F4090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827F4094: 489B4124  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F4098 size=608
    let mut pc: u32 = 0x827F4098;
    'dispatch: loop {
        match pc {
            0x827F4098 => {
    //   block [0x827F4098..0x827F42F8)
	// 827F4098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F409C: 489B40D1  bl 0x831a816c
	ctx.lr = 0x827F40A0;
	sub_831A8130(ctx, base);
	// 827F40A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F40A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F40A8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 827F40AC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F40B0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F40B4: 419A00E0  beq cr6, 0x827f4194
	if ctx.cr[6].eq {
	pc = 0x827F4194; continue 'dispatch;
	}
	// 827F40B8: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 827F40BC: 409A0234  bne cr6, 0x827f42f0
	if !ctx.cr[6].eq {
	pc = 0x827F42F0; continue 'dispatch;
	}
	// 827F40C0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F40C4: 4BFFEFB5  bl 0x827f3078
	ctx.lr = 0x827F40C8;
	sub_827F3078(ctx, base);
	// 827F40C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F40CC: 41820010  beq 0x827f40dc
	if ctx.cr[0].eq {
	pc = 0x827F40DC; continue 'dispatch;
	}
	// 827F40D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 827F40D4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 827F40D8: 48000218  b 0x827f42f0
	pc = 0x827F42F0; continue 'dispatch;
	// 827F40DC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 827F40E0: 3BDF0038  addi r30, r31, 0x38
	ctx.r[30].s64 = ctx.r[31].s64 + 56;
	// 827F40E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F40E8: 419A0014  beq cr6, 0x827f40fc
	if ctx.cr[6].eq {
	pc = 0x827F40FC; continue 'dispatch;
	}
	// 827F40EC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F40F0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F40F4: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F40F8: 40820010  bne 0x827f4108
	if !ctx.cr[0].eq {
	pc = 0x827F4108; continue 'dispatch;
	}
	// 827F40FC: 39600050  li r11, 0x50
	ctx.r[11].s64 = 80;
	// 827F4100: 13FF58C7  vcmpequd (lvx128) v31, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F4104: 48000010  b 0x827f4114
	pc = 0x827F4114; continue 'dispatch;
	// 827F4108: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F410C: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 827F4110: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F4114: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 827F4118: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F411C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 827F4120: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 827F4124: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 827F4128: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827F412C: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F42F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F42F8 size=116
    let mut pc: u32 = 0x827F42F8;
    'dispatch: loop {
        match pc {
            0x827F42F8 => {
    //   block [0x827F42F8..0x827F436C)
	// 827F42F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F42FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F4300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F4304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F4308: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F430C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 827F4310: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F4314: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 827F4318: 13C030C7  vcmpequd (lvx128) v30, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F431C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F4320: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 827F4324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F4370 size=108
    let mut pc: u32 = 0x827F4370;
    'dispatch: loop {
        match pc {
            0x827F4370 => {
    //   block [0x827F4370..0x827F43DC)
	// 827F4370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F4374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F4378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 827F437C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F4380: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 827F4384: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4388: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 827F438C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F4390: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 827F4394: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 827F4398: 4BFF9979  bl 0x827edd10
	ctx.lr = 0x827F439C;
	sub_827EDD10(ctx, base);
	// 827F439C: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F43A0: C1830000  lfs f12, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 827F43A4: C1BE0034  lfs f13, 0x34(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F43A8: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 827F43AC: 4098000C  bge cr6, 0x827f43b8
	if !ctx.cr[6].lt {
	pc = 0x827F43B8; continue 'dispatch;
	}
	// 827F43B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F43B4: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F43B8: EC0D07FA  fmadds f0, f13, f31, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 827F43BC: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827F43C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 827F43C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F43C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F43CC: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 827F43D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 827F43D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F43D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F43E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F43E0 size=8
    let mut pc: u32 = 0x827F43E0;
    'dispatch: loop {
        match pc {
            0x827F43E0 => {
    //   block [0x827F43E0..0x827F43E8)
	// 827F43E0: 38630020  addi r3, r3, 0x20
	ctx.r[3].s64 = ctx.r[3].s64 + 32;
	// 827F43E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F43E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F43E8 size=8
    let mut pc: u32 = 0x827F43E8;
    'dispatch: loop {
        match pc {
            0x827F43E8 => {
    //   block [0x827F43E8..0x827F43F0)
	// 827F43E8: C0230030  lfs f1, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F43EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F43F0 size=96
    let mut pc: u32 = 0x827F43F0;
    'dispatch: loop {
        match pc {
            0x827F43F0 => {
    //   block [0x827F43F0..0x827F4450)
	// 827F43F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F43F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F43F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F43FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4400: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F4404: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F4408: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 827F440C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F4410: 4E800421  bctrl
	ctx.lr = 0x827F4414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F4414: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F4418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F441C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 827F4420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F4424: 4E800421  bctrl
	ctx.lr = 0x827F4428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F4428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F442C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F4430: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 827F4434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F4438: 4E800421  bctrl
	ctx.lr = 0x827F443C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F443C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F4440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F4444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F4448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F444C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F4450 size=100
    let mut pc: u32 = 0x827F4450;
    'dispatch: loop {
        match pc {
            0x827F4450 => {
    //   block [0x827F4450..0x827F44B4)
	// 827F4450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F4454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F4458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F445C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F4464: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F4468: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F446C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F4470: 4E800421  bctrl
	ctx.lr = 0x827F4474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F4474: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F4478: 41820024  beq 0x827f449c
	if ctx.cr[0].eq {
	pc = 0x827F449C; continue 'dispatch;
	}
	// 827F447C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F4480: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F4484: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 827F4488: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F448C: 4E800421  bctrl
	ctx.lr = 0x827F4490;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F4490: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F4494: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F4498: 40820008  bne 0x827f44a0
	if !ctx.cr[0].eq {
	pc = 0x827F44A0; continue 'dispatch;
	}
	// 827F449C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F44A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F44A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F44A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F44AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F44B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F44B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F44B8 size=28
    let mut pc: u32 = 0x827F44B8;
    'dispatch: loop {
        match pc {
            0x827F44B8 => {
    //   block [0x827F44B8..0x827F44D4)
	// 827F44B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F44BC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F44C0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 827F44C4: 419A0008  beq cr6, 0x827f44cc
	if ctx.cr[6].eq {
	pc = 0x827F44CC; continue 'dispatch;
	}
	// 827F44C8: EC020824  fdivs f0, f2, f1
	ctx.f[0].f64 = ((ctx.f[2].f64 / ctx.f[1].f64) as f32) as f64;
	// 827F44CC: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 827F44D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F44D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F44D8 size=16
    let mut pc: u32 = 0x827F44D8;
    'dispatch: loop {
        match pc {
            0x827F44D8 => {
    //   block [0x827F44D8..0x827F44E8)
	// 827F44D8: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F44DC: 396B6314  addi r11, r11, 0x6314
	ctx.r[11].s64 = ctx.r[11].s64 + 25364;
	// 827F44E0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F44E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F44E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F44E8 size=92
    let mut pc: u32 = 0x827F44E8;
    'dispatch: loop {
        match pc {
            0x827F44E8 => {
    //   block [0x827F44E8..0x827F4544)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F4544 size=8
    let mut pc: u32 = 0x827F4544;
    'dispatch: loop {
        match pc {
            0x827F4544 => {
    //   block [0x827F4544..0x827F454C)
	// 827F4544: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F4548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F4550 size=24
    let mut pc: u32 = 0x827F4550;
    'dispatch: loop {
        match pc {
            0x827F4550 => {
    //   block [0x827F4550..0x827F4568)
	// 827F4550: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F4554: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 827F4558: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827F455C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F4568 size=36
    let mut pc: u32 = 0x827F4568;
    'dispatch: loop {
        match pc {
            0x827F4568 => {
    //   block [0x827F4568..0x827F458C)
	// 827F4568: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F456C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F4570: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 827F4574: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F4578: D1A30008  stfs f13, 8(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 827F457C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 827F4580: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 827F4584: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 827F4588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F4590 size=64
    let mut pc: u32 = 0x827F4590;
    'dispatch: loop {
        match pc {
            0x827F4590 => {
    //   block [0x827F4590..0x827F45D0)
	// 827F4590: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F4594: 3D408207  lis r10, -0x7df9
	ctx.r[10].s64 = -2113470464;
	// 827F4598: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 827F459C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 827F45A0: 394A6314  addi r10, r10, 0x6314
	ctx.r[10].s64 = ctx.r[10].s64 + 25364;
	// 827F45A4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 827F45A8: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 827F45AC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827F45B0: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F45D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F45D0 size=68
    let mut pc: u32 = 0x827F45D0;
    'dispatch: loop {
        match pc {
            0x827F45D0 => {
    //   block [0x827F45D0..0x827F4614)
	// 827F45D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F45D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F45D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F45DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F45E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F45E4: 3D608207  lis r11, -0x7df9
	ctx.r[11].s64 = -2113470464;
	// 827F45E8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 827F45EC: 396B6314  addi r11, r11, 0x6314
	ctx.r[11].s64 = ctx.r[11].s64 + 25364;
	// 827F45F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 827F45F4: 41820008  beq 0x827f45fc
	if ctx.cr[0].eq {
	pc = 0x827F45FC; continue 'dispatch;
	}
	// 827F45F8: 485FDDE1  bl 0x82df23d8
	ctx.lr = 0x827F45FC;
	sub_82DF23D8(ctx, base);
	// 827F45FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F4600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F4604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F4608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F460C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F4610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F4618 size=472
    let mut pc: u32 = 0x827F4618;
    'dispatch: loop {
        match pc {
            0x827F4618 => {
    //   block [0x827F4618..0x827F47F0)
	// 827F4618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F461C: 489B3B51  bl 0x831a816c
	ctx.lr = 0x827F4620;
	sub_831A8130(ctx, base);
	// 827F4620: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 827F4624: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4628: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 827F462C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F4630: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F4634: D3E100E4  stfs f31, 0xe4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 827F4638: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 827F463C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F4640: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 827F4644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F4648: 4E800421  bctrl
	ctx.lr = 0x827F464C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F464C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F4650: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 827F4654: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F4658: 3BAB6910  addi r29, r11, 0x6910
	ctx.r[29].s64 = ctx.r[11].s64 + 26896;
	// 827F465C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 827F4660: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 827F4664: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 827F4668: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F47F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F47F0 size=340
    let mut pc: u32 = 0x827F47F0;
    'dispatch: loop {
        match pc {
            0x827F47F0 => {
    //   block [0x827F47F0..0x827F4944)
	// 827F47F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F47F4: 489B3975  bl 0x831a8168
	ctx.lr = 0x827F47F8;
	sub_831A8130(ctx, base);
	// 827F47F8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 827F47FC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 827F4800: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4804: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F4808: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 827F480C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F4810: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 827F4814: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 827F4818: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F481C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F4820: 4E800421  bctrl
	ctx.lr = 0x827F4824;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F4824: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 827F4828: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 827F482C: 4868766D  bl 0x82e7be98
	ctx.lr = 0x827F4830;
	sub_82E7BE98(ctx, base);
	// 827F4830: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 827F4834: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 827F4838: C1BF0030  lfs f13, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 827F483C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 827F4840: EFCD07F2  fmuls f30, f13, f31
	ctx.f[30].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 827F4844: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 827F4848: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 827F484C: 3BAB6910  addi r29, r11, 0x6910
	ctx.r[29].s64 = ctx.r[11].s64 + 26896;
	// 827F4850: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F4854: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F4948 size=76
    let mut pc: u32 = 0x827F4948;
    'dispatch: loop {
        match pc {
            0x827F4948 => {
    //   block [0x827F4948..0x827F4994)
	// 827F4948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F494C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F4950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 827F4954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4958: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 827F495C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F4960: 4BFFAE39  bl 0x827ef798
	ctx.lr = 0x827F4964;
	sub_827EF798(ctx, base);
	// 827F4964: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 827F4968: 4182000C  beq 0x827f4974
	if ctx.cr[0].eq {
	pc = 0x827F4974; continue 'dispatch;
	}
	// 827F496C: 4BFFD84D  bl 0x827f21b8
	ctx.lr = 0x827F4970;
	sub_827F21B8(ctx, base);
	// 827F4970: 48000010  b 0x827f4980
	pc = 0x827F4980; continue 'dispatch;
	// 827F4974: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 827F4978: 4BFF93B1  bl 0x827edd28
	ctx.lr = 0x827F497C;
	sub_827EDD28(ctx, base);
	// 827F497C: C0230008  lfs f1, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F4980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F4984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F4988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F498C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 827F4990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F4998 size=20
    let mut pc: u32 = 0x827F4998;
    'dispatch: loop {
        match pc {
            0x827F4998 => {
    //   block [0x827F4998..0x827F49AC)
	// 827F4998: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 827F499C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 827F49A0: 806B0040  lwz r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 827F49A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 827F49A8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F49AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F49AC size=12
    let mut pc: u32 = 0x827F49AC;
    'dispatch: loop {
        match pc {
            0x827F49AC => {
    //   block [0x827F49AC..0x827F49B8)
	// 827F49AC: C02A0000  lfs f1, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 827F49B0: 388B0070  addi r4, r11, 0x70
	ctx.r[4].s64 = ctx.r[11].s64 + 112;
	// 827F49B4: 4BFFEBCC  b 0x827f3580
	sub_827F3580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F49B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F49B8 size=4
    let mut pc: u32 = 0x827F49B8;
    'dispatch: loop {
        match pc {
            0x827F49B8 => {
    //   block [0x827F49B8..0x827F49BC)
	// 827F49B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F49C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F49C0 size=92
    let mut pc: u32 = 0x827F49C0;
    'dispatch: loop {
        match pc {
            0x827F49C0 => {
    //   block [0x827F49C0..0x827F4A1C)
	// 827F49C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F49C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F49C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F49CC: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 827F49D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 827F49D4: 419A0014  beq cr6, 0x827f49e8
	if ctx.cr[6].eq {
	pc = 0x827F49E8; continue 'dispatch;
	}
	// 827F49D8: 81430060  lwz r10, 0x60(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 827F49DC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F49E0: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F49E4: 40820024  bne 0x827f4a08
	if !ctx.cr[0].eq {
	pc = 0x827F4A08; continue 'dispatch;
	}
	// 827F49E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 827F49EC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 827F49F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 827F49F4: 4E800421  bctrl
	ctx.lr = 0x827F49F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 827F49F8: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 827F49FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 827F4A00: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 827F4A04: 419A0008  beq cr6, 0x827f4a0c
	if ctx.cr[6].eq {
	pc = 0x827F4A0C; continue 'dispatch;
	}
	// 827F4A08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 827F4A0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 827F4A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827F4A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827F4A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827F4A20 size=132
    let mut pc: u32 = 0x827F4A20;
    'dispatch: loop {
        match pc {
            0x827F4A20 => {
    //   block [0x827F4A20..0x827F4AA4)
	// 827F4A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F4A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827F4A28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4A2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x827F4AA8 size=600
    let mut pc: u32 = 0x827F4AA8;
    'dispatch: loop {
        match pc {
            0x827F4AA8 => {
    //   block [0x827F4AA8..0x827F4D00)
	// 827F4AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F4AAC: 489B36BD  bl 0x831a8168
	ctx.lr = 0x827F4AB0;
	sub_831A8130(ctx, base);
	// 827F4AB0: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 827F4AB4: 489B3FC5  bl 0x831a8a78
	ctx.lr = 0x827F4AB8;
	sub_831A8A40(ctx, base);
	// 827F4AB8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827F4ABC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 827F4AC0: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 827F4AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 827F4AC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 827F4ACC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 827F4AD0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 827F4AD4: 486873C5  bl 0x82e7be98
	ctx.lr = 0x827F4AD8;
	sub_82E7BE98(ctx, base);
	// 827F4AD8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 827F4ADC: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 827F4AE0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 827F4AE4: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 827F4AE8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 827F4AEC: 13FE58C7  vcmpequd (lvx128) v31, v30, v11
	tmp.u32 = ctx.r[30].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F4D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F4D00 size=800
    let mut pc: u32 = 0x827F4D00;
    'dispatch: loop {
        match pc {
            0x827F4D00 => {
    //   block [0x827F4D00..0x827F5020)
	// 827F4D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827F4D04: 489B3445  bl 0x831a8148
	ctx.lr = 0x827F4D08;
	sub_831A8130(ctx, base);
	// 827F4D08: DBA1FF80  stfd f29, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[29].u64 ) };
	// 827F4D0C: DBC1FF88  stfd f30, -0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[30].u64 ) };
	// 827F4D10: DBE1FF90  stfd f31, -0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[31].u64 ) };
	// 827F4D14: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x827F5020 size=40
    let mut pc: u32 = 0x827F5020;
    'dispatch: loop {
        match pc {
            0x827F5020 => {
    //   block [0x827F5020..0x827F5048)
	// 827F5020: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 827F5024: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827F5048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827F5048 size=16
    let mut pc: u32 = 0x827F5048;
    'dispatch: loop {
        match pc {
            0x827F5048 => {
    //   block [0x827F5048..0x827F5058)
	// 827F5048: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 827F504C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 827F5050: 7D6B2E71  srawi. r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 827F5054: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


