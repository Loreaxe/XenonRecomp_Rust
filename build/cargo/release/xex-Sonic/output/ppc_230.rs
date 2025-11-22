pub fn sub_82FDF7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF7D8 size=500
    let mut pc: u32 = 0x82FDF7D8;
    'dispatch: loop {
        match pc {
            0x82FDF7D8 => {
    //   block [0x82FDF7D8..0x82FDF9CC)
	// 82FDF7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF7DC: 481C8991  bl 0x831a816c
	ctx.lr = 0x82FDF7E0;
	sub_831A8130(ctx, base);
	// 82FDF7E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF7E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FDF7E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDF7EC: 409A000C  bne cr6, 0x82fdf7f8
	if !ctx.cr[6].eq {
	pc = 0x82FDF7F8; continue 'dispatch;
	}
	// 82FDF7F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDF7F4: 480001D0  b 0x82fdf9c4
	pc = 0x82FDF9C4; continue 'dispatch;
	// 82FDF7F8: 3963FFFC  addi r11, r3, -4
	ctx.r[11].s64 = ctx.r[3].s64 + -4;
	// 82FDF7FC: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 82FDF800: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDF804: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF808: 4182000C  beq 0x82fdf814
	if ctx.cr[0].eq {
	pc = 0x82FDF814; continue 'dispatch;
	}
	// 82FDF80C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FDF810: 480001B4  b 0x82fdf9c4
	pc = 0x82FDF9C4; continue 'dispatch;
	// 82FDF814: 3BC3FFFC  addi r30, r3, -4
	ctx.r[30].s64 = ctx.r[3].s64 + -4;
	// 82FDF818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF81C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF820: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDF824: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF828: 4E800421  bctrl
	ctx.lr = 0x82FDF82C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF82C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF830: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FDF834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF838: 7D5D0734  extsh r29, r10
	ctx.r[29].s64 = ctx.r[10].s16 as i64;
	// 82FDF83C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDF840: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF844: 4E800421  bctrl
	ctx.lr = 0x82FDF848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF848: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FDF84C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82FDF850: 409AFFA0  bne cr6, 0x82fdf7f0
	if !ctx.cr[6].eq {
	pc = 0x82FDF7F0; continue 'dispatch;
	}
	// 82FDF854: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF85C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF864: 4E800421  bctrl
	ctx.lr = 0x82FDF868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF868: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF86C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDF870: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF874: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDF878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF87C: 4E800421  bctrl
	ctx.lr = 0x82FDF880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF880: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDF884: 4BFF43BD  bl 0x82fd3c40
	ctx.lr = 0x82FDF888;
	sub_82FD3C40(ctx, base);
	// 82FDF888: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF88C: 4182FF64  beq 0x82fdf7f0
	if ctx.cr[0].eq {
	pc = 0x82FDF7F0; continue 'dispatch;
	}
	// 82FDF890: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF898: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FDF89C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF8A0: 4E800421  bctrl
	ctx.lr = 0x82FDF8A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF8A4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF8A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDF8AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF8B0: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FDF8B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF8B8: 4E800421  bctrl
	ctx.lr = 0x82FDF8BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF8BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDF8C0: 4BFF4381  bl 0x82fd3c40
	ctx.lr = 0x82FDF8C4;
	sub_82FD3C40(ctx, base);
	// 82FDF8C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF8C8: 4182FF28  beq 0x82fdf7f0
	if ctx.cr[0].eq {
	pc = 0x82FDF7F0; continue 'dispatch;
	}
	// 82FDF8CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF8D4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FDF8D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF8DC: 4E800421  bctrl
	ctx.lr = 0x82FDF8E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF8E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF8E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDF8E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF8EC: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FDF8F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF8F4: 4E800421  bctrl
	ctx.lr = 0x82FDF8F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF8F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDF8FC: 4BFF4345  bl 0x82fd3c40
	ctx.lr = 0x82FDF900;
	sub_82FD3C40(ctx, base);
	// 82FDF900: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF904: 4182FEEC  beq 0x82fdf7f0
	if ctx.cr[0].eq {
	pc = 0x82FDF7F0; continue 'dispatch;
	}
	// 82FDF908: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF90C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF910: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FDF914: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF918: 4E800421  bctrl
	ctx.lr = 0x82FDF91C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF91C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF920: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDF924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF928: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FDF92C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF930: 4E800421  bctrl
	ctx.lr = 0x82FDF934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF934: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDF938: 4BFF4309  bl 0x82fd3c40
	ctx.lr = 0x82FDF93C;
	sub_82FD3C40(ctx, base);
	// 82FDF93C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF940: 4182FEB0  beq 0x82fdf7f0
	if ctx.cr[0].eq {
	pc = 0x82FDF7F0; continue 'dispatch;
	}
	// 82FDF944: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF94C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF954: 4E800421  bctrl
	ctx.lr = 0x82FDF958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF958: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF95C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FDF960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF964: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDF968: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF96C: 4E800421  bctrl
	ctx.lr = 0x82FDF970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF970: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDF974: 4BFF42CD  bl 0x82fd3c40
	ctx.lr = 0x82FDF978;
	sub_82FD3C40(ctx, base);
	// 82FDF978: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDF97C: 4182FE74  beq 0x82fdf7f0
	if ctx.cr[0].eq {
	pc = 0x82FDF7F0; continue 'dispatch;
	}
	// 82FDF980: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF988: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FDF98C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF990: 4E800421  bctrl
	ctx.lr = 0x82FDF994;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF994: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDF99C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDF9A0: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FDF9A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF9A8: 4E800421  bctrl
	ctx.lr = 0x82FDF9AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDF9AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDF9B0: 4BFF4291  bl 0x82fd3c40
	ctx.lr = 0x82FDF9B4;
	sub_82FD3C40(ctx, base);
	// 82FDF9B4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82FDF9B8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FDF9BC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FDF9C0: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FDF9C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FDF9C8: 481C87F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDF9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDF9D0 size=112
    let mut pc: u32 = 0x82FDF9D0;
    'dispatch: loop {
        match pc {
            0x82FDF9D0 => {
    //   block [0x82FDF9D0..0x82FDFA40)
	// 82FDF9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDF9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDF9D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FDF9DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDF9E0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF9E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDF9E8: 4800002C  b 0x82fdfa14
	pc = 0x82FDFA14; continue 'dispatch;
	// 82FDF9EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDF9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDF9F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDF9F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDF9FC: 4E800421  bctrl
	ctx.lr = 0x82FDFA00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFA00: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FDFA04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFA08: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FDFA0C: 419A0020  beq cr6, 0x82fdfa2c
	if ctx.cr[6].eq {
	pc = 0x82FDFA2C; continue 'dispatch;
	}
	// 82FDFA10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFA14: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDFA18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFA1C: 4E800421  bctrl
	ctx.lr = 0x82FDFA20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFA20: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FDFA24: 4082FFC8  bne 0x82fdf9ec
	if !ctx.cr[0].eq {
	pc = 0x82FDF9EC; continue 'dispatch;
	}
	// 82FDFA28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDFA2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDFA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDFA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDFA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FDFA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDFA40 size=624
    let mut pc: u32 = 0x82FDFA40;
    'dispatch: loop {
        match pc {
            0x82FDFA40 => {
    //   block [0x82FDFA40..0x82FDFCB0)
	// 82FDFA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDFA44: 481C8701  bl 0x831a8144
	ctx.lr = 0x82FDFA48;
	sub_831A8130(ctx, base);
	// 82FDFA48: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDFA4C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDFA50: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82FDFA54: 3AAB8018  addi r21, r11, -0x7fe8
	ctx.r[21].s64 = ctx.r[11].s64 + -32744;
	// 82FDFA58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDFA5C: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 82FDFA60: 7CB32B78  mr r19, r5
	ctx.r[19].u64 = ctx.r[5].u64;
	// 82FDFA64: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82FDFA68: 3A8B8034  addi r20, r11, -0x7fcc
	ctx.r[20].s64 = ctx.r[11].s64 + -32716;
	// 82FDFA6C: 3B57FFFC  addi r26, r23, -4
	ctx.r[26].s64 = ctx.r[23].s64 + -4;
	// 82FDFA70: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDFA74: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFA78: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FDFA7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFA80: 4E800421  bctrl
	ctx.lr = 0x82FDFA84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFA84: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFA88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDFA8C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDFA90: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FDFA94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFA98: 4E800421  bctrl
	ctx.lr = 0x82FDFA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFA9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDFAA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDFAA4: 419A0058  beq cr6, 0x82fdfafc
	if ctx.cr[6].eq {
	pc = 0x82FDFAFC; continue 'dispatch;
	}
	// 82FDFAA8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FDFAAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFAB0: 4BFF4191  bl 0x82fd3c40
	ctx.lr = 0x82FDFAB4;
	sub_82FD3C40(ctx, base);
	// 82FDFAB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFAB8: 41820044  beq 0x82fdfafc
	if ctx.cr[0].eq {
	pc = 0x82FDFAFC; continue 'dispatch;
	}
	// 82FDFABC: 566B063F  clrlwi. r11, r19, 0x18
	ctx.r[11].u64 = ctx.r[19].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFAC0: 4082000C  bne 0x82fdfacc
	if !ctx.cr[0].eq {
	pc = 0x82FDFACC; continue 'dispatch;
	}
	// 82FDFAC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDFAC8: 419A0034  beq cr6, 0x82fdfafc
	if ctx.cr[6].eq {
	pc = 0x82FDFAFC; continue 'dispatch;
	}
	// 82FDFACC: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFAD0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FDFAD4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDFAD8: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FDFADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFAE0: 4E800421  bctrl
	ctx.lr = 0x82FDFAE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFAE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFAE8: 41820014  beq 0x82fdfafc
	if ctx.cr[0].eq {
	pc = 0x82FDFAFC; continue 'dispatch;
	}
	// 82FDFAEC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FDFAF0: 4BFF4151  bl 0x82fd3c40
	ctx.lr = 0x82FDFAF4;
	sub_82FD3C40(ctx, base);
	// 82FDFAF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFAF8: 4082019C  bne 0x82fdfc94
	if !ctx.cr[0].eq {
	pc = 0x82FDFC94; continue 'dispatch;
	}
	// 82FDFAFC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB00: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDFB04: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FDFB08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFB0C: 4E800421  bctrl
	ctx.lr = 0x82FDFB10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFB10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFB14: 41820164  beq 0x82fdfc78
	if ctx.cr[0].eq {
	pc = 0x82FDFC78; continue 'dispatch;
	}
	// 82FDFB18: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB1C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDFB20: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDFB24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFB28: 4E800421  bctrl
	ctx.lr = 0x82FDFB2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFB2C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FDFB30: 41820148  beq 0x82fdfc78
	if ctx.cr[0].eq {
	pc = 0x82FDFC78; continue 'dispatch;
	}
	// 82FDFB34: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFB3C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDFB40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFB44: 4E800421  bctrl
	ctx.lr = 0x82FDFB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFB48: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FDFB4C: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82FDFB50: 40810128  ble 0x82fdfc78
	if !ctx.cr[0].gt {
	pc = 0x82FDFC78; continue 'dispatch;
	}
	// 82FDFB54: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB58: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDFB5C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFB60: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDFB64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFB68: 4E800421  bctrl
	ctx.lr = 0x82FDFB6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFB6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDFB70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB74: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FDFB78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFB7C: 4E800421  bctrl
	ctx.lr = 0x82FDFB80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFB80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FDFB88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFB8C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDFB90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFB94: 4E800421  bctrl
	ctx.lr = 0x82FDFB98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFB98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFB9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDFBA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFBA4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FDFBA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFBAC: 4E800421  bctrl
	ctx.lr = 0x82FDFBB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFBB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFBB4: 418200B8  beq 0x82fdfc6c
	if ctx.cr[0].eq {
	pc = 0x82FDFC6C; continue 'dispatch;
	}
	// 82FDFBB8: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82FDFBBC: 4BFF4085  bl 0x82fd3c40
	ctx.lr = 0x82FDFBC0;
	sub_82FD3C40(ctx, base);
	// 82FDFBC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFBC4: 418200A8  beq 0x82fdfc6c
	if ctx.cr[0].eq {
	pc = 0x82FDFC6C; continue 'dispatch;
	}
	// 82FDFBC8: 566B063F  clrlwi. r11, r19, 0x18
	ctx.r[11].u64 = ctx.r[19].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFBCC: 41820028  beq 0x82fdfbf4
	if ctx.cr[0].eq {
	pc = 0x82FDFBF4; continue 'dispatch;
	}
	// 82FDFBD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFBD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFBD8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDFBDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFBE0: 4E800421  bctrl
	ctx.lr = 0x82FDFBE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFBE4: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FDFBE8: 4BFF4059  bl 0x82fd3c40
	ctx.lr = 0x82FDFBEC;
	sub_82FD3C40(ctx, base);
	// 82FDFBEC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFBF0: 40820034  bne 0x82fdfc24
	if !ctx.cr[0].eq {
	pc = 0x82FDFC24; continue 'dispatch;
	}
	// 82FDFBF4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FDFBF8: 419A0074  beq cr6, 0x82fdfc6c
	if ctx.cr[6].eq {
	pc = 0x82FDFC6C; continue 'dispatch;
	}
	// 82FDFBFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDFC00: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FDFC04: 4BFF403D  bl 0x82fd3c40
	ctx.lr = 0x82FDFC08;
	sub_82FD3C40(ctx, base);
	// 82FDFC08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFC0C: 41820060  beq 0x82fdfc6c
	if ctx.cr[0].eq {
	pc = 0x82FDFC6C; continue 'dispatch;
	}
	// 82FDFC10: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FDFC14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDFC18: 4BFF4029  bl 0x82fd3c40
	ctx.lr = 0x82FDFC1C;
	sub_82FD3C40(ctx, base);
	// 82FDFC1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFC20: 4182004C  beq 0x82fdfc6c
	if ctx.cr[0].eq {
	pc = 0x82FDFC6C; continue 'dispatch;
	}
	// 82FDFC24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFC28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFC2C: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FDFC30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFC34: 4E800421  bctrl
	ctx.lr = 0x82FDFC38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFC38: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFC3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDFC40: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82FDFC44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FDFC48: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FDFC4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFC50: 4E800421  bctrl
	ctx.lr = 0x82FDFC54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFC54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFC58: 41820014  beq 0x82fdfc6c
	if ctx.cr[0].eq {
	pc = 0x82FDFC6C; continue 'dispatch;
	}
	// 82FDFC5C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82FDFC60: 4BFF3FE1  bl 0x82fd3c40
	ctx.lr = 0x82FDFC64;
	sub_82FD3C40(ctx, base);
	// 82FDFC64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFC68: 40820034  bne 0x82fdfc9c
	if !ctx.cr[0].eq {
	pc = 0x82FDFC9C; continue 'dispatch;
	}
	// 82FDFC6C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FDFC70: 7F1BC800  cmpw cr6, r27, r25
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FDFC74: 4198FEE0  blt cr6, 0x82fdfb54
	if ctx.cr[6].lt {
	pc = 0x82FDFB54; continue 'dispatch;
	}
	// 82FDFC78: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FDFC7C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82FDFC80: 4BFFFD51  bl 0x82fdf9d0
	ctx.lr = 0x82FDFC84;
	sub_82FDF9D0(ctx, base);
	// 82FDFC84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFC88: 4182001C  beq 0x82fdfca4
	if ctx.cr[0].eq {
	pc = 0x82FDFCA4; continue 'dispatch;
	}
	// 82FDFC8C: 3AE30004  addi r23, r3, 4
	ctx.r[23].s64 = ctx.r[3].s64 + 4;
	// 82FDFC90: 4BFFFDDC  b 0x82fdfa6c
	pc = 0x82FDFA6C; continue 'dispatch;
	// 82FDFC94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDFC98: 48000010  b 0x82fdfca8
	pc = 0x82FDFCA8; continue 'dispatch;
	// 82FDFC9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFCA0: 48000008  b 0x82fdfca8
	pc = 0x82FDFCA8; continue 'dispatch;
	// 82FDFCA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDFCA8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82FDFCAC: 481C84E8  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDFCB0 size=664
    let mut pc: u32 = 0x82FDFCB0;
    'dispatch: loop {
        match pc {
            0x82FDFCB0 => {
    //   block [0x82FDFCB0..0x82FDFF48)
	// 82FDFCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDFCB4: 481C8499  bl 0x831a814c
	ctx.lr = 0x82FDFCB8;
	sub_831A8130(ctx, base);
	// 82FDFCB8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDFCBC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82FDFCC0: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 82FDFCC4: 3BB6FFFC  addi r29, r22, -4
	ctx.r[29].s64 = ctx.r[22].s64 + -4;
	// 82FDFCC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFCCC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFCD0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDFCD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFCD8: 4E800421  bctrl
	ctx.lr = 0x82FDFCDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFCDC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FDFCE0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FDFCE4: 419A0094  beq cr6, 0x82fdfd78
	if ctx.cr[6].eq {
	pc = 0x82FDFD78; continue 'dispatch;
	}
	// 82FDFCE8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FDFCEC: 419A0064  beq cr6, 0x82fdfd50
	if ctx.cr[6].eq {
	pc = 0x82FDFD50; continue 'dispatch;
	}
	// 82FDFCF0: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FDFCF4: 419A0248  beq cr6, 0x82fdff3c
	if ctx.cr[6].eq {
	pc = 0x82FDFF3C; continue 'dispatch;
	}
	// 82FDFCF8: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82FDFCFC: 419A003C  beq cr6, 0x82fdfd38
	if ctx.cr[6].eq {
	pc = 0x82FDFD38; continue 'dispatch;
	}
	// 82FDFD00: 4099000C  ble cr6, 0x82fdfd0c
	if !ctx.cr[6].gt {
	pc = 0x82FDFD0C; continue 'dispatch;
	}
	// 82FDFD04: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82FDFD08: 40990234  ble cr6, 0x82fdff3c
	if !ctx.cr[6].gt {
	pc = 0x82FDFF3C; continue 'dispatch;
	}
	// 82FDFD0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FDFD10: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82FDFD14: 4BFFFCBD  bl 0x82fdf9d0
	ctx.lr = 0x82FDFD18;
	sub_82FDF9D0(ctx, base);
	// 82FDFD18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFD1C: 41820220  beq 0x82fdff3c
	if ctx.cr[0].eq {
	pc = 0x82FDFF3C; continue 'dispatch;
	}
	// 82FDFD20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD24: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FDFD28: 816B0090  lwz r11, 0x90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FDFD2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFD30: 4E800421  bctrl
	ctx.lr = 0x82FDFD34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFD34: 4800020C  b 0x82fdff40
	pc = 0x82FDFF40; continue 'dispatch;
	// 82FDFD38: 387DFFF4  addi r3, r29, -0xc
	ctx.r[3].s64 = ctx.r[29].s64 + -12;
	// 82FDFD3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD40: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FDFD44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFD48: 4E800421  bctrl
	ctx.lr = 0x82FDFD4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFD4C: 4BFFFFD4  b 0x82fdfd20
	pc = 0x82FDFD20; continue 'dispatch;
	// 82FDFD50: 80760000  lwz r3, 0(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD58: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FDFD5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFD60: 4E800421  bctrl
	ctx.lr = 0x82FDFD64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFD64: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FDFD68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FDFD6C: 409A01D0  bne cr6, 0x82fdff3c
	if !ctx.cr[6].eq {
	pc = 0x82FDFF3C; continue 'dispatch;
	}
	// 82FDFD70: 80760000  lwz r3, 0(r22)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD74: 4BFFFFAC  b 0x82fdfd20
	pc = 0x82FDFD20; continue 'dispatch;
	// 82FDFD78: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFD80: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FDFD84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFD88: 4E800421  bctrl
	ctx.lr = 0x82FDFD8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFD8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFD90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDFD94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFD98: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FDFD9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFDA0: 4E800421  bctrl
	ctx.lr = 0x82FDFDA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFDA4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FDFDA8: 419A0034  beq cr6, 0x82fdfddc
	if ctx.cr[6].eq {
	pc = 0x82FDFDDC; continue 'dispatch;
	}
	// 82FDFDAC: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82FDFDB0: 409A0014  bne cr6, 0x82fdfdc4
	if !ctx.cr[6].eq {
	pc = 0x82FDFDC4; continue 'dispatch;
	}
	// 82FDFDB4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FDFDB8: 409A0014  bne cr6, 0x82fdfdcc
	if !ctx.cr[6].eq {
	pc = 0x82FDFDCC; continue 'dispatch;
	}
	// 82FDFDBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFDC0: 48000180  b 0x82fdff40
	pc = 0x82FDFF40; continue 'dispatch;
	// 82FDFDC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FDFDC8: 419A0014  beq cr6, 0x82fdfddc
	if ctx.cr[6].eq {
	pc = 0x82FDFDDC; continue 'dispatch;
	}
	// 82FDFDCC: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FDFDD0: 4BFF3E71  bl 0x82fd3c40
	ctx.lr = 0x82FDFDD4;
	sub_82FD3C40(ctx, base);
	// 82FDFDD4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFDD8: 4082FFE4  bne 0x82fdfdbc
	if !ctx.cr[0].eq {
	pc = 0x82FDFDBC; continue 'dispatch;
	}
	// 82FDFDDC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFDE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFDE4: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FDFDE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFDEC: 4E800421  bctrl
	ctx.lr = 0x82FDFDF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFDF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFDF4: 4182FF18  beq 0x82fdfd0c
	if ctx.cr[0].eq {
	pc = 0x82FDFD0C; continue 'dispatch;
	}
	// 82FDFDF8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFDFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FDFE00: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FDFE04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFE08: 4E800421  bctrl
	ctx.lr = 0x82FDFE0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFE0C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FDFE10: 4182FEFC  beq 0x82fdfd0c
	if ctx.cr[0].eq {
	pc = 0x82FDFD0C; continue 'dispatch;
	}
	// 82FDFE14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFE18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDFE1C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDFE20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFE24: 4E800421  bctrl
	ctx.lr = 0x82FDFE28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFE28: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FDFE2C: 7C771B79  or. r23, r3, r3
	ctx.r[23].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82FDFE30: 4081FEDC  ble 0x82fdfd0c
	if !ctx.cr[0].gt {
	pc = 0x82FDFD0C; continue 'dispatch;
	}
	// 82FDFE34: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDFE38: 3B2B8018  addi r25, r11, -0x7fe8
	ctx.r[25].s64 = ctx.r[11].s64 + -32744;
	// 82FDFE3C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FDFE40: 3B0B8034  addi r24, r11, -0x7fcc
	ctx.r[24].s64 = ctx.r[11].s64 + -32716;
	// 82FDFE44: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFE48: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FDFE4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FDFE50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDFE54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFE58: 4E800421  bctrl
	ctx.lr = 0x82FDFE5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFE5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FDFE60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFE64: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FDFE68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFE6C: 4E800421  bctrl
	ctx.lr = 0x82FDFE70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFE70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFE74: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FDFE78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFE7C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FDFE80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFE84: 4E800421  bctrl
	ctx.lr = 0x82FDFE88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFE88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFE8C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FDFE90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFE94: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FDFE98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFE9C: 4E800421  bctrl
	ctx.lr = 0x82FDFEA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFEA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFEA4: 41820080  beq 0x82fdff24
	if ctx.cr[0].eq {
	pc = 0x82FDFF24; continue 'dispatch;
	}
	// 82FDFEA8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FDFEAC: 4BFF3D95  bl 0x82fd3c40
	ctx.lr = 0x82FDFEB0;
	sub_82FD3C40(ctx, base);
	// 82FDFEB0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFEB4: 41820070  beq 0x82fdff24
	if ctx.cr[0].eq {
	pc = 0x82FDFF24; continue 'dispatch;
	}
	// 82FDFEB8: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82FDFEBC: 409A0028  bne cr6, 0x82fdfee4
	if !ctx.cr[6].eq {
	pc = 0x82FDFEE4; continue 'dispatch;
	}
	// 82FDFEC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFEC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFEC8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FDFECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFED0: 4E800421  bctrl
	ctx.lr = 0x82FDFED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFED4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDFED8: 4BFF3D69  bl 0x82fd3c40
	ctx.lr = 0x82FDFEDC;
	sub_82FD3C40(ctx, base);
	// 82FDFEDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFEE0: 40820054  bne 0x82fdff34
	if !ctx.cr[0].eq {
	pc = 0x82FDFF34; continue 'dispatch;
	}
	// 82FDFEE4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FDFEE8: 419A003C  beq cr6, 0x82fdff24
	if ctx.cr[6].eq {
	pc = 0x82FDFF24; continue 'dispatch;
	}
	// 82FDFEEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FDFEF0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82FDFEF4: 4BFF3D4D  bl 0x82fd3c40
	ctx.lr = 0x82FDFEF8;
	sub_82FD3C40(ctx, base);
	// 82FDFEF8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFEFC: 41820028  beq 0x82fdff24
	if ctx.cr[0].eq {
	pc = 0x82FDFF24; continue 'dispatch;
	}
	// 82FDFF00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFF04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FDFF08: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FDFF0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFF10: 4E800421  bctrl
	ctx.lr = 0x82FDFF14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFF14: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82FDFF18: 4BFF3D29  bl 0x82fd3c40
	ctx.lr = 0x82FDFF1C;
	sub_82FD3C40(ctx, base);
	// 82FDFF1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFF20: 40820014  bne 0x82fdff34
	if !ctx.cr[0].eq {
	pc = 0x82FDFF34; continue 'dispatch;
	}
	// 82FDFF24: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FDFF28: 7F1BB800  cmpw cr6, r27, r23
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[23].s32, &mut ctx.xer);
	// 82FDFF2C: 4198FF18  blt cr6, 0x82fdfe44
	if ctx.cr[6].lt {
	pc = 0x82FDFE44; continue 'dispatch;
	}
	// 82FDFF30: 4BFFFDDC  b 0x82fdfd0c
	pc = 0x82FDFD0C; continue 'dispatch;
	// 82FDFF34: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FDFF38: 48000008  b 0x82fdff40
	pc = 0x82FDFF40; continue 'dispatch;
	// 82FDFF3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDFF40: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FDFF44: 481C8258  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDFF48 size=80
    let mut pc: u32 = 0x82FDFF48;
    'dispatch: loop {
        match pc {
            0x82FDFF48 => {
    //   block [0x82FDFF48..0x82FDFF98)
	// 82FDFF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDFF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FDFF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDFF54: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FDFF58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFF5C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FDFF60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFF64: 4E800421  bctrl
	ctx.lr = 0x82FDFF68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFF68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FDFF6C: 41820018  beq 0x82fdff84
	if ctx.cr[0].eq {
	pc = 0x82FDFF84; continue 'dispatch;
	}
	// 82FDFF70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FDFF74: 816B0078  lwz r11, 0x78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FDFF78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FDFF7C: 4E800421  bctrl
	ctx.lr = 0x82FDFF80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FDFF80: 48000008  b 0x82fdff88
	pc = 0x82FDFF88; continue 'dispatch;
	// 82FDFF84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FDFF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FDFF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FDFF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FDFF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDFF98 size=20
    let mut pc: u32 = 0x82FDFF98;
    'dispatch: loop {
        match pc {
            0x82FDFF98 => {
    //   block [0x82FDFF98..0x82FDFFAC)
	// 82FDFF98: 7C8B0734  extsh r11, r4
	ctx.r[11].s64 = ctx.r[4].s16 as i64;
	// 82FDFF9C: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FDFFA0: 4182000C  beq 0x82fdffac
	if ctx.cr[0].eq {
		sub_82FDFFAC(ctx, base);
		return;
	}
	// 82FDFFA4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FDFFA8: 48000010  b 0x82fdffb8
	sub_82FDFFAC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFFAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDFFAC size=32
    let mut pc: u32 = 0x82FDFFAC;
    'dispatch: loop {
        match pc {
            0x82FDFFAC => {
    //   block [0x82FDFFAC..0x82FDFFCC)
	// 82FDFFAC: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFFB0: 41820008  beq 0x82fdffb8
	if ctx.cr[0].eq {
	pc = 0x82FDFFB8; continue 'dispatch;
	}
	// 82FDFFB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FDFFB8: 7C8B0734  extsh r11, r4
	ctx.r[11].s64 = ctx.r[4].s16 as i64;
	// 82FDFFBC: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FDFFC0: 4182000C  beq 0x82fdffcc
	if ctx.cr[0].eq {
		sub_82FDFFCC(ctx, base);
		return;
	}
	// 82FDFFC4: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82FDFFC8: 48000010  b 0x82fdffd8
	sub_82FDFFCC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFFCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FDFFCC size=20
    let mut pc: u32 = 0x82FDFFCC;
    'dispatch: loop {
        match pc {
            0x82FDFFCC => {
    //   block [0x82FDFFCC..0x82FDFFE0)
	// 82FDFFCC: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FDFFD0: 41820008  beq 0x82fdffd8
	if ctx.cr[0].eq {
	pc = 0x82FDFFD8; continue 'dispatch;
	}
	// 82FDFFD4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FDFFD8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FDFFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FDFFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FDFFE0 size=460
    let mut pc: u32 = 0x82FDFFE0;
    'dispatch: loop {
        match pc {
            0x82FDFFE0 => {
    //   block [0x82FDFFE0..0x82FE01AC)
	// 82FDFFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FDFFE4: 481C8181  bl 0x831a8164
	ctx.lr = 0x82FDFFE8;
	sub_831A8130(ctx, base);
	// 82FDFFE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FDFFEC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82FDFFF0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FDFFF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FDFFF8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FDFFFC: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0000: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FE0004: 419A0008  beq cr6, 0x82fe000c
	if ctx.cr[6].eq {
	pc = 0x82FE000C; continue 'dispatch;
	}
	// 82FE0008: B37C0000  sth r27, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 82FE000C: 3BC3FFFC  addi r30, r3, -4
	ctx.r[30].s64 = ctx.r[3].s64 + -4;
	// 82FE0010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0014: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0018: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE001C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0020: 4E800421  bctrl
	ctx.lr = 0x82FE0024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0024: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE0028: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE002C: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 82FE0030: 41990170  bgt cr6, 0x82fe01a0
	if ctx.cr[6].gt {
	pc = 0x82FE01A0; continue 'dispatch;
	}
	// 82FE0034: 3D808214  lis r12, -0x7dec
	ctx.r[12].s64 = -2112618496;
	// 82FE0038: 398CA6D0  addi r12, r12, -0x5930
	ctx.r[12].s64 = ctx.r[12].s64 + -22832;
	// 82FE003C: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE0040: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82FE0044: 3D8082FE  lis r12, -0x7d02
	ctx.r[12].s64 = -2097283072;
	// 82FE0048: 398C005C  addi r12, r12, 0x5c
	ctx.r[12].s64 = ctx.r[12].s64 + 92;
	// 82FE004C: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FE0050: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FE0054: 60000000  nop
	// 82FE0058: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82FE005C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0060: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE0064: 480000A0  b 0x82fe0104
	pc = 0x82FE0104; continue 'dispatch;
	// 82FE0068: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE006C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0070: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE0074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0078: 4E800421  bctrl
	ctx.lr = 0x82FE007C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE007C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE0080: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 82FE0084: 419A0078  beq cr6, 0x82fe00fc
	if ctx.cr[6].eq {
	pc = 0x82FE00FC; continue 'dispatch;
	}
	// 82FE0088: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE008C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0090: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE0094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0098: 4E800421  bctrl
	ctx.lr = 0x82FE009C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE009C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE00A0: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82FE00A4: 419A0058  beq cr6, 0x82fe00fc
	if ctx.cr[6].eq {
	pc = 0x82FE00FC; continue 'dispatch;
	}
	// 82FE00A8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE00AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE00B0: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82FE00B4: 419A002C  beq cr6, 0x82fe00e0
	if ctx.cr[6].eq {
	pc = 0x82FE00E0; continue 'dispatch;
	}
	// 82FE00B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE00BC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FE00C0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE00C4: 7C8BE214  add r4, r11, r28
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FE00C8: 4BFFFF19  bl 0x82fdffe0
	ctx.lr = 0x82FE00CC;
	sub_82FDFFE0(ctx, base);
	// 82FE00CC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE00D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE00D4: 7FABE850  subf r29, r11, r29
	ctx.r[29].s64 = ctx.r[29].s64 - ctx.r[11].s64;
	// 82FE00D8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FE00DC: 4800001C  b 0x82fe00f8
	pc = 0x82FE00F8; continue 'dispatch;
	// 82FE00E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE00E4: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 82FE00E8: 4BFFFEF9  bl 0x82fdffe0
	ctx.lr = 0x82FE00EC;
	sub_82FDFFE0(ctx, base);
	// 82FE00EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE00F0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE00F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FE00F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE00FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0100: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE0104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0108: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE010C: 4E800421  bctrl
	ctx.lr = 0x82FE0110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0110: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE0114: 4082FF54  bne 0x82fe0068
	if !ctx.cr[0].eq {
	pc = 0x82FE0068; continue 'dispatch;
	}
	// 82FE0118: 48000088  b 0x82fe01a0
	pc = 0x82FE01A0; continue 'dispatch;
	// 82FE011C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0124: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE0128: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE012C: 4E800421  bctrl
	ctx.lr = 0x82FE0130;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0130: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FE0134: 41820034  beq 0x82fe0168
	if ctx.cr[0].eq {
	pc = 0x82FE0168; continue 'dispatch;
	}
	// 82FE0138: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE013C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0140: 41820028  beq 0x82fe0168
	if ctx.cr[0].eq {
	pc = 0x82FE0168; continue 'dispatch;
	}
	// 82FE0144: 39640002  addi r11, r4, 2
	ctx.r[11].s64 = ctx.r[4].s64 + 2;
	// 82FE0148: 48000008  b 0x82fe0150
	pc = 0x82FE0150; continue 'dispatch;
	// 82FE014C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE0150: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0154: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0158: 4082FFF4  bne 0x82fe014c
	if !ctx.cr[0].eq {
	pc = 0x82FE014C; continue 'dispatch;
	}
	// 82FE015C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82FE0160: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FE0164: 48000008  b 0x82fe016c
	pc = 0x82FE016C; continue 'dispatch;
	// 82FE0168: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82FE016C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE0170: 419A0024  beq cr6, 0x82fe0194
	if ctx.cr[6].eq {
	pc = 0x82FE0194; continue 'dispatch;
	}
	// 82FE0174: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FE0178: 40980008  bge cr6, 0x82fe0180
	if !ctx.cr[6].lt {
	pc = 0x82FE0180; continue 'dispatch;
	}
	// 82FE017C: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82FE0180: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0184: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE0188: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE018C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82FE0190: 4BFF1A19  bl 0x82fd1ba8
	ctx.lr = 0x82FE0194;
	sub_82FD1BA8(ctx, base);
	// 82FE0194: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0198: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82FE019C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE01A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE01A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE01A8: 481C800C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE01B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE01B0 size=496
    let mut pc: u32 = 0x82FE01B0;
    'dispatch: loop {
        match pc {
            0x82FE01B0 => {
    //   block [0x82FE01B0..0x82FE03A0)
	// 82FE01B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE01B4: 481C7FB9  bl 0x831a816c
	ctx.lr = 0x82FE01B8;
	sub_831A8130(ctx, base);
	// 82FE01B8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE01BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE01C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE01C4: 3BFEFFFC  addi r31, r30, -4
	ctx.r[31].s64 = ctx.r[30].s64 + -4;
	// 82FE01C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE01CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE01D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE01D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE01D8: 4E800421  bctrl
	ctx.lr = 0x82FE01DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE01DC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE01E0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE01E4: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82FE01E8: 41990168  bgt cr6, 0x82fe0350
	if ctx.cr[6].gt {
	pc = 0x82FE0350; continue 'dispatch;
	}
	// 82FE01EC: 3D808214  lis r12, -0x7dec
	ctx.r[12].s64 = -2112618496;
	// 82FE01F0: 398CA6E0  addi r12, r12, -0x5920
	ctx.r[12].s64 = ctx.r[12].s64 + -22816;
	// 82FE01F4: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE01F8: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82FE01FC: 3D8082FE  lis r12, -0x7d02
	ctx.r[12].s64 = -2097283072;
	// 82FE0200: 398C0214  addi r12, r12, 0x214
	ctx.r[12].s64 = ctx.r[12].s64 + 532;
	// 82FE0204: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FE0208: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FE020C: 60000000  nop
	// 82FE0210: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82FE0214: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE0218: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE021C: 41820064  beq 0x82fe0280
	if ctx.cr[0].eq {
	pc = 0x82FE0280; continue 'dispatch;
	}
	// 82FE0220: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0224: 4BFFF005  bl 0x82fdf228
	ctx.lr = 0x82FE0228;
	sub_82FDF228(ctx, base);
	// 82FE0228: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE022C: 41820014  beq 0x82fe0240
	if ctx.cr[0].eq {
	pc = 0x82FE0240; continue 'dispatch;
	}
	// 82FE0230: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0234: 4BFFEFF5  bl 0x82fdf228
	ctx.lr = 0x82FE0238;
	sub_82FDF228(ctx, base);
	// 82FE0238: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE023C: 4800000C  b 0x82fe0248
	pc = 0x82FE0248; continue 'dispatch;
	// 82FE0240: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE0244: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE0248: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE024C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FE0250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE0254: 48019C7D  bl 0x82ff9ed0
	ctx.lr = 0x82FE0258;
	sub_82FF9ED0(ctx, base);
	// 82FE0258: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE025C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE0260: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE0264: 481D09C5  bl 0x831b0c28
	ctx.lr = 0x82FE0268;
	sub_831B0C28(ctx, base);
	// 82FE0268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE026C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE0270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0274: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE0278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE027C: 4E800421  bctrl
	ctx.lr = 0x82FE0280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0280: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0288: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE028C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0290: 4E800421  bctrl
	ctx.lr = 0x82FE0294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0294: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0298: 4082FFD0  bne 0x82fe0268
	if !ctx.cr[0].eq {
	pc = 0x82FE0268; continue 'dispatch;
	}
	// 82FE029C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE02A0: 419A0044  beq cr6, 0x82fe02e4
	if ctx.cr[6].eq {
	pc = 0x82FE02E4; continue 'dispatch;
	}
	// 82FE02A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE02A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE02AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE02B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE02B4: 4E800421  bctrl
	ctx.lr = 0x82FE02B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE02B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE02BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE02C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE02C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE02C8: 4E800421  bctrl
	ctx.lr = 0x82FE02CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE02CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE02D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE02D4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE02D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE02DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE02E0: 4E800421  bctrl
	ctx.lr = 0x82FE02E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE02E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82FE02E8: 481C7ED4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE02EC: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE02F0: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE02F4: 4182004C  beq 0x82fe0340
	if ctx.cr[0].eq {
	pc = 0x82FE0340; continue 'dispatch;
	}
	// 82FE02F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE02FC: 4BFFEF2D  bl 0x82fdf228
	ctx.lr = 0x82FE0300;
	sub_82FDF228(ctx, base);
	// 82FE0300: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0304: 41820014  beq 0x82fe0318
	if ctx.cr[0].eq {
	pc = 0x82FE0318; continue 'dispatch;
	}
	// 82FE0308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE030C: 4BFFEF1D  bl 0x82fdf228
	ctx.lr = 0x82FE0310;
	sub_82FDF228(ctx, base);
	// 82FE0310: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE0314: 4800000C  b 0x82fe0320
	pc = 0x82FE0320; continue 'dispatch;
	// 82FE0318: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE031C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE0320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE0324: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82FE0328: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FE032C: 48019BA5  bl 0x82ff9ed0
	ctx.lr = 0x82FE0330;
	sub_82FF9ED0(ctx, base);
	// 82FE0330: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE0334: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82FE0338: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE033C: 481D08ED  bl 0x831b0c28
	ctx.lr = 0x82FE0340;
	sub_831B0C28(ctx, base);
	// 82FE0340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0344: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE0348: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82FE034C: 4BFFFF8C  b 0x82fe02d8
	pc = 0x82FE02D8; continue 'dispatch;
	// 82FE0350: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE0354: 419A0024  beq cr6, 0x82fe0378
	if ctx.cr[6].eq {
	pc = 0x82FE0378; continue 'dispatch;
	}
	// 82FE0358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE035C: 4BFFEECD  bl 0x82fdf228
	ctx.lr = 0x82FE0360;
	sub_82FDF228(ctx, base);
	// 82FE0360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0364: 41820014  beq 0x82fe0378
	if ctx.cr[0].eq {
	pc = 0x82FE0378; continue 'dispatch;
	}
	// 82FE0368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE036C: 4BFFEEBD  bl 0x82fdf228
	ctx.lr = 0x82FE0370;
	sub_82FDF228(ctx, base);
	// 82FE0370: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE0374: 4800000C  b 0x82fe0380
	pc = 0x82FE0380; continue 'dispatch;
	// 82FE0378: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE037C: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE0380: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE0384: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE0388: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82FE038C: 48019B45  bl 0x82ff9ed0
	ctx.lr = 0x82FE0390;
	sub_82FF9ED0(ctx, base);
	// 82FE0390: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE0394: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82FE0398: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE039C: 481D088D  bl 0x831b0c28
	ctx.lr = 0x82FE03A0;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE03A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE03A0 size=372
    let mut pc: u32 = 0x82FE03A0;
    'dispatch: loop {
        match pc {
            0x82FE03A0 => {
    //   block [0x82FE03A0..0x82FE0514)
	// 82FE03A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE03A4: 481C7DC9  bl 0x831a816c
	ctx.lr = 0x82FE03A8;
	sub_831A8130(ctx, base);
	// 82FE03A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE03AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE03B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE03B4: 3BFEFFFC  addi r31, r30, -4
	ctx.r[31].s64 = ctx.r[30].s64 + -4;
	// 82FE03B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE03BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE03C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE03C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE03C8: 4E800421  bctrl
	ctx.lr = 0x82FE03CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE03CC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE03D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE03D4: 419A0094  beq cr6, 0x82fe0468
	if ctx.cr[6].eq {
	pc = 0x82FE0468; continue 'dispatch;
	}
	// 82FE03D8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FE03DC: 419A0064  beq cr6, 0x82fe0440
	if ctx.cr[6].eq {
	pc = 0x82FE0440; continue 'dispatch;
	}
	// 82FE03E0: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FE03E4: 419A0100  beq cr6, 0x82fe04e4
	if ctx.cr[6].eq {
	pc = 0x82FE04E4; continue 'dispatch;
	}
	// 82FE03E8: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82FE03EC: 419A003C  beq cr6, 0x82fe0428
	if ctx.cr[6].eq {
	pc = 0x82FE0428; continue 'dispatch;
	}
	// 82FE03F0: 4099000C  ble cr6, 0x82fe03fc
	if !ctx.cr[6].gt {
	pc = 0x82FE03FC; continue 'dispatch;
	}
	// 82FE03F4: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82FE03F8: 409900EC  ble cr6, 0x82fe04e4
	if !ctx.cr[6].gt {
	pc = 0x82FE04E4; continue 'dispatch;
	}
	// 82FE03FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE0400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0404: 4BFFF5CD  bl 0x82fdf9d0
	ctx.lr = 0x82FE0408;
	sub_82FDF9D0(ctx, base);
	// 82FE0408: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE040C: 418200D8  beq 0x82fe04e4
	if ctx.cr[0].eq {
	pc = 0x82FE04E4; continue 'dispatch;
	}
	// 82FE0410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE0418: 816B008C  lwz r11, 0x8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE041C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0420: 4E800421  bctrl
	ctx.lr = 0x82FE0424;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0424: 480000E8  b 0x82fe050c
	pc = 0x82FE050C; continue 'dispatch;
	// 82FE0428: 387FFFF4  addi r3, r31, -0xc
	ctx.r[3].s64 = ctx.r[31].s64 + -12;
	// 82FE042C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0430: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FE0434: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0438: 4E800421  bctrl
	ctx.lr = 0x82FE043C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE043C: 4BFFFFD4  b 0x82fe0410
	pc = 0x82FE0410; continue 'dispatch;
	// 82FE0440: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0444: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0448: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE044C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0450: 4E800421  bctrl
	ctx.lr = 0x82FE0454;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0454: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE0458: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE045C: 409A0088  bne cr6, 0x82fe04e4
	if !ctx.cr[6].eq {
	pc = 0x82FE04E4; continue 'dispatch;
	}
	// 82FE0460: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0464: 4BFFFFAC  b 0x82fe0410
	pc = 0x82FE0410; continue 'dispatch;
	// 82FE0468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE046C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0470: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FE0474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0478: 4E800421  bctrl
	ctx.lr = 0x82FE047C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE047C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0480: 4182006C  beq 0x82fe04ec
	if ctx.cr[0].eq {
	pc = 0x82FE04EC; continue 'dispatch;
	}
	// 82FE0484: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0488: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE048C: 41820060  beq 0x82fe04ec
	if ctx.cr[0].eq {
	pc = 0x82FE04EC; continue 'dispatch;
	}
	// 82FE0490: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0498: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FE049C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE04A0: 4E800421  bctrl
	ctx.lr = 0x82FE04A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE04A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE04A8: 4182FF54  beq 0x82fe03fc
	if ctx.cr[0].eq {
	pc = 0x82FE03FC; continue 'dispatch;
	}
	// 82FE04AC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE04B0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE04B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE04B8: 38AB8018  addi r5, r11, -0x7fe8
	ctx.r[5].s64 = ctx.r[11].s64 + -32744;
	// 82FE04BC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE04C0: 388B8034  addi r4, r11, -0x7fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -32716;
	// 82FE04C4: 816A00C8  lwz r11, 0xc8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FE04C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE04CC: 4E800421  bctrl
	ctx.lr = 0x82FE04D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE04D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE04D4: 4182FF28  beq 0x82fe03fc
	if ctx.cr[0].eq {
	pc = 0x82FE03FC; continue 'dispatch;
	}
	// 82FE04D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE04DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE04E0: 48000018  b 0x82fe04f8
	pc = 0x82FE04F8; continue 'dispatch;
	// 82FE04E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE04E8: 48000024  b 0x82fe050c
	pc = 0x82FE050C; continue 'dispatch;
	// 82FE04EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE04F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE04F4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE04F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE04FC: 4E800421  bctrl
	ctx.lr = 0x82FE0500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0500: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE0504: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE0508: 4BFF3739  bl 0x82fd3c40
	ctx.lr = 0x82FE050C;
	sub_82FD3C40(ctx, base);
	// 82FE050C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE0510: 481C7CAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0518 size=96
    let mut pc: u32 = 0x82FE0518;
    'dispatch: loop {
        match pc {
            0x82FE0518 => {
    //   block [0x82FE0518..0x82FE0578)
	// 82FE0518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE051C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0520: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0524: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0528: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE052C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE0530: 419A0020  beq cr6, 0x82fe0550
	if ctx.cr[6].eq {
	pc = 0x82FE0550; continue 'dispatch;
	}
	// 82FE0534: 4BFFECF5  bl 0x82fdf228
	ctx.lr = 0x82FE0538;
	sub_82FDF228(ctx, base);
	// 82FE0538: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE053C: 41820014  beq 0x82fe0550
	if ctx.cr[0].eq {
	pc = 0x82FE0550; continue 'dispatch;
	}
	// 82FE0540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0544: 4BFFECE5  bl 0x82fdf228
	ctx.lr = 0x82FE0548;
	sub_82FDF228(ctx, base);
	// 82FE0548: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE054C: 4800000C  b 0x82fe0558
	pc = 0x82FE0558; continue 'dispatch;
	// 82FE0550: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE0554: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE0558: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE055C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE0560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE0564: 4801996D  bl 0x82ff9ed0
	ctx.lr = 0x82FE0568;
	sub_82FF9ED0(ctx, base);
	// 82FE0568: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE056C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE0570: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE0574: 481D06B5  bl 0x831b0c28
	ctx.lr = 0x82FE0578;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0578 size=128
    let mut pc: u32 = 0x82FE0578;
    'dispatch: loop {
        match pc {
            0x82FE0578 => {
    //   block [0x82FE0578..0x82FE05F8)
	// 82FE0578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE0584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0588: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE058C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE0590: 3BEBB838  addi r31, r11, -0x47c8
	ctx.r[31].s64 = ctx.r[11].s64 + -18376;
	// 82FE0594: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0598: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE059C: 419A0018  beq cr6, 0x82fe05b4
	if ctx.cr[6].eq {
	pc = 0x82FE05B4; continue 'dispatch;
	}
	// 82FE05A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE05A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE05A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE05AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE05B0: 4E800421  bctrl
	ctx.lr = 0x82FE05B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE05B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE05B8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE05BC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE05C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE05C4: 419A0014  beq cr6, 0x82fe05d8
	if ctx.cr[6].eq {
	pc = 0x82FE05D8; continue 'dispatch;
	}
	// 82FE05C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE05CC: 480151BD  bl 0x82ff5788
	ctx.lr = 0x82FE05D0;
	sub_82FF5788(ctx, base);
	// 82FE05D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE05D4: 4BFF7D0D  bl 0x82fd82e0
	ctx.lr = 0x82FE05D8;
	sub_82FD82E0(ctx, base);
	// 82FE05D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE05DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE05E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE05E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE05E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE05EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE05F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE05F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE05F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE05F8 size=96
    let mut pc: u32 = 0x82FE05F8;
    'dispatch: loop {
        match pc {
            0x82FE05F8 => {
    //   block [0x82FE05F8..0x82FE0658)
	// 82FE05F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE05FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0608: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE060C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0610: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82FE0614: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82FE0618: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82FE061C: 4BFFEC0D  bl 0x82fdf228
	ctx.lr = 0x82FE0620;
	sub_82FDF228(ctx, base);
	// 82FE0620: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0624: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82FE0628: 40820008  bne 0x82fe0630
	if !ctx.cr[0].eq {
	pc = 0x82FE0630; continue 'dispatch;
	}
	// 82FE062C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE0630: A15F0004  lhz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE0634: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0638: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE063C: 714AFFF7  andi. r10, r10, 0xfff7
	ctx.r[10].u64 = ctx.r[10].u64 & 65527;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FE0640: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FE0644: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE064C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE0654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0658 size=8
    let mut pc: u32 = 0x82FE0658;
    'dispatch: loop {
        match pc {
            0x82FE0658 => {
    //   block [0x82FE0658..0x82FE0660)
	// 82FE0658: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE065C: 8213A710  lwz r16, -0x58f0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-22768 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0660 size=252
    let mut pc: u32 = 0x82FE0660;
    'dispatch: loop {
        match pc {
            0x82FE0660 => {
    //   block [0x82FE0660..0x82FE075C)
	// 82FE0660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE066C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0670: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE0674: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0678: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE067C: 3BCBB838  addi r30, r11, -0x47c8
	ctx.r[30].s64 = ctx.r[11].s64 + -18376;
	// 82FE0680: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0684: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE0688: 409A00BC  bne cr6, 0x82fe0744
	if !ctx.cr[6].eq {
	pc = 0x82FE0744; continue 'dispatch;
	}
	// 82FE068C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE0690: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE0694: 409A0050  bne cr6, 0x82fe06e4
	if !ctx.cr[6].eq {
	pc = 0x82FE06E4; continue 'dispatch;
	}
	// 82FE0698: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE069C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE06A0: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 82FE06A4: 48015135  bl 0x82ff57d8
	ctx.lr = 0x82FE06A8;
	sub_82FF57D8(ctx, base);
	// 82FE06A8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE06AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE06B0: 409A0028  bne cr6, 0x82fe06d8
	if !ctx.cr[6].eq {
	pc = 0x82FE06D8; continue 'dispatch;
	}
	// 82FE06B4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FE06B8: 4BFF7B91  bl 0x82fd8248
	ctx.lr = 0x82FE06BC;
	sub_82FD8248(ctx, base);
	// 82FE06BC: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82FE06C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE06C4: 4182000C  beq 0x82fe06d0
	if ctx.cr[0].eq {
	pc = 0x82FE06D0; continue 'dispatch;
	}
	// 82FE06C8: 48015081  bl 0x82ff5748
	ctx.lr = 0x82FE06CC;
	sub_82FF5748(ctx, base);
	// 82FE06CC: 48000008  b 0x82fe06d4
	pc = 0x82FE06D4; continue 'dispatch;
	// 82FE06D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE06D4: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FE06D8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE06DC: 48015135  bl 0x82ff5810
	ctx.lr = 0x82FE06E0;
	sub_82FF5810(ctx, base);
	// 82FE06E0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE06E4: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82FE06E8: 480150F1  bl 0x82ff57d8
	ctx.lr = 0x82FE06EC;
	sub_82FF57D8(ctx, base);
	// 82FE06EC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE06F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE06F4: 409A0044  bne cr6, 0x82fe0738
	if !ctx.cr[6].eq {
	pc = 0x82FE0738; continue 'dispatch;
	}
	// 82FE06F8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82FE06FC: 4B2E023D  bl 0x822c0938
	ctx.lr = 0x82FE0700;
	sub_822C0938(ctx, base);
	// 82FE0700: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE0704: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0708: 41820014  beq 0x82fe071c
	if ctx.cr[0].eq {
	pc = 0x82FE071C; continue 'dispatch;
	}
	// 82FE070C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE0710: 480198E1  bl 0x82ff9ff0
	ctx.lr = 0x82FE0714;
	sub_82FF9FF0(ctx, base);
	// 82FE0714: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FE0718: 48000008  b 0x82fe0720
	pc = 0x82FE0720; continue 'dispatch;
	// 82FE071C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FE0720: 3D6082FE  lis r11, -0x7d02
	ctx.r[11].s64 = -2097283072;
	// 82FE0724: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82FE0728: 913E0000  stw r9, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FE072C: 388B0578  addi r4, r11, 0x578
	ctx.r[4].s64 = ctx.r[11].s64 + 1400;
	// 82FE0730: 386AB840  addi r3, r10, -0x47c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18368;
	// 82FE0734: 48017405  bl 0x82ff7b38
	ctx.lr = 0x82FE0738;
	sub_82FF7B38(ctx, base);
	// 82FE0738: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82FE073C: 480150D5  bl 0x82ff5810
	ctx.lr = 0x82FE0740;
	sub_82FF5810(ctx, base);
	// 82FE0740: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0744: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE0748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE074C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0750: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE0754: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE0758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE075C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE075C size=40
    let mut pc: u32 = 0x82FE075C;
    'dispatch: loop {
        match pc {
            0x82FE075C => {
    //   block [0x82FE075C..0x82FE0784)
	// 82FE075C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE0760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE076C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE0770: 480150A1  bl 0x82ff5810
	ctx.lr = 0x82FE0774;
	sub_82FF5810(ctx, base);
	// 82FE0774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE077C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0784 size=40
    let mut pc: u32 = 0x82FE0784;
    'dispatch: loop {
        match pc {
            0x82FE0784 => {
    //   block [0x82FE0784..0x82FE07AC)
	// 82FE0784: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE0788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE078C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0794: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE0798: 4BFF7B49  bl 0x82fd82e0
	ctx.lr = 0x82FE079C;
	sub_82FD82E0(ctx, base);
	// 82FE079C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE07A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE07A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE07A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE07AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE07AC size=40
    let mut pc: u32 = 0x82FE07AC;
    'dispatch: loop {
        match pc {
            0x82FE07AC => {
    //   block [0x82FE07AC..0x82FE07D4)
	// 82FE07AC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE07B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE07B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE07B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE07BC: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 82FE07C0: 48015051  bl 0x82ff5810
	ctx.lr = 0x82FE07C4;
	sub_82FF5810(ctx, base);
	// 82FE07C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE07C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE07CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE07D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE07D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE07D4 size=40
    let mut pc: u32 = 0x82FE07D4;
    'dispatch: loop {
        match pc {
            0x82FE07D4 => {
    //   block [0x82FE07D4..0x82FE07FC)
	// 82FE07D4: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE07D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE07DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE07E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE07E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE07E8: 4B2DFA81  bl 0x822c0268
	ctx.lr = 0x82FE07EC;
	sub_822C0268(ctx, base);
	// 82FE07EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE07F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE07F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE07F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0800 size=252
    let mut pc: u32 = 0x82FE0800;
    'dispatch: loop {
        match pc {
            0x82FE0800 => {
    //   block [0x82FE0800..0x82FE08FC)
	// 82FE0800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0804: 481C7965  bl 0x831a8168
	ctx.lr = 0x82FE0808;
	sub_831A8130(ctx, base);
	// 82FE0808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE080C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE0810: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0814: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE0818: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE081C: 409A000C  bne cr6, 0x82fe0828
	if !ctx.cr[6].eq {
	pc = 0x82FE0828; continue 'dispatch;
	}
	// 82FE0820: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE0824: 480000D0  b 0x82fe08f4
	pc = 0x82FE08F4; continue 'dispatch;
	// 82FE0828: 3BFEFFFC  addi r31, r30, -4
	ctx.r[31].s64 = ctx.r[30].s64 + -4;
	// 82FE082C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0830: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0834: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE0838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE083C: 4E800421  bctrl
	ctx.lr = 0x82FE0840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0840: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE0844: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE0848: 419A0098  beq cr6, 0x82fe08e0
	if ctx.cr[6].eq {
	pc = 0x82FE08E0; continue 'dispatch;
	}
	// 82FE084C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FE0850: 419A0068  beq cr6, 0x82fe08b8
	if ctx.cr[6].eq {
	pc = 0x82FE08B8; continue 'dispatch;
	}
	// 82FE0854: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FE0858: 419AFFC8  beq cr6, 0x82fe0820
	if ctx.cr[6].eq {
	pc = 0x82FE0820; continue 'dispatch;
	}
	// 82FE085C: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82FE0860: 419A0028  beq cr6, 0x82fe0888
	if ctx.cr[6].eq {
	pc = 0x82FE0888; continue 'dispatch;
	}
	// 82FE0864: 4099000C  ble cr6, 0x82fe0870
	if !ctx.cr[6].gt {
	pc = 0x82FE0870; continue 'dispatch;
	}
	// 82FE0868: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82FE086C: 4099FFB4  ble cr6, 0x82fe0820
	if !ctx.cr[6].gt {
	pc = 0x82FE0820; continue 'dispatch;
	}
	// 82FE0870: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE0874: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0878: 4BFFF159  bl 0x82fdf9d0
	ctx.lr = 0x82FE087C;
	sub_82FDF9D0(ctx, base);
	// 82FE087C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0880: 4182FFA0  beq 0x82fe0820
	if ctx.cr[0].eq {
	pc = 0x82FE0820; continue 'dispatch;
	}
	// 82FE0884: 48000018  b 0x82fe089c
	pc = 0x82FE089C; continue 'dispatch;
	// 82FE0888: 387FFFF4  addi r3, r31, -0xc
	ctx.r[3].s64 = ctx.r[31].s64 + -12;
	// 82FE088C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0890: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82FE0894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0898: 4E800421  bctrl
	ctx.lr = 0x82FE089C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE089C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE08A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE08A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE08A8: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE08AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE08B0: 4E800421  bctrl
	ctx.lr = 0x82FE08B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE08B4: 48000040  b 0x82fe08f4
	pc = 0x82FE08F4; continue 'dispatch;
	// 82FE08B8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE08BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE08C0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE08C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE08C8: 4E800421  bctrl
	ctx.lr = 0x82FE08CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE08CC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE08D0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE08D4: 409AFF4C  bne cr6, 0x82fe0820
	if !ctx.cr[6].eq {
	pc = 0x82FE0820; continue 'dispatch;
	}
	// 82FE08D8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE08DC: 4BFFFFC0  b 0x82fe089c
	pc = 0x82FE089C; continue 'dispatch;
	// 82FE08E0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82FE08E4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE08E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE08EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE08F0: 4BFFF151  bl 0x82fdfa40
	ctx.lr = 0x82FE08F4;
	sub_82FDFA40(ctx, base);
	// 82FE08F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE08F8: 481C78C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0900 size=952
    let mut pc: u32 = 0x82FE0900;
    'dispatch: loop {
        match pc {
            0x82FE0900 => {
    //   block [0x82FE0900..0x82FE0CB8)
	// 82FE0900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0904: 481C7855  bl 0x831a8158
	ctx.lr = 0x82FE0908;
	sub_831A8130(ctx, base);
	// 82FE0908: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE090C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE0910: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE0914: 3B9DFFFC  addi r28, r29, -4
	ctx.r[28].s64 = ctx.r[29].s64 + -4;
	// 82FE0918: 7F1CF840  cmplw cr6, r28, r31
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FE091C: 409A000C  bne cr6, 0x82fe0928
	if !ctx.cr[6].eq {
	pc = 0x82FE0928; continue 'dispatch;
	}
	// 82FE0920: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82FE0924: 4800038C  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE0928: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE092C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE0930: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE0934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0938: 4E800421  bctrl
	ctx.lr = 0x82FE093C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE093C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0940: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0948: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE094C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0950: 4E800421  bctrl
	ctx.lr = 0x82FE0954;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0954: 7FCB0734  extsh r11, r30
	ctx.r[11].s64 = ctx.r[30].s16 as i64;
	// 82FE0958: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FE095C: 419A0350  beq cr6, 0x82fe0cac
	if ctx.cr[6].eq {
	pc = 0x82FE0CAC; continue 'dispatch;
	}
	// 82FE0960: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82FE0964: 419A0348  beq cr6, 0x82fe0cac
	if ctx.cr[6].eq {
	pc = 0x82FE0CAC; continue 'dispatch;
	}
	// 82FE0968: 7C6A0734  extsh r10, r3
	ctx.r[10].s64 = ctx.r[3].s16 as i64;
	// 82FE096C: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82FE0970: 419A033C  beq cr6, 0x82fe0cac
	if ctx.cr[6].eq {
	pc = 0x82FE0CAC; continue 'dispatch;
	}
	// 82FE0974: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 82FE0978: 419A0334  beq cr6, 0x82fe0cac
	if ctx.cr[6].eq {
	pc = 0x82FE0CAC; continue 'dispatch;
	}
	// 82FE097C: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82FE0980: 4199032C  bgt cr6, 0x82fe0cac
	if ctx.cr[6].gt {
	pc = 0x82FE0CAC; continue 'dispatch;
	}
	// 82FE0984: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 82FE0988: 4099002C  ble cr6, 0x82fe09b4
	if !ctx.cr[6].gt {
	pc = 0x82FE09B4; continue 'dispatch;
	}
	// 82FE098C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0990: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE0994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0998: 816B007C  lwz r11, 0x7c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 82FE099C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE09A0: 4E800421  bctrl
	ctx.lr = 0x82FE09A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE09A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE09A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE09AC: 4BFFF5ED  bl 0x82fdff98
	ctx.lr = 0x82FE09B0;
	sub_82FDFF98(ctx, base);
	// 82FE09B0: 48000300  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE09B4: 7F83E379  or. r3, r28, r28
	ctx.r[3].u64 = ctx.r[28].u64 | ctx.r[28].u64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82FE09B8: 7F9AE378  mr r26, r28
	ctx.r[26].u64 = ctx.r[28].u64;
	// 82FE09BC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FE09C0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FE09C4: 4182002C  beq 0x82fe09f0
	if ctx.cr[0].eq {
	pc = 0x82FE09F0; continue 'dispatch;
	}
	// 82FE09C8: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82FE09CC: 7F03F840  cmplw cr6, r3, r31
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82FE09D0: 419A00EC  beq cr6, 0x82fe0abc
	if ctx.cr[6].eq {
	pc = 0x82FE0ABC; continue 'dispatch;
	}
	// 82FE09D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE09D8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FE09DC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE09E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE09E4: 4E800421  bctrl
	ctx.lr = 0x82FE09E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE09E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE09EC: 4082FFDC  bne 0x82fe09c8
	if !ctx.cr[0].eq {
	pc = 0x82FE09C8; continue 'dispatch;
	}
	// 82FE09F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE09F4: 3BDDFFFC  addi r30, r29, -4
	ctx.r[30].s64 = ctx.r[29].s64 + -4;
	// 82FE09F8: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82FE09FC: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FE0A00: 419A00C4  beq cr6, 0x82fe0ac4
	if ctx.cr[6].eq {
	pc = 0x82FE0AC4; continue 'dispatch;
	}
	// 82FE0A04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0A08: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE0A0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0A14: 4E800421  bctrl
	ctx.lr = 0x82FE0A18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0A18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0A1C: 4082FFDC  bne 0x82fe09f8
	if !ctx.cr[0].eq {
	pc = 0x82FE09F8; continue 'dispatch;
	}
	// 82FE0A20: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0A24: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82FE0A28: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FE0A2C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE0A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0A34: 4E800421  bctrl
	ctx.lr = 0x82FE0A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0A38: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0A40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE0A44: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE0A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0A4C: 4E800421  bctrl
	ctx.lr = 0x82FE0A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0A50: 7FFF0734  extsh r31, r31
	ctx.r[31].s64 = ctx.r[31].s16 as i64;
	// 82FE0A54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0A58: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 82FE0A5C: 409A001C  bne cr6, 0x82fe0a78
	if !ctx.cr[6].eq {
	pc = 0x82FE0A78; continue 'dispatch;
	}
	// 82FE0A60: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0A64: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FE0A68: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FE0A6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0A70: 4E800421  bctrl
	ctx.lr = 0x82FE0A74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0A74: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE0A78: 7FDE0734  extsh r30, r30
	ctx.r[30].s64 = ctx.r[30].s16 as i64;
	// 82FE0A7C: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82FE0A80: 409A001C  bne cr6, 0x82fe0a9c
	if !ctx.cr[6].eq {
	pc = 0x82FE0A9C; continue 'dispatch;
	}
	// 82FE0A84: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0A88: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE0A8C: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FE0A90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0A94: 4E800421  bctrl
	ctx.lr = 0x82FE0A98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0A98: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE0A9C: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 82FE0AA0: 409A0098  bne cr6, 0x82fe0b38
	if !ctx.cr[6].eq {
	pc = 0x82FE0B38; continue 'dispatch;
	}
	// 82FE0AA4: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82FE0AA8: 409A0024  bne cr6, 0x82fe0acc
	if !ctx.cr[6].eq {
	pc = 0x82FE0ACC; continue 'dispatch;
	}
	// 82FE0AAC: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FE0AB0: 409A001C  bne cr6, 0x82fe0acc
	if !ctx.cr[6].eq {
	pc = 0x82FE0ACC; continue 'dispatch;
	}
	// 82FE0AB4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FE0AB8: 480001F8  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE0ABC: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82FE0AC0: 480001F0  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE0AC4: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82FE0AC8: 480001E8  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE0ACC: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 82FE0AD0: 409A0068  bne cr6, 0x82fe0b38
	if !ctx.cr[6].eq {
	pc = 0x82FE0B38; continue 'dispatch;
	}
	// 82FE0AD4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FE0AD8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE0ADC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE0AE0: 419A002C  beq cr6, 0x82fe0b0c
	if ctx.cr[6].eq {
	pc = 0x82FE0B0C; continue 'dispatch;
	}
	// 82FE0AE4: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82FE0AE8: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FE0AEC: 419A00F4  beq cr6, 0x82fe0be0
	if ctx.cr[6].eq {
	pc = 0x82FE0BE0; continue 'dispatch;
	}
	// 82FE0AF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0AF4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FE0AF8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0B00: 4E800421  bctrl
	ctx.lr = 0x82FE0B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0B04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0B08: 4082FFDC  bne 0x82fe0ae4
	if !ctx.cr[0].eq {
	pc = 0x82FE0AE4; continue 'dispatch;
	}
	// 82FE0B0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE0B10: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE0B14: 419A0024  beq cr6, 0x82fe0b38
	if ctx.cr[6].eq {
	pc = 0x82FE0B38; continue 'dispatch;
	}
	// 82FE0B18: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FE0B1C: 419A00CC  beq cr6, 0x82fe0be8
	if ctx.cr[6].eq {
	pc = 0x82FE0BE8; continue 'dispatch;
	}
	// 82FE0B20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0B24: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0B28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0B2C: 4E800421  bctrl
	ctx.lr = 0x82FE0B30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0B30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0B34: 4082FFE4  bne 0x82fe0b18
	if !ctx.cr[0].eq {
	pc = 0x82FE0B18; continue 'dispatch;
	}
	// 82FE0B38: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82FE0B3C: 409A0068  bne cr6, 0x82fe0ba4
	if !ctx.cr[6].eq {
	pc = 0x82FE0BA4; continue 'dispatch;
	}
	// 82FE0B40: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82FE0B44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE0B48: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE0B4C: 419A002C  beq cr6, 0x82fe0b78
	if ctx.cr[6].eq {
	pc = 0x82FE0B78; continue 'dispatch;
	}
	// 82FE0B50: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 82FE0B54: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FE0B58: 419A0090  beq cr6, 0x82fe0be8
	if ctx.cr[6].eq {
	pc = 0x82FE0BE8; continue 'dispatch;
	}
	// 82FE0B5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0B60: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE0B64: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0B68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0B6C: 4E800421  bctrl
	ctx.lr = 0x82FE0B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0B70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0B74: 4082FFDC  bne 0x82fe0b50
	if !ctx.cr[0].eq {
	pc = 0x82FE0B50; continue 'dispatch;
	}
	// 82FE0B78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE0B7C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE0B80: 419A0024  beq cr6, 0x82fe0ba4
	if ctx.cr[6].eq {
	pc = 0x82FE0BA4; continue 'dispatch;
	}
	// 82FE0B84: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FE0B88: 419A0058  beq cr6, 0x82fe0be0
	if ctx.cr[6].eq {
	pc = 0x82FE0BE0; continue 'dispatch;
	}
	// 82FE0B8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0B90: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0B94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0B98: 4E800421  bctrl
	ctx.lr = 0x82FE0B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0B9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0BA0: 4082FFE4  bne 0x82fe0b84
	if !ctx.cr[0].eq {
	pc = 0x82FE0B84; continue 'dispatch;
	}
	// 82FE0BA4: 7F1AD840  cmplw cr6, r26, r27
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82FE0BA8: 409A0104  bne cr6, 0x82fe0cac
	if !ctx.cr[6].eq {
	pc = 0x82FE0CAC; continue 'dispatch;
	}
	// 82FE0BAC: 7F18C800  cmpw cr6, r24, r25
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82FE0BB0: 40990040  ble cr6, 0x82fe0bf0
	if !ctx.cr[6].gt {
	pc = 0x82FE0BF0; continue 'dispatch;
	}
	// 82FE0BB4: 7FF9C051  subf. r31, r25, r24
	ctx.r[31].s64 = ctx.r[24].s64 - ctx.r[25].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FE0BB8: 40810060  ble 0x82fe0c18
	if !ctx.cr[0].gt {
	pc = 0x82FE0C18; continue 'dispatch;
	}
	// 82FE0BBC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0BC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE0BC4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0BC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0BCC: 4E800421  bctrl
	ctx.lr = 0x82FE0BD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0BD0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FE0BD4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE0BD8: 4082FFE4  bne 0x82fe0bbc
	if !ctx.cr[0].eq {
	pc = 0x82FE0BBC; continue 'dispatch;
	}
	// 82FE0BDC: 4800003C  b 0x82fe0c18
	pc = 0x82FE0C18; continue 'dispatch;
	// 82FE0BE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE0BE4: 480000CC  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE0BE8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82FE0BEC: 480000C4  b 0x82fe0cb0
	pc = 0x82FE0CB0; continue 'dispatch;
	// 82FE0BF0: 7FF8C851  subf. r31, r24, r25
	ctx.r[31].s64 = ctx.r[25].s64 - ctx.r[24].s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FE0BF4: 40810024  ble 0x82fe0c18
	if !ctx.cr[0].gt {
	pc = 0x82FE0C18; continue 'dispatch;
	}
	// 82FE0BF8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0BFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE0C00: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0C08: 4E800421  bctrl
	ctx.lr = 0x82FE0C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0C0C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FE0C10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE0C14: 4082FFE4  bne 0x82fe0bf8
	if !ctx.cr[0].eq {
	pc = 0x82FE0BF8; continue 'dispatch;
	}
	// 82FE0C18: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0C1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE0C20: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0C28: 4E800421  bctrl
	ctx.lr = 0x82FE0C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0C2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0C30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0C34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE0C38: 48000024  b 0x82fe0c5c
	pc = 0x82FE0C5C; continue 'dispatch;
	// 82FE0C3C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0C40: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82FE0C44: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82FE0C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0C4C: 4E800421  bctrl
	ctx.lr = 0x82FE0C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0C50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0C54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0C58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0C5C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE0C60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0C64: 4E800421  bctrl
	ctx.lr = 0x82FE0C68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0C68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0C6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0C74: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FE0C78: 409AFFC4  bne cr6, 0x82fe0c3c
	if !ctx.cr[6].eq {
	pc = 0x82FE0C3C; continue 'dispatch;
	}
	// 82FE0C7C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE0C80: 4800001C  b 0x82fe0c9c
	pc = 0x82FE0C9C; continue 'dispatch;
	// 82FE0C84: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FE0C88: 419AFF58  beq cr6, 0x82fe0be0
	if ctx.cr[6].eq {
	pc = 0x82FE0BE0; continue 'dispatch;
	}
	// 82FE0C8C: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FE0C90: 419AFF58  beq cr6, 0x82fe0be8
	if ctx.cr[6].eq {
	pc = 0x82FE0BE8; continue 'dispatch;
	}
	// 82FE0C94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE0C98: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE0C9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE0CA0: 4E800421  bctrl
	ctx.lr = 0x82FE0CA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE0CA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE0CA8: 4082FFDC  bne 0x82fe0c84
	if !ctx.cr[0].eq {
	pc = 0x82FE0C84; continue 'dispatch;
	}
	// 82FE0CAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE0CB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FE0CB4: 481C74F4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0CB8 size=104
    let mut pc: u32 = 0x82FE0CB8;
    'dispatch: loop {
        match pc {
            0x82FE0CB8 => {
    //   block [0x82FE0CB8..0x82FE0D20)
	// 82FE0CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0CBC: 481C74B1  bl 0x831a816c
	ctx.lr = 0x82FE0CC0;
	sub_831A8130(ctx, base);
	// 82FE0CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0CC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE0CC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE0CCC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE0CD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0CD4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FE0CD8: 4BFFF309  bl 0x82fdffe0
	ctx.lr = 0x82FE0CDC;
	sub_82FDFFE0(ctx, base);
	// 82FE0CDC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE0CE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0CE4: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82FE0CE8: 4BFFE541  bl 0x82fdf228
	ctx.lr = 0x82FE0CEC;
	sub_82FDF228(ctx, base);
	// 82FE0CEC: 57C4083C  slwi r4, r30, 1
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE0CF0: 48000D01  bl 0x82fe19f0
	ctx.lr = 0x82FE0CF4;
	sub_82FE19F0(ctx, base);
	// 82FE0CF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0CF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82FE0CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0D00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE0D04: 4BFFF2DD  bl 0x82fdffe0
	ctx.lr = 0x82FE0D08;
	sub_82FDFFE0(ctx, base);
	// 82FE0D08: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE0D0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0D10: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE0D14: 7FABF32E  sthx r29, r11, r30
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u16) };
	// 82FE0D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE0D1C: 481C74A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0D20 size=8
    let mut pc: u32 = 0x82FE0D20;
    'dispatch: loop {
        match pc {
            0x82FE0D20 => {
    //   block [0x82FE0D20..0x82FE0D28)
	// 82FE0D20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE0D24: 8213A7B8  lwz r16, -0x5848(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-22600 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0D28 size=72
    let mut pc: u32 = 0x82FE0D28;
    'dispatch: loop {
        match pc {
            0x82FE0D28 => {
    //   block [0x82FE0D28..0x82FE0D70)
	// 82FE0D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0D2C: 481C7441  bl 0x831a816c
	ctx.lr = 0x82FE0D30;
	sub_831A8130(ctx, base);
	// 82FE0D30: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE0D34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0D38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0D3C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FE0D40: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FE0D44: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE0D48: 4BFF81E9  bl 0x82fd8f30
	ctx.lr = 0x82FE0D4C;
	sub_82FD8F30(ctx, base);
	// 82FE0D4C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0D50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE0D54: 396BA7A0  addi r11, r11, -0x5860
	ctx.r[11].s64 = ctx.r[11].s64 + -22624;
	// 82FE0D58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0D5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0D60: 4BFF8559  bl 0x82fd92b8
	ctx.lr = 0x82FE0D64;
	sub_82FD92B8(ctx, base);
	// 82FE0D64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0D68: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE0D6C: 481C7450  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0D70 size=40
    let mut pc: u32 = 0x82FE0D70;
    'dispatch: loop {
        match pc {
            0x82FE0D70 => {
    //   block [0x82FE0D70..0x82FE0D98)
	// 82FE0D70: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE0D74: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0D78: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0D7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0D80: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE0D84: 4BFF80F5  bl 0x82fd8e78
	ctx.lr = 0x82FE0D88;
	sub_82FD8E78(ctx, base);
	// 82FE0D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE0D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0D98 size=60
    let mut pc: u32 = 0x82FE0D98;
    'dispatch: loop {
        match pc {
            0x82FE0D98 => {
    //   block [0x82FE0D98..0x82FE0DD4)
	// 82FE0D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0DA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0DA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0DA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0DAC: 4BFF81FD  bl 0x82fd8fa8
	ctx.lr = 0x82FE0DB0;
	sub_82FD8FA8(ctx, base);
	// 82FE0DB0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0DB8: 396BA7A0  addi r11, r11, -0x5860
	ctx.r[11].s64 = ctx.r[11].s64 + -22624;
	// 82FE0DBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE0DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE0DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0DD8 size=16
    let mut pc: u32 = 0x82FE0DD8;
    'dispatch: loop {
        match pc {
            0x82FE0DD8 => {
    //   block [0x82FE0DD8..0x82FE0DE8)
	// 82FE0DD8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0DDC: 396BA7A0  addi r11, r11, -0x5860
	ctx.r[11].s64 = ctx.r[11].s64 + -22624;
	// 82FE0DE0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0DE4: 4BFF8094  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0DE8 size=8
    let mut pc: u32 = 0x82FE0DE8;
    'dispatch: loop {
        match pc {
            0x82FE0DE8 => {
    //   block [0x82FE0DE8..0x82FE0DF0)
	// 82FE0DE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE0DEC: 8213A7F0  lwz r16, -0x5810(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-22544 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0DF0 size=92
    let mut pc: u32 = 0x82FE0DF0;
    'dispatch: loop {
        match pc {
            0x82FE0DF0 => {
    //   block [0x82FE0DF0..0x82FE0E4C)
	// 82FE0DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0DF4: 481C7379  bl 0x831a816c
	ctx.lr = 0x82FE0DF8;
	sub_831A8130(ctx, base);
	// 82FE0DF8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE0DFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0E00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE0E04: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE0E08: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE0E0C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE0E10: 4BFF7489  bl 0x82fd8298
	ctx.lr = 0x82FE0E14;
	sub_82FD8298(ctx, base);
	// 82FE0E14: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE0E18: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FE0E1C: 41820024  beq 0x82fe0e40
	if ctx.cr[0].eq {
	pc = 0x82FE0E40; continue 'dispatch;
	}
	// 82FE0E20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE0E24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0E28: 4BFF8181  bl 0x82fd8fa8
	ctx.lr = 0x82FE0E2C;
	sub_82FD8FA8(ctx, base);
	// 82FE0E2C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0E30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0E34: 396BA7A0  addi r11, r11, -0x5860
	ctx.r[11].s64 = ctx.r[11].s64 + -22624;
	// 82FE0E38: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0E3C: 48000008  b 0x82fe0e44
	pc = 0x82FE0E44; continue 'dispatch;
	// 82FE0E40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE0E44: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE0E48: 481C7374  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0E4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0E4C size=48
    let mut pc: u32 = 0x82FE0E4C;
    'dispatch: loop {
        match pc {
            0x82FE0E4C => {
    //   block [0x82FE0E4C..0x82FE0E7C)
	// 82FE0E4C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE0E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0E5C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE0E60: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE0E64: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE0E68: 4BFF7479  bl 0x82fd82e0
	ctx.lr = 0x82FE0E6C;
	sub_82FD82E0(ctx, base);
	// 82FE0E6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE0E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0E80 size=12
    let mut pc: u32 = 0x82FE0E80;
    'dispatch: loop {
        match pc {
            0x82FE0E80 => {
    //   block [0x82FE0E80..0x82FE0E8C)
	// 82FE0E80: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0E84: 386B8320  addi r3, r11, -0x7ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -31968;
	// 82FE0E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0E90 size=88
    let mut pc: u32 = 0x82FE0E90;
    'dispatch: loop {
        match pc {
            0x82FE0E90 => {
    //   block [0x82FE0E90..0x82FE0EE8)
	// 82FE0E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0E98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE0E9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0EA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0EA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0EA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0EAC: 396BA7A0  addi r11, r11, -0x5860
	ctx.r[11].s64 = ctx.r[11].s64 + -22624;
	// 82FE0EB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE0EB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0EB8: 4BFF7FC1  bl 0x82fd8e78
	ctx.lr = 0x82FE0EBC;
	sub_82FD8E78(ctx, base);
	// 82FE0EBC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE0EC0: 4182000C  beq 0x82fe0ecc
	if ctx.cr[0].eq {
	pc = 0x82FE0ECC; continue 'dispatch;
	}
	// 82FE0EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0EC8: 4BFF7419  bl 0x82fd82e0
	ctx.lr = 0x82FE0ECC;
	sub_82FD82E0(ctx, base);
	// 82FE0ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0ED0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE0ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE0ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0EDC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE0EE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE0EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0EE8 size=8
    let mut pc: u32 = 0x82FE0EE8;
    'dispatch: loop {
        match pc {
            0x82FE0EE8 => {
    //   block [0x82FE0EE8..0x82FE0EF0)
	// 82FE0EE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE0EEC: 8213A838  lwz r16, -0x57c8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-22472 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0EF0 size=72
    let mut pc: u32 = 0x82FE0EF0;
    'dispatch: loop {
        match pc {
            0x82FE0EF0 => {
    //   block [0x82FE0EF0..0x82FE0F38)
	// 82FE0EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0EF4: 481C7279  bl 0x831a816c
	ctx.lr = 0x82FE0EF8;
	sub_831A8130(ctx, base);
	// 82FE0EF8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE0EFC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0F00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE0F04: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FE0F08: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82FE0F0C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE0F10: 4BFF8021  bl 0x82fd8f30
	ctx.lr = 0x82FE0F14;
	sub_82FD8F30(ctx, base);
	// 82FE0F14: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0F18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE0F1C: 396BA820  addi r11, r11, -0x57e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22496;
	// 82FE0F20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0F24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0F28: 4BFF8391  bl 0x82fd92b8
	ctx.lr = 0x82FE0F2C;
	sub_82FD92B8(ctx, base);
	// 82FE0F2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0F30: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE0F34: 481C7288  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0F38 size=40
    let mut pc: u32 = 0x82FE0F38;
    'dispatch: loop {
        match pc {
            0x82FE0F38 => {
    //   block [0x82FE0F38..0x82FE0F60)
	// 82FE0F38: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE0F3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0F40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0F48: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE0F4C: 4BFF7F2D  bl 0x82fd8e78
	ctx.lr = 0x82FE0F50;
	sub_82FD8E78(ctx, base);
	// 82FE0F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE0F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0F60 size=60
    let mut pc: u32 = 0x82FE0F60;
    'dispatch: loop {
        match pc {
            0x82FE0F60 => {
    //   block [0x82FE0F60..0x82FE0F9C)
	// 82FE0F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE0F68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE0F6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0F70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE0F74: 4BFF8035  bl 0x82fd8fa8
	ctx.lr = 0x82FE0F78;
	sub_82FD8FA8(ctx, base);
	// 82FE0F78: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE0F80: 396BA820  addi r11, r11, -0x57e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22496;
	// 82FE0F84: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE0F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE0F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE0F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE0F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0FA0 size=16
    let mut pc: u32 = 0x82FE0FA0;
    'dispatch: loop {
        match pc {
            0x82FE0FA0 => {
    //   block [0x82FE0FA0..0x82FE0FB0)
	// 82FE0FA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0FA4: 396BA820  addi r11, r11, -0x57e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22496;
	// 82FE0FA8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE0FAC: 4BFF7ECC  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE0FB0 size=8
    let mut pc: u32 = 0x82FE0FB0;
    'dispatch: loop {
        match pc {
            0x82FE0FB0 => {
    //   block [0x82FE0FB0..0x82FE0FB8)
	// 82FE0FB0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE0FB4: 8213A870  lwz r16, -0x5790(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-22416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE0FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE0FB8 size=92
    let mut pc: u32 = 0x82FE0FB8;
    'dispatch: loop {
        match pc {
            0x82FE0FB8 => {
    //   block [0x82FE0FB8..0x82FE1014)
	// 82FE0FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE0FBC: 481C71B1  bl 0x831a816c
	ctx.lr = 0x82FE0FC0;
	sub_831A8130(ctx, base);
	// 82FE0FC0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE0FC4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE0FC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE0FCC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE0FD0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE0FD4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE0FD8: 4BFF72C1  bl 0x82fd8298
	ctx.lr = 0x82FE0FDC;
	sub_82FD8298(ctx, base);
	// 82FE0FDC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE0FE0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FE0FE4: 41820024  beq 0x82fe1008
	if ctx.cr[0].eq {
	pc = 0x82FE1008; continue 'dispatch;
	}
	// 82FE0FE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE0FEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0FF0: 4BFF7FB9  bl 0x82fd8fa8
	ctx.lr = 0x82FE0FF4;
	sub_82FD8FA8(ctx, base);
	// 82FE0FF4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE0FF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE0FFC: 396BA820  addi r11, r11, -0x57e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22496;
	// 82FE1000: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE1004: 48000008  b 0x82fe100c
	pc = 0x82FE100C; continue 'dispatch;
	// 82FE1008: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE100C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE1010: 481C71AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1014 size=48
    let mut pc: u32 = 0x82FE1014;
    'dispatch: loop {
        match pc {
            0x82FE1014 => {
    //   block [0x82FE1014..0x82FE1044)
	// 82FE1014: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE1018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE101C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1024: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE1028: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE102C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE1030: 4BFF72B1  bl 0x82fd82e0
	ctx.lr = 0x82FE1034;
	sub_82FD82E0(ctx, base);
	// 82FE1034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE1038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE103C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1048 size=12
    let mut pc: u32 = 0x82FE1048;
    'dispatch: loop {
        match pc {
            0x82FE1048 => {
    //   block [0x82FE1048..0x82FE1054)
	// 82FE1048: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE104C: 386B8350  addi r3, r11, -0x7cb0
	ctx.r[3].s64 = ctx.r[11].s64 + -31920;
	// 82FE1050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1058 size=88
    let mut pc: u32 = 0x82FE1058;
    'dispatch: loop {
        match pc {
            0x82FE1058 => {
    //   block [0x82FE1058..0x82FE10B0)
	// 82FE1058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1060: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE1064: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1068: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE106C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE1070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE1074: 396BA820  addi r11, r11, -0x57e0
	ctx.r[11].s64 = ctx.r[11].s64 + -22496;
	// 82FE1078: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE107C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE1080: 4BFF7DF9  bl 0x82fd8e78
	ctx.lr = 0x82FE1084;
	sub_82FD8E78(ctx, base);
	// 82FE1084: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE1088: 4182000C  beq 0x82fe1094
	if ctx.cr[0].eq {
	pc = 0x82FE1094; continue 'dispatch;
	}
	// 82FE108C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1090: 4BFF7251  bl 0x82fd82e0
	ctx.lr = 0x82FE1094;
	sub_82FD82E0(ctx, base);
	// 82FE1094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1098: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE109C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE10A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE10A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE10A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE10AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE10B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE10B0 size=68
    let mut pc: u32 = 0x82FE10B0;
    'dispatch: loop {
        match pc {
            0x82FE10B0 => {
    //   block [0x82FE10B0..0x82FE10F4)
	// 82FE10B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE10B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE10B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE10BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE10C0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE10C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE10C8: 396BA8A0  addi r11, r11, -0x5760
	ctx.r[11].s64 = ctx.r[11].s64 + -22368;
	// 82FE10CC: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FE10D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE10D4: 41820008  beq 0x82fe10dc
	if ctx.cr[0].eq {
	pc = 0x82FE10DC; continue 'dispatch;
	}
	// 82FE10D8: 4B2DF191  bl 0x822c0268
	ctx.lr = 0x82FE10DC;
	sub_822C0268(ctx, base);
	// 82FE10DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE10E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE10E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE10E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE10EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE10F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE10F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE10F8 size=68
    let mut pc: u32 = 0x82FE10F8;
    'dispatch: loop {
        match pc {
            0x82FE10F8 => {
    //   block [0x82FE10F8..0x82FE113C)
	// 82FE10F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE10FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1108: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE110C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE1110: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FE1114: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FE1118: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE111C: 41820008  beq 0x82fe1124
	if ctx.cr[0].eq {
	pc = 0x82FE1124; continue 'dispatch;
	}
	// 82FE1120: 4B2DF149  bl 0x822c0268
	ctx.lr = 0x82FE1124;
	sub_822C0268(ctx, base);
	// 82FE1124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE112C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1140 size=88
    let mut pc: u32 = 0x82FE1140;
    'dispatch: loop {
        match pc {
            0x82FE1140 => {
    //   block [0x82FE1140..0x82FE1198)
	// 82FE1140: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE1144: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE1148: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE114C: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FE1150: 3CE08214  lis r7, -0x7dec
	ctx.r[7].s64 = -2112618496;
	// 82FE1154: 396BA9F8  addi r11, r11, -0x5608
	ctx.r[11].s64 = ctx.r[11].s64 + -22024;
	// 82FE1158: 394AA9EC  addi r10, r10, -0x5614
	ctx.r[10].s64 = ctx.r[10].s64 + -22036;
	// 82FE115C: 3929A950  addi r9, r9, -0x56b0
	ctx.r[9].s64 = ctx.r[9].s64 + -22192;
	// 82FE1160: 3CC08214  lis r6, -0x7dec
	ctx.r[6].s64 = -2112618496;
	// 82FE1164: 3908A8A0  addi r8, r8, -0x5760
	ctx.r[8].s64 = ctx.r[8].s64 + -22368;
	// 82FE1168: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82FE116C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE1170: 38E7D930  addi r7, r7, -0x26d0
	ctx.r[7].s64 = ctx.r[7].s64 + -9936;
	// 82FE1174: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE1178: 38C6A93C  addi r6, r6, -0x56c4
	ctx.r[6].s64 = ctx.r[6].s64 + -22212;
	// 82FE117C: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82FE1180: 38A57840  addi r5, r5, 0x7840
	ctx.r[5].s64 = ctx.r[5].s64 + 30784;
	// 82FE1184: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FE1188: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FE118C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82FE1190: 90A30000  stw r5, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FE1194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1198 size=140
    let mut pc: u32 = 0x82FE1198;
    'dispatch: loop {
        match pc {
            0x82FE1198 => {
    //   block [0x82FE1198..0x82FE1224)
	// 82FE1198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE119C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE11A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE11A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE11A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE11AC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE11B0: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE11B4: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FE11B8: 3CE08214  lis r7, -0x7dec
	ctx.r[7].s64 = -2112618496;
	// 82FE11BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE11C0: 396BA9F8  addi r11, r11, -0x5608
	ctx.r[11].s64 = ctx.r[11].s64 + -22024;
	// 82FE11C4: 394AA9EC  addi r10, r10, -0x5614
	ctx.r[10].s64 = ctx.r[10].s64 + -22036;
	// 82FE11C8: 3929A950  addi r9, r9, -0x56b0
	ctx.r[9].s64 = ctx.r[9].s64 + -22192;
	// 82FE11CC: 3CC08214  lis r6, -0x7dec
	ctx.r[6].s64 = -2112618496;
	// 82FE11D0: 3908A8A0  addi r8, r8, -0x5760
	ctx.r[8].s64 = ctx.r[8].s64 + -22368;
	// 82FE11D4: 3CA08213  lis r5, -0x7ded
	ctx.r[5].s64 = -2112684032;
	// 82FE11D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE11DC: 38E7D930  addi r7, r7, -0x26d0
	ctx.r[7].s64 = ctx.r[7].s64 + -9936;
	// 82FE11E0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE11E4: 38C6A93C  addi r6, r6, -0x56c4
	ctx.r[6].s64 = ctx.r[6].s64 + -22212;
	// 82FE11E8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82FE11EC: 38A57840  addi r5, r5, 0x7840
	ctx.r[5].s64 = ctx.r[5].s64 + 30784;
	// 82FE11F0: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82FE11F4: 548407FF  clrlwi. r4, r4, 0x1f
	ctx.r[4].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FE11F8: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82FE11FC: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82FE1200: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82FE1204: 41820008  beq 0x82fe120c
	if ctx.cr[0].eq {
	pc = 0x82FE120C; continue 'dispatch;
	}
	// 82FE1208: 4B2DF061  bl 0x822c0268
	ctx.lr = 0x82FE120C;
	sub_822C0268(ctx, base);
	// 82FE120C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE1214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE121C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1228 size=8
    let mut pc: u32 = 0x82FE1228;
    'dispatch: loop {
        match pc {
            0x82FE1228 => {
    //   block [0x82FE1228..0x82FE1230)
	// 82FE1228: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82FE122C: 48018DDC  b 0x82ffa008
	sub_82FFA008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1230 size=200
    let mut pc: u32 = 0x82FE1230;
    'dispatch: loop {
        match pc {
            0x82FE1230 => {
    //   block [0x82FE1230..0x82FE12F8)
	// 82FE1230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE123C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1244: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE1248: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE124C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE1250: 419A0090  beq cr6, 0x82fe12e0
	if ctx.cr[6].eq {
	pc = 0x82FE12E0; continue 'dispatch;
	}
	// 82FE1254: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE125C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE1260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1264: 4E800421  bctrl
	ctx.lr = 0x82FE1268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1268: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE126C: 41820044  beq 0x82fe12b0
	if ctx.cr[0].eq {
	pc = 0x82FE12B0; continue 'dispatch;
	}
	// 82FE1270: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1278: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE127C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1280: 4E800421  bctrl
	ctx.lr = 0x82FE1284;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1284: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FE1288: 419A0028  beq cr6, 0x82fe12b0
	if ctx.cr[6].eq {
	pc = 0x82FE12B0; continue 'dispatch;
	}
	// 82FE128C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE1290: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1294: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FE1298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE129C: 48018C35  bl 0x82ff9ed0
	ctx.lr = 0x82FE12A0;
	sub_82FF9ED0(ctx, base);
	// 82FE12A0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE12A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE12A8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE12AC: 481CF97D  bl 0x831b0c28
	ctx.lr = 0x82FE12B0;
	sub_831B0C28(ctx, base);
	// 82FE12B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE12B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE12B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE12BC: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE12C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE12C4: 4E800421  bctrl
	ctx.lr = 0x82FE12C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE12C8: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82FE12CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE12D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE12D4: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE12D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE12DC: 4E800421  bctrl
	ctx.lr = 0x82FE12E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE12E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE12E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE12E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE12EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE12F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE12F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE12F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE12F8 size=12
    let mut pc: u32 = 0x82FE12F8;
    'dispatch: loop {
        match pc {
            0x82FE12F8 => {
    //   block [0x82FE12F8..0x82FE1304)
	// 82FE12F8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE12FC: 386BAA08  addi r3, r11, -0x55f8
	ctx.r[3].s64 = ctx.r[11].s64 + -22008;
	// 82FE1300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1308 size=8
    let mut pc: u32 = 0x82FE1308;
    'dispatch: loop {
        match pc {
            0x82FE1308 => {
    //   block [0x82FE1308..0x82FE1310)
	// 82FE1308: 38600009  li r3, 9
	ctx.r[3].s64 = 9;
	// 82FE130C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1310 size=8
    let mut pc: u32 = 0x82FE1310;
    'dispatch: loop {
        match pc {
            0x82FE1310 => {
    //   block [0x82FE1310..0x82FE1318)
	// 82FE1310: 8063008C  lwz r3, 0x8c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE1314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1318 size=48
    let mut pc: u32 = 0x82FE1318;
    'dispatch: loop {
        match pc {
            0x82FE1318 => {
    //   block [0x82FE1318..0x82FE1348)
	// 82FE1318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE131C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1324: 80C3008C  lwz r6, 0x8c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE1328: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE132C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE1330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1334: 48018B9D  bl 0x82ff9ed0
	ctx.lr = 0x82FE1338;
	sub_82FF9ED0(ctx, base);
	// 82FE1338: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE133C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1340: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE1344: 481CF8E5  bl 0x831b0c28
	ctx.lr = 0x82FE1348;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1348 size=8
    let mut pc: u32 = 0x82FE1348;
    'dispatch: loop {
        match pc {
            0x82FE1348 => {
    //   block [0x82FE1348..0x82FE1350)
	// 82FE1348: 80630078  lwz r3, 0x78(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(120 as u32) ) } as u64;
	// 82FE134C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1350 size=4
    let mut pc: u32 = 0x82FE1350;
    'dispatch: loop {
        match pc {
            0x82FE1350 => {
    //   block [0x82FE1350..0x82FE1354)
	// 82FE1350: 4800EA88  b 0x82fefdd8
	sub_82FEFDD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1358 size=336
    let mut pc: u32 = 0x82FE1358;
    'dispatch: loop {
        match pc {
            0x82FE1358 => {
    //   block [0x82FE1358..0x82FE14A8)
	// 82FE1358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE135C: 481C6E11  bl 0x831a816c
	ctx.lr = 0x82FE1360;
	sub_831A8130(ctx, base);
	// 82FE1360: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1364: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE1368: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE136C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE1370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1374: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1378: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE137C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1380: 4E800421  bctrl
	ctx.lr = 0x82FE1384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1384: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1388: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE138C: 409A0010  bne cr6, 0x82fe139c
	if !ctx.cr[6].eq {
	pc = 0x82FE139C; continue 'dispatch;
	}
	// 82FE1390: 817E0070  lwz r11, 0x70(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE1394: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE1398: 409A0030  bne cr6, 0x82fe13c8
	if !ctx.cr[6].eq {
	pc = 0x82FE13C8; continue 'dispatch;
	}
	// 82FE139C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE13A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE13A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE13A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE13AC: 4E800421  bctrl
	ctx.lr = 0x82FE13B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE13B0: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE13B4: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FE13B8: 409A0034  bne cr6, 0x82fe13ec
	if !ctx.cr[6].eq {
	pc = 0x82FE13EC; continue 'dispatch;
	}
	// 82FE13BC: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE13C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE13C4: 419A0028  beq cr6, 0x82fe13ec
	if ctx.cr[6].eq {
	pc = 0x82FE13EC; continue 'dispatch;
	}
	// 82FE13C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE13CC: 80DE0084  lwz r6, 0x84(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE13D0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FE13D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE13D8: 48018AF9  bl 0x82ff9ed0
	ctx.lr = 0x82FE13DC;
	sub_82FF9ED0(ctx, base);
	// 82FE13DC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE13E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE13E4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE13E8: 481CF841  bl 0x831b0c28
	ctx.lr = 0x82FE13EC;
	sub_831B0C28(ctx, base);
	// 82FE13EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE13F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE13F4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE13F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE13FC: 4E800421  bctrl
	ctx.lr = 0x82FE1400;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1400: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1404: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FE1408: 409A0038  bne cr6, 0x82fe1440
	if !ctx.cr[6].eq {
	pc = 0x82FE1440; continue 'dispatch;
	}
	// 82FE140C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1414: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE1418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE141C: 4E800421  bctrl
	ctx.lr = 0x82FE1420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1420: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1424: 4082001C  bne 0x82fe1440
	if !ctx.cr[0].eq {
	pc = 0x82FE1440; continue 'dispatch;
	}
	// 82FE1428: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE142C: 389EFFF4  addi r4, r30, -0xc
	ctx.r[4].s64 = ctx.r[30].s64 + -12;
	// 82FE1430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1434: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE1438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE143C: 4E800421  bctrl
	ctx.lr = 0x82FE1440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1440: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE1444: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE1448: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82FE144C: 4801B2FD  bl 0x82ffc748
	ctx.lr = 0x82FE1450;
	sub_82FFC748(ctx, base);
	// 82FE1450: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1458: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE145C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1460: 4E800421  bctrl
	ctx.lr = 0x82FE1464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1464: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1468: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE146C: 409A000C  bne cr6, 0x82fe1478
	if !ctx.cr[6].eq {
	pc = 0x82FE1478; continue 'dispatch;
	}
	// 82FE1470: 93FE0070  stw r31, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82FE1474: 48000028  b 0x82fe149c
	pc = 0x82FE149C; continue 'dispatch;
	// 82FE1478: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE147C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1480: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1484: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1488: 4E800421  bctrl
	ctx.lr = 0x82FE148C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE148C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1490: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FE1494: 409A0008  bne cr6, 0x82fe149c
	if !ctx.cr[6].eq {
	pc = 0x82FE149C; continue 'dispatch;
	}
	// 82FE1498: 93FE006C  stw r31, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82FE149C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE14A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE14A4: 481C6D18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE14A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE14A8 size=8
    let mut pc: u32 = 0x82FE14A8;
    'dispatch: loop {
        match pc {
            0x82FE14A8 => {
    //   block [0x82FE14A8..0x82FE14B0)
	// 82FE14A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE14AC: 8213AA64  lwz r16, -0x559c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-21916 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE14B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE14B0 size=312
    let mut pc: u32 = 0x82FE14B0;
    'dispatch: loop {
        match pc {
            0x82FE14B0 => {
    //   block [0x82FE14B0..0x82FE15E8)
	// 82FE14B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE14B4: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 82FE14B8: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 82FE14BC: 481C6CAD  bl 0x831a8168
	ctx.lr = 0x82FE14C0;
	sub_831A8130(ctx, base);
	// 82FE14C0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE14C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE14C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE14CC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FE14D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE14D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE14D8: 815D006C  lwz r10, 0x6c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE14DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE14E0: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE14E4: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82FE14E8: 815D0070  lwz r10, 0x70(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE14EC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE14F0: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82FE14F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE14F8: 4E800421  bctrl
	ctx.lr = 0x82FE14FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE14FC: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1500: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FE1504: 409A0010  bne cr6, 0x82fe1514
	if !ctx.cr[6].eq {
	pc = 0x82FE1514; continue 'dispatch;
	}
	// 82FE1508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE150C: 917D006C  stw r11, 0x6c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FE1510: 4800002C  b 0x82fe153c
	pc = 0x82FE153C; continue 'dispatch;
	// 82FE1514: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1518: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE151C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1520: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1524: 4E800421  bctrl
	ctx.lr = 0x82FE1528;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1528: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE152C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE1530: 409A000C  bne cr6, 0x82fe153c
	if !ctx.cr[6].eq {
	pc = 0x82FE153C; continue 'dispatch;
	}
	// 82FE1534: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE1538: 917D0070  stw r11, 0x70(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FE153C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1540: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE1544: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE1548: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE154C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FE1550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1554: 4E800421  bctrl
	ctx.lr = 0x82FE1558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1558: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE155C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE1560: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1564: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE1568: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE156C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1570: 4E800421  bctrl
	ctx.lr = 0x82FE1574;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1574: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE1578: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE157C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1580: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FE1584: 419A0048  beq cr6, 0x82fe15cc
	if ctx.cr[6].eq {
	pc = 0x82FE15CC; continue 'dispatch;
	}
	// 82FE1588: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE158C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE1590: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1594: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1598: 4E800421  bctrl
	ctx.lr = 0x82FE159C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE159C: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE15A0: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE15A4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE15A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE15AC: 419A0020  beq cr6, 0x82fe15cc
	if ctx.cr[6].eq {
	pc = 0x82FE15CC; continue 'dispatch;
	}
	// 82FE15B0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE15B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE15B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE15BC: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82FE15C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE15C4: 4E800421  bctrl
	ctx.lr = 0x82FE15C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE15C8: 48000010  b 0x82fe15d8
	pc = 0x82FE15D8; continue 'dispatch;
	// 82FE15CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE15D0: 387D000C  addi r3, r29, 0xc
	ctx.r[3].s64 = ctx.r[29].s64 + 12;
	// 82FE15D4: 4801B875  bl 0x82ffce48
	ctx.lr = 0x82FE15D8;
	sub_82FFCE48(ctx, base);
	// 82FE15D8: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE15DC: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 82FE15E0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE15E4: 481C6BD4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE15E8 size=8
    let mut pc: u32 = 0x82FE15E8;
    'dispatch: loop {
        match pc {
            0x82FE15E8 => {
    //   block [0x82FE15E8..0x82FE15F0)
	// 82FE15E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE15EC: 8213AA64  lwz r16, -0x559c(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-21916 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE15F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE15F0 size=24
    let mut pc: u32 = 0x82FE15F0;
    'dispatch: loop {
        match pc {
            0x82FE15F0 => {
    //   block [0x82FE15F0..0x82FE1608)
	// 82FE15F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE15F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE15F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE15FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE1600: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE1604: 481CF625  bl 0x831b0c28
	ctx.lr = 0x82FE1608;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1610 size=48
    let mut pc: u32 = 0x82FE1610;
    'dispatch: loop {
        match pc {
            0x82FE1610 => {
    //   block [0x82FE1610..0x82FE1640)
	// 82FE1610: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE1614: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1618: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE161C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1620: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE1624: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE1628: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE162C: 813F0054  lwz r9, 0x54(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE1630: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE1634: 914B006C  stw r10, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82FE1638: 912B0070  stw r9, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82FE163C: 481CF5ED  bl 0x831b0c28
	ctx.lr = 0x82FE1640;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1640 size=200
    let mut pc: u32 = 0x82FE1640;
    'dispatch: loop {
        match pc {
            0x82FE1640 => {
    //   block [0x82FE1640..0x82FE1708)
	// 82FE1640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1648: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE164C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1650: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE1654: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82FE1658: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE165C: 388B7CB8  addi r4, r11, 0x7cb8
	ctx.r[4].s64 = ctx.r[11].s64 + 31928;
	// 82FE1660: 4BFF25E1  bl 0x82fd3c40
	ctx.lr = 0x82FE1664;
	sub_82FD3C40(ctx, base);
	// 82FE1664: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE1668: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE166C: 41820048  beq 0x82fe16b4
	if ctx.cr[0].eq {
	pc = 0x82FE16B4; continue 'dispatch;
	}
	// 82FE1670: 419A0034  beq cr6, 0x82fe16a4
	if ctx.cr[6].eq {
	pc = 0x82FE16A4; continue 'dispatch;
	}
	// 82FE1674: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1678: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE167C: 41820028  beq 0x82fe16a4
	if ctx.cr[0].eq {
	pc = 0x82FE16A4; continue 'dispatch;
	}
	// 82FE1680: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FE1684: 48000008  b 0x82fe168c
	pc = 0x82FE168C; continue 'dispatch;
	// 82FE1688: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE168C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1690: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1694: 4082FFF4  bne 0x82fe1688
	if !ctx.cr[0].eq {
	pc = 0x82FE1688; continue 'dispatch;
	}
	// 82FE1698: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82FE169C: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FE16A0: 48000008  b 0x82fe16a8
	pc = 0x82FE16A8; continue 'dispatch;
	// 82FE16A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE16A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE16AC: 4BFF8315  bl 0x82fd99c0
	ctx.lr = 0x82FE16B0;
	sub_82FD99C0(ctx, base);
	// 82FE16B0: 48000044  b 0x82fe16f4
	pc = 0x82FE16F4; continue 'dispatch;
	// 82FE16B4: 419A0034  beq cr6, 0x82fe16e8
	if ctx.cr[6].eq {
	pc = 0x82FE16E8; continue 'dispatch;
	}
	// 82FE16B8: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE16BC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE16C0: 41820028  beq 0x82fe16e8
	if ctx.cr[0].eq {
	pc = 0x82FE16E8; continue 'dispatch;
	}
	// 82FE16C4: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FE16C8: 48000008  b 0x82fe16d0
	pc = 0x82FE16D0; continue 'dispatch;
	// 82FE16CC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE16D0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE16D4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE16D8: 4082FFF4  bne 0x82fe16cc
	if !ctx.cr[0].eq {
	pc = 0x82FE16CC; continue 'dispatch;
	}
	// 82FE16DC: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82FE16E0: 7D640E70  srawi r4, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FE16E4: 48000008  b 0x82fe16ec
	pc = 0x82FE16EC; continue 'dispatch;
	// 82FE16E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE16EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE16F0: 4BFF8009  bl 0x82fd96f8
	ctx.lr = 0x82FE16F4;
	sub_82FD96F8(ctx, base);
	// 82FE16F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE16F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE16FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1708 size=148
    let mut pc: u32 = 0x82FE1708;
    'dispatch: loop {
        match pc {
            0x82FE1708 => {
    //   block [0x82FE1708..0x82FE179C)
	// 82FE1708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE1714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE171C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE1720: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE1724: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82FE1728: 4801B721  bl 0x82ffce48
	ctx.lr = 0x82FE172C;
	sub_82FFCE48(ctx, base);
	// 82FE172C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1734: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE173C: 4E800421  bctrl
	ctx.lr = 0x82FE1740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1740: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1744: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE1748: 409A0010  bne cr6, 0x82fe1758
	if !ctx.cr[6].eq {
	pc = 0x82FE1758; continue 'dispatch;
	}
	// 82FE174C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE1750: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82FE1754: 4800002C  b 0x82fe1780
	pc = 0x82FE1780; continue 'dispatch;
	// 82FE1758: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE175C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1760: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1768: 4E800421  bctrl
	ctx.lr = 0x82FE176C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE176C: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1770: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 82FE1774: 409A000C  bne cr6, 0x82fe1780
	if !ctx.cr[6].eq {
	pc = 0x82FE1780; continue 'dispatch;
	}
	// 82FE1778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE177C: 917E006C  stw r11, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82FE1780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE178C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE1794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE17A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE17A0 size=8
    let mut pc: u32 = 0x82FE17A0;
    'dispatch: loop {
        match pc {
            0x82FE17A0 => {
    //   block [0x82FE17A0..0x82FE17A8)
	// 82FE17A0: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82FE17A4: 480E633C  b 0x830c7ae0
	sub_830C7AE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE17A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE17A8 size=20
    let mut pc: u32 = 0x82FE17A8;
    'dispatch: loop {
        match pc {
            0x82FE17A8 => {
    //   block [0x82FE17A8..0x82FE17BC)
	// 82FE17A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE17AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE17B0: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE17B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE17B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE17C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE17C0 size=76
    let mut pc: u32 = 0x82FE17C0;
    'dispatch: loop {
        match pc {
            0x82FE17C0 => {
    //   block [0x82FE17C0..0x82FE180C)
	// 82FE17C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE17C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE17C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE17CC: 80630028  lwz r3, 0x28(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FE17D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE17D4: 4082000C  bne 0x82fe17e0
	if !ctx.cr[0].eq {
	pc = 0x82FE17E0; continue 'dispatch;
	}
	// 82FE17D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE17DC: 48000020  b 0x82fe17fc
	pc = 0x82FE17FC; continue 'dispatch;
	// 82FE17E0: 480101C1  bl 0x82ff19a0
	ctx.lr = 0x82FE17E4;
	sub_82FF19A0(ctx, base);
	// 82FE17E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE17E8: 4182FFF0  beq 0x82fe17d8
	if ctx.cr[0].eq {
	pc = 0x82FE17D8; continue 'dispatch;
	}
	// 82FE17EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE17F0: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FE17F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE17F8: 4E800421  bctrl
	ctx.lr = 0x82FE17FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE17FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE1800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1810 size=32
    let mut pc: u32 = 0x82FE1810;
    'dispatch: loop {
        match pc {
            0x82FE1810 => {
    //   block [0x82FE1810..0x82FE1830)
	// 82FE1810: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82FE1814: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FE1818: 419A0034  beq cr6, 0x82fe184c
	if ctx.cr[6].eq {
		sub_82FE184C(ctx, base);
		return;
	}
	// 82FE181C: A1690000  lhz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1820: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1824: 41820028  beq 0x82fe184c
	if ctx.cr[0].eq {
		sub_82FE184C(ctx, base);
		return;
	}
	// 82FE1828: 39690002  addi r11, r9, 2
	ctx.r[11].s64 = ctx.r[9].s64 + 2;
	// 82FE182C: 48000008  b 0x82fe1834
	sub_82FE1830(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1830 size=28
    let mut pc: u32 = 0x82FE1830;
    'dispatch: loop {
        match pc {
            0x82FE1830 => {
    //   block [0x82FE1830..0x82FE184C)
	// 82FE1830: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE1834: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1838: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE183C: 4082FFF4  bne 0x82fe1830
	if !ctx.cr[0].eq {
	pc = 0x82FE1830; continue 'dispatch;
	}
	// 82FE1840: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82FE1844: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FE1848: 48000008  b 0x82fe1850
	sub_82FE184C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE184C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE184C size=108
    let mut pc: u32 = 0x82FE184C;
    'dispatch: loop {
        match pc {
            0x82FE184C => {
    //   block [0x82FE184C..0x82FE18B8)
	// 82FE184C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE1850: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82FE1854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE1858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE185C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FE1860: 4099002C  ble cr6, 0x82fe188c
	if !ctx.cr[6].gt {
	pc = 0x82FE188C; continue 'dispatch;
	}
	// 82FE1864: A0C90000  lhz r6, 0(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1868: 2B06003A  cmplwi cr6, r6, 0x3a
	ctx.cr[6].compare_u32(ctx.r[6].u32, 58 as u32, &mut ctx.xer);
	// 82FE186C: 409A000C  bne cr6, 0x82fe1878
	if !ctx.cr[6].eq {
	pc = 0x82FE1878; continue 'dispatch;
	}
	// 82FE1870: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 82FE1874: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82FE1878: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE187C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82FE1880: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82FE1884: 4198FFE0  blt cr6, 0x82fe1864
	if ctx.cr[6].lt {
	pc = 0x82FE1864; continue 'dispatch;
	}
	// 82FE1888: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82FE188C: 419A0034  beq cr6, 0x82fe18c0
	if ctx.cr[6].eq {
		sub_82FE18C0(ctx, base);
		return;
	}
	// 82FE1890: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 82FE1894: 4199002C  bgt cr6, 0x82fe18c0
	if ctx.cr[6].gt {
		sub_82FE18C0(ctx, base);
		return;
	}
	// 82FE1898: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82FE189C: 419A0024  beq cr6, 0x82fe18c0
	if ctx.cr[6].eq {
		sub_82FE18C0(ctx, base);
		return;
	}
	// 82FE18A0: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82FE18A4: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FE18A8: 419A0018  beq cr6, 0x82fe18c0
	if ctx.cr[6].eq {
		sub_82FE18C0(ctx, base);
		return;
	}
	// 82FE18AC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82FE18B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE18B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE18B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE18B8 size=8
    let mut pc: u32 = 0x82FE18B8;
    'dispatch: loop {
        match pc {
            0x82FE18B8 => {
    //   block [0x82FE18B8..0x82FE18C0)
	// 82FE18B8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82FE18BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE18C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE18C0 size=8
    let mut pc: u32 = 0x82FE18C0;
    'dispatch: loop {
        match pc {
            0x82FE18C0 => {
    //   block [0x82FE18C0..0x82FE18C8)
	// 82FE18C0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82FE18C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE18C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE18C8 size=8
    let mut pc: u32 = 0x82FE18C8;
    'dispatch: loop {
        match pc {
            0x82FE18C8 => {
    //   block [0x82FE18C8..0x82FE18D0)
	// 82FE18C8: 80630088  lwz r3, 0x88(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE18CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE18D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE18D0 size=212
    let mut pc: u32 = 0x82FE18D0;
    'dispatch: loop {
        match pc {
            0x82FE18D0 => {
    //   block [0x82FE18D0..0x82FE19A4)
	// 82FE18D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE18D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE18D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE18DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE18E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE18E4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE18E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE18EC: 3BEBB84C  addi r31, r11, -0x47b4
	ctx.r[31].s64 = ctx.r[11].s64 + -18356;
	// 82FE18F0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE18F4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE18F8: 409A0040  bne cr6, 0x82fe1938
	if !ctx.cr[6].eq {
	pc = 0x82FE1938; continue 'dispatch;
	}
	// 82FE18FC: 39600582  li r11, 0x582
	ctx.r[11].s64 = 1410;
	// 82FE1900: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82FE1904: 396001BA  li r11, 0x1ba
	ctx.r[11].s64 = 442;
	// 82FE1908: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE190C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FE1910: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82FE1914: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82FE1918: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 82FE191C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE1920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE1924: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82FE1928: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FE192C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE1930: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82FE1934: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FE1938: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE193C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1940: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1944: 4E800421  bctrl
	ctx.lr = 0x82FE1948;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1948: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE194C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FE1950: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE1954: 7D5E0734  extsh r30, r10
	ctx.r[30].s64 = ctx.r[10].s16 as i64;
	// 82FE1958: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE195C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1960: 4E800421  bctrl
	ctx.lr = 0x82FE1964;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1964: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82FE196C: 57C9103A  slwi r9, r30, 2
	ctx.r[9].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FE1970: 7D29F82E  lwzx r9, r9, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82FE1974: 7D4B5830  slw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 82FE1978: 7D2B5838  and r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 & ctx.r[11].u64;
	// 82FE197C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE1980: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82FE1984: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82FE1988: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82FE198C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE199C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE19A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE19A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE19A8 size=16
    let mut pc: u32 = 0x82FE19A8;
    'dispatch: loop {
        match pc {
            0x82FE19A8 => {
    //   block [0x82FE19A8..0x82FE19B8)
	// 82FE19A8: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE19AC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE19B0: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82FE19B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE19B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE19B8 size=20
    let mut pc: u32 = 0x82FE19B8;
    'dispatch: loop {
        match pc {
            0x82FE19B8 => {
    //   block [0x82FE19B8..0x82FE19CC)
	// 82FE19B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE19BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE19C0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FE19C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE19C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE19D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE19D0 size=8
    let mut pc: u32 = 0x82FE19D0;
    'dispatch: loop {
        match pc {
            0x82FE19D0 => {
    //   block [0x82FE19D0..0x82FE19D8)
	// 82FE19D0: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 82FE19D4: 4801B89C  b 0x82ffd270
	sub_82FFD270(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE19D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE19D8 size=16
    let mut pc: u32 = 0x82FE19D8;
    'dispatch: loop {
        match pc {
            0x82FE19D8 => {
    //   block [0x82FE19D8..0x82FE19E8)
	// 82FE19D8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE19DC: 409A000C  bne cr6, 0x82fe19e8
	if !ctx.cr[6].eq {
		sub_82FE19E8(ctx, base);
		return;
	}
	// 82FE19E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE19E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE19E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE19E8 size=8
    let mut pc: u32 = 0x82FE19E8;
    'dispatch: loop {
        match pc {
            0x82FE19E8 => {
    //   block [0x82FE19E8..0x82FE19F0)
	// 82FE19E8: 80630080  lwz r3, 0x80(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 82FE19EC: 4801BAB4  b 0x82ffd4a0
	sub_82FFD4A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE19F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE19F0 size=248
    let mut pc: u32 = 0x82FE19F0;
    'dispatch: loop {
        match pc {
            0x82FE19F0 => {
    //   block [0x82FE19F0..0x82FE1AE8)
	// 82FE19F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE19F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE19F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE19FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1A04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE1A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE1A0C: 57CB077F  clrlwi. r11, r30, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE1A10: 4182000C  beq 0x82fe1a1c
	if ctx.cr[0].eq {
	pc = 0x82FE1A1C; continue 'dispatch;
	}
	// 82FE1A14: 7D6BF050  subf r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	// 82FE1A18: 3BCB0008  addi r30, r11, 8
	ctx.r[30].s64 = ctx.r[11].s64 + 8;
	// 82FE1A1C: 2B1E1000  cmplwi cr6, r30, 0x1000
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4096 as u32, &mut ctx.xer);
	// 82FE1A20: 40990054  ble cr6, 0x82fe1a74
	if !ctx.cr[6].gt {
	pc = 0x82FE1A74; continue 'dispatch;
	}
	// 82FE1A24: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1A28: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82FE1A2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1A30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE1A34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1A38: 4E800421  bctrl
	ctx.lr = 0x82FE1A3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1A3C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE1A40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1A44: 41820018  beq 0x82fe1a5c
	if ctx.cr[0].eq {
	pc = 0x82FE1A5C; continue 'dispatch;
	}
	// 82FE1A48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1A4C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE1A50: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE1A54: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FE1A58: 48000014  b 0x82fe1a6c
	pc = 0x82FE1A6C; continue 'dispatch;
	// 82FE1A5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE1A60: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82FE1A64: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FE1A68: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FE1A6C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82FE1A70: 48000060  b 0x82fe1ad0
	pc = 0x82FE1AD0; continue 'dispatch;
	// 82FE1A74: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE1A78: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE1A7C: 4099003C  ble cr6, 0x82fe1ab8
	if !ctx.cr[6].gt {
	pc = 0x82FE1AB8; continue 'dispatch;
	}
	// 82FE1A80: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1A84: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 82FE1A88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1A8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE1A90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1A94: 4E800421  bctrl
	ctx.lr = 0x82FE1A98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1A98: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82FE1A9C: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE1AA0: 39230008  addi r9, r3, 8
	ctx.r[9].s64 = ctx.r[3].s64 + 8;
	// 82FE1AA4: 616BFFF8  ori r11, r11, 0xfff8
	ctx.r[11].u64 = ctx.r[11].u64 | 65528;
	// 82FE1AA8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82FE1AAC: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82FE1AB0: 913F0064  stw r9, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82FE1AB4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FE1AB8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 82FE1ABC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FE1AC0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FE1AC4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82FE1AC8: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 82FE1ACC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82FE1AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1AD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1AD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1ADC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE1AE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1AE8 size=96
    let mut pc: u32 = 0x82FE1AE8;
    'dispatch: loop {
        match pc {
            0x82FE1AE8 => {
    //   block [0x82FE1AE8..0x82FE1B48)
	// 82FE1AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1AF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE1AF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1AF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE1B00: 48000024  b 0x82fe1b24
	pc = 0x82FE1B24; continue 'dispatch;
	// 82FE1B04: 807F0090  lwz r3, 0x90(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1B08: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE1B0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1B10: 83C40000  lwz r30, 0(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1B14: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE1B18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1B1C: 4E800421  bctrl
	ctx.lr = 0x82FE1B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1B20: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82FE1B24: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82FE1B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE1B2C: 409AFFD8  bne cr6, 0x82fe1b04
	if !ctx.cr[6].eq {
	pc = 0x82FE1B04; continue 'dispatch;
	}
	// 82FE1B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1B3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE1B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1B48 size=8
    let mut pc: u32 = 0x82FE1B48;
    'dispatch: loop {
        match pc {
            0x82FE1B48 => {
    //   block [0x82FE1B48..0x82FE1B50)
	// 82FE1B48: 88630034  lbz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82FE1B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1B50 size=8
    let mut pc: u32 = 0x82FE1B50;
    'dispatch: loop {
        match pc {
            0x82FE1B50 => {
    //   block [0x82FE1B50..0x82FE1B58)
	// 82FE1B50: 98830034  stb r4, 0x34(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[4].u8 ) };
	// 82FE1B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1B58 size=128
    let mut pc: u32 = 0x82FE1B58;
    'dispatch: loop {
        match pc {
            0x82FE1B58 => {
    //   block [0x82FE1B58..0x82FE1BD8)
	// 82FE1B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1B5C: 481C6611  bl 0x831a816c
	ctx.lr = 0x82FE1B60;
	sub_831A8130(ctx, base);
	// 82FE1B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1B64: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE1B68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE1B6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE1B70: 419A0058  beq cr6, 0x82fe1bc8
	if ctx.cr[6].eq {
	pc = 0x82FE1BC8; continue 'dispatch;
	}
	// 82FE1B74: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1B78: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1B7C: 4182004C  beq 0x82fe1bc8
	if ctx.cr[0].eq {
	pc = 0x82FE1BC8; continue 'dispatch;
	}
	// 82FE1B80: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82FE1B84: 48000008  b 0x82fe1b8c
	pc = 0x82FE1B8C; continue 'dispatch;
	// 82FE1B88: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE1B8C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1B90: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1B94: 4082FFF4  bne 0x82fe1b88
	if !ctx.cr[0].eq {
	pc = 0x82FE1B88; continue 'dispatch;
	}
	// 82FE1B98: 7D7F5850  subf r11, r31, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82FE1B9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE1BA0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FE1BA4: 396B0009  addi r11, r11, 9
	ctx.r[11].s64 = ctx.r[11].s64 + 9;
	// 82FE1BA8: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE1BAC: 4BFFFE45  bl 0x82fe19f0
	ctx.lr = 0x82FE1BB0;
	sub_82FE19F0(ctx, base);
	// 82FE1BB0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE1BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1BB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE1BBC: 4BFF094D  bl 0x82fd2508
	ctx.lr = 0x82FE1BC0;
	sub_82FD2508(ctx, base);
	// 82FE1BC0: 93DD003C  stw r30, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82FE1BC4: 4800000C  b 0x82fe1bd0
	pc = 0x82FE1BD0; continue 'dispatch;
	// 82FE1BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE1BCC: 917D003C  stw r11, 0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82FE1BD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1BD4: 481C65E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1BD8 size=8
    let mut pc: u32 = 0x82FE1BD8;
    'dispatch: loop {
        match pc {
            0x82FE1BD8 => {
    //   block [0x82FE1BD8..0x82FE1BE0)
	// 82FE1BD8: 88630098  lbz r3, 0x98(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FE1BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1BE0 size=8
    let mut pc: u32 = 0x82FE1BE0;
    'dispatch: loop {
        match pc {
            0x82FE1BE0 => {
    //   block [0x82FE1BE0..0x82FE1BE8)
	// 82FE1BE0: 98830098  stb r4, 0x98(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[4].u8 ) };
	// 82FE1BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1BE8 size=48
    let mut pc: u32 = 0x82FE1BE8;
    'dispatch: loop {
        match pc {
            0x82FE1BE8 => {
    //   block [0x82FE1BE8..0x82FE1C18)
	// 82FE1BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1BF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1BF4: 80C30090  lwz r6, 0x90(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1BF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE1BFC: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE1C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1C04: 480182CD  bl 0x82ff9ed0
	ctx.lr = 0x82FE1C08;
	sub_82FF9ED0(ctx, base);
	// 82FE1C08: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE1C0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1C10: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE1C14: 481CF015  bl 0x831b0c28
	ctx.lr = 0x82FE1C18;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1C18 size=8
    let mut pc: u32 = 0x82FE1C18;
    'dispatch: loop {
        match pc {
            0x82FE1C18 => {
    //   block [0x82FE1C18..0x82FE1C20)
	// 82FE1C18: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE1C1C: 8213AAA0  lwz r16, -0x5560(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-21856 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1C20 size=124
    let mut pc: u32 = 0x82FE1C20;
    'dispatch: loop {
        match pc {
            0x82FE1C20 => {
    //   block [0x82FE1C20..0x82FE1C9C)
	// 82FE1C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE1C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1C30: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE1C34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1C38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE1C3C: 817E0084  lwz r11, 0x84(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE1C40: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE1C44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE1C48: 409A0030  bne cr6, 0x82fe1c78
	if !ctx.cr[6].eq {
	pc = 0x82FE1C78; continue 'dispatch;
	}
	// 82FE1C4C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE1C50: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1C54: 4BFF6645  bl 0x82fd8298
	ctx.lr = 0x82FE1C58;
	sub_82FD8298(ctx, base);
	// 82FE1C58: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE1C5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1C60: 41820010  beq 0x82fe1c70
	if ctx.cr[0].eq {
	pc = 0x82FE1C70; continue 'dispatch;
	}
	// 82FE1C64: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1C68: 48019351  bl 0x82ffafb8
	ctx.lr = 0x82FE1C6C;
	sub_82FFAFB8(ctx, base);
	// 82FE1C6C: 48000008  b 0x82fe1c74
	pc = 0x82FE1C74; continue 'dispatch;
	// 82FE1C70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE1C74: 907E0084  stw r3, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[3].u32 ) };
	// 82FE1C78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE1C7C: 807E0084  lwz r3, 0x84(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE1C80: 4801A749  bl 0x82ffc3c8
	ctx.lr = 0x82FE1C84;
	sub_82FFC3C8(ctx, base);
	// 82FE1C84: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE1C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1C90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE1C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1C9C size=48
    let mut pc: u32 = 0x82FE1C9C;
    'dispatch: loop {
        match pc {
            0x82FE1C9C => {
    //   block [0x82FE1C9C..0x82FE1CCC)
	// 82FE1C9C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE1CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1CAC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE1CB0: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1CB4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE1CB8: 4BFF6629  bl 0x82fd82e0
	ctx.lr = 0x82FE1CBC;
	sub_82FD82E0(ctx, base);
	// 82FE1CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE1CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE1CD0 size=8
    let mut pc: u32 = 0x82FE1CD0;
    'dispatch: loop {
        match pc {
            0x82FE1CD0 => {
    //   block [0x82FE1CD0..0x82FE1CD8)
	// 82FE1CD0: 90830040  stw r4, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[4].u32 ) };
	// 82FE1CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1CD8 size=256
    let mut pc: u32 = 0x82FE1CD8;
    'dispatch: loop {
        match pc {
            0x82FE1CD8 => {
    //   block [0x82FE1CD8..0x82FE1DD8)
	// 82FE1CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1CDC: 481C648D  bl 0x831a8168
	ctx.lr = 0x82FE1CE0;
	sub_831A8130(ctx, base);
	// 82FE1CE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1CE4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE1CE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE1CEC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE1CF0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FE1CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1CF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1CFC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE1D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1D04: 4E800421  bctrl
	ctx.lr = 0x82FE1D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1D08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1D0C: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82FE1D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1D14: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE1D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1D1C: 419A0050  beq cr6, 0x82fe1d6c
	if ctx.cr[6].eq {
	pc = 0x82FE1D6C; continue 'dispatch;
	}
	// 82FE1D20: 4E800421  bctrl
	ctx.lr = 0x82FE1D24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1D24: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1D28: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE1D30: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 82FE1D34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1D38: 409A001C  bne cr6, 0x82fe1d54
	if !ctx.cr[6].eq {
	pc = 0x82FE1D54; continue 'dispatch;
	}
	// 82FE1D3C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE1D40: 48018191  bl 0x82ff9ed0
	ctx.lr = 0x82FE1D44;
	sub_82FF9ED0(ctx, base);
	// 82FE1D44: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE1D48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1D4C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE1D50: 481CEED9  bl 0x831b0c28
	ctx.lr = 0x82FE1D54;
	sub_831B0C28(ctx, base);
	// 82FE1D54: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82FE1D58: 48018179  bl 0x82ff9ed0
	ctx.lr = 0x82FE1D5C;
	sub_82FF9ED0(ctx, base);
	// 82FE1D5C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE1D60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1D64: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE1D68: 481CEEC1  bl 0x831b0c28
	ctx.lr = 0x82FE1D6C;
	sub_831B0C28(ctx, base);
	// 82FE1D6C: 4E800421  bctrl
	ctx.lr = 0x82FE1D70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1D70: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE1D74: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82FE1D78: 419A003C  beq cr6, 0x82fe1db4
	if ctx.cr[6].eq {
	pc = 0x82FE1DB4; continue 'dispatch;
	}
	// 82FE1D7C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82FE1D80: 419A0028  beq cr6, 0x82fe1da8
	if ctx.cr[6].eq {
	pc = 0x82FE1DA8; continue 'dispatch;
	}
	// 82FE1D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE1D88: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE1D8C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE1D90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1D94: 4801813D  bl 0x82ff9ed0
	ctx.lr = 0x82FE1D98;
	sub_82FF9ED0(ctx, base);
	// 82FE1D98: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE1D9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1DA0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE1DA4: 481CEE85  bl 0x831b0c28
	ctx.lr = 0x82FE1DA8;
	sub_831B0C28(ctx, base);
	// 82FE1DA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1DAC: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE1DB0: 4800000C  b 0x82fe1dbc
	pc = 0x82FE1DBC; continue 'dispatch;
	// 82FE1DB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1DB8: 816B00FC  lwz r11, 0xfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(252 as u32) ) } as u64;
	// 82FE1DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1DC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE1DC4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE1DC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1DCC: 4E800421  bctrl
	ctx.lr = 0x82FE1DD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1DD0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE1DD4: 481C63E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1DD8 size=232
    let mut pc: u32 = 0x82FE1DD8;
    'dispatch: loop {
        match pc {
            0x82FE1DD8 => {
    //   block [0x82FE1DD8..0x82FE1EC0)
	// 82FE1DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1DDC: 481C6389  bl 0x831a8164
	ctx.lr = 0x82FE1DE0;
	sub_831A8130(ctx, base);
	// 82FE1DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1DE4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FE1DE8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE1DEC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE1DF0: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1DF4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE1DF8: 4800009C  b 0x82fe1e94
	pc = 0x82FE1E94; continue 'dispatch;
	// 82FE1DFC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE1E04: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FE1E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1E0C: 4E800421  bctrl
	ctx.lr = 0x82FE1E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1E10: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82FE1E14: 41820068  beq 0x82fe1e7c
	if ctx.cr[0].eq {
	pc = 0x82FE1E7C; continue 'dispatch;
	}
	// 82FE1E18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1E1C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE1E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1E24: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE1E28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1E2C: 4E800421  bctrl
	ctx.lr = 0x82FE1E30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1E30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE1E34: 41820048  beq 0x82fe1e7c
	if ctx.cr[0].eq {
	pc = 0x82FE1E7C; continue 'dispatch;
	}
	// 82FE1E38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1E3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE1E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1E44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE1E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1E4C: 4E800421  bctrl
	ctx.lr = 0x82FE1E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1E50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE1E54: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE1E58: 4BFFFF81  bl 0x82fe1dd8
	ctx.lr = 0x82FE1E5C;
	sub_82FE1DD8(ctx, base);
	// 82FE1E5C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1E60: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FE1E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1E68: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE1E6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1E70: 4E800421  bctrl
	ctx.lr = 0x82FE1E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1E74: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FE1E78: 4198FFC0  blt cr6, 0x82fe1e38
	if ctx.cr[6].lt {
	pc = 0x82FE1E38; continue 'dispatch;
	}
	// 82FE1E7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE1E80: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE1E84: 4BFFFF55  bl 0x82fe1dd8
	ctx.lr = 0x82FE1E88;
	sub_82FE1DD8(ctx, base);
	// 82FE1E88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1E8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE1E90: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE1E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1E98: 4E800421  bctrl
	ctx.lr = 0x82FE1E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1E9C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE1EA0: 4082FF5C  bne 0x82fe1dfc
	if !ctx.cr[0].eq {
	pc = 0x82FE1DFC; continue 'dispatch;
	}
	// 82FE1EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE1EA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE1EAC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FE1EB0: 387B0004  addi r3, r27, 4
	ctx.r[3].s64 = ctx.r[27].s64 + 4;
	// 82FE1EB4: 4BFFD8C5  bl 0x82fdf778
	ctx.lr = 0x82FE1EB8;
	sub_82FDF778(ctx, base);
	// 82FE1EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE1EBC: 481C62F8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1EC0 size=148
    let mut pc: u32 = 0x82FE1EC0;
    'dispatch: loop {
        match pc {
            0x82FE1EC0 => {
    //   block [0x82FE1EC0..0x82FE1F54)
	// 82FE1EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1EC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE1ECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE1ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1ED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE1ED8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82FE1EDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FE1EE0: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE1EE4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FE1EE8: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82FE1EEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE1EF0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82FE1EF4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1EF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE1EFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE1F00: 4E800421  bctrl
	ctx.lr = 0x82FE1F04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE1F04: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1F08: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82FE1F0C: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FE1F10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FE1F14: 40990024  ble cr6, 0x82fe1f38
	if !ctx.cr[6].gt {
	pc = 0x82FE1F38; continue 'dispatch;
	}
	// 82FE1F18: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FE1F1C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE1F20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE1F24: 7FC9512E  stwx r30, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82FE1F28: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FE1F2C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1F30: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE1F34: 4198FFE8  blt cr6, 0x82fe1f1c
	if ctx.cr[6].lt {
	pc = 0x82FE1F1C; continue 'dispatch;
	}
	// 82FE1F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE1F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE1F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE1F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1F58 size=96
    let mut pc: u32 = 0x82FE1F58;
    'dispatch: loop {
        match pc {
            0x82FE1F58 => {
    //   block [0x82FE1F58..0x82FE1FB8)
	// 82FE1F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1F64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE1F68: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE1F6C: 41980030  blt cr6, 0x82fe1f9c
	if ctx.cr[6].lt {
	pc = 0x82FE1F9C; continue 'dispatch;
	}
	// 82FE1F70: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE1F74: 80E30008  lwz r7, 8(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE1F78: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82FE1F7C: 388BAAD8  addi r4, r11, -0x5528
	ctx.r[4].s64 = ctx.r[11].s64 + -21800;
	// 82FE1F80: 38A00076  li r5, 0x76
	ctx.r[5].s64 = 118;
	// 82FE1F84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1F88: 4BFEE9D1  bl 0x82fd0958
	ctx.lr = 0x82FE1F8C;
	sub_82FD0958(ctx, base);
	// 82FE1F8C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE1F90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE1F94: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FE1F98: 481CEC91  bl 0x831b0c28
	ctx.lr = 0x82FE1F9C;
	sub_831B0C28(ctx, base);
	// 82FE1F9C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE1FA0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE1FA4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FE1FA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE1FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE1FB8 size=104
    let mut pc: u32 = 0x82FE1FB8;
    'dispatch: loop {
        match pc {
            0x82FE1FB8 => {
    //   block [0x82FE1FB8..0x82FE2020)
	// 82FE1FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE1FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE1FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE1FC4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE1FC8: 419A002C  beq cr6, 0x82fe1ff4
	if ctx.cr[6].eq {
	pc = 0x82FE1FF4; continue 'dispatch;
	}
	// 82FE1FCC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE1FD0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE1FD4: 41990020  bgt cr6, 0x82fe1ff4
	if ctx.cr[6].gt {
	pc = 0x82FE1FF4; continue 'dispatch;
	}
	// 82FE1FD8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE1FDC: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE1FE0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE1FE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE1FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE1FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE1FF0: 4E800020  blr
	return;
	// 82FE1FF4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE1FF8: 80E3001C  lwz r7, 0x1c(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE1FFC: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 82FE2000: 388BAB08  addi r4, r11, -0x54f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21752;
	// 82FE2004: 38A00102  li r5, 0x102
	ctx.r[5].s64 = 258;
	// 82FE2008: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE200C: 4BFEECBD  bl 0x82fd0cc8
	ctx.lr = 0x82FE2010;
	sub_82FD0CC8(ctx, base);
	// 82FE2010: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE2014: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2018: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 82FE201C: 481CEC0D  bl 0x831b0c28
	ctx.lr = 0x82FE2020;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2020 size=12
    let mut pc: u32 = 0x82FE2020;
    'dispatch: loop {
        match pc {
            0x82FE2020 => {
    //   block [0x82FE2020..0x82FE202C)
	// 82FE2020: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2024: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2028: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE202C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE202C size=20
    let mut pc: u32 = 0x82FE202C;
    'dispatch: loop {
        match pc {
            0x82FE202C => {
    //   block [0x82FE202C..0x82FE2040)
	// 82FE202C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE2030: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2034: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2038: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE203C: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2040 size=20
    let mut pc: u32 = 0x82FE2040;
    'dispatch: loop {
        match pc {
            0x82FE2040 => {
    //   block [0x82FE2040..0x82FE2054)
	// 82FE2040: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE2044: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE2048: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE204C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2050: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2054(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2054 size=20
    let mut pc: u32 = 0x82FE2054;
    'dispatch: loop {
        match pc {
            0x82FE2054 => {
    //   block [0x82FE2054..0x82FE2068)
	// 82FE2054: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2058: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE205C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2064: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2068 size=4
    let mut pc: u32 = 0x82FE2068;
    'dispatch: loop {
        match pc {
            0x82FE2068 => {
    //   block [0x82FE2068..0x82FE206C)
	// 82FE2068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2070 size=36
    let mut pc: u32 = 0x82FE2070;
    'dispatch: loop {
        match pc {
            0x82FE2070 => {
    //   block [0x82FE2070..0x82FE2094)
	// 82FE2070: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2074: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE2078: 409A001C  bne cr6, 0x82fe2094
	if !ctx.cr[6].eq {
		sub_82FE2094(ctx, base);
		return;
	}
	// 82FE207C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE2080: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2084: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE2088: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE208C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE2090: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2094(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2094 size=8
    let mut pc: u32 = 0x82FE2094;
    'dispatch: loop {
        match pc {
            0x82FE2094 => {
    //   block [0x82FE2094..0x82FE209C)
	// 82FE2094: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82FE2098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE20A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE20A0 size=16
    let mut pc: u32 = 0x82FE20A0;
    'dispatch: loop {
        match pc {
            0x82FE20A0 => {
    //   block [0x82FE20A0..0x82FE20B0)
	// 82FE20A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE20A4: 396BAB44  addi r11, r11, -0x54bc
	ctx.r[11].s64 = ctx.r[11].s64 + -21692;
	// 82FE20A8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE20AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE20B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE20B0 size=108
    let mut pc: u32 = 0x82FE20B0;
    'dispatch: loop {
        match pc {
            0x82FE20B0 => {
    //   block [0x82FE20B0..0x82FE211C)
	// 82FE20B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE20B4: 481C60B9  bl 0x831a816c
	ctx.lr = 0x82FE20B8;
	sub_831A8130(ctx, base);
	// 82FE20B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE20BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE20C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE20C4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82FE20C8: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82FE20CC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82FE20D0: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FE20D4: 911F0004  stw r8, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82FE20D8: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82FE20DC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE20E0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FE20E4: 419A0014  beq cr6, 0x82fe20f8
	if ctx.cr[6].eq {
	pc = 0x82FE20F8; continue 'dispatch;
	}
	// 82FE20E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE20EC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82FE20F0: 4BFEEA91  bl 0x82fd0b80
	ctx.lr = 0x82FE20F4;
	sub_82FD0B80(ctx, base);
	// 82FE20F4: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FE20F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE20FC: 419A0014  beq cr6, 0x82fe2110
	if ctx.cr[6].eq {
	pc = 0x82FE2110; continue 'dispatch;
	}
	// 82FE2100: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE2104: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE2108: 4BFEEA79  bl 0x82fd0b80
	ctx.lr = 0x82FE210C;
	sub_82FD0B80(ctx, base);
	// 82FE210C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FE2110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE2114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE2118: 481C60A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2120 size=176
    let mut pc: u32 = 0x82FE2120;
    'dispatch: loop {
        match pc {
            0x82FE2120 => {
    //   block [0x82FE2120..0x82FE21D0)
	// 82FE2120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE212C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2130: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE2134: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE2138: 409A0030  bne cr6, 0x82fe2168
	if !ctx.cr[6].eq {
	pc = 0x82FE2168; continue 'dispatch;
	}
	// 82FE213C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE2140: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2144: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82FE2148: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE214C: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 82FE2150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2154: 4BFEEB75  bl 0x82fd0cc8
	ctx.lr = 0x82FE2158;
	sub_82FD0CC8(ctx, base);
	// 82FE2158: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE215C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2160: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 82FE2164: 481CEAC5  bl 0x831b0c28
	ctx.lr = 0x82FE2168;
	sub_831B0C28(ctx, base);
	// 82FE2168: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE216C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2170: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE2174: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2178: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE217C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2180: 4E800421  bctrl
	ctx.lr = 0x82FE2184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2184: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE218C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82FE2190: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FE2194: 40990028  ble cr6, 0x82fe21bc
	if !ctx.cr[6].gt {
	pc = 0x82FE21BC; continue 'dispatch;
	}
	// 82FE2198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE219C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE21A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE21A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE21A8: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 82FE21AC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FE21B0: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE21B4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE21B8: 4198FFE4  blt cr6, 0x82fe219c
	if ctx.cr[6].lt {
	pc = 0x82FE219C; continue 'dispatch;
	}
	// 82FE21BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE21C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE21C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE21C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE21CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE21D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE21D0 size=308
    let mut pc: u32 = 0x82FE21D0;
    'dispatch: loop {
        match pc {
            0x82FE21D0 => {
    //   block [0x82FE21D0..0x82FE2304)
	// 82FE21D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE21D4: 481C5F89  bl 0x831a815c
	ctx.lr = 0x82FE21D8;
	sub_831A8130(ctx, base);
	// 82FE21D8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE21DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE21E0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FE21E4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82FE21E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FE21EC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82FE21F0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE21F4: 80DF001C  lwz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE21F8: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE21FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2200: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2208: 4E800421  bctrl
	ctx.lr = 0x82FE220C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE220C: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FE2210: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2214: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE2218: 40990030  ble cr6, 0x82fe2248
	if !ctx.cr[6].gt {
	pc = 0x82FE2248; continue 'dispatch;
	}
	// 82FE221C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE2220: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE2224: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FE2228: 388BAB08  addi r4, r11, -0x54f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21752;
	// 82FE222C: 38A0016E  li r5, 0x16e
	ctx.r[5].s64 = 366;
	// 82FE2230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2234: 4BFEEE25  bl 0x82fd1058
	ctx.lr = 0x82FE2238;
	sub_82FD1058(ctx, base);
	// 82FE2238: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE223C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2240: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FE2244: 481CE9E5  bl 0x831b0c28
	ctx.lr = 0x82FE2248;
	sub_831B0C28(ctx, base);
	// 82FE2248: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE224C: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE2250: 7FCA582E  lwzx r30, r10, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE2254: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2258: 418200A0  beq 0x82fe22f8
	if ctx.cr[0].eq {
	pc = 0x82FE22F8; continue 'dispatch;
	}
	// 82FE225C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2260: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE2264: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2268: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE226C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2270: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2274: 4E800421  bctrl
	ctx.lr = 0x82FE2278;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2278: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE227C: 41820070  beq 0x82fe22ec
	if ctx.cr[0].eq {
	pc = 0x82FE22EC; continue 'dispatch;
	}
	// 82FE2280: 839E000C  lwz r28, 0xc(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2284: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FE2288: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE228C: 4BFF19B5  bl 0x82fd3c40
	ctx.lr = 0x82FE2290;
	sub_82FD3C40(ctx, base);
	// 82FE2290: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2294: 41820058  beq 0x82fe22ec
	if ctx.cr[0].eq {
	pc = 0x82FE22EC; continue 'dispatch;
	}
	// 82FE2298: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE229C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE22A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE22A4: 4BFF199D  bl 0x82fd3c40
	ctx.lr = 0x82FE22A8;
	sub_82FD3C40(ctx, base);
	// 82FE22A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE22AC: 41820040  beq 0x82fe22ec
	if ctx.cr[0].eq {
	pc = 0x82FE22EC; continue 'dispatch;
	}
	// 82FE22B0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82FE22B4: 419A0010  beq cr6, 0x82fe22c4
	if ctx.cr[6].eq {
	pc = 0x82FE22C4; continue 'dispatch;
	}
	// 82FE22B8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE22BC: 409A0010  bne cr6, 0x82fe22cc
	if !ctx.cr[6].eq {
	pc = 0x82FE22CC; continue 'dispatch;
	}
	// 82FE22C0: 4800002C  b 0x82fe22ec
	pc = 0x82FE22EC; continue 'dispatch;
	// 82FE22C4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE22C8: 409A0024  bne cr6, 0x82fe22ec
	if !ctx.cr[6].eq {
	pc = 0x82FE22EC; continue 'dispatch;
	}
	// 82FE22CC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 82FE22D0: 419A0014  beq cr6, 0x82fe22e4
	if ctx.cr[6].eq {
	pc = 0x82FE22E4; continue 'dispatch;
	}
	// 82FE22D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE22D8: 419A0014  beq cr6, 0x82fe22ec
	if ctx.cr[6].eq {
	pc = 0x82FE22EC; continue 'dispatch;
	}
	// 82FE22DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE22E0: 4800001C  b 0x82fe22fc
	pc = 0x82FE22FC; continue 'dispatch;
	// 82FE22E4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE22E8: 419AFFF4  beq cr6, 0x82fe22dc
	if ctx.cr[6].eq {
	pc = 0x82FE22DC; continue 'dispatch;
	}
	// 82FE22EC: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE22F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE22F4: 409AFF68  bne cr6, 0x82fe225c
	if !ctx.cr[6].eq {
	pc = 0x82FE225C; continue 'dispatch;
	}
	// 82FE22F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE22FC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82FE2300: 481C5EAC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2308 size=176
    let mut pc: u32 = 0x82FE2308;
    'dispatch: loop {
        match pc {
            0x82FE2308 => {
    //   block [0x82FE2308..0x82FE23B8)
	// 82FE2308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE230C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2310: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE2314: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2318: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE231C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE2320: 409A0030  bne cr6, 0x82fe2350
	if !ctx.cr[6].eq {
	pc = 0x82FE2350; continue 'dispatch;
	}
	// 82FE2324: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE2328: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE232C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82FE2330: 388BAB08  addi r4, r11, -0x54f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21752;
	// 82FE2334: 38A00085  li r5, 0x85
	ctx.r[5].s64 = 133;
	// 82FE2338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE233C: 4BFEE98D  bl 0x82fd0cc8
	ctx.lr = 0x82FE2340;
	sub_82FD0CC8(ctx, base);
	// 82FE2340: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE2344: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2348: 388BC3FC  addi r4, r11, -0x3c04
	ctx.r[4].s64 = ctx.r[11].s64 + -15364;
	// 82FE234C: 481CE8DD  bl 0x831b0c28
	ctx.lr = 0x82FE2350;
	sub_831B0C28(ctx, base);
	// 82FE2350: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE2354: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2358: 5544103A  slwi r4, r10, 2
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE235C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2360: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2368: 4E800421  bctrl
	ctx.lr = 0x82FE236C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE236C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE2374: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82FE2378: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82FE237C: 40990028  ble cr6, 0x82fe23a4
	if !ctx.cr[6].gt {
	pc = 0x82FE23A4; continue 'dispatch;
	}
	// 82FE2380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE2384: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE238C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE2390: 7D09512E  stwx r8, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u32) };
	// 82FE2394: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82FE2398: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE239C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE23A0: 4198FFE4  blt cr6, 0x82fe2384
	if ctx.cr[6].lt {
	pc = 0x82FE2384; continue 'dispatch;
	}
	// 82FE23A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE23A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE23AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE23B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE23B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE23B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE23B8 size=316
    let mut pc: u32 = 0x82FE23B8;
    'dispatch: loop {
        match pc {
            0x82FE23B8 => {
    //   block [0x82FE23B8..0x82FE24F4)
	// 82FE23B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE23BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE23C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE23C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE23C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE23CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE23D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE23D4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE23D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE23DC: 419A0080  beq cr6, 0x82fe245c
	if ctx.cr[6].eq {
	pc = 0x82FE245C; continue 'dispatch;
	}
	// 82FE23E0: 4082004C  bne 0x82fe242c
	if !ctx.cr[0].eq {
	pc = 0x82FE242C; continue 'dispatch;
	}
	// 82FE23E4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE23E8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE23EC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE23F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE23F4: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE23F8: 48000038  b 0x82fe2430
	pc = 0x82FE2430; continue 'dispatch;
	// 82FE23FC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE2400: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2404: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE2408: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE240C: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2410: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2414: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE241C: 4E800421  bctrl
	ctx.lr = 0x82FE2420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2420: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2424: 4082001C  bne 0x82fe2440
	if !ctx.cr[0].eq {
	pc = 0x82FE2440; continue 'dispatch;
	}
	// 82FE2428: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE242C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2430: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE2434: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE2438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE243C: 409AFFC0  bne cr6, 0x82fe23fc
	if !ctx.cr[6].eq {
	pc = 0x82FE23FC; continue 'dispatch;
	}
	// 82FE2440: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2444: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE2448: 409A0098  bne cr6, 0x82fe24e0
	if !ctx.cr[6].eq {
	pc = 0x82FE24E0; continue 'dispatch;
	}
	// 82FE244C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE2450: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2454: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE2458: 48000088  b 0x82fe24e0
	pc = 0x82FE24E0; continue 'dispatch;
	// 82FE245C: 4182000C  beq 0x82fe2468
	if ctx.cr[0].eq {
	pc = 0x82FE2468; continue 'dispatch;
	}
	// 82FE2460: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2464: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE2468: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE246C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE2470: 409A0070  bne cr6, 0x82fe24e0
	if !ctx.cr[6].eq {
	pc = 0x82FE24E0; continue 'dispatch;
	}
	// 82FE2474: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2478: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE247C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE2480: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE2484: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2488: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE248C: 419A0054  beq cr6, 0x82fe24e0
	if ctx.cr[6].eq {
	pc = 0x82FE24E0; continue 'dispatch;
	}
	// 82FE2490: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2494: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FE2498: 48000028  b 0x82fe24c0
	pc = 0x82FE24C0; continue 'dispatch;
	// 82FE249C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE24A0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE24A4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE24A8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE24AC: 419A0034  beq cr6, 0x82fe24e0
	if ctx.cr[6].eq {
	pc = 0x82FE24E0; continue 'dispatch;
	}
	// 82FE24B0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE24B4: 5568003E  slwi r8, r11, 0
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FE24B8: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FE24BC: 81290008  lwz r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE24C0: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82FE24C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82FE24C8: 419AFFD4  beq cr6, 0x82fe249c
	if ctx.cr[6].eq {
	pc = 0x82FE249C; continue 'dispatch;
	}
	// 82FE24CC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE24D0: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE24D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE24D8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE24DC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE24E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE24E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE24E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE24EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE24F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE24F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE24F8 size=128
    let mut pc: u32 = 0x82FE24F8;
    'dispatch: loop {
        match pc {
            0x82FE24F8 => {
    //   block [0x82FE24F8..0x82FE2578)
	// 82FE24F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE24FC: 481C5C71  bl 0x831a816c
	ctx.lr = 0x82FE2500;
	sub_831A8130(ctx, base);
	// 82FE2500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2504: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE2508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE250C: 396BAB44  addi r11, r11, -0x54bc
	ctx.r[11].s64 = ctx.r[11].s64 + -21692;
	// 82FE2510: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE2514: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE2518: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE251C: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FE2520: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82FE2524: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE2528: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82FE252C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FE2530: 90DF0014  stw r6, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FE2534: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FE2538: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE253C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2544: 4E800421  bctrl
	ctx.lr = 0x82FE2548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2548: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE254C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FE2550: 419A001C  beq cr6, 0x82fe256c
	if ctx.cr[6].eq {
	pc = 0x82FE256C; continue 'dispatch;
	}
	// 82FE2554: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82FE2558: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE255C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE2560: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FE2564: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FE2568: 4082FFF0  bne 0x82fe2558
	if !ctx.cr[0].eq {
	pc = 0x82FE2558; continue 'dispatch;
	}
	// 82FE256C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE2570: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE2574: 481C5C48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2578 size=8
    let mut pc: u32 = 0x82FE2578;
    'dispatch: loop {
        match pc {
            0x82FE2578 => {
    //   block [0x82FE2578..0x82FE2580)
	// 82FE2578: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82FE257C: 4BFFEC1C  b 0x82fe1198
	sub_82FE1198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2580 size=8
    let mut pc: u32 = 0x82FE2580;
    'dispatch: loop {
        match pc {
            0x82FE2580 => {
    //   block [0x82FE2580..0x82FE2588)
	// 82FE2580: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FE2584: 4BFFEC14  b 0x82fe1198
	sub_82FE1198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2588 size=8
    let mut pc: u32 = 0x82FE2588;
    'dispatch: loop {
        match pc {
            0x82FE2588 => {
    //   block [0x82FE2588..0x82FE2590)
	// 82FE2588: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FE258C: 4BFFEC0C  b 0x82fe1198
	sub_82FE1198(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2590 size=8
    let mut pc: u32 = 0x82FE2590;
    'dispatch: loop {
        match pc {
            0x82FE2590 => {
    //   block [0x82FE2590..0x82FE2598)
	// 82FE2590: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE2594: 8213AD40  lwz r16, -0x52c0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-21184 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2598 size=312
    let mut pc: u32 = 0x82FE2598;
    'dispatch: loop {
        match pc {
            0x82FE2598 => {
    //   block [0x82FE2598..0x82FE26D0)
	// 82FE2598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE259C: 481C5BCD  bl 0x831a8168
	ctx.lr = 0x82FE25A0;
	sub_831A8130(ctx, base);
	// 82FE25A0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE25A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE25A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE25AC: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE25B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE25B4: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE25B8: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 82FE25BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE25C0: 394AD930  addi r10, r10, -0x26d0
	ctx.r[10].s64 = ctx.r[10].s64 + -9936;
	// 82FE25C4: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 82FE25C8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE25CC: 3929A8A0  addi r9, r9, -0x5760
	ctx.r[9].s64 = ctx.r[9].s64 + -22368;
	// 82FE25D0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE25D4: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE25D8: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82FE25DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE25E0: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE25E4: 3D208214  lis r9, -0x7dec
	ctx.r[9].s64 = -2112618496;
	// 82FE25E8: 3D008214  lis r8, -0x7dec
	ctx.r[8].s64 = -2112618496;
	// 82FE25EC: 396BAC50  addi r11, r11, -0x53b0
	ctx.r[11].s64 = ctx.r[11].s64 + -21424;
	// 82FE25F0: 394AAC40  addi r10, r10, -0x53c0
	ctx.r[10].s64 = ctx.r[10].s64 + -21440;
	// 82FE25F4: 3929AC34  addi r9, r9, -0x53cc
	ctx.r[9].s64 = ctx.r[9].s64 + -21452;
	// 82FE25F8: 3908AB98  addi r8, r8, -0x5468
	ctx.r[8].s64 = ctx.r[8].s64 + -21608;
	// 82FE25FC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82FE2600: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE2604: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82FE2608: 913E0008  stw r9, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82FE260C: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FE2610: 4BFFCC09  bl 0x82fdf218
	ctx.lr = 0x82FE2614;
	sub_82FDF218(ctx, base);
	// 82FE2614: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE2618: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 82FE261C: 48019E7D  bl 0x82ffc498
	ctx.lr = 0x82FE2620;
	sub_82FFC498(ctx, base);
	// 82FE2620: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE2624: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE2628: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 82FE262C: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 82FE2630: 93BE0028  stw r29, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82FE2634: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 82FE2638: 93BE0030  stw r29, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 82FE263C: 9BBE0034  stb r29, 0x34(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 82FE2640: 93BE0038  stw r29, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 82FE2644: 93BE003C  stw r29, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 82FE2648: 93BE0040  stw r29, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 82FE264C: 4801B49D  bl 0x82ffdae8
	ctx.lr = 0x82FE2650;
	sub_82FFDAE8(ctx, base);
	// 82FE2650: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82FE2654: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82FE2658: 93BE005C  stw r29, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82FE265C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2660: 93BE0060  stw r29, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82FE2664: 93BE0064  stw r29, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82FE2668: 93BE0068  stw r29, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 82FE266C: 93BE006C  stw r29, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82FE2670: 93BE0070  stw r29, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82FE2674: 93BE0074  stw r29, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82FE2678: 93BE0078  stw r29, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82FE267C: 93BE007C  stw r29, 0x7c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82FE2680: 93BE0080  stw r29, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82FE2684: 93BE0084  stw r29, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82FE2688: 93BE0088  stw r29, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 82FE268C: 93BE008C  stw r29, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82FE2690: 939E0090  stw r28, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 82FE2694: 93BE0094  stw r29, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE2698: 997E0098  stb r11, 0x98(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 82FE269C: 4BFFF355  bl 0x82fe19f0
	ctx.lr = 0x82FE26A0;
	sub_82FE19F0(ctx, base);
	// 82FE26A0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE26A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE26A8: 41820014  beq 0x82fe26bc
	if ctx.cr[0].eq {
	pc = 0x82FE26BC; continue 'dispatch;
	}
	// 82FE26AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE26B0: 38800101  li r4, 0x101
	ctx.r[4].s64 = 257;
	// 82FE26B4: 4801AD6D  bl 0x82ffd420
	ctx.lr = 0x82FE26B8;
	sub_82FFD420(ctx, base);
	// 82FE26B8: 48000008  b 0x82fe26c0
	pc = 0x82FE26C0; continue 'dispatch;
	// 82FE26BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE26C0: 907E0080  stw r3, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 82FE26C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE26C8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE26CC: 481C5AEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE26D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE26D0 size=40
    let mut pc: u32 = 0x82FE26D0;
    'dispatch: loop {
        match pc {
            0x82FE26D0 => {
    //   block [0x82FE26D0..0x82FE26F8)
	// 82FE26D0: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE26D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE26D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE26DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE26E0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE26E4: 4BFFEA5D  bl 0x82fe1140
	ctx.lr = 0x82FE26E8;
	sub_82FE1140(ctx, base);
	// 82FE26E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE26EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE26F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE26F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE26F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE26F8 size=44
    let mut pc: u32 = 0x82FE26F8;
    'dispatch: loop {
        match pc {
            0x82FE26F8 => {
    //   block [0x82FE26F8..0x82FE2724)
	// 82FE26F8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE26FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2700: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2704: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2708: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE270C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82FE2710: 480E53D1  bl 0x830c7ae0
	ctx.lr = 0x82FE2714;
	sub_830C7AE0(ctx, base);
	// 82FE2714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE271C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2724 size=44
    let mut pc: u32 = 0x82FE2724;
    'dispatch: loop {
        match pc {
            0x82FE2724 => {
    //   block [0x82FE2724..0x82FE2750)
	// 82FE2724: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE2728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE272C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2734: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE2738: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 82FE273C: 4BFFEAED  bl 0x82fe1228
	ctx.lr = 0x82FE2740;
	sub_82FE1228(ctx, base);
	// 82FE2740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE274C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2750 size=44
    let mut pc: u32 = 0x82FE2750;
    'dispatch: loop {
        match pc {
            0x82FE2750 => {
    //   block [0x82FE2750..0x82FE277C)
	// 82FE2750: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE2754: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2758: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE275C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2760: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE2764: 386B0044  addi r3, r11, 0x44
	ctx.r[3].s64 = ctx.r[11].s64 + 68;
	// 82FE2768: 4801B9F9  bl 0x82ffe160
	ctx.lr = 0x82FE276C;
	sub_82FFE160(ctx, base);
	// 82FE276C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE277C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE277C size=44
    let mut pc: u32 = 0x82FE277C;
    'dispatch: loop {
        match pc {
            0x82FE277C => {
    //   block [0x82FE277C..0x82FE27A8)
	// 82FE277C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE2780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE278C: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE2790: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE2794: 480E534D  bl 0x830c7ae0
	ctx.lr = 0x82FE2798;
	sub_830C7AE0(ctx, base);
	// 82FE2798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE27A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE27A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE27A8 size=8
    let mut pc: u32 = 0x82FE27A8;
    'dispatch: loop {
        match pc {
            0x82FE27A8 => {
    //   block [0x82FE27A8..0x82FE27B0)
	// 82FE27A8: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 82FE27AC: 48003D94  b 0x82fe6540
	sub_82FE6540(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE27B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE27B0 size=8
    let mut pc: u32 = 0x82FE27B0;
    'dispatch: loop {
        match pc {
            0x82FE27B0 => {
    //   block [0x82FE27B0..0x82FE27B8)
	// 82FE27B0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FE27B4: 48003D8C  b 0x82fe6540
	sub_82FE6540(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE27B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE27B8 size=8
    let mut pc: u32 = 0x82FE27B8;
    'dispatch: loop {
        match pc {
            0x82FE27B8 => {
    //   block [0x82FE27B8..0x82FE27C0)
	// 82FE27B8: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82FE27BC: 48003D84  b 0x82fe6540
	sub_82FE6540(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE27C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE27C0 size=8
    let mut pc: u32 = 0x82FE27C0;
    'dispatch: loop {
        match pc {
            0x82FE27C0 => {
    //   block [0x82FE27C0..0x82FE27C8)
	// 82FE27C0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE27C4: 8213ADA0  lwz r16, -0x5260(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-21088 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE27C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE27C8 size=356
    let mut pc: u32 = 0x82FE27C8;
    'dispatch: loop {
        match pc {
            0x82FE27C8 => {
    //   block [0x82FE27C8..0x82FE292C)
	// 82FE27C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE27CC: 481C5995  bl 0x831a8160
	ctx.lr = 0x82FE27D0;
	sub_831A8130(ctx, base);
	// 82FE27D0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FE27D4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE27D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE27DC: 3860009C  li r3, 0x9c
	ctx.r[3].s64 = 156;
	// 82FE27E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE27E4: 809C0084  lwz r4, 0x84(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE27E8: 939F00A4  stw r28, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[28].u32 ) };
	// 82FE27EC: 4BFF5AAD  bl 0x82fd8298
	ctx.lr = 0x82FE27F0;
	sub_82FD8298(ctx, base);
	// 82FE27F0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE27F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE27F8: 41820014  beq 0x82fe280c
	if ctx.cr[0].eq {
	pc = 0x82FE280C; continue 'dispatch;
	}
	// 82FE27FC: 809C0084  lwz r4, 0x84(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE2800: 4BFFFD99  bl 0x82fe2598
	ctx.lr = 0x82FE2804;
	sub_82FE2598(ctx, base);
	// 82FE2804: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE2808: 48000008  b 0x82fe2810
	pc = 0x82FE2810; continue 'dispatch;
	// 82FE280C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FE2810: 809C0024  lwz r4, 0x24(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE2814: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2818: 41820024  beq 0x82fe283c
	if ctx.cr[0].eq {
	pc = 0x82FE283C; continue 'dispatch;
	}
	// 82FE281C: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2820: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2824: 41820018  beq 0x82fe283c
	if ctx.cr[0].eq {
	pc = 0x82FE283C; continue 'dispatch;
	}
	// 82FE2828: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE282C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2830: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE2834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2838: 4E800421  bctrl
	ctx.lr = 0x82FE283C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE283C: 809C002C  lwz r4, 0x2c(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) } as u64;
	// 82FE2840: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2844: 41820024  beq 0x82fe2868
	if ctx.cr[0].eq {
	pc = 0x82FE2868; continue 'dispatch;
	}
	// 82FE2848: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE284C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2850: 41820018  beq 0x82fe2868
	if ctx.cr[0].eq {
	pc = 0x82FE2868; continue 'dispatch;
	}
	// 82FE2854: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2858: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE285C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE2860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2864: 4E800421  bctrl
	ctx.lr = 0x82FE2868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2868: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE286C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2870: 889C0028  lbz r4, 0x28(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FE2874: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 82FE2878: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE287C: 4E800421  bctrl
	ctx.lr = 0x82FE2880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2880: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2884: 41820078  beq 0x82fe28fc
	if ctx.cr[0].eq {
	pc = 0x82FE28FC; continue 'dispatch;
	}
	// 82FE2888: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE288C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE2890: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE2894: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2898: 4E800421  bctrl
	ctx.lr = 0x82FE289C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE289C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE28A0: 4182005C  beq 0x82fe28fc
	if ctx.cr[0].eq {
	pc = 0x82FE28FC; continue 'dispatch;
	}
	// 82FE28A4: 3B7E000C  addi r27, r30, 0xc
	ctx.r[27].s64 = ctx.r[30].s64 + 12;
	// 82FE28A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE28AC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82FE28B0: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE28B4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE28B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE28BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE28C0: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE28C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE28C8: 4E800421  bctrl
	ctx.lr = 0x82FE28CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE28CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE28D0: 817A0040  lwz r11, 0x40(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE28D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE28D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE28DC: 4E800421  bctrl
	ctx.lr = 0x82FE28E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE28E0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE28E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE28E8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE28EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE28F0: 4E800421  bctrl
	ctx.lr = 0x82FE28F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE28F4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE28F8: 4082FFB0  bne 0x82fe28a8
	if !ctx.cr[0].eq {
	pc = 0x82FE28A8; continue 'dispatch;
	}
	// 82FE28FC: 357CFFF4  addic. r11, r28, -0xc
	ctx.xer.ca = (ctx.r[28].u32 > (!(-12 as u32)));
	ctx.r[11].s64 = ctx.r[28].s64 + -12;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2900: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82FE2904: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE2908: 40820008  bne 0x82fe2910
	if !ctx.cr[0].eq {
	pc = 0x82FE2910; continue 'dispatch;
	}
	// 82FE290C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE2910: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82FE2914: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE2918: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82FE291C: 4BFFCE5D  bl 0x82fdf778
	ctx.lr = 0x82FE2920;
	sub_82FDF778(ctx, base);
	// 82FE2920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2924: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FE2928: 481C5888  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE292C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE292C size=52
    let mut pc: u32 = 0x82FE292C;
    'dispatch: loop {
        match pc {
            0x82FE292C => {
    //   block [0x82FE292C..0x82FE2960)
	// 82FE292C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE2930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE293C: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE2940: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 82FE2944: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE2948: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE294C: 4BFF5995  bl 0x82fd82e0
	ctx.lr = 0x82FE2950;
	sub_82FD82E0(ctx, base);
	// 82FE2950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE295C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2960 size=124
    let mut pc: u32 = 0x82FE2960;
    'dispatch: loop {
        match pc {
            0x82FE2960 => {
    //   block [0x82FE2960..0x82FE29DC)
	// 82FE2960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2964: 481C5805  bl 0x831a8168
	ctx.lr = 0x82FE2968;
	sub_831A8130(ctx, base);
	// 82FE2968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE296C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE2970: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE2974: 817E008C  lwz r11, 0x8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE2978: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE297C: 41820058  beq 0x82fe29d4
	if ctx.cr[0].eq {
	pc = 0x82FE29D4; continue 'dispatch;
	}
	// 82FE2980: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2984: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2988: 4182004C  beq 0x82fe29d4
	if ctx.cr[0].eq {
	pc = 0x82FE29D4; continue 'dispatch;
	}
	// 82FE298C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FE2990: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE2994: 419A0040  beq cr6, 0x82fe29d4
	if ctx.cr[6].eq {
	pc = 0x82FE29D4; continue 'dispatch;
	}
	// 82FE2998: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE299C: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE29A0: 48049ED1  bl 0x8302c870
	ctx.lr = 0x82FE29A4;
	sub_8302C870(ctx, base);
	// 82FE29A4: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FE29A8: 419A0014  beq cr6, 0x82fe29bc
	if ctx.cr[6].eq {
	pc = 0x82FE29BC; continue 'dispatch;
	}
	// 82FE29AC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FE29B0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FE29B4: 4198FFE4  blt cr6, 0x82fe2998
	if ctx.cr[6].lt {
	pc = 0x82FE2998; continue 'dispatch;
	}
	// 82FE29B8: 4800001C  b 0x82fe29d4
	pc = 0x82FE29D4; continue 'dispatch;
	// 82FE29BC: 807E008C  lwz r3, 0x8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(140 as u32) ) } as u64;
	// 82FE29C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE29C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE29C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE29CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE29D0: 4E800421  bctrl
	ctx.lr = 0x82FE29D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE29D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE29D8: 481C57E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE29E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE29E0 size=8
    let mut pc: u32 = 0x82FE29E0;
    'dispatch: loop {
        match pc {
            0x82FE29E0 => {
    //   block [0x82FE29E0..0x82FE29E8)
	// 82FE29E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE29E4: 8213ADE8  lwz r16, -0x5218(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-21016 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE29E8 size=140
    let mut pc: u32 = 0x82FE29E8;
    'dispatch: loop {
        match pc {
            0x82FE29E8 => {
    //   block [0x82FE29E8..0x82FE2A74)
	// 82FE29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE29EC: 481C5779  bl 0x831a8164
	ctx.lr = 0x82FE29F0;
	sub_831A8130(ctx, base);
	// 82FE29F0: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FE29F4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE29F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE29FC: 907F00C4  stw r3, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[3].u32 ) };
	// 82FE2A00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE2A04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FE2A08: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82FE2A0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE2A10: 409A0028  bne cr6, 0x82fe2a38
	if !ctx.cr[6].eq {
	pc = 0x82FE2A38; continue 'dispatch;
	}
	// 82FE2A14: 80C30088  lwz r6, 0x88(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE2A18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE2A1C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE2A20: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE2A24: 480174AD  bl 0x82ff9ed0
	ctx.lr = 0x82FE2A28;
	sub_82FF9ED0(ctx, base);
	// 82FE2A28: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE2A2C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE2A30: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE2A34: 481CE1F5  bl 0x831b0c28
	ctx.lr = 0x82FE2A38;
	sub_831B0C28(ctx, base);
	// 82FE2A38: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82FE2A3C: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82FE2A40: 4BFFEFB1  bl 0x82fe19f0
	ctx.lr = 0x82FE2A44;
	sub_82FE19F0(ctx, base);
	// 82FE2A44: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE2A48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2A4C: 4182001C  beq 0x82fe2a68
	if ctx.cr[0].eq {
	pc = 0x82FE2A68; continue 'dispatch;
	}
	// 82FE2A50: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82FE2A54: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE2A58: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE2A5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE2A60: 4801B831  bl 0x82ffe290
	ctx.lr = 0x82FE2A64;
	sub_82FFE290(ctx, base);
	// 82FE2A64: 48000008  b 0x82fe2a6c
	pc = 0x82FE2A6C; continue 'dispatch;
	// 82FE2A68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE2A6C: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FE2A70: 481C5744  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2A74 size=48
    let mut pc: u32 = 0x82FE2A74;
    'dispatch: loop {
        match pc {
            0x82FE2A74 => {
    //   block [0x82FE2A74..0x82FE2AA4)
	// 82FE2A74: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FE2A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2A84: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FE2A88: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 82FE2A8C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE2A90: 480E5051  bl 0x830c7ae0
	ctx.lr = 0x82FE2A94;
	sub_830C7AE0(ctx, base);
	// 82FE2A94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2AA8 size=8
    let mut pc: u32 = 0x82FE2AA8;
    'dispatch: loop {
        match pc {
            0x82FE2AA8 => {
    //   block [0x82FE2AA8..0x82FE2AB0)
	// 82FE2AA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE2AAC: 8213AE20  lwz r16, -0x51e0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20960 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2AB0 size=164
    let mut pc: u32 = 0x82FE2AB0;
    'dispatch: loop {
        match pc {
            0x82FE2AB0 => {
    //   block [0x82FE2AB0..0x82FE2B54)
	// 82FE2AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2AB4: 481C56AD  bl 0x831a8160
	ctx.lr = 0x82FE2AB8;
	sub_831A8130(ctx, base);
	// 82FE2AB8: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FE2ABC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2AC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE2AC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE2AC8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE2ACC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FE2AD0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82FE2AD4: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82FE2AD8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE2ADC: 419A0054  beq cr6, 0x82fe2b30
	if ctx.cr[6].eq {
	pc = 0x82FE2B30; continue 'dispatch;
	}
	// 82FE2AE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE2AE4: 4BFFEB5D  bl 0x82fe1640
	ctx.lr = 0x82FE2AE8;
	sub_82FE1640(ctx, base);
	// 82FE2AE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2AEC: 41820044  beq 0x82fe2b30
	if ctx.cr[0].eq {
	pc = 0x82FE2B30; continue 'dispatch;
	}
	// 82FE2AF0: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 82FE2AF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2AF8: 4BFFEEF9  bl 0x82fe19f0
	ctx.lr = 0x82FE2AFC;
	sub_82FE19F0(ctx, base);
	// 82FE2AFC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE2B00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2B04: 41820020  beq 0x82fe2b24
	if ctx.cr[0].eq {
	pc = 0x82FE2B24; continue 'dispatch;
	}
	// 82FE2B08: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82FE2B0C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82FE2B10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FE2B14: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE2B18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE2B1C: 4801BFBD  bl 0x82ffead8
	ctx.lr = 0x82FE2B20;
	sub_82FFEAD8(ctx, base);
	// 82FE2B20: 48000008  b 0x82fe2b28
	pc = 0x82FE2B28; continue 'dispatch;
	// 82FE2B24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE2B28: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FE2B2C: 481C5684  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82FE2B30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE2B34: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE2B38: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE2B3C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE2B40: 48017391  bl 0x82ff9ed0
	ctx.lr = 0x82FE2B44;
	sub_82FF9ED0(ctx, base);
	// 82FE2B44: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE2B48: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE2B4C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE2B50: 481CE0D9  bl 0x831b0c28
	ctx.lr = 0x82FE2B54;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2B54 size=44
    let mut pc: u32 = 0x82FE2B54;
    'dispatch: loop {
        match pc {
            0x82FE2B54 => {
    //   block [0x82FE2B54..0x82FE2B80)
	// 82FE2B54: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FE2B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2B64: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FE2B68: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE2B6C: 480E4F75  bl 0x830c7ae0
	ctx.lr = 0x82FE2B70;
	sub_830C7AE0(ctx, base);
	// 82FE2B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2B80 size=124
    let mut pc: u32 = 0x82FE2B80;
    'dispatch: loop {
        match pc {
            0x82FE2B80 => {
    //   block [0x82FE2B80..0x82FE2BFC)
	// 82FE2B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2B84: 481C55E5  bl 0x831a8168
	ctx.lr = 0x82FE2B88;
	sub_831A8130(ctx, base);
	// 82FE2B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2B8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE2B90: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE2B94: 817E0088  lwz r11, 0x88(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE2B98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2B9C: 41820058  beq 0x82fe2bf4
	if ctx.cr[0].eq {
	pc = 0x82FE2BF4; continue 'dispatch;
	}
	// 82FE2BA0: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2BA4: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2BA8: 4182004C  beq 0x82fe2bf4
	if ctx.cr[0].eq {
	pc = 0x82FE2BF4; continue 'dispatch;
	}
	// 82FE2BAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82FE2BB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE2BB4: 419A0040  beq cr6, 0x82fe2bf4
	if ctx.cr[6].eq {
	pc = 0x82FE2BF4; continue 'dispatch;
	}
	// 82FE2BB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE2BBC: 807E0088  lwz r3, 0x88(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE2BC0: 48049CB1  bl 0x8302c870
	ctx.lr = 0x82FE2BC4;
	sub_8302C870(ctx, base);
	// 82FE2BC4: 7F03E040  cmplw cr6, r3, r28
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82FE2BC8: 419A0014  beq cr6, 0x82fe2bdc
	if ctx.cr[6].eq {
	pc = 0x82FE2BDC; continue 'dispatch;
	}
	// 82FE2BCC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82FE2BD0: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82FE2BD4: 4198FFE4  blt cr6, 0x82fe2bb8
	if ctx.cr[6].lt {
	pc = 0x82FE2BB8; continue 'dispatch;
	}
	// 82FE2BD8: 4800001C  b 0x82fe2bf4
	pc = 0x82FE2BF4; continue 'dispatch;
	// 82FE2BDC: 807E0088  lwz r3, 0x88(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE2BE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE2BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2BE8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2BF0: 4E800421  bctrl
	ctx.lr = 0x82FE2BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE2BF8: 481C55C0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2C00 size=152
    let mut pc: u32 = 0x82FE2C00;
    'dispatch: loop {
        match pc {
            0x82FE2C00 => {
    //   block [0x82FE2C00..0x82FE2C98)
	// 82FE2C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2C08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE2C0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE2C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2C14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE2C18: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE2C1C: 409A000C  bne cr6, 0x82fe2c28
	if !ctx.cr[6].eq {
	pc = 0x82FE2C28; continue 'dispatch;
	}
	// 82FE2C20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE2C24: 4800005C  b 0x82fe2c80
	pc = 0x82FE2C80; continue 'dispatch;
	// 82FE2C28: A17E0000  lhz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2C2C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2C30: 41820028  beq 0x82fe2c58
	if ctx.cr[0].eq {
	pc = 0x82FE2C58; continue 'dispatch;
	}
	// 82FE2C34: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 82FE2C38: 48000008  b 0x82fe2c40
	pc = 0x82FE2C40; continue 'dispatch;
	// 82FE2C3C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82FE2C40: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2C44: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2C48: 4082FFF4  bne 0x82fe2c3c
	if !ctx.cr[0].eq {
	pc = 0x82FE2C3C; continue 'dispatch;
	}
	// 82FE2C4C: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 82FE2C50: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82FE2C54: 48000008  b 0x82fe2c5c
	pc = 0x82FE2C5C; continue 'dispatch;
	// 82FE2C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE2C5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82FE2C60: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE2C64: 556A07BE  clrlwi r10, r11, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 82FE2C68: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82FE2C6C: 4BFFED85  bl 0x82fe19f0
	ctx.lr = 0x82FE2C70;
	sub_82FE19F0(ctx, base);
	// 82FE2C70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE2C74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE2C78: 4BFEEEF1  bl 0x82fd1b68
	ctx.lr = 0x82FE2C7C;
	sub_82FD1B68(ctx, base);
	// 82FE2C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE2C80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE2C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2C8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE2C90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE2C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2C98 size=48
    let mut pc: u32 = 0x82FE2C98;
    'dispatch: loop {
        match pc {
            0x82FE2C98 => {
    //   block [0x82FE2C98..0x82FE2CC8)
	// 82FE2C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2CA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE2CA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2CA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE2CAC: 4BFFFF55  bl 0x82fe2c00
	ctx.lr = 0x82FE2CB0;
	sub_82FE2C00(ctx, base);
	// 82FE2CB0: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82FE2CB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE2CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2CC8 size=48
    let mut pc: u32 = 0x82FE2CC8;
    'dispatch: loop {
        match pc {
            0x82FE2CC8 => {
    //   block [0x82FE2CC8..0x82FE2CF8)
	// 82FE2CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE2CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE2CDC: 4BFFFF25  bl 0x82fe2c00
	ctx.lr = 0x82FE2CE0;
	sub_82FE2C00(ctx, base);
	// 82FE2CE0: 907F0030  stw r3, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 82FE2CE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2CF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE2CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2CF8 size=172
    let mut pc: u32 = 0x82FE2CF8;
    'dispatch: loop {
        match pc {
            0x82FE2CF8 => {
    //   block [0x82FE2CF8..0x82FE2DA4)
	// 82FE2CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE2D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE2D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2D0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82FE2D10: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE2D14: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82FE2D18: 419A0064  beq cr6, 0x82fe2d7c
	if ctx.cr[6].eq {
	pc = 0x82FE2D7C; continue 'dispatch;
	}
	// 82FE2D1C: A17F0000  lhz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2D20: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2D24: 41820058  beq 0x82fe2d7c
	if ctx.cr[0].eq {
	pc = 0x82FE2D7C; continue 'dispatch;
	}
	// 82FE2D28: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE2D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE2D30: 388B7CB0  addi r4, r11, 0x7cb0
	ctx.r[4].s64 = ctx.r[11].s64 + 31920;
	// 82FE2D34: 4BFF0F0D  bl 0x82fd3c40
	ctx.lr = 0x82FE2D38;
	sub_82FD3C40(ctx, base);
	// 82FE2D38: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2D3C: 40820040  bne 0x82fe2d7c
	if !ctx.cr[0].eq {
	pc = 0x82FE2D7C; continue 'dispatch;
	}
	// 82FE2D40: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE2D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE2D48: 388B7CB8  addi r4, r11, 0x7cb8
	ctx.r[4].s64 = ctx.r[11].s64 + 31928;
	// 82FE2D4C: 4BFF0EF5  bl 0x82fd3c40
	ctx.lr = 0x82FE2D50;
	sub_82FD3C40(ctx, base);
	// 82FE2D50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2D54: 40820028  bne 0x82fe2d7c
	if !ctx.cr[0].eq {
	pc = 0x82FE2D7C; continue 'dispatch;
	}
	// 82FE2D58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE2D5C: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE2D60: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE2D64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2D68: 48017169  bl 0x82ff9ed0
	ctx.lr = 0x82FE2D6C;
	sub_82FF9ED0(ctx, base);
	// 82FE2D6C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE2D70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE2D74: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE2D78: 481CDEB1  bl 0x831b0c28
	ctx.lr = 0x82FE2D7C;
	sub_831B0C28(ctx, base);
	// 82FE2D7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE2D80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2D84: 4BFFFE7D  bl 0x82fe2c00
	ctx.lr = 0x82FE2D88;
	sub_82FE2C00(ctx, base);
	// 82FE2D88: 907E0038  stw r3, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82FE2D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE2D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2D98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE2D9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE2DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2DA8 size=8
    let mut pc: u32 = 0x82FE2DA8;
    'dispatch: loop {
        match pc {
            0x82FE2DA8 => {
    //   block [0x82FE2DA8..0x82FE2DB0)
	// 82FE2DA8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE2DAC: 8213AE70  lwz r16, -0x5190(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20880 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2DB0 size=112
    let mut pc: u32 = 0x82FE2DB0;
    'dispatch: loop {
        match pc {
            0x82FE2DB0 => {
    //   block [0x82FE2DB0..0x82FE2E20)
	// 82FE2DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2DB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE2DBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE2DC0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE2DC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2DC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE2DCC: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE2DD0: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE2DD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE2DD8: 409A002C  bne cr6, 0x82fe2e04
	if !ctx.cr[6].eq {
	pc = 0x82FE2E04; continue 'dispatch;
	}
	// 82FE2DDC: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82FE2DE0: 4BFFEC11  bl 0x82fe19f0
	ctx.lr = 0x82FE2DE4;
	sub_82FE19F0(ctx, base);
	// 82FE2DE4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE2DE8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2DEC: 41820010  beq 0x82fe2dfc
	if ctx.cr[0].eq {
	pc = 0x82FE2DFC; continue 'dispatch;
	}
	// 82FE2DF0: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE2DF4: 4801BE7D  bl 0x82ffec70
	ctx.lr = 0x82FE2DF8;
	sub_82FFEC70(ctx, base);
	// 82FE2DF8: 48000008  b 0x82fe2e00
	pc = 0x82FE2E00; continue 'dispatch;
	// 82FE2DFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE2E00: 907E0040  stw r3, 0x40(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(64 as u32), ctx.r[3].u32 ) };
	// 82FE2E04: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE2E08: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE2E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2E14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE2E18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE2E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2E20 size=44
    let mut pc: u32 = 0x82FE2E20;
    'dispatch: loop {
        match pc {
            0x82FE2E20 => {
    //   block [0x82FE2E20..0x82FE2E4C)
	// 82FE2E20: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE2E24: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2E28: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE2E2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2E30: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE2E34: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE2E38: 480E4CA9  bl 0x830c7ae0
	ctx.lr = 0x82FE2E3C;
	sub_830C7AE0(ctx, base);
	// 82FE2E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE2E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE2E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE2E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE2E50 size=8
    let mut pc: u32 = 0x82FE2E50;
    'dispatch: loop {
        match pc {
            0x82FE2E50 => {
    //   block [0x82FE2E50..0x82FE2E58)
	// 82FE2E50: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE2E54: 8213AEA8  lwz r16, -0x5158(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20824 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE2E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE2E58 size=2248
    let mut pc: u32 = 0x82FE2E58;
    'dispatch: loop {
        match pc {
            0x82FE2E58 => {
    //   block [0x82FE2E58..0x82FE3720)
	// 82FE2E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE2E5C: 481C52ED  bl 0x831a8148
	ctx.lr = 0x82FE2E60;
	sub_831A8130(ctx, base);
	// 82FE2E60: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 82FE2E64: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE2E68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE2E6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE2E70: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82FE2E74: 7CD43378  mr r20, r6
	ctx.r[20].u64 = ctx.r[6].u64;
	// 82FE2E78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2E7C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2E80: 8ABD0098  lbz r21, 0x98(r29)
	ctx.r[21].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FE2E84: 93BF0114  stw r29, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[29].u32 ) };
	// 82FE2E88: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE2E8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2E90: 4E800421  bctrl
	ctx.lr = 0x82FE2E94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2E94: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE2E98: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE2E9C: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 82FE2EA0: 4199085C  bgt cr6, 0x82fe36fc
	if ctx.cr[6].gt {
	pc = 0x82FE36FC; continue 'dispatch;
	}
	// 82FE2EA4: 3D808214  lis r12, -0x7dec
	ctx.r[12].s64 = -2112618496;
	// 82FE2EA8: 398CA788  addi r12, r12, -0x5878
	ctx.r[12].s64 = ctx.r[12].s64 + -22648;
	// 82FE2EAC: 5560083C  slwi r0, r11, 1
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82FE2EB0: 7C0C022E  lhzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82FE2EB4: 3D8082FE  lis r12, -0x7d02
	ctx.r[12].s64 = -2097283072;
	// 82FE2EB8: 398C2ECC  addi r12, r12, 0x2ecc
	ctx.r[12].s64 = ctx.r[12].s64 + 11980;
	// 82FE2EBC: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82FE2EC0: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82FE2EC4: 60000000  nop
	// 82FE2EC8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 82FE2ECC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2ED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2ED4: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE2ED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2EDC: 4E800421  bctrl
	ctx.lr = 0x82FE2EE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2EE0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2EE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2EE8: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2EEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2EF0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE2EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2EF8: 40820020  bne 0x82fe2f18
	if !ctx.cr[0].eq {
	pc = 0x82FE2F18; continue 'dispatch;
	}
	// 82FE2EFC: 4E800421  bctrl
	ctx.lr = 0x82FE2F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE2F04: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2F08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE2F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2F10: 4E800421  bctrl
	ctx.lr = 0x82FE2F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F14: 48000038  b 0x82fe2f4c
	pc = 0x82FE2F4C; continue 'dispatch;
	// 82FE2F18: 4E800421  bctrl
	ctx.lr = 0x82FE2F1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2F20: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE2F24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2F28: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE2F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2F30: 4E800421  bctrl
	ctx.lr = 0x82FE2F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE2F38: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE2F3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE2F40: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE2F44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2F48: 4E800421  bctrl
	ctx.lr = 0x82FE2F4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2F50: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE2F54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE2F58: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FE2F5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2F60: 4E800421  bctrl
	ctx.lr = 0x82FE2F64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F64: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82FE2F68: 418206AC  beq 0x82fe3614
	if ctx.cr[0].eq {
	pc = 0x82FE3614; continue 'dispatch;
	}
	// 82FE2F6C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2F70: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82FE2F74: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE2F78: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE2F7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2F80: 4E800421  bctrl
	ctx.lr = 0x82FE2F84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2F84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE2F88: 4182068C  beq 0x82fe3614
	if ctx.cr[0].eq {
	pc = 0x82FE3614; continue 'dispatch;
	}
	// 82FE2F8C: 3EE08214  lis r23, -0x7dec
	ctx.r[23].s64 = -2112618496;
	// 82FE2F90: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2F94: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82FE2F98: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE2F9C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE2FA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2FA4: 4E800421  bctrl
	ctx.lr = 0x82FE2FA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2FA8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE2FAC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2FB0: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FE2FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2FB8: 4E800421  bctrl
	ctx.lr = 0x82FE2FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2FBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE2FC0: 418200DC  beq 0x82fe309c
	if ctx.cr[0].eq {
	pc = 0x82FE309C; continue 'dispatch;
	}
	// 82FE2FC4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2FC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE2FCC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE2FD0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE2FD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE2FD8: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE2FDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2FE0: 4E800421  bctrl
	ctx.lr = 0x82FE2FE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2FE4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE2FE8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FE2FEC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE2FF0: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE2FF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE2FF8: 4E800421  bctrl
	ctx.lr = 0x82FE2FFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE2FFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3000: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3004: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE3008: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE300C: 4082000C  bne 0x82fe3018
	if !ctx.cr[0].eq {
	pc = 0x82FE3018; continue 'dispatch;
	}
	// 82FE3010: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FE3014: 48000008  b 0x82fe301c
	pc = 0x82FE301C; continue 'dispatch;
	// 82FE3018: 816B00CC  lwz r11, 0xcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 82FE301C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3020: 4E800421  bctrl
	ctx.lr = 0x82FE3024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3024: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3028: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE302C: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FE3030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3034: 4E800421  bctrl
	ctx.lr = 0x82FE3038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3038: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE303C: 41820060  beq 0x82fe309c
	if ctx.cr[0].eq {
	pc = 0x82FE309C; continue 'dispatch;
	}
	// 82FE3040: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 82FE3044: A157A6B8  lhz r10, -0x5948(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[23].u32.wrapping_add(-22856 as u32) ) } as u64;
	// 82FE3048: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE304C: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82FE3050: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FE3054: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FE3058: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE305C: 409A0034  bne cr6, 0x82fe3090
	if !ctx.cr[6].eq {
	pc = 0x82FE3090; continue 'dispatch;
	}
	// 82FE3060: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FE3064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3068: 4BFFE989  bl 0x82fe19f0
	ctx.lr = 0x82FE306C;
	sub_82FE19F0(ctx, base);
	// 82FE306C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE3070: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3074: 41820014  beq 0x82fe3088
	if ctx.cr[0].eq {
	pc = 0x82FE3088; continue 'dispatch;
	}
	// 82FE3078: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE307C: 388001F4  li r4, 0x1f4
	ctx.r[4].s64 = 500;
	// 82FE3080: 4800E649  bl 0x82ff16c8
	ctx.lr = 0x82FE3084;
	sub_82FF16C8(ctx, base);
	// 82FE3084: 48000008  b 0x82fe308c
	pc = 0x82FE308C; continue 'dispatch;
	// 82FE3088: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE308C: 907D0028  stw r3, 0x28(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 82FE3090: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE3094: 807D0028  lwz r3, 0x28(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 82FE3098: 4800E779  bl 0x82ff1810
	ctx.lr = 0x82FE309C;
	sub_82FF1810(ctx, base);
	// 82FE309C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE30A0: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 82FE30A4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE30A8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE30AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE30B0: 4E800421  bctrl
	ctx.lr = 0x82FE30B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE30B4: 7F181840  cmplw cr6, r24, r3
	ctx.cr[6].compare_u32(ctx.r[24].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FE30B8: 4198FED8  blt cr6, 0x82fe2f90
	if ctx.cr[6].lt {
	pc = 0x82FE2F90; continue 'dispatch;
	}
	// 82FE30BC: 48000558  b 0x82fe3614
	pc = 0x82FE3614; continue 'dispatch;
	// 82FE30C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE30C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE30C8: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE30CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE30D0: 4E800421  bctrl
	ctx.lr = 0x82FE30D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE30D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE30D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE30DC: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE30E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE30E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE30E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE30EC: 40820020  bne 0x82fe310c
	if !ctx.cr[0].eq {
	pc = 0x82FE310C; continue 'dispatch;
	}
	// 82FE30F0: 4E800421  bctrl
	ctx.lr = 0x82FE30F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE30F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE30F8: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE30FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3104: 4E800421  bctrl
	ctx.lr = 0x82FE3108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3108: 48000038  b 0x82fe3140
	pc = 0x82FE3140; continue 'dispatch;
	// 82FE310C: 4E800421  bctrl
	ctx.lr = 0x82FE3110;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3110: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3114: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE3118: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE311C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82FE3120: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3124: 4E800421  bctrl
	ctx.lr = 0x82FE3128;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3128: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE312C: 817C0044  lwz r11, 0x44(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) } as u64;
	// 82FE3130: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3134: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE3138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE313C: 4E800421  bctrl
	ctx.lr = 0x82FE3140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3140: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE3144: 3AC00001  li r22, 1
	ctx.r[22].s64 = 1;
	// 82FE3148: 480004CC  b 0x82fe3614
	pc = 0x82FE3614; continue 'dispatch;
	// 82FE314C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3154: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3158: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE315C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3160: 4E800421  bctrl
	ctx.lr = 0x82FE3164;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3164: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3168: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE316C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3170: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3174: 4E800421  bctrl
	ctx.lr = 0x82FE3178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3178: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE317C: 48000498  b 0x82fe3614
	pc = 0x82FE3614; continue 'dispatch;
	// 82FE3180: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3188: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE318C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3190: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3194: 4E800421  bctrl
	ctx.lr = 0x82FE3198;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3198: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE319C: 4BFFFFCC  b 0x82fe3168
	pc = 0x82FE3168; continue 'dispatch;
	// 82FE31A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE31A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE31A8: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE31AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE31B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE31B4: 4E800421  bctrl
	ctx.lr = 0x82FE31B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE31B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE31BC: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE31C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE31C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE31C8: 4E800421  bctrl
	ctx.lr = 0x82FE31CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE31CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE31D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE31D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE31D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE31DC: 997D0098  stb r11, 0x98(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 82FE31E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE31E4: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FE31E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE31EC: 4E800421  bctrl
	ctx.lr = 0x82FE31F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE31F0: 48000424  b 0x82fe3614
	pc = 0x82FE3614; continue 'dispatch;
	// 82FE31F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE31F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE31FC: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3200: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3204: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3208: 4E800421  bctrl
	ctx.lr = 0x82FE320C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE320C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3210: 817C0090  lwz r11, 0x90(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE3214: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3218: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE321C: 4E800421  bctrl
	ctx.lr = 0x82FE3220;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3220: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3224: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE3228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE322C: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FE3230: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3238: 4E800421  bctrl
	ctx.lr = 0x82FE323C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE323C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3240: 817B00C4  lwz r11, 0xc4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FE3244: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE3248: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE324C: 4E800421  bctrl
	ctx.lr = 0x82FE3250;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3250: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3254: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3258: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE325C: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FE3260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3264: 4E800421  bctrl
	ctx.lr = 0x82FE3268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3268: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE326C: 817B00C8  lwz r11, 0xc8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(200 as u32) ) } as u64;
	// 82FE3270: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE3274: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3278: 4E800421  bctrl
	ctx.lr = 0x82FE327C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE327C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3284: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3288: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE328C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3290: 4E800421  bctrl
	ctx.lr = 0x82FE3294;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3294: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3298: 817B00C0  lwz r11, 0xc0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(192 as u32) ) } as u64;
	// 82FE329C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE32A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE32A4: 4E800421  bctrl
	ctx.lr = 0x82FE32A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE32A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE32AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE32B0: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82FE32B4: 4BFFC0DD  bl 0x82fdf390
	ctx.lr = 0x82FE32B8;
	sub_82FDF390(ctx, base);
	// 82FE32B8: 4800035C  b 0x82fe3614
	pc = 0x82FE3614; continue 'dispatch;
	// 82FE32BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE32C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE32C4: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE32C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE32CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE32D0: 4E800421  bctrl
	ctx.lr = 0x82FE32D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE32D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE32D8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE32DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE32E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE32E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE32E8: 4E800421  bctrl
	ctx.lr = 0x82FE32EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE32EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE32F0: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE32F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE32F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE32FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3300: 4E800421  bctrl
	ctx.lr = 0x82FE3304;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3304: 4BFFFE74  b 0x82fe3178
	pc = 0x82FE3178; continue 'dispatch;
	// 82FE3308: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE330C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3310: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3314: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE331C: 4E800421  bctrl
	ctx.lr = 0x82FE3320;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3320: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3324: 4BFFFE44  b 0x82fe3168
	pc = 0x82FE3168; continue 'dispatch;
	// 82FE3328: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE332C: 40820028  bne 0x82fe3354
	if !ctx.cr[0].eq {
	pc = 0x82FE3354; continue 'dispatch;
	}
	// 82FE3330: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE3334: 80DD0090  lwz r6, 0x90(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE3338: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE333C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE3340: 48016B91  bl 0x82ff9ed0
	ctx.lr = 0x82FE3344;
	sub_82FF9ED0(ctx, base);
	// 82FE3344: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3348: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE334C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE3350: 481CD8D9  bl 0x831b0c28
	ctx.lr = 0x82FE3354;
	sub_831B0C28(ctx, base);
	// 82FE3354: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE335C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3360: 816B00AC  lwz r11, 0xac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 82FE3364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3368: 4E800421  bctrl
	ctx.lr = 0x82FE336C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE336C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3370: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE3374: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3378: 816B00A8  lwz r11, 0xa8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FE337C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3380: 4E800421  bctrl
	ctx.lr = 0x82FE3384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3384: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3388: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82FE338C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3390: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3394: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3398: 4E800421  bctrl
	ctx.lr = 0x82FE339C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE339C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE33A0: 817C0094  lwz r11, 0x94(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE33A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE33A8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82FE33AC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FE33B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE33B4: 4E800421  bctrl
	ctx.lr = 0x82FE33B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE33B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE33BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE33C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE33C4: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FE33C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE33CC: 4E800421  bctrl
	ctx.lr = 0x82FE33D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE33D0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE33D4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE33D8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE33DC: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FE33E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE33E4: 4E800421  bctrl
	ctx.lr = 0x82FE33E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE33E8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FE33EC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FE33F0: 419A0094  beq cr6, 0x82fe3484
	if ctx.cr[6].eq {
	pc = 0x82FE3484; continue 'dispatch;
	}
	// 82FE33F4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE33F8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE33FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE3400: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3404: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3408: 4E800421  bctrl
	ctx.lr = 0x82FE340C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE340C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3410: 41820074  beq 0x82fe3484
	if ctx.cr[0].eq {
	pc = 0x82FE3484; continue 'dispatch;
	}
	// 82FE3414: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3418: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE341C: 83190000  lwz r24, 0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3420: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE3424: 82FD0000  lwz r23, 0(r29)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3428: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE342C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3430: 4E800421  bctrl
	ctx.lr = 0x82FE3434;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3434: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3438: 817700BC  lwz r11, 0xbc(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE343C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3440: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE3444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE3448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE344C: 4E800421  bctrl
	ctx.lr = 0x82FE3450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3450: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3454: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3458: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE345C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3460: 4E800421  bctrl
	ctx.lr = 0x82FE3464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3464: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3468: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FE346C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE3470: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3474: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3478: 4E800421  bctrl
	ctx.lr = 0x82FE347C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE347C: 7F1A1840  cmplw cr6, r26, r3
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FE3480: 4198FF94  blt cr6, 0x82fe3414
	if ctx.cr[6].lt {
	pc = 0x82FE3414; continue 'dispatch;
	}
	// 82FE3484: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE348C: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE3490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3494: 4E800421  bctrl
	ctx.lr = 0x82FE3498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3498: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE349C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82FE34A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE34A4: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE34A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE34AC: 4E800421  bctrl
	ctx.lr = 0x82FE34B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE34B0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82FE34B4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FE34B8: 419A0094  beq cr6, 0x82fe354c
	if ctx.cr[6].eq {
	pc = 0x82FE354C; continue 'dispatch;
	}
	// 82FE34BC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE34C0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE34C4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE34C8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE34CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE34D0: 4E800421  bctrl
	ctx.lr = 0x82FE34D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE34D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE34D8: 41820074  beq 0x82fe354c
	if ctx.cr[0].eq {
	pc = 0x82FE354C; continue 'dispatch;
	}
	// 82FE34DC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE34E0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82FE34E4: 83190000  lwz r24, 0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE34E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE34EC: 82FD0000  lwz r23, 0(r29)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE34F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE34F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE34F8: 4E800421  bctrl
	ctx.lr = 0x82FE34FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE34FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3500: 817700BC  lwz r11, 0xbc(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE3504: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3508: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE350C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE3510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3514: 4E800421  bctrl
	ctx.lr = 0x82FE3518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3518: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE351C: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3520: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82FE3524: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3528: 4E800421  bctrl
	ctx.lr = 0x82FE352C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE352C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3530: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82FE3534: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE3538: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE353C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3540: 4E800421  bctrl
	ctx.lr = 0x82FE3544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3544: 7F1A1840  cmplw cr6, r26, r3
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82FE3548: 4198FF94  blt cr6, 0x82fe34dc
	if ctx.cr[6].lt {
	pc = 0x82FE34DC; continue 'dispatch;
	}
	// 82FE354C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3550: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3554: 816B00B0  lwz r11, 0xb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(176 as u32) ) } as u64;
	// 82FE3558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE355C: 4E800421  bctrl
	ctx.lr = 0x82FE3560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3560: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82FE3564: 418200B0  beq 0x82fe3614
	if ctx.cr[0].eq {
	pc = 0x82FE3614; continue 'dispatch;
	}
	// 82FE3568: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE356C: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE3570: 48000098  b 0x82fe3608
	pc = 0x82FE3608; continue 'dispatch;
	// 82FE3574: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3578: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE357C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE3580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3584: 4E800421  bctrl
	ctx.lr = 0x82FE3588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3588: 4BFFFBF0  b 0x82fe3178
	pc = 0x82FE3178; continue 'dispatch;
	// 82FE358C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3594: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3598: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE359C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE35A0: 4E800421  bctrl
	ctx.lr = 0x82FE35A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE35A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE35A8: 817C009C  lwz r11, 0x9c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FE35AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE35B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE35B4: 4E800421  bctrl
	ctx.lr = 0x82FE35B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE35B8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE35BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE35C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE35C4: 816B009C  lwz r11, 0x9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 82FE35C8: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE35CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE35D0: 4E800421  bctrl
	ctx.lr = 0x82FE35D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE35D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE35D8: 817B00A4  lwz r11, 0xa4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE35DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE35E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE35E4: 4E800421  bctrl
	ctx.lr = 0x82FE35E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE35E8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE35EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE35F0: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE35F4: 816B00A0  lwz r11, 0xa0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) } as u64;
	// 82FE35F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE35FC: 4E800421  bctrl
	ctx.lr = 0x82FE3600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3600: 817B00A8  lwz r11, 0xa8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(168 as u32) ) } as u64;
	// 82FE3604: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3608: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE360C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3610: 4E800421  bctrl
	ctx.lr = 0x82FE3614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3614: 56CB063F  clrlwi. r11, r22, 0x18
	ctx.r[11].u64 = ctx.r[22].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE3618: 41820068  beq 0x82fe3680
	if ctx.cr[0].eq {
	pc = 0x82FE3680; continue 'dispatch;
	}
	// 82FE361C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3620: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3624: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE3628: 48000048  b 0x82fe3670
	pc = 0x82FE3670; continue 'dispatch;
	// 82FE362C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3630: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE3634: 835C0000  lwz r26, 0(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3638: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE363C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE3640: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE3644: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82FE3648: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE364C: 4E800421  bctrl
	ctx.lr = 0x82FE3650;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3650: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE3654: 817A0040  lwz r11, 0x40(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(64 as u32) ) } as u64;
	// 82FE3658: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE365C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3660: 4E800421  bctrl
	ctx.lr = 0x82FE3664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3664: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3668: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE366C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82FE3670: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3674: 4E800421  bctrl
	ctx.lr = 0x82FE3678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3678: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82FE367C: 4082FFB0  bne 0x82fe362c
	if !ctx.cr[0].eq {
	pc = 0x82FE362C; continue 'dispatch;
	}
	// 82FE3680: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3684: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE3688: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE368C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3690: 4E800421  bctrl
	ctx.lr = 0x82FE3694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3694: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE3698: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82FE369C: 419A0024  beq cr6, 0x82fe36c0
	if ctx.cr[6].eq {
	pc = 0x82FE36C0; continue 'dispatch;
	}
	// 82FE36A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE36A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE36A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE36AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE36B0: 4E800421  bctrl
	ctx.lr = 0x82FE36B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE36B4: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 82FE36B8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 82FE36BC: 409A0018  bne cr6, 0x82fe36d4
	if !ctx.cr[6].eq {
	pc = 0x82FE36D4; continue 'dispatch;
	}
	// 82FE36C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE36C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE36C8: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 82FE36CC: 4BFFBCC5  bl 0x82fdf390
	ctx.lr = 0x82FE36D0;
	sub_82FDF390(ctx, base);
	// 82FE36D0: 9ABD0098  stb r21, 0x98(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(152 as u32), ctx.r[21].u8 ) };
	// 82FE36D4: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE36D8: 40820018  bne 0x82fe36f0
	if !ctx.cr[0].eq {
	pc = 0x82FE36F0; continue 'dispatch;
	}
	// 82FE36DC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE36E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82FE36E4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82FE36E8: 387D0010  addi r3, r29, 0x10
	ctx.r[3].s64 = ctx.r[29].s64 + 16;
	// 82FE36EC: 4BFFC08D  bl 0x82fdf778
	ctx.lr = 0x82FE36F0;
	sub_82FDF778(ctx, base);
	// 82FE36F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE36F4: 383F0100  addi r1, r31, 0x100
	ctx.r[1].s64 = ctx.r[31].s64 + 256;
	// 82FE36F8: 481C4AA0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
	// 82FE36FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE3700: 80DD0090  lwz r6, 0x90(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE3704: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE3708: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE370C: 480167C5  bl 0x82ff9ed0
	ctx.lr = 0x82FE3710;
	sub_82FF9ED0(ctx, base);
	// 82FE3710: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3714: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 82FE3718: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE371C: 481CD50D  bl 0x831b0c28
	ctx.lr = 0x82FE3720;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3720 size=44
    let mut pc: u32 = 0x82FE3720;
    'dispatch: loop {
        match pc {
            0x82FE3720 => {
    //   block [0x82FE3720..0x82FE374C)
	// 82FE3720: 3BECFF00  addi r31, r12, -0x100
	ctx.r[31].s64 = ctx.r[12].s64 + -256;
	// 82FE3724: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3728: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE372C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3730: 809F0114  lwz r4, 0x114(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 82FE3734: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE3738: 480E43A9  bl 0x830c7ae0
	ctx.lr = 0x82FE373C;
	sub_830C7AE0(ctx, base);
	// 82FE373C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE3740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3750 size=196
    let mut pc: u32 = 0x82FE3750;
    'dispatch: loop {
        match pc {
            0x82FE3750 => {
    //   block [0x82FE3750..0x82FE3814)
	// 82FE3750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE375C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE3760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3764: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3768: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE376C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE3770: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82FE3774: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82FE3778: 3BDFFFF4  addi r30, r31, -0xc
	ctx.r[30].s64 = ctx.r[31].s64 + -12;
	// 82FE377C: 4BFFBFFD  bl 0x82fdf778
	ctx.lr = 0x82FE3780;
	sub_82FDF778(ctx, base);
	// 82FE3780: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE3784: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE3788: 419A001C  beq cr6, 0x82fe37a4
	if ctx.cr[6].eq {
	pc = 0x82FE37A4; continue 'dispatch;
	}
	// 82FE378C: 357FFFF4  addic. r11, r31, -0xc
	ctx.xer.ca = (ctx.r[31].u32 > (!(-12 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -12;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE3790: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82FE3794: 40820008  bne 0x82fe379c
	if !ctx.cr[0].eq {
	pc = 0x82FE379C; continue 'dispatch;
	}
	// 82FE3798: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82FE379C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE37A0: 4BFFE639  bl 0x82fe1dd8
	ctx.lr = 0x82FE37A4;
	sub_82FE1DD8(ctx, base);
	// 82FE37A4: 817F006C  lwz r11, 0x6c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE37A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE37AC: 41820030  beq 0x82fe37dc
	if ctx.cr[0].eq {
	pc = 0x82FE37DC; continue 'dispatch;
	}
	// 82FE37B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FE37B4: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 82FE37B8: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE37BC: A14AA6C8  lhz r10, -0x5938(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(-22840 as u32) ) } as u64;
	// 82FE37C0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82FE37C4: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82FE37C8: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE37CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE37D0: 816B0098  lwz r11, 0x98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82FE37D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE37D8: 4E800421  bctrl
	ctx.lr = 0x82FE37DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE37DC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82FE37E0: 419A001C  beq cr6, 0x82fe37fc
	if ctx.cr[6].eq {
	pc = 0x82FE37FC; continue 'dispatch;
	}
	// 82FE37E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE37E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE37EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE37F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE37F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE37F8: 4E800421  bctrl
	ctx.lr = 0x82FE37FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE37FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE3800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3808: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE380C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE3810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3818 size=88
    let mut pc: u32 = 0x82FE3818;
    'dispatch: loop {
        match pc {
            0x82FE3818 => {
    //   block [0x82FE3818..0x82FE3870)
	// 82FE3818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE381C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE3824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE3828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE382C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE3834: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82FE3838: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82FE383C: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FE3840: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE3844: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82FE3848: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FE384C: 4BFFE8D5  bl 0x82fe2120
	ctx.lr = 0x82FE3850;
	sub_82FE2120(ctx, base);
	// 82FE3850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE3854: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82FE3858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE385C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3864: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE3868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE386C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE3870 size=8
    let mut pc: u32 = 0x82FE3870;
    'dispatch: loop {
        match pc {
            0x82FE3870 => {
    //   block [0x82FE3870..0x82FE3878)
	// 82FE3870: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE3874: 8213AF48  lwz r16, -0x50b8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3878 size=184
    let mut pc: u32 = 0x82FE3878;
    'dispatch: loop {
        match pc {
            0x82FE3878 => {
    //   block [0x82FE3878..0x82FE3930)
	// 82FE3878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE387C: 481C48F1  bl 0x831a816c
	ctx.lr = 0x82FE3880;
	sub_831A8130(ctx, base);
	// 82FE3880: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE3884: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3888: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE388C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE3890: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE3894: 98BE0000  stb r5, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[5].u8 ) };
	// 82FE3898: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82FE389C: 909E0008  stw r4, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82FE38A0: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82FE38A4: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82FE38A8: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FE38AC: 90DE0014  stw r6, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82FE38B0: 816BB7E8  lwz r11, -0x4818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE38B4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE38B8: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82FE38BC: 4BFFEA4D  bl 0x82fe2308
	ctx.lr = 0x82FE38C0;
	sub_82FE2308(ctx, base);
	// 82FE38C0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82FE38C4: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE38C8: 4BFF49D1  bl 0x82fd8298
	ctx.lr = 0x82FE38CC;
	sub_82FD8298(ctx, base);
	// 82FE38CC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE38D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE38D4: 4182000C  beq 0x82fe38e0
	if ctx.cr[0].eq {
	pc = 0x82FE38E0; continue 'dispatch;
	}
	// 82FE38D8: 4801BCB9  bl 0x82fff590
	ctx.lr = 0x82FE38DC;
	sub_82FFF590(ctx, base);
	// 82FE38DC: 48000008  b 0x82fe38e4
	pc = 0x82FE38E4; continue 'dispatch;
	// 82FE38E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE38E4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE38E8: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82FE38EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE38F0: 409A000C  bne cr6, 0x82fe38fc
	if !ctx.cr[6].eq {
	pc = 0x82FE38FC; continue 'dispatch;
	}
	// 82FE38F4: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 82FE38F8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82FE38FC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3900: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3904: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE3908: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE390C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3910: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3914: 4E800421  bctrl
	ctx.lr = 0x82FE3918;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE391C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3920: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82FE3924: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82FE3928: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE392C: 481C4890  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3930 size=48
    let mut pc: u32 = 0x82FE3930;
    'dispatch: loop {
        match pc {
            0x82FE3930 => {
    //   block [0x82FE3930..0x82FE3960)
	// 82FE3930: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE3934: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3938: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE393C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3940: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE3944: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3948: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE394C: 4BFF4995  bl 0x82fd82e0
	ctx.lr = 0x82FE3950;
	sub_82FD82E0(ctx, base);
	// 82FE3950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE3954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE395C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE3960 size=8
    let mut pc: u32 = 0x82FE3960;
    'dispatch: loop {
        match pc {
            0x82FE3960 => {
    //   block [0x82FE3960..0x82FE3968)
	// 82FE3960: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE3964: 8213AF90  lwz r16, -0x5070(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20592 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82FE3968 size=452
    let mut pc: u32 = 0x82FE3968;
    'dispatch: loop {
        match pc {
            0x82FE3968 => {
    //   block [0x82FE3968..0x82FE3B2C)
	// 82FE3968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE396C: 481C47F1  bl 0x831a815c
	ctx.lr = 0x82FE3970;
	sub_831A8130(ctx, base);
	// 82FE3970: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE3974: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3978: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE397C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82FE3980: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 82FE3984: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE3988: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FE398C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE3990: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FE3994: 4BFFE83D  bl 0x82fe21d0
	ctx.lr = 0x82FE3998;
	sub_82FE21D0(ctx, base);
	// 82FE3998: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE399C: 41820090  beq 0x82fe3a2c
	if ctx.cr[0].eq {
	pc = 0x82FE3A2C; continue 'dispatch;
	}
	// 82FE39A0: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE39A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE39A8: 41820024  beq 0x82fe39cc
	if ctx.cr[0].eq {
	pc = 0x82FE39CC; continue 'dispatch;
	}
	// 82FE39AC: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE39B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE39B4: 41820018  beq 0x82fe39cc
	if ctx.cr[0].eq {
	pc = 0x82FE39CC; continue 'dispatch;
	}
	// 82FE39B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE39BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE39C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE39C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE39C8: 4E800421  bctrl
	ctx.lr = 0x82FE39CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE39CC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE39D0: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE39D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE39D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE39DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE39E0: 4E800421  bctrl
	ctx.lr = 0x82FE39E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE39E4: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE39E8: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE39EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE39F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE39F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE39F8: 4E800421  bctrl
	ctx.lr = 0x82FE39FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE39FC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82FE3A00: 933D0000  stw r25, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82FE3A04: 939D0008  stw r28, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82FE3A08: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3A0C: 4BFED175  bl 0x82fd0b80
	ctx.lr = 0x82FE3A10;
	sub_82FD0B80(ctx, base);
	// 82FE3A10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82FE3A14: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82FE3A18: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82FE3A1C: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3A20: 4BFED161  bl 0x82fd0b80
	ctx.lr = 0x82FE3A24;
	sub_82FD0B80(ctx, base);
	// 82FE3A24: 907D0010  stw r3, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82FE3A28: 48000058  b 0x82fe3a80
	pc = 0x82FE3A80; continue 'dispatch;
	// 82FE3A2C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82FE3A30: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3A34: 4BFF4865  bl 0x82fd8298
	ctx.lr = 0x82FE3A38;
	sub_82FD8298(ctx, base);
	// 82FE3A38: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82FE3A3C: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE3A40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3A44: 4182002C  beq 0x82fe3a70
	if ctx.cr[0].eq {
	pc = 0x82FE3A70; continue 'dispatch;
	}
	// 82FE3A48: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3A4C: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3A50: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82FE3A54: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3A58: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82FE3A5C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE3A60: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE3A64: 7D0A582E  lwzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE3A68: 4BFFE649  bl 0x82fe20b0
	ctx.lr = 0x82FE3A6C;
	sub_82FE20B0(ctx, base);
	// 82FE3A6C: 48000008  b 0x82fe3a74
	pc = 0x82FE3A74; continue 'dispatch;
	// 82FE3A70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE3A74: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3A78: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3A7C: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 82FE3A80: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE3A84: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3A88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FE3A8C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3A90: 409A007C  bne cr6, 0x82fe3b0c
	if !ctx.cr[6].eq {
	pc = 0x82FE3B0C; continue 'dispatch;
	}
	// 82FE3A94: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82FE3A98: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3A9C: 395F0050  addi r10, r31, 0x50
	ctx.r[10].s64 = ctx.r[31].s64 + 80;
	// 82FE3AA0: F97F0058  std r11, 0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82FE3AA4: C81F0058  lfd f0, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 82FE3AA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3AAC: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82FE3AB0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3AB4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82FE3AB8: C80BA8D0  lfd f0, -0x5730(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-22320 as u32) ) };
	// 82FE3ABC: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82FE3AC0: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 82FE3AC4: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82FE3AC8: 839F0050  lwz r28, 0x50(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE3ACC: 5784103A  slwi r4, r28, 2
	ctx.r[4].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82FE3AD0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82FE3AD4: 4E800421  bctrl
	ctx.lr = 0x82FE3AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3AD8: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3ADC: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3AE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE3AE4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82FE3AE8: 481C4A29  bl 0x831a8510
	ctx.lr = 0x82FE3AEC;
	sub_831A8510(ctx, base);
	// 82FE3AEC: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3AF0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3AF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3AF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3AFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3B00: 4E800421  bctrl
	ctx.lr = 0x82FE3B04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3B04: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82FE3B08: 939E0014  stw r28, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82FE3B0C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82FE3B10: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3B14: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82FE3B18: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE3B1C: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 82FE3B20: 7F2B512E  stwx r25, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[25].u32) };
	// 82FE3B24: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE3B28: 481C4684  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3B2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3B2C size=48
    let mut pc: u32 = 0x82FE3B2C;
    'dispatch: loop {
        match pc {
            0x82FE3B2C => {
    //   block [0x82FE3B2C..0x82FE3B5C)
	// 82FE3B2C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE3B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3B3C: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE3B40: 808B001C  lwz r4, 0x1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE3B44: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82FE3B48: 4BFF4799  bl 0x82fd82e0
	ctx.lr = 0x82FE3B4C;
	sub_82FD82E0(ctx, base);
	// 82FE3B4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE3B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE3B60 size=8
    let mut pc: u32 = 0x82FE3B60;
    'dispatch: loop {
        match pc {
            0x82FE3B60 => {
    //   block [0x82FE3B60..0x82FE3B68)
	// 82FE3B60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE3B64: 8213AFE8  lwz r16, -0x5018(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20504 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3B68 size=164
    let mut pc: u32 = 0x82FE3B68;
    'dispatch: loop {
        match pc {
            0x82FE3B68 => {
    //   block [0x82FE3B68..0x82FE3C0C)
	// 82FE3B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE3B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE3B78: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE3B7C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3B80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE3B84: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82FE3B88: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE3B8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE3B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE3B94: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 82FE3B98: 396BAFD0  addi r11, r11, -0x5030
	ctx.r[11].s64 = ctx.r[11].s64 + -20528;
	// 82FE3B9C: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82FE3BA0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82FE3BA4: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82FE3BA8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82FE3BAC: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82FE3BB0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE3BB4: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82FE3BB8: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82FE3BBC: 409A002C  bne cr6, 0x82fe3be8
	if !ctx.cr[6].eq {
	pc = 0x82FE3BE8; continue 'dispatch;
	}
	// 82FE3BC0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE3BC4: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82FE3BC8: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE3BCC: 38A001B8  li r5, 0x1b8
	ctx.r[5].s64 = 440;
	// 82FE3BD0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE3BD4: 4BFFD31D  bl 0x82fe0ef0
	ctx.lr = 0x82FE3BD8;
	sub_82FE0EF0(ctx, base);
	// 82FE3BD8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3BDC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 82FE3BE0: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 82FE3BE4: 481CD045  bl 0x831b0c28
	ctx.lr = 0x82FE3BE8;
	sub_831B0C28(ctx, base);
	// 82FE3BE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3BEC: 4BFFE7CD  bl 0x82fe23b8
	ctx.lr = 0x82FE3BF0;
	sub_82FE23B8(ctx, base);
	// 82FE3BF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE3BF4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE3BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3C00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE3C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE3C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3C0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3C0C size=40
    let mut pc: u32 = 0x82FE3C0C;
    'dispatch: loop {
        match pc {
            0x82FE3C0C => {
    //   block [0x82FE3C0C..0x82FE3C34)
	// 82FE3C0C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE3C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3C1C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE3C20: 48068B41  bl 0x8304c760
	ctx.lr = 0x82FE3C24;
	sub_8304C760(ctx, base);
	// 82FE3C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE3C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3C38 size=124
    let mut pc: u32 = 0x82FE3C38;
    'dispatch: loop {
        match pc {
            0x82FE3C38 => {
    //   block [0x82FE3C38..0x82FE3CB4)
	// 82FE3C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE3C44: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3C4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3C50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3C58: 4E800421  bctrl
	ctx.lr = 0x82FE3C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3C5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE3C60: 40820030  bne 0x82fe3c90
	if !ctx.cr[0].eq {
	pc = 0x82FE3C90; continue 'dispatch;
	}
	// 82FE3C64: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE3C68: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3C6C: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82FE3C70: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE3C74: 38A001DD  li r5, 0x1dd
	ctx.r[5].s64 = 477;
	// 82FE3C78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3C7C: 4BFFD0AD  bl 0x82fe0d28
	ctx.lr = 0x82FE3C80;
	sub_82FE0D28(ctx, base);
	// 82FE3C80: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3C84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3C88: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 82FE3C8C: 481CCF9D  bl 0x831b0c28
	ctx.lr = 0x82FE3C90;
	sub_831B0C28(ctx, base);
	// 82FE3C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE3C94: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3C98: 4BFFE721  bl 0x82fe23b8
	ctx.lr = 0x82FE3C9C;
	sub_82FE23B8(ctx, base);
	// 82FE3C9C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3CA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE3CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3CAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE3CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3CB8 size=128
    let mut pc: u32 = 0x82FE3CB8;
    'dispatch: loop {
        match pc {
            0x82FE3CB8 => {
    //   block [0x82FE3CB8..0x82FE3D38)
	// 82FE3CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3CBC: 481C44B1  bl 0x831a816c
	ctx.lr = 0x82FE3CC0;
	sub_831A8130(ctx, base);
	// 82FE3CC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3CC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE3CCC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE3CD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3CD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3CD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3CDC: 4E800421  bctrl
	ctx.lr = 0x82FE3CE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3CE0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE3CE4: 40820030  bne 0x82fe3d14
	if !ctx.cr[0].eq {
	pc = 0x82FE3D14; continue 'dispatch;
	}
	// 82FE3CE8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE3CEC: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3CF0: 38C0001E  li r6, 0x1e
	ctx.r[6].s64 = 30;
	// 82FE3CF4: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE3CF8: 38A001ED  li r5, 0x1ed
	ctx.r[5].s64 = 493;
	// 82FE3CFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3D00: 4BFFD029  bl 0x82fe0d28
	ctx.lr = 0x82FE3D04;
	sub_82FE0D28(ctx, base);
	// 82FE3D04: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3D08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3D0C: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 82FE3D10: 481CCF19  bl 0x831b0c28
	ctx.lr = 0x82FE3D14;
	sub_831B0C28(ctx, base);
	// 82FE3D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE3D18: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3D1C: 4BFFE69D  bl 0x82fe23b8
	ctx.lr = 0x82FE3D20;
	sub_82FE23B8(ctx, base);
	// 82FE3D20: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3D24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE3D28: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE3D2C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE3D30: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE3D34: 481C4488  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE3D38 size=8
    let mut pc: u32 = 0x82FE3D38;
    'dispatch: loop {
        match pc {
            0x82FE3D38 => {
    //   block [0x82FE3D38..0x82FE3D40)
	// 82FE3D38: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE3D3C: 8213B038  lwz r16, -0x4fc8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20424 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3D40 size=152
    let mut pc: u32 = 0x82FE3D40;
    'dispatch: loop {
        match pc {
            0x82FE3D40 => {
    //   block [0x82FE3D40..0x82FE3DD8)
	// 82FE3D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3D44: 481C4425  bl 0x831a8168
	ctx.lr = 0x82FE3D48;
	sub_831A8130(ctx, base);
	// 82FE3D48: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE3D4C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3D50: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE3D54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE3D58: 396BB018  addi r11, r11, -0x4fe8
	ctx.r[11].s64 = ctx.r[11].s64 + -20456;
	// 82FE3D5C: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE3D60: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE3D64: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3D68: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3D6C: 41820040  beq 0x82fe3dac
	if ctx.cr[0].eq {
	pc = 0x82FE3DAC; continue 'dispatch;
	}
	// 82FE3D70: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3D74: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE3D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE3D7C: 40990030  ble cr6, 0x82fe3dac
	if !ctx.cr[6].gt {
	pc = 0x82FE3DAC; continue 'dispatch;
	}
	// 82FE3D80: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE3D84: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3D88: 7C6BE82E  lwzx r3, r11, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FE3D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3D90: 41820008  beq 0x82fe3d98
	if ctx.cr[0].eq {
	pc = 0x82FE3D98; continue 'dispatch;
	}
	// 82FE3D94: 4B2DC4D5  bl 0x822c0268
	ctx.lr = 0x82FE3D98;
	sub_822C0268(ctx, base);
	// 82FE3D98: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3D9C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82FE3DA0: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FE3DA4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3DA8: 4198FFDC  blt cr6, 0x82fe3d84
	if ctx.cr[6].lt {
	pc = 0x82FE3D84; continue 'dispatch;
	}
	// 82FE3DAC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3DB0: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3DB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE3DB8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE3DC0: 4E800421  bctrl
	ctx.lr = 0x82FE3DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE3DC4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE3DC8: 396BAB44  addi r11, r11, -0x54bc
	ctx.r[11].s64 = ctx.r[11].s64 + -21692;
	// 82FE3DCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE3DD0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE3DD4: 481C43E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3DD8 size=40
    let mut pc: u32 = 0x82FE3DD8;
    'dispatch: loop {
        match pc {
            0x82FE3DD8 => {
    //   block [0x82FE3DD8..0x82FE3E00)
	// 82FE3DD8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE3DDC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3DE0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3DE8: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE3DEC: 4BFFE2B5  bl 0x82fe20a0
	ctx.lr = 0x82FE3DF0;
	sub_82FE20A0(ctx, base);
	// 82FE3DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE3DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3E00 size=136
    let mut pc: u32 = 0x82FE3E00;
    'dispatch: loop {
        match pc {
            0x82FE3E00 => {
    //   block [0x82FE3E00..0x82FE3E88)
	// 82FE3E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3E04: 481C4369  bl 0x831a816c
	ctx.lr = 0x82FE3E08;
	sub_831A8130(ctx, base);
	// 82FE3E08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3E10: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82FE3E14: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE3E18: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3E1C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3E20: 41980030  blt cr6, 0x82fe3e50
	if ctx.cr[6].lt {
	pc = 0x82FE3E50; continue 'dispatch;
	}
	// 82FE3E24: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE3E28: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3E2C: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FE3E30: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FE3E34: 38A00043  li r5, 0x43
	ctx.r[5].s64 = 67;
	// 82FE3E38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3E3C: 4BFECB1D  bl 0x82fd0958
	ctx.lr = 0x82FE3E40;
	sub_82FD0958(ctx, base);
	// 82FE3E40: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3E44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3E48: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FE3E4C: 481CCDDD  bl 0x831b0c28
	ctx.lr = 0x82FE3E50;
	sub_831B0C28(ctx, base);
	// 82FE3E50: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3E54: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3E58: 4182001C  beq 0x82fe3e74
	if ctx.cr[0].eq {
	pc = 0x82FE3E74; continue 'dispatch;
	}
	// 82FE3E5C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3E60: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3E64: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE3E68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3E6C: 41820008  beq 0x82fe3e74
	if ctx.cr[0].eq {
	pc = 0x82FE3E74; continue 'dispatch;
	}
	// 82FE3E70: 4B2DC3F9  bl 0x822c0268
	ctx.lr = 0x82FE3E74;
	sub_822C0268(ctx, base);
	// 82FE3E74: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3E78: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3E7C: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 82FE3E80: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE3E84: 481C4338  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3E88 size=112
    let mut pc: u32 = 0x82FE3E88;
    'dispatch: loop {
        match pc {
            0x82FE3E88 => {
    //   block [0x82FE3E88..0x82FE3EF8)
	// 82FE3E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3E8C: 481C42DD  bl 0x831a8168
	ctx.lr = 0x82FE3E90;
	sub_831A8130(ctx, base);
	// 82FE3E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3E98: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE3E9C: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 82FE3EA0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3EA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE3EA8: 40990044  ble cr6, 0x82fe3eec
	if !ctx.cr[6].gt {
	pc = 0x82FE3EEC; continue 'dispatch;
	}
	// 82FE3EAC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82FE3EB0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3EB4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3EB8: 41820018  beq 0x82fe3ed0
	if ctx.cr[0].eq {
	pc = 0x82FE3ED0; continue 'dispatch;
	}
	// 82FE3EBC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3EC0: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FE3EC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3EC8: 41820008  beq 0x82fe3ed0
	if ctx.cr[0].eq {
	pc = 0x82FE3ED0; continue 'dispatch;
	}
	// 82FE3ECC: 4B2DC39D  bl 0x822c0268
	ctx.lr = 0x82FE3ED0;
	sub_822C0268(ctx, base);
	// 82FE3ED0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3ED4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FE3ED8: 7F8BF12E  stwx r28, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[28].u32) };
	// 82FE3EDC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FE3EE0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3EE4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3EE8: 4198FFC8  blt cr6, 0x82fe3eb0
	if ctx.cr[6].lt {
	pc = 0x82FE3EB0; continue 'dispatch;
	}
	// 82FE3EEC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82FE3EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE3EF4: 481C42C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE3EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE3EF8 size=272
    let mut pc: u32 = 0x82FE3EF8;
    'dispatch: loop {
        match pc {
            0x82FE3EF8 => {
    //   block [0x82FE3EF8..0x82FE4008)
	// 82FE3EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE3EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE3F00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE3F04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE3F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE3F0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE3F10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE3F14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3F18: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3F1C: 41980030  blt cr6, 0x82fe3f4c
	if ctx.cr[6].lt {
	pc = 0x82FE3F4C; continue 'dispatch;
	}
	// 82FE3F20: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82FE3F24: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE3F28: 38C00074  li r6, 0x74
	ctx.r[6].s64 = 116;
	// 82FE3F2C: 388B6B80  addi r4, r11, 0x6b80
	ctx.r[4].s64 = ctx.r[11].s64 + 27520;
	// 82FE3F30: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82FE3F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3F38: 4BFECA21  bl 0x82fd0958
	ctx.lr = 0x82FE3F3C;
	sub_82FD0958(ctx, base);
	// 82FE3F3C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE3F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE3F44: 388BC49C  addi r4, r11, -0x3b64
	ctx.r[4].s64 = ctx.r[11].s64 + -15204;
	// 82FE3F48: 481CCCE1  bl 0x831b0c28
	ctx.lr = 0x82FE3F4C;
	sub_831B0C28(ctx, base);
	// 82FE3F4C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3F50: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3F54: 4182001C  beq 0x82fe3f70
	if ctx.cr[0].eq {
	pc = 0x82FE3F70; continue 'dispatch;
	}
	// 82FE3F58: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3F5C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3F60: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE3F64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE3F68: 41820008  beq 0x82fe3f70
	if ctx.cr[0].eq {
	pc = 0x82FE3F70; continue 'dispatch;
	}
	// 82FE3F6C: 4B2DC2FD  bl 0x822c0268
	ctx.lr = 0x82FE3F70;
	sub_822C0268(ctx, base);
	// 82FE3F70: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3F74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE3F78: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3F7C: 409A0018  bne cr6, 0x82fe3f94
	if !ctx.cr[6].eq {
	pc = 0x82FE3F94; continue 'dispatch;
	}
	// 82FE3F80: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3F84: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3F88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FE3F8C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82FE3F90: 48000054  b 0x82fe3fe4
	pc = 0x82FE3FE4; continue 'dispatch;
	// 82FE3F94: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FE3F98: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE3F9C: 40980030  bge cr6, 0x82fe3fcc
	if !ctx.cr[6].lt {
	pc = 0x82FE3FCC; continue 'dispatch;
	}
	// 82FE3FA0: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE3FA4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3FA8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82FE3FAC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82FE3FB0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82FE3FB4: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE3FB8: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82FE3FBC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3FC0: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82FE3FC4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82FE3FC8: 4198FFDC  blt cr6, 0x82fe3fa4
	if ctx.cr[6].lt {
	pc = 0x82FE3FA4; continue 'dispatch;
	}
	// 82FE3FCC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3FD0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82FE3FD4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE3FD8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82FE3FDC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82FE3FE0: 912BFFFC  stw r9, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u32 ) };
	// 82FE3FE4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE3FE8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE3FEC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE3FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE3FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE3FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE3FFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE4000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE4004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4008 size=12
    let mut pc: u32 = 0x82FE4008;
    'dispatch: loop {
        match pc {
            0x82FE4008 => {
    //   block [0x82FE4008..0x82FE4014)
	// 82FE4008: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE400C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4010: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4014 size=20
    let mut pc: u32 = 0x82FE4014;
    'dispatch: loop {
        match pc {
            0x82FE4014 => {
    //   block [0x82FE4014..0x82FE4028)
	// 82FE4014: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82FE4018: 89430004  lbz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE401C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4020: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82FE4024: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4028 size=20
    let mut pc: u32 = 0x82FE4028;
    'dispatch: loop {
        match pc {
            0x82FE4028 => {
    //   block [0x82FE4028..0x82FE403C)
	// 82FE4028: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE402C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE4030: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE4034: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4038: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE403C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE403C size=8
    let mut pc: u32 = 0x82FE403C;
    'dispatch: loop {
        match pc {
            0x82FE403C => {
    //   block [0x82FE403C..0x82FE4044)
	// 82FE403C: 4B2DC22C  b 0x822c0268
	sub_822C0268(ctx, base);
	return;
	// 82FE4040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4048 size=120
    let mut pc: u32 = 0x82FE4048;
    'dispatch: loop {
        match pc {
            0x82FE4048 => {
    //   block [0x82FE4048..0x82FE40C0)
	// 82FE4048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE404C: 481C4121  bl 0x831a816c
	ctx.lr = 0x82FE4050;
	sub_831A8130(ctx, base);
	// 82FE4050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE4058: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE405C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4060: 41820040  beq 0x82fe40a0
	if ctx.cr[0].eq {
	pc = 0x82FE40A0; continue 'dispatch;
	}
	// 82FE4064: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4068: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE406C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE4070: 40990030  ble cr6, 0x82fe40a0
	if !ctx.cr[6].gt {
	pc = 0x82FE40A0; continue 'dispatch;
	}
	// 82FE4074: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82FE4078: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE407C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82FE4080: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4084: 41820008  beq 0x82fe408c
	if ctx.cr[0].eq {
	pc = 0x82FE408C; continue 'dispatch;
	}
	// 82FE4088: 4B2DC1E1  bl 0x822c0268
	ctx.lr = 0x82FE408C;
	sub_822C0268(ctx, base);
	// 82FE408C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4090: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82FE4094: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82FE4098: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE409C: 4198FFDC  blt cr6, 0x82fe4078
	if ctx.cr[6].lt {
	pc = 0x82FE4078; continue 'dispatch;
	}
	// 82FE40A0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82FE40A4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE40A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE40AC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE40B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE40B4: 4E800421  bctrl
	ctx.lr = 0x82FE40B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE40B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE40BC: 481C4100  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE40C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE40C0 size=76
    let mut pc: u32 = 0x82FE40C0;
    'dispatch: loop {
        match pc {
            0x82FE40C0 => {
    //   block [0x82FE40C0..0x82FE410C)
	// 82FE40C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE40C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE40C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE40CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE40D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE40D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE40D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE40DC: 4BFFFC65  bl 0x82fe3d40
	ctx.lr = 0x82FE40E0;
	sub_82FE3D40(ctx, base);
	// 82FE40E0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE40E4: 4182000C  beq 0x82fe40f0
	if ctx.cr[0].eq {
	pc = 0x82FE40F0; continue 'dispatch;
	}
	// 82FE40E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE40EC: 4BFF41F5  bl 0x82fd82e0
	ctx.lr = 0x82FE40F0;
	sub_82FD82E0(ctx, base);
	// 82FE40F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE40F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE40F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE40FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4100: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE4104: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE4108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4110 size=144
    let mut pc: u32 = 0x82FE4110;
    'dispatch: loop {
        match pc {
            0x82FE4110 => {
    //   block [0x82FE4110..0x82FE41A0)
	// 82FE4110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4114: 481C4051  bl 0x831a8164
	ctx.lr = 0x82FE4118;
	sub_831A8130(ctx, base);
	// 82FE4118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE411C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4120: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FE4124: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE4128: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE412C: 4099006C  ble cr6, 0x82fe4198
	if !ctx.cr[6].gt {
	pc = 0x82FE4198; continue 'dispatch;
	}
	// 82FE4130: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE4134: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4138: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82FE413C: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4140: 41820038  beq 0x82fe4178
	if ctx.cr[0].eq {
	pc = 0x82FE4178; continue 'dispatch;
	}
	// 82FE4144: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE4148: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE414C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4150: 41820014  beq 0x82fe4164
	if ctx.cr[0].eq {
	pc = 0x82FE4164; continue 'dispatch;
	}
	// 82FE4154: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4158: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE415C: 41820008  beq 0x82fe4164
	if ctx.cr[0].eq {
	pc = 0x82FE4164; continue 'dispatch;
	}
	// 82FE4160: 4BFF4181  bl 0x82fd82e0
	ctx.lr = 0x82FE4164;
	sub_82FD82E0(ctx, base);
	// 82FE4164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE4168: 4BFF4179  bl 0x82fd82e0
	ctx.lr = 0x82FE416C;
	sub_82FD82E0(ctx, base);
	// 82FE416C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82FE4170: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE4174: 409AFFD0  bne cr6, 0x82fe4144
	if !ctx.cr[6].eq {
	pc = 0x82FE4144; continue 'dispatch;
	}
	// 82FE4178: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE417C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE4180: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FE4184: 7D4BE92E  stwx r10, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 82FE4188: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82FE418C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE4190: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE4194: 4198FFA0  blt cr6, 0x82fe4134
	if ctx.cr[6].lt {
	pc = 0x82FE4134; continue 'dispatch;
	}
	// 82FE4198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82FE419C: 481C4018  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE41A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE41A0 size=332
    let mut pc: u32 = 0x82FE41A0;
    'dispatch: loop {
        match pc {
            0x82FE41A0 => {
    //   block [0x82FE41A0..0x82FE42EC)
	// 82FE41A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE41A4: 481C3FBD  bl 0x831a8160
	ctx.lr = 0x82FE41A8;
	sub_831A8130(ctx, base);
	// 82FE41A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE41AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE41B0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 82FE41B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82FE41B8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82FE41BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE41C0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE41C4: 80CBB7E8  lwz r6, -0x4818(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 82FE41C8: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE41CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE41D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE41D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE41D8: 4E800421  bctrl
	ctx.lr = 0x82FE41DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE41DC: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82FE41E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE41E4: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE41E8: 40990030  ble cr6, 0x82fe4218
	if !ctx.cr[6].gt {
	pc = 0x82FE4218; continue 'dispatch;
	}
	// 82FE41EC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE41F0: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE41F4: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 82FE41F8: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE41FC: 38A0017E  li r5, 0x17e
	ctx.r[5].s64 = 382;
	// 82FE4200: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE4204: 4BFECE55  bl 0x82fd1058
	ctx.lr = 0x82FE4208;
	sub_82FD1058(ctx, base);
	// 82FE4208: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE420C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE4210: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 82FE4214: 481CCA15  bl 0x831b0c28
	ctx.lr = 0x82FE4218;
	sub_831B0C28(ctx, base);
	// 82FE4218: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE421C: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE4220: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82FE4224: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82FE4228: 4800003C  b 0x82fe4264
	pc = 0x82FE4264; continue 'dispatch;
	// 82FE422C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE4230: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4234: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4238: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE423C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE4240: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE4244: 4E800421  bctrl
	ctx.lr = 0x82FE4248;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4248: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE424C: 41820010  beq 0x82fe425c
	if ctx.cr[0].eq {
	pc = 0x82FE425C; continue 'dispatch;
	}
	// 82FE4250: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE4254: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82FE4258: 419A0040  beq cr6, 0x82fe4298
	if ctx.cr[6].eq {
	pc = 0x82FE4298; continue 'dispatch;
	}
	// 82FE425C: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 82FE4260: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE4264: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4268: 4082FFC4  bne 0x82fe422c
	if !ctx.cr[0].eq {
	pc = 0x82FE422C; continue 'dispatch;
	}
	// 82FE426C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE4270: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4274: 38C00032  li r6, 0x32
	ctx.r[6].s64 = 50;
	// 82FE4278: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 82FE427C: 38A001A6  li r5, 0x1a6
	ctx.r[5].s64 = 422;
	// 82FE4280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE4284: 4BFFCAA5  bl 0x82fe0d28
	ctx.lr = 0x82FE4288;
	sub_82FE0D28(ctx, base);
	// 82FE4288: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE428C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82FE4290: 388BC690  addi r4, r11, -0x3970
	ctx.r[4].s64 = ctx.r[11].s64 + -14704;
	// 82FE4294: 481CC995  bl 0x831b0c28
	ctx.lr = 0x82FE4298;
	sub_831B0C28(ctx, base);
	// 82FE4298: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82FE429C: 409A001C  bne cr6, 0x82fe42b8
	if !ctx.cr[6].eq {
	pc = 0x82FE42B8; continue 'dispatch;
	}
	// 82FE42A0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE42A4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE42A8: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE42AC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82FE42B0: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82FE42B4: 4800000C  b 0x82fe42c0
	pc = 0x82FE42C0; continue 'dispatch;
	// 82FE42B8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE42BC: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE42C0: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE42C4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE42C8: 41820014  beq 0x82fe42dc
	if ctx.cr[0].eq {
	pc = 0x82FE42DC; continue 'dispatch;
	}
	// 82FE42CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE42D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE42D4: 41820008  beq 0x82fe42dc
	if ctx.cr[0].eq {
	pc = 0x82FE42DC; continue 'dispatch;
	}
	// 82FE42D8: 4BFF4009  bl 0x82fd82e0
	ctx.lr = 0x82FE42DC;
	sub_82FD82E0(ctx, base);
	// 82FE42DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE42E0: 4BFF4001  bl 0x82fd82e0
	ctx.lr = 0x82FE42E4;
	sub_82FD82E0(ctx, base);
	// 82FE42E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82FE42E8: 481C3EC8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE42F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE42F0 size=8
    let mut pc: u32 = 0x82FE42F0;
    'dispatch: loop {
        match pc {
            0x82FE42F0 => {
    //   block [0x82FE42F0..0x82FE42F8)
	// 82FE42F0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE42F4: 8213B078  lwz r16, -0x4f88(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE42F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE42F8 size=256
    let mut pc: u32 = 0x82FE42F8;
    'dispatch: loop {
        match pc {
            0x82FE42F8 => {
    //   block [0x82FE42F8..0x82FE43F8)
	// 82FE42F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE42FC: 481C3E61  bl 0x831a815c
	ctx.lr = 0x82FE4300;
	sub_831A8130(ctx, base);
	// 82FE4300: 3BE1FF40  addi r31, r1, -0xc0
	ctx.r[31].s64 = ctx.r[1].s64 + -192;
	// 82FE4304: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4308: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE430C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE4310: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82FE4314: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82FE4318: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82FE431C: 93BF00D4  stw r29, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 82FE4320: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82FE4324: 409A0028  bne cr6, 0x82fe434c
	if !ctx.cr[6].eq {
	pc = 0x82FE434C; continue 'dispatch;
	}
	// 82FE4328: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE432C: 80DD0088  lwz r6, 0x88(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE4330: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82FE4334: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4338: 48015B99  bl 0x82ff9ed0
	ctx.lr = 0x82FE433C;
	sub_82FF9ED0(ctx, base);
	// 82FE433C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE4340: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4344: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE4348: 481CC8E1  bl 0x831b0c28
	ctx.lr = 0x82FE434C;
	sub_831B0C28(ctx, base);
	// 82FE434C: 3BDDFFF8  addi r30, r29, -8
	ctx.r[30].s64 = ctx.r[29].s64 + -8;
	// 82FE4350: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FE4354: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4358: 4BFFD699  bl 0x82fe19f0
	ctx.lr = 0x82FE435C;
	sub_82FE19F0(ctx, base);
	// 82FE435C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4364: 41820024  beq 0x82fe4388
	if ctx.cr[0].eq {
	pc = 0x82FE4388; continue 'dispatch;
	}
	// 82FE4368: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82FE436C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82FE4370: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FE4374: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE4378: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE437C: 4801B29D  bl 0x82fff618
	ctx.lr = 0x82FE4380;
	sub_82FFF618(ctx, base);
	// 82FE4380: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE4384: 48000008  b 0x82fe438c
	pc = 0x82FE438C; continue 'dispatch;
	// 82FE4388: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE438C: 817D0084  lwz r11, 0x84(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE4390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE4394: 409A004C  bne cr6, 0x82fe43e0
	if !ctx.cr[6].eq {
	pc = 0x82FE43E0; continue 'dispatch;
	}
	// 82FE4398: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE439C: 809D0088  lwz r4, 0x88(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE43A0: 4BFF3EF9  bl 0x82fd8298
	ctx.lr = 0x82FE43A4;
	sub_82FD8298(ctx, base);
	// 82FE43A4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE43A8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FE43AC: 4182002C  beq 0x82fe43d8
	if ctx.cr[0].eq {
	pc = 0x82FE43D8; continue 'dispatch;
	}
	// 82FE43B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE43B4: 80DD0088  lwz r6, 0x88(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE43B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE43BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE43C0: 48068431  bl 0x8304c7f0
	ctx.lr = 0x82FE43C4;
	sub_8304C7F0(ctx, base);
	// 82FE43C4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FE43C8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FE43CC: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 82FE43D0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE43D4: 48000008  b 0x82fe43dc
	pc = 0x82FE43DC; continue 'dispatch;
	// 82FE43D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE43DC: 915D0084  stw r10, 0x84(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 82FE43E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE43E4: 807D0084  lwz r3, 0x84(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE43E8: 48056D69  bl 0x8303b150
	ctx.lr = 0x82FE43EC;
	sub_8303B150(ctx, base);
	// 82FE43EC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE43F0: 383F00C0  addi r1, r31, 0xc0
	ctx.r[1].s64 = ctx.r[31].s64 + 192;
	// 82FE43F4: 481C3DB8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE43F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE43F8 size=48
    let mut pc: u32 = 0x82FE43F8;
    'dispatch: loop {
        match pc {
            0x82FE43F8 => {
    //   block [0x82FE43F8..0x82FE4428)
	// 82FE43F8: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE43FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4400: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4404: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4408: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE440C: 388BFFF8  addi r4, r11, -8
	ctx.r[4].s64 = ctx.r[11].s64 + -8;
	// 82FE4410: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4414: 480E36CD  bl 0x830c7ae0
	ctx.lr = 0x82FE4418;
	sub_830C7AE0(ctx, base);
	// 82FE4418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE441C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4428 size=52
    let mut pc: u32 = 0x82FE4428;
    'dispatch: loop {
        match pc {
            0x82FE4428 => {
    //   block [0x82FE4428..0x82FE445C)
	// 82FE4428: 3BECFF40  addi r31, r12, -0xc0
	ctx.r[31].s64 = ctx.r[12].s64 + -192;
	// 82FE442C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4430: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4434: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4438: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82FE443C: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82FE4440: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE4444: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4448: 4BFF3E99  bl 0x82fd82e0
	ctx.lr = 0x82FE444C;
	sub_82FD82E0(ctx, base);
	// 82FE444C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4460 size=8
    let mut pc: u32 = 0x82FE4460;
    'dispatch: loop {
        match pc {
            0x82FE4460 => {
    //   block [0x82FE4460..0x82FE4468)
	// 82FE4460: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4464: 8213B0D8  lwz r16, -0x4f28(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20264 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4468 size=176
    let mut pc: u32 = 0x82FE4468;
    'dispatch: loop {
        match pc {
            0x82FE4468 => {
    //   block [0x82FE4468..0x82FE4518)
	// 82FE4468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE446C: 481C3CFD  bl 0x831a8168
	ctx.lr = 0x82FE4470;
	sub_831A8130(ctx, base);
	// 82FE4470: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE4474: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4478: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE447C: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82FE4480: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE4484: 4BFFD56D  bl 0x82fe19f0
	ctx.lr = 0x82FE4488;
	sub_82FE19F0(ctx, base);
	// 82FE4488: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE448C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4490: 41820018  beq 0x82fe44a8
	if ctx.cr[0].eq {
	pc = 0x82FE44A8; continue 'dispatch;
	}
	// 82FE4494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4498: 80BD0090  lwz r5, 0x90(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE449C: 4801B8CD  bl 0x82fffd68
	ctx.lr = 0x82FE44A0;
	sub_82FFFD68(ctx, base);
	// 82FE44A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82FE44A4: 48000008  b 0x82fe44ac
	pc = 0x82FE44AC; continue 'dispatch;
	// 82FE44A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82FE44AC: 817D0088  lwz r11, 0x88(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE44B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE44B4: 409A004C  bne cr6, 0x82fe4500
	if !ctx.cr[6].eq {
	pc = 0x82FE4500; continue 'dispatch;
	}
	// 82FE44B8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE44BC: 809D0090  lwz r4, 0x90(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE44C0: 4BFF3DD9  bl 0x82fd8298
	ctx.lr = 0x82FE44C4;
	sub_82FD8298(ctx, base);
	// 82FE44C4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE44C8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FE44CC: 4182002C  beq 0x82fe44f8
	if ctx.cr[0].eq {
	pc = 0x82FE44F8; continue 'dispatch;
	}
	// 82FE44D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE44D4: 80DD0090  lwz r6, 0x90(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE44D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE44DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE44E0: 48068311  bl 0x8304c7f0
	ctx.lr = 0x82FE44E4;
	sub_8304C7F0(ctx, base);
	// 82FE44E4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FE44E8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FE44EC: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 82FE44F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE44F4: 48000008  b 0x82fe44fc
	pc = 0x82FE44FC; continue 'dispatch;
	// 82FE44F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE44FC: 915D0088  stw r10, 0x88(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82FE4500: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE4504: 807D0088  lwz r3, 0x88(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82FE4508: 48056C49  bl 0x8303b150
	ctx.lr = 0x82FE450C;
	sub_8303B150(ctx, base);
	// 82FE450C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82FE4510: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE4514: 481C3CA4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4518 size=44
    let mut pc: u32 = 0x82FE4518;
    'dispatch: loop {
        match pc {
            0x82FE4518 => {
    //   block [0x82FE4518..0x82FE4544)
	// 82FE4518: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE451C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4520: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4524: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4528: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE452C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4530: 480E35B1  bl 0x830c7ae0
	ctx.lr = 0x82FE4534;
	sub_830C7AE0(ctx, base);
	// 82FE4534: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE453C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4544(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4544 size=48
    let mut pc: u32 = 0x82FE4544;
    'dispatch: loop {
        match pc {
            0x82FE4544 => {
    //   block [0x82FE4544..0x82FE4574)
	// 82FE4544: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE4548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE454C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4554: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE4558: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE455C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4560: 4BFF3D81  bl 0x82fd82e0
	ctx.lr = 0x82FE4564;
	sub_82FD82E0(ctx, base);
	// 82FE4564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE456C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4578 size=8
    let mut pc: u32 = 0x82FE4578;
    'dispatch: loop {
        match pc {
            0x82FE4578 => {
    //   block [0x82FE4578..0x82FE4580)
	// 82FE4578: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE457C: 8213B138  lwz r16, -0x4ec8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20168 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4580 size=224
    let mut pc: u32 = 0x82FE4580;
    'dispatch: loop {
        match pc {
            0x82FE4580 => {
    //   block [0x82FE4580..0x82FE4660)
	// 82FE4580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4584: 481C3BE5  bl 0x831a8168
	ctx.lr = 0x82FE4588;
	sub_831A8130(ctx, base);
	// 82FE4588: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE458C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4590: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4594: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4598: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE459C: 817E0074  lwz r11, 0x74(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE45A0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE45A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE45A8: 409A0034  bne cr6, 0x82fe45dc
	if !ctx.cr[6].eq {
	pc = 0x82FE45DC; continue 'dispatch;
	}
	// 82FE45AC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FE45B0: 4BFFD441  bl 0x82fe19f0
	ctx.lr = 0x82FE45B4;
	sub_82FE19F0(ctx, base);
	// 82FE45B4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE45B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE45BC: 41820018  beq 0x82fe45d4
	if ctx.cr[0].eq {
	pc = 0x82FE45D4; continue 'dispatch;
	}
	// 82FE45C0: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82FE45C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE45C8: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 82FE45CC: 4BFFF2AD  bl 0x82fe3878
	ctx.lr = 0x82FE45D0;
	sub_82FE3878(ctx, base);
	// 82FE45D0: 48000008  b 0x82fe45d8
	pc = 0x82FE45D8; continue 'dispatch;
	// 82FE45D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE45D8: 907E0074  stw r3, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82FE45DC: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 82FE45E0: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE45E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE45E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE45EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE45F0: 4BFFDBE1  bl 0x82fe21d0
	ctx.lr = 0x82FE45F4;
	sub_82FE21D0(ctx, base);
	// 82FE45F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE45F8: 41820010  beq 0x82fe4608
	if ctx.cr[0].eq {
	pc = 0x82FE4608; continue 'dispatch;
	}
	// 82FE45FC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4600: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4604: 40820054  bne 0x82fe4658
	if !ctx.cr[0].eq {
	pc = 0x82FE4658; continue 'dispatch;
	}
	// 82FE4608: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FE460C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4610: 4BFFD3E1  bl 0x82fe19f0
	ctx.lr = 0x82FE4614;
	sub_82FE19F0(ctx, base);
	// 82FE4614: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4618: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE461C: 41820018  beq 0x82fe4634
	if ctx.cr[0].eq {
	pc = 0x82FE4634; continue 'dispatch;
	}
	// 82FE4620: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE4624: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4628: 4801F969  bl 0x83003f90
	ctx.lr = 0x82FE462C;
	sub_83003F90(ctx, base);
	// 82FE462C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82FE4630: 48000008  b 0x82fe4638
	pc = 0x82FE4638; continue 'dispatch;
	// 82FE4634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE4638: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE463C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE4640: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE4644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4648: 4BFFF321  bl 0x82fe3968
	ctx.lr = 0x82FE464C;
	sub_82FE3968(ctx, base);
	// 82FE464C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE4650: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE4654: 4BFFD965  bl 0x82fe1fb8
	ctx.lr = 0x82FE4658;
	sub_82FE1FB8(ctx, base);
	// 82FE4658: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE465C: 481C3B5C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4660 size=44
    let mut pc: u32 = 0x82FE4660;
    'dispatch: loop {
        match pc {
            0x82FE4660 => {
    //   block [0x82FE4660..0x82FE468C)
	// 82FE4660: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE4664: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4668: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE466C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4670: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE4674: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4678: 480E3469  bl 0x830c7ae0
	ctx.lr = 0x82FE467C;
	sub_830C7AE0(ctx, base);
	// 82FE467C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE468C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE468C size=44
    let mut pc: u32 = 0x82FE468C;
    'dispatch: loop {
        match pc {
            0x82FE468C => {
    //   block [0x82FE468C..0x82FE46B8)
	// 82FE468C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE4690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4698: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE469C: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE46A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE46A4: 480E343D  bl 0x830c7ae0
	ctx.lr = 0x82FE46A8;
	sub_830C7AE0(ctx, base);
	// 82FE46A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE46AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE46B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE46B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE46B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE46B8 size=8
    let mut pc: u32 = 0x82FE46B8;
    'dispatch: loop {
        match pc {
            0x82FE46B8 => {
    //   block [0x82FE46B8..0x82FE46C0)
	// 82FE46B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE46BC: 8213B198  lwz r16, -0x4e68(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-20072 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE46C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE46C0 size=232
    let mut pc: u32 = 0x82FE46C0;
    'dispatch: loop {
        match pc {
            0x82FE46C0 => {
    //   block [0x82FE46C0..0x82FE47A8)
	// 82FE46C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE46C4: 481C3AA1  bl 0x831a8164
	ctx.lr = 0x82FE46C8;
	sub_831A8130(ctx, base);
	// 82FE46C8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FE46CC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE46D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE46D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE46D8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE46DC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FE46E0: 817E0074  lwz r11, 0x74(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE46E4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FE46E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE46EC: 409A0034  bne cr6, 0x82fe4720
	if !ctx.cr[6].eq {
	pc = 0x82FE4720; continue 'dispatch;
	}
	// 82FE46F0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82FE46F4: 4BFFD2FD  bl 0x82fe19f0
	ctx.lr = 0x82FE46F8;
	sub_82FE19F0(ctx, base);
	// 82FE46F8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE46FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4700: 41820018  beq 0x82fe4718
	if ctx.cr[0].eq {
	pc = 0x82FE4718; continue 'dispatch;
	}
	// 82FE4704: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82FE4708: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE470C: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 82FE4710: 4BFFF169  bl 0x82fe3878
	ctx.lr = 0x82FE4714;
	sub_82FE3878(ctx, base);
	// 82FE4714: 48000008  b 0x82fe471c
	pc = 0x82FE471C; continue 'dispatch;
	// 82FE4718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE471C: 907E0074  stw r3, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82FE4720: 38FF0050  addi r7, r31, 0x50
	ctx.r[7].s64 = ctx.r[31].s64 + 80;
	// 82FE4724: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE4728: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE472C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE4730: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4734: 4BFFDA9D  bl 0x82fe21d0
	ctx.lr = 0x82FE4738;
	sub_82FE21D0(ctx, base);
	// 82FE4738: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE473C: 41820010  beq 0x82fe474c
	if ctx.cr[0].eq {
	pc = 0x82FE474C; continue 'dispatch;
	}
	// 82FE4740: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4744: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4748: 40820058  bne 0x82fe47a0
	if !ctx.cr[0].eq {
	pc = 0x82FE47A0; continue 'dispatch;
	}
	// 82FE474C: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FE4750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4754: 4BFFD29D  bl 0x82fe19f0
	ctx.lr = 0x82FE4758;
	sub_82FE19F0(ctx, base);
	// 82FE4758: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE475C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4760: 4182001C  beq 0x82fe477c
	if ctx.cr[0].eq {
	pc = 0x82FE477C; continue 'dispatch;
	}
	// 82FE4764: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82FE4768: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE476C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4770: 4801F8C9  bl 0x83004038
	ctx.lr = 0x82FE4774;
	sub_83004038(ctx, base);
	// 82FE4774: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82FE4778: 48000008  b 0x82fe4780
	pc = 0x82FE4780; continue 'dispatch;
	// 82FE477C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82FE4780: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE4784: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82FE4788: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE478C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE4790: 4BFFF1D9  bl 0x82fe3968
	ctx.lr = 0x82FE4794;
	sub_82FE3968(ctx, base);
	// 82FE4794: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82FE4798: 807E0074  lwz r3, 0x74(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82FE479C: 4BFFD81D  bl 0x82fe1fb8
	ctx.lr = 0x82FE47A0;
	sub_82FE1FB8(ctx, base);
	// 82FE47A0: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FE47A4: 481C3A10  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE47A8 size=44
    let mut pc: u32 = 0x82FE47A8;
    'dispatch: loop {
        match pc {
            0x82FE47A8 => {
    //   block [0x82FE47A8..0x82FE47D4)
	// 82FE47A8: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE47AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE47B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE47B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE47B8: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE47BC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE47C0: 480E3321  bl 0x830c7ae0
	ctx.lr = 0x82FE47C4;
	sub_830C7AE0(ctx, base);
	// 82FE47C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE47C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE47CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE47D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE47D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE47D4 size=44
    let mut pc: u32 = 0x82FE47D4;
    'dispatch: loop {
        match pc {
            0x82FE47D4 => {
    //   block [0x82FE47D4..0x82FE4800)
	// 82FE47D4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE47D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE47DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE47E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE47E4: 809F00A4  lwz r4, 0xa4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE47E8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE47EC: 480E32F5  bl 0x830c7ae0
	ctx.lr = 0x82FE47F0;
	sub_830C7AE0(ctx, base);
	// 82FE47F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE47F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE47F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE47FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4800 size=144
    let mut pc: u32 = 0x82FE4800;
    'dispatch: loop {
        match pc {
            0x82FE4800 => {
    //   block [0x82FE4800..0x82FE4890)
	// 82FE4800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE480C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE4810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE4818: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE481C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE4820: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE4824: 419A0050  beq cr6, 0x82fe4874
	if ctx.cr[6].eq {
	pc = 0x82FE4874; continue 'dispatch;
	}
	// 82FE4828: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 82FE482C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FE4830: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4834: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82FE4838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE483C: 4E800421  bctrl
	ctx.lr = 0x82FE4840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4840: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82FE4844: 41820030  beq 0x82fe4874
	if ctx.cr[0].eq {
	pc = 0x82FE4874; continue 'dispatch;
	}
	// 82FE4848: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FE484C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82FE4850: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4854: 480843AD  bl 0x83068c00
	ctx.lr = 0x82FE4858;
	sub_83068C00(ctx, base);
	// 82FE4858: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE485C: 41820018  beq 0x82fe4874
	if ctx.cr[0].eq {
	pc = 0x82FE4874; continue 'dispatch;
	}
	// 82FE4860: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4864: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4868: 4182000C  beq 0x82fe4874
	if ctx.cr[0].eq {
	pc = 0x82FE4874; continue 'dispatch;
	}
	// 82FE486C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4870: 48000008  b 0x82fe4878
	pc = 0x82FE4878; continue 'dispatch;
	// 82FE4874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE487C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4884: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE4888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE488C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4890 size=32
    let mut pc: u32 = 0x82FE4890;
    'dispatch: loop {
        match pc {
            0x82FE4890 => {
    //   block [0x82FE4890..0x82FE48B0)
	// 82FE4890: 80630070  lwz r3, 0x70(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE4894: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4898: 41820018  beq 0x82fe48b0
	if ctx.cr[0].eq {
		sub_82FE48B0(ctx, base);
		return;
	}
	// 82FE489C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE48A0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE48A4: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE48A8: 40820008  bne 0x82fe48b0
	if !ctx.cr[0].eq {
		sub_82FE48B0(ctx, base);
		return;
	}
	// 82FE48AC: 4800933C  b 0x82fedbe8
	sub_82FEDBE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE48B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE48B0 size=8
    let mut pc: u32 = 0x82FE48B0;
    'dispatch: loop {
        match pc {
            0x82FE48B0 => {
    //   block [0x82FE48B0..0x82FE48B8)
	// 82FE48B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE48B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE48B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE48B8 size=120
    let mut pc: u32 = 0x82FE48B8;
    'dispatch: loop {
        match pc {
            0x82FE48B8 => {
    //   block [0x82FE48B8..0x82FE4930)
	// 82FE48B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE48BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE48C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE48C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE48C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE48CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE48D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82FE48D4: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE48D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE48DC: 41820030  beq 0x82fe490c
	if ctx.cr[0].eq {
	pc = 0x82FE490C; continue 'dispatch;
	}
	// 82FE48E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82FE48E4: 4BFFD675  bl 0x82fe1f58
	ctx.lr = 0x82FE48E8;
	sub_82FE1F58(ctx, base);
	// 82FE48E8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE48EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE48F0: 4182001C  beq 0x82fe490c
	if ctx.cr[0].eq {
	pc = 0x82FE490C; continue 'dispatch;
	}
	// 82FE48F4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE48F8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82FE48FC: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE4900: 4082000C  bne 0x82fe490c
	if !ctx.cr[0].eq {
	pc = 0x82FE490C; continue 'dispatch;
	}
	// 82FE4904: 480092E5  bl 0x82fedbe8
	ctx.lr = 0x82FE4908;
	sub_82FEDBE8(ctx, base);
	// 82FE4908: 48000010  b 0x82fe4918
	pc = 0x82FE4918; continue 'dispatch;
	// 82FE490C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE4914: 4BFFD0DD  bl 0x82fe19f0
	ctx.lr = 0x82FE4918;
	sub_82FE19F0(ctx, base);
	// 82FE4918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82FE491C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4924: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE4928: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE492C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4930 size=100
    let mut pc: u32 = 0x82FE4930;
    'dispatch: loop {
        match pc {
            0x82FE4930 => {
    //   block [0x82FE4930..0x82FE4994)
	// 82FE4930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE493C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE4944: 4BFFF7CD  bl 0x82fe4110
	ctx.lr = 0x82FE4948;
	sub_82FE4110(ctx, base);
	// 82FE4948: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE494C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4950: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4954: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE495C: 4E800421  bctrl
	ctx.lr = 0x82FE4960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4960: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE4964: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4968: 41820018  beq 0x82fe4980
	if ctx.cr[0].eq {
	pc = 0x82FE4980; continue 'dispatch;
	}
	// 82FE496C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4970: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE4974: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4978: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE497C: 4E800421  bctrl
	ctx.lr = 0x82FE4980;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE498C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE4990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4998 size=172
    let mut pc: u32 = 0x82FE4998;
    'dispatch: loop {
        match pc {
            0x82FE4998 => {
    //   block [0x82FE4998..0x82FE4A44)
	// 82FE4998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE499C: 481C37C9  bl 0x831a8164
	ctx.lr = 0x82FE49A0;
	sub_831A8130(ctx, base);
	// 82FE49A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE49A4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FE49A8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82FE49AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82FE49B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE49B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE49B8: 48084249  bl 0x83068c00
	ctx.lr = 0x82FE49BC;
	sub_83068C00(ctx, base);
	// 82FE49BC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE49C0: 41820030  beq 0x82fe49f0
	if ctx.cr[0].eq {
	pc = 0x82FE49F0; continue 'dispatch;
	}
	// 82FE49C4: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE49C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE49CC: 41820014  beq 0x82fe49e0
	if ctx.cr[0].eq {
	pc = 0x82FE49E0; continue 'dispatch;
	}
	// 82FE49D0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE49D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE49D8: 41820008  beq 0x82fe49e0
	if ctx.cr[0].eq {
	pc = 0x82FE49E0; continue 'dispatch;
	}
	// 82FE49DC: 4BFF3905  bl 0x82fd82e0
	ctx.lr = 0x82FE49E0;
	sub_82FD82E0(ctx, base);
	// 82FE49E0: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FE49E4: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FE49E8: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FE49EC: 48000050  b 0x82fe4a3c
	pc = 0x82FE4A3C; continue 'dispatch;
	// 82FE49F0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82FE49F4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE49F8: 4BFF38A1  bl 0x82fd8298
	ctx.lr = 0x82FE49FC;
	sub_82FD8298(ctx, base);
	// 82FE49FC: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4A00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4A04: 41820028  beq 0x82fe4a2c
	if ctx.cr[0].eq {
	pc = 0x82FE4A2C; continue 'dispatch;
	}
	// 82FE4A08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4A0C: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82FE4A10: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82FE4A14: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82FE4A18: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82FE4A1C: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82FE4A20: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82FE4A24: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82FE4A28: 48000008  b 0x82fe4a30
	pc = 0x82FE4A30; continue 'dispatch;
	// 82FE4A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE4A30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4A34: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82FE4A38: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82FE4A3C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE4A40: 481C3774  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4A48 size=212
    let mut pc: u32 = 0x82FE4A48;
    'dispatch: loop {
        match pc {
            0x82FE4A48 => {
    //   block [0x82FE4A48..0x82FE4B1C)
	// 82FE4A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4A4C: 481C3715  bl 0x831a8160
	ctx.lr = 0x82FE4A50;
	sub_831A8130(ctx, base);
	// 82FE4A50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4A54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4A58: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82FE4A5C: 7F5BD378  mr r27, r26
	ctx.r[27].u64 = ctx.r[26].u64;
	// 82FE4A60: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4A64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE4A68: 409900A8  ble cr6, 0x82fe4b10
	if !ctx.cr[6].gt {
	pc = 0x82FE4B10; continue 'dispatch;
	}
	// 82FE4A6C: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82FE4A70: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE4A74: 7FEBE02E  lwzx r31, r11, r28
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82FE4A78: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4A7C: 41820078  beq 0x82fe4af4
	if ctx.cr[0].eq {
	pc = 0x82FE4AF4; continue 'dispatch;
	}
	// 82FE4A80: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4A84: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE4A88: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4A8C: 41820024  beq 0x82fe4ab0
	if ctx.cr[0].eq {
	pc = 0x82FE4AB0; continue 'dispatch;
	}
	// 82FE4A90: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4A94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4A98: 41820018  beq 0x82fe4ab0
	if ctx.cr[0].eq {
	pc = 0x82FE4AB0; continue 'dispatch;
	}
	// 82FE4A9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4AA0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82FE4AA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4AA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE4AAC: 4E800421  bctrl
	ctx.lr = 0x82FE4AB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4AB0: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE4AB4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82FE4AB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4ABC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE4AC4: 4E800421  bctrl
	ctx.lr = 0x82FE4AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4AC8: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82FE4ACC: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82FE4AD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE4AD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4AD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82FE4ADC: 4E800421  bctrl
	ctx.lr = 0x82FE4AE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82FE4AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82FE4AE4: 4BFF37FD  bl 0x82fd82e0
	ctx.lr = 0x82FE4AE8;
	sub_82FD82E0(ctx, base);
	// 82FE4AE8: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82FE4AEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE4AF0: 409AFF90  bne cr6, 0x82fe4a80
	if !ctx.cr[6].eq {
	pc = 0x82FE4A80; continue 'dispatch;
	}
	// 82FE4AF4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82FE4AF8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82FE4AFC: 7F4BE12E  stwx r26, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	// 82FE4B00: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 82FE4B04: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82FE4B08: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82FE4B0C: 4198FF64  blt cr6, 0x82fe4a70
	if ctx.cr[6].lt {
	pc = 0x82FE4A70; continue 'dispatch;
	}
	// 82FE4B10: 935E0018  stw r26, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 82FE4B14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82FE4B18: 481C3698  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4B20 size=8
    let mut pc: u32 = 0x82FE4B20;
    'dispatch: loop {
        match pc {
            0x82FE4B20 => {
    //   block [0x82FE4B20..0x82FE4B28)
	// 82FE4B20: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4B24: 8213B1F0  lwz r16, -0x4e10(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19984 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4B28 size=140
    let mut pc: u32 = 0x82FE4B28;
    'dispatch: loop {
        match pc {
            0x82FE4B28 => {
    //   block [0x82FE4B28..0x82FE4BB4)
	// 82FE4B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4B2C: 481C3641  bl 0x831a816c
	ctx.lr = 0x82FE4B30;
	sub_831A8130(ctx, base);
	// 82FE4B30: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE4B34: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4B38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4B3C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4B40: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE4B44: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE4B48: 419A0048  beq cr6, 0x82fe4b90
	if ctx.cr[6].eq {
	pc = 0x82FE4B90; continue 'dispatch;
	}
	// 82FE4B4C: 4BFFCAF5  bl 0x82fe1640
	ctx.lr = 0x82FE4B50;
	sub_82FE1640(ctx, base);
	// 82FE4B50: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE4B54: 4182003C  beq 0x82fe4b90
	if ctx.cr[0].eq {
	pc = 0x82FE4B90; continue 'dispatch;
	}
	// 82FE4B58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE4B5C: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FE4B60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4B64: 4BFFFD55  bl 0x82fe48b8
	ctx.lr = 0x82FE4B68;
	sub_82FE48B8(ctx, base);
	// 82FE4B68: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4B6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4B70: 41820014  beq 0x82fe4b84
	if ctx.cr[0].eq {
	pc = 0x82FE4B84; continue 'dispatch;
	}
	// 82FE4B74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE4B78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4B7C: 4802018D  bl 0x83004d08
	ctx.lr = 0x82FE4B80;
	sub_83004D08(ctx, base);
	// 82FE4B80: 48000008  b 0x82fe4b88
	pc = 0x82FE4B88; continue 'dispatch;
	// 82FE4B84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4B88: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE4B8C: 481C3630  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE4B90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE4B94: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE4B98: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE4B9C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4BA0: 48015331  bl 0x82ff9ed0
	ctx.lr = 0x82FE4BA4;
	sub_82FF9ED0(ctx, base);
	// 82FE4BA4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE4BA8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4BAC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE4BB0: 481CC079  bl 0x831b0c28
	ctx.lr = 0x82FE4BB4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4BB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4BB4 size=48
    let mut pc: u32 = 0x82FE4BB4;
    'dispatch: loop {
        match pc {
            0x82FE4BB4 => {
    //   block [0x82FE4BB4..0x82FE4BE4)
	// 82FE4BB4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE4BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE4BC8: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE4BCC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4BD0: 480E2F11  bl 0x830c7ae0
	ctx.lr = 0x82FE4BD4;
	sub_830C7AE0(ctx, base);
	// 82FE4BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4BE8 size=8
    let mut pc: u32 = 0x82FE4BE8;
    'dispatch: loop {
        match pc {
            0x82FE4BE8 => {
    //   block [0x82FE4BE8..0x82FE4BF0)
	// 82FE4BE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4BEC: 8213B240  lwz r16, -0x4dc0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4BF0 size=80
    let mut pc: u32 = 0x82FE4BF0;
    'dispatch: loop {
        match pc {
            0x82FE4BF0 => {
    //   block [0x82FE4BF0..0x82FE4C40)
	// 82FE4BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4BF4: 481C3579  bl 0x831a816c
	ctx.lr = 0x82FE4BF8;
	sub_831A8130(ctx, base);
	// 82FE4BF8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE4BFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4C00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4C04: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4C08: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FE4C0C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82FE4C10: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE4C14: 4BFFFCA5  bl 0x82fe48b8
	ctx.lr = 0x82FE4C18;
	sub_82FE48B8(ctx, base);
	// 82FE4C18: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4C1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4C20: 41820014  beq 0x82fe4c34
	if ctx.cr[0].eq {
	pc = 0x82FE4C34; continue 'dispatch;
	}
	// 82FE4C24: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE4C28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4C2C: 48020BAD  bl 0x830057d8
	ctx.lr = 0x82FE4C30;
	sub_830057D8(ctx, base);
	// 82FE4C30: 48000008  b 0x82fe4c38
	pc = 0x82FE4C38; continue 'dispatch;
	// 82FE4C34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4C38: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE4C3C: 481C3580  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4C40 size=48
    let mut pc: u32 = 0x82FE4C40;
    'dispatch: loop {
        match pc {
            0x82FE4C40 => {
    //   block [0x82FE4C40..0x82FE4C70)
	// 82FE4C40: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE4C44: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4C48: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4C4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4C50: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82FE4C54: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE4C58: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4C5C: 480E2E85  bl 0x830c7ae0
	ctx.lr = 0x82FE4C60;
	sub_830C7AE0(ctx, base);
	// 82FE4C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4C70 size=8
    let mut pc: u32 = 0x82FE4C70;
    'dispatch: loop {
        match pc {
            0x82FE4C70 => {
    //   block [0x82FE4C70..0x82FE4C78)
	// 82FE4C70: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4C74: 8213B278  lwz r16, -0x4d88(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19848 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4C78 size=80
    let mut pc: u32 = 0x82FE4C78;
    'dispatch: loop {
        match pc {
            0x82FE4C78 => {
    //   block [0x82FE4C78..0x82FE4CC8)
	// 82FE4C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4C7C: 481C34F1  bl 0x831a816c
	ctx.lr = 0x82FE4C80;
	sub_831A8130(ctx, base);
	// 82FE4C80: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE4C84: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4C88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4C8C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4C90: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82FE4C94: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FE4C98: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE4C9C: 4BFFFC1D  bl 0x82fe48b8
	ctx.lr = 0x82FE4CA0;
	sub_82FE48B8(ctx, base);
	// 82FE4CA0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4CA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4CA8: 41820014  beq 0x82fe4cbc
	if ctx.cr[0].eq {
	pc = 0x82FE4CBC; continue 'dispatch;
	}
	// 82FE4CAC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE4CB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4CB4: 48020EE5  bl 0x83005b98
	ctx.lr = 0x82FE4CB8;
	sub_83005B98(ctx, base);
	// 82FE4CB8: 48000008  b 0x82fe4cc0
	pc = 0x82FE4CC0; continue 'dispatch;
	// 82FE4CBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4CC0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE4CC4: 481C34F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4CC8 size=48
    let mut pc: u32 = 0x82FE4CC8;
    'dispatch: loop {
        match pc {
            0x82FE4CC8 => {
    //   block [0x82FE4CC8..0x82FE4CF8)
	// 82FE4CC8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE4CCC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4CD0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4CD8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82FE4CDC: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE4CE0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4CE4: 480E2DFD  bl 0x830c7ae0
	ctx.lr = 0x82FE4CE8;
	sub_830C7AE0(ctx, base);
	// 82FE4CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4CF8 size=8
    let mut pc: u32 = 0x82FE4CF8;
    'dispatch: loop {
        match pc {
            0x82FE4CF8 => {
    //   block [0x82FE4CF8..0x82FE4D00)
	// 82FE4CF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4CFC: 8213B2B0  lwz r16, -0x4d50(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19792 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4D00 size=96
    let mut pc: u32 = 0x82FE4D00;
    'dispatch: loop {
        match pc {
            0x82FE4D00 => {
    //   block [0x82FE4D00..0x82FE4D60)
	// 82FE4D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4D08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82FE4D0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82FE4D10: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 82FE4D14: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4D18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4D1C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FE4D20: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FE4D24: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 82FE4D28: 4BFFFB91  bl 0x82fe48b8
	ctx.lr = 0x82FE4D2C;
	sub_82FE48B8(ctx, base);
	// 82FE4D2C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4D30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4D34: 41820010  beq 0x82fe4d44
	if ctx.cr[0].eq {
	pc = 0x82FE4D44; continue 'dispatch;
	}
	// 82FE4D38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4D3C: 48021A7D  bl 0x830067b8
	ctx.lr = 0x82FE4D40;
	sub_830067B8(ctx, base);
	// 82FE4D40: 48000008  b 0x82fe4d48
	pc = 0x82FE4D48; continue 'dispatch;
	// 82FE4D44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4D48: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 82FE4D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4D54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82FE4D58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82FE4D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4D60 size=48
    let mut pc: u32 = 0x82FE4D60;
    'dispatch: loop {
        match pc {
            0x82FE4D60 => {
    //   block [0x82FE4D60..0x82FE4D90)
	// 82FE4D60: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 82FE4D64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4D68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4D70: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82FE4D74: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82FE4D78: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4D7C: 480E2D65  bl 0x830c7ae0
	ctx.lr = 0x82FE4D80;
	sub_830C7AE0(ctx, base);
	// 82FE4D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4D90 size=8
    let mut pc: u32 = 0x82FE4D90;
    'dispatch: loop {
        match pc {
            0x82FE4D90 => {
    //   block [0x82FE4D90..0x82FE4D98)
	// 82FE4D90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4D94: 8213B2E8  lwz r16, -0x4d18(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19736 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4D98 size=144
    let mut pc: u32 = 0x82FE4D98;
    'dispatch: loop {
        match pc {
            0x82FE4D98 => {
    //   block [0x82FE4D98..0x82FE4E28)
	// 82FE4D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4D9C: 481C33D1  bl 0x831a816c
	ctx.lr = 0x82FE4DA0;
	sub_831A8130(ctx, base);
	// 82FE4DA0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE4DA4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4DA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4DAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4DB0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE4DB4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE4DB8: 419A004C  beq cr6, 0x82fe4e04
	if ctx.cr[6].eq {
	pc = 0x82FE4E04; continue 'dispatch;
	}
	// 82FE4DBC: 4BFFC885  bl 0x82fe1640
	ctx.lr = 0x82FE4DC0;
	sub_82FE1640(ctx, base);
	// 82FE4DC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE4DC4: 41820040  beq 0x82fe4e04
	if ctx.cr[0].eq {
	pc = 0x82FE4E04; continue 'dispatch;
	}
	// 82FE4DC8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FE4DCC: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82FE4DD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4DD4: 4BFFFAE5  bl 0x82fe48b8
	ctx.lr = 0x82FE4DD8;
	sub_82FE48B8(ctx, base);
	// 82FE4DD8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4DDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4DE0: 41820018  beq 0x82fe4df8
	if ctx.cr[0].eq {
	pc = 0x82FE4DF8; continue 'dispatch;
	}
	// 82FE4DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE4DE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE4DEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4DF0: 4800BED1  bl 0x82ff0cc0
	ctx.lr = 0x82FE4DF4;
	sub_82FF0CC0(ctx, base);
	// 82FE4DF4: 48000008  b 0x82fe4dfc
	pc = 0x82FE4DFC; continue 'dispatch;
	// 82FE4DF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4DFC: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE4E00: 481C33BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE4E04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE4E08: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE4E0C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE4E10: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4E14: 480150BD  bl 0x82ff9ed0
	ctx.lr = 0x82FE4E18;
	sub_82FF9ED0(ctx, base);
	// 82FE4E18: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE4E1C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4E20: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE4E24: 481CBE05  bl 0x831b0c28
	ctx.lr = 0x82FE4E28;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4E28 size=48
    let mut pc: u32 = 0x82FE4E28;
    'dispatch: loop {
        match pc {
            0x82FE4E28 => {
    //   block [0x82FE4E28..0x82FE4E58)
	// 82FE4E28: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE4E2C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4E30: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4E34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4E38: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FE4E3C: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE4E40: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4E44: 480E2C9D  bl 0x830c7ae0
	ctx.lr = 0x82FE4E48;
	sub_830C7AE0(ctx, base);
	// 82FE4E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4E58 size=8
    let mut pc: u32 = 0x82FE4E58;
    'dispatch: loop {
        match pc {
            0x82FE4E58 => {
    //   block [0x82FE4E58..0x82FE4E60)
	// 82FE4E58: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4E5C: 8213B338  lwz r16, -0x4cc8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19656 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4E60 size=160
    let mut pc: u32 = 0x82FE4E60;
    'dispatch: loop {
        match pc {
            0x82FE4E60 => {
    //   block [0x82FE4E60..0x82FE4F00)
	// 82FE4E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4E64: 481C3301  bl 0x831a8164
	ctx.lr = 0x82FE4E68;
	sub_831A8130(ctx, base);
	// 82FE4E68: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 82FE4E6C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4E70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4E74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4E78: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE4E7C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82FE4E80: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE4E84: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82FE4E88: 419A0054  beq cr6, 0x82fe4edc
	if ctx.cr[6].eq {
	pc = 0x82FE4EDC; continue 'dispatch;
	}
	// 82FE4E8C: 4BFFC7B5  bl 0x82fe1640
	ctx.lr = 0x82FE4E90;
	sub_82FE1640(ctx, base);
	// 82FE4E90: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE4E94: 41820048  beq 0x82fe4edc
	if ctx.cr[0].eq {
	pc = 0x82FE4EDC; continue 'dispatch;
	}
	// 82FE4E98: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FE4E9C: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82FE4EA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4EA4: 4BFFFA15  bl 0x82fe48b8
	ctx.lr = 0x82FE4EA8;
	sub_82FE48B8(ctx, base);
	// 82FE4EA8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4EAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4EB0: 41820020  beq 0x82fe4ed0
	if ctx.cr[0].eq {
	pc = 0x82FE4ED0; continue 'dispatch;
	}
	// 82FE4EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82FE4EB8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82FE4EBC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE4EC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE4EC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4EC8: 4800C1F1  bl 0x82ff10b8
	ctx.lr = 0x82FE4ECC;
	sub_82FF10B8(ctx, base);
	// 82FE4ECC: 48000008  b 0x82fe4ed4
	pc = 0x82FE4ED4; continue 'dispatch;
	// 82FE4ED0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4ED4: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 82FE4ED8: 481C32DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 82FE4EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE4EE0: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE4EE4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE4EE8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4EEC: 48014FE5  bl 0x82ff9ed0
	ctx.lr = 0x82FE4EF0;
	sub_82FF9ED0(ctx, base);
	// 82FE4EF0: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE4EF4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4EF8: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE4EFC: 481CBD2D  bl 0x831b0c28
	ctx.lr = 0x82FE4F00;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4F00 size=48
    let mut pc: u32 = 0x82FE4F00;
    'dispatch: loop {
        match pc {
            0x82FE4F00 => {
    //   block [0x82FE4F00..0x82FE4F30)
	// 82FE4F00: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 82FE4F04: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4F08: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4F10: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82FE4F14: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82FE4F18: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4F1C: 480E2BC5  bl 0x830c7ae0
	ctx.lr = 0x82FE4F20;
	sub_830C7AE0(ctx, base);
	// 82FE4F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4F30 size=8
    let mut pc: u32 = 0x82FE4F30;
    'dispatch: loop {
        match pc {
            0x82FE4F30 => {
    //   block [0x82FE4F30..0x82FE4F38)
	// 82FE4F30: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4F34: 8213B388  lwz r16, -0x4c78(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19576 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4F38 size=140
    let mut pc: u32 = 0x82FE4F38;
    'dispatch: loop {
        match pc {
            0x82FE4F38 => {
    //   block [0x82FE4F38..0x82FE4FC4)
	// 82FE4F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4F3C: 481C3231  bl 0x831a816c
	ctx.lr = 0x82FE4F40;
	sub_831A8130(ctx, base);
	// 82FE4F40: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE4F44: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4F48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE4F4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE4F50: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE4F54: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE4F58: 419A0048  beq cr6, 0x82fe4fa0
	if ctx.cr[6].eq {
	pc = 0x82FE4FA0; continue 'dispatch;
	}
	// 82FE4F5C: 4BFFC6E5  bl 0x82fe1640
	ctx.lr = 0x82FE4F60;
	sub_82FE1640(ctx, base);
	// 82FE4F60: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE4F64: 4182003C  beq 0x82fe4fa0
	if ctx.cr[0].eq {
	pc = 0x82FE4FA0; continue 'dispatch;
	}
	// 82FE4F68: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82FE4F6C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82FE4F70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE4F74: 4BFFF945  bl 0x82fe48b8
	ctx.lr = 0x82FE4F78;
	sub_82FE48B8(ctx, base);
	// 82FE4F78: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE4F7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE4F80: 41820014  beq 0x82fe4f94
	if ctx.cr[0].eq {
	pc = 0x82FE4F94; continue 'dispatch;
	}
	// 82FE4F84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE4F88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE4F8C: 4802354D  bl 0x830084d8
	ctx.lr = 0x82FE4F90;
	sub_830084D8(ctx, base);
	// 82FE4F90: 48000008  b 0x82fe4f98
	pc = 0x82FE4F98; continue 'dispatch;
	// 82FE4F94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE4F98: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE4F9C: 481C3220  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE4FA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE4FA4: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE4FA8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE4FAC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4FB0: 48014F21  bl 0x82ff9ed0
	ctx.lr = 0x82FE4FB4;
	sub_82FF9ED0(ctx, base);
	// 82FE4FB4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE4FB8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE4FBC: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE4FC0: 481CBC69  bl 0x831b0c28
	ctx.lr = 0x82FE4FC4;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE4FC4 size=48
    let mut pc: u32 = 0x82FE4FC4;
    'dispatch: loop {
        match pc {
            0x82FE4FC4 => {
    //   block [0x82FE4FC4..0x82FE4FF4)
	// 82FE4FC4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE4FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE4FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE4FD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE4FD4: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82FE4FD8: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE4FDC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE4FE0: 480E2B01  bl 0x830c7ae0
	ctx.lr = 0x82FE4FE4;
	sub_830C7AE0(ctx, base);
	// 82FE4FE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE4FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE4FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE4FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE4FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE4FF8 size=8
    let mut pc: u32 = 0x82FE4FF8;
    'dispatch: loop {
        match pc {
            0x82FE4FF8 => {
    //   block [0x82FE4FF8..0x82FE5000)
	// 82FE4FF8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE4FFC: 8213B3D8  lwz r16, -0x4c28(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19496 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5000 size=80
    let mut pc: u32 = 0x82FE5000;
    'dispatch: loop {
        match pc {
            0x82FE5000 => {
    //   block [0x82FE5000..0x82FE5050)
	// 82FE5000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5004: 481C3169  bl 0x831a816c
	ctx.lr = 0x82FE5008;
	sub_831A8130(ctx, base);
	// 82FE5008: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE500C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5010: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5014: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE5018: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82FE501C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82FE5020: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE5024: 4BFFF895  bl 0x82fe48b8
	ctx.lr = 0x82FE5028;
	sub_82FE48B8(ctx, base);
	// 82FE5028: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE502C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5030: 41820014  beq 0x82fe5044
	if ctx.cr[0].eq {
	pc = 0x82FE5044; continue 'dispatch;
	}
	// 82FE5034: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE5038: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE503C: 4802349D  bl 0x830084d8
	ctx.lr = 0x82FE5040;
	sub_830084D8(ctx, base);
	// 82FE5040: 48000008  b 0x82fe5048
	pc = 0x82FE5048; continue 'dispatch;
	// 82FE5044: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE5048: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE504C: 481C3170  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5050 size=48
    let mut pc: u32 = 0x82FE5050;
    'dispatch: loop {
        match pc {
            0x82FE5050 => {
    //   block [0x82FE5050..0x82FE5080)
	// 82FE5050: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE5054: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5058: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE505C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5060: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82FE5064: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE5068: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE506C: 480E2A75  bl 0x830c7ae0
	ctx.lr = 0x82FE5070;
	sub_830C7AE0(ctx, base);
	// 82FE5070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE507C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5080 size=8
    let mut pc: u32 = 0x82FE5080;
    'dispatch: loop {
        match pc {
            0x82FE5080 => {
    //   block [0x82FE5080..0x82FE5088)
	// 82FE5080: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5084: 8213B410  lwz r16, -0x4bf0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19440 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5088 size=140
    let mut pc: u32 = 0x82FE5088;
    'dispatch: loop {
        match pc {
            0x82FE5088 => {
    //   block [0x82FE5088..0x82FE5114)
	// 82FE5088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE508C: 481C30E1  bl 0x831a816c
	ctx.lr = 0x82FE5090;
	sub_831A8130(ctx, base);
	// 82FE5090: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE5094: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5098: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE509C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE50A0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE50A4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE50A8: 419A0048  beq cr6, 0x82fe50f0
	if ctx.cr[6].eq {
	pc = 0x82FE50F0; continue 'dispatch;
	}
	// 82FE50AC: 4BFFC595  bl 0x82fe1640
	ctx.lr = 0x82FE50B0;
	sub_82FE1640(ctx, base);
	// 82FE50B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE50B4: 4182003C  beq 0x82fe50f0
	if ctx.cr[0].eq {
	pc = 0x82FE50F0; continue 'dispatch;
	}
	// 82FE50B8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82FE50BC: 38800044  li r4, 0x44
	ctx.r[4].s64 = 68;
	// 82FE50C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE50C4: 4BFFF7F5  bl 0x82fe48b8
	ctx.lr = 0x82FE50C8;
	sub_82FE48B8(ctx, base);
	// 82FE50C8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE50CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE50D0: 41820014  beq 0x82fe50e4
	if ctx.cr[0].eq {
	pc = 0x82FE50E4; continue 'dispatch;
	}
	// 82FE50D4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE50D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE50DC: 4802445D  bl 0x83009538
	ctx.lr = 0x82FE50E0;
	sub_83009538(ctx, base);
	// 82FE50E0: 48000008  b 0x82fe50e8
	pc = 0x82FE50E8; continue 'dispatch;
	// 82FE50E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE50E8: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE50EC: 481C30D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE50F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE50F4: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE50F8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE50FC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5100: 48014DD1  bl 0x82ff9ed0
	ctx.lr = 0x82FE5104;
	sub_82FF9ED0(ctx, base);
	// 82FE5104: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE5108: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE510C: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE5110: 481CBB19  bl 0x831b0c28
	ctx.lr = 0x82FE5114;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5114 size=48
    let mut pc: u32 = 0x82FE5114;
    'dispatch: loop {
        match pc {
            0x82FE5114 => {
    //   block [0x82FE5114..0x82FE5144)
	// 82FE5114: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE5118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE511C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5124: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82FE5128: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE512C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE5130: 480E29B1  bl 0x830c7ae0
	ctx.lr = 0x82FE5134;
	sub_830C7AE0(ctx, base);
	// 82FE5134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE513C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5148 size=8
    let mut pc: u32 = 0x82FE5148;
    'dispatch: loop {
        match pc {
            0x82FE5148 => {
    //   block [0x82FE5148..0x82FE5150)
	// 82FE5148: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE514C: 8213B460  lwz r16, -0x4ba0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19360 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5150 size=140
    let mut pc: u32 = 0x82FE5150;
    'dispatch: loop {
        match pc {
            0x82FE5150 => {
    //   block [0x82FE5150..0x82FE51DC)
	// 82FE5150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5154: 481C3019  bl 0x831a816c
	ctx.lr = 0x82FE5158;
	sub_831A8130(ctx, base);
	// 82FE5158: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE515C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5160: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5164: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE5168: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE516C: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE5170: 419A0048  beq cr6, 0x82fe51b8
	if ctx.cr[6].eq {
	pc = 0x82FE51B8; continue 'dispatch;
	}
	// 82FE5174: 4BFFC4CD  bl 0x82fe1640
	ctx.lr = 0x82FE5178;
	sub_82FE1640(ctx, base);
	// 82FE5178: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE517C: 4182003C  beq 0x82fe51b8
	if ctx.cr[0].eq {
	pc = 0x82FE51B8; continue 'dispatch;
	}
	// 82FE5180: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82FE5184: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82FE5188: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE518C: 4BFFF72D  bl 0x82fe48b8
	ctx.lr = 0x82FE5190;
	sub_82FE48B8(ctx, base);
	// 82FE5190: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE5194: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5198: 41820014  beq 0x82fe51ac
	if ctx.cr[0].eq {
	pc = 0x82FE51AC; continue 'dispatch;
	}
	// 82FE519C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE51A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE51A4: 48024A75  bl 0x83009c18
	ctx.lr = 0x82FE51A8;
	sub_83009C18(ctx, base);
	// 82FE51A8: 48000008  b 0x82fe51b0
	pc = 0x82FE51B0; continue 'dispatch;
	// 82FE51AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE51B0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE51B4: 481C3008  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE51B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE51BC: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE51C0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE51C4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE51C8: 48014D09  bl 0x82ff9ed0
	ctx.lr = 0x82FE51CC;
	sub_82FF9ED0(ctx, base);
	// 82FE51CC: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE51D0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE51D4: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE51D8: 481CBA51  bl 0x831b0c28
	ctx.lr = 0x82FE51DC;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE51DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE51DC size=48
    let mut pc: u32 = 0x82FE51DC;
    'dispatch: loop {
        match pc {
            0x82FE51DC => {
    //   block [0x82FE51DC..0x82FE520C)
	// 82FE51DC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE51E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE51E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE51E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE51EC: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82FE51F0: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE51F4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE51F8: 480E28E9  bl 0x830c7ae0
	ctx.lr = 0x82FE51FC;
	sub_830C7AE0(ctx, base);
	// 82FE51FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5210 size=8
    let mut pc: u32 = 0x82FE5210;
    'dispatch: loop {
        match pc {
            0x82FE5210 => {
    //   block [0x82FE5210..0x82FE5218)
	// 82FE5210: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5214: 8213B4B0  lwz r16, -0x4b50(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19280 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5218 size=144
    let mut pc: u32 = 0x82FE5218;
    'dispatch: loop {
        match pc {
            0x82FE5218 => {
    //   block [0x82FE5218..0x82FE52A8)
	// 82FE5218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE521C: 481C2F51  bl 0x831a816c
	ctx.lr = 0x82FE5220;
	sub_831A8130(ctx, base);
	// 82FE5220: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE5224: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5228: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE522C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE5230: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE5234: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE5238: 419A004C  beq cr6, 0x82fe5284
	if ctx.cr[6].eq {
	pc = 0x82FE5284; continue 'dispatch;
	}
	// 82FE523C: 4BFFC405  bl 0x82fe1640
	ctx.lr = 0x82FE5240;
	sub_82FE1640(ctx, base);
	// 82FE5240: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE5244: 41820040  beq 0x82fe5284
	if ctx.cr[0].eq {
	pc = 0x82FE5284; continue 'dispatch;
	}
	// 82FE5248: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82FE524C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82FE5250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5254: 4BFFF665  bl 0x82fe48b8
	ctx.lr = 0x82FE5258;
	sub_82FE48B8(ctx, base);
	// 82FE5258: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE525C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5260: 41820018  beq 0x82fe5278
	if ctx.cr[0].eq {
	pc = 0x82FE5278; continue 'dispatch;
	}
	// 82FE5264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82FE5268: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE526C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE5270: 48024C11  bl 0x83009e80
	ctx.lr = 0x82FE5274;
	sub_83009E80(ctx, base);
	// 82FE5274: 48000008  b 0x82fe527c
	pc = 0x82FE527C; continue 'dispatch;
	// 82FE5278: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE527C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE5280: 481C2F3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE5284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5288: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE528C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE5290: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5294: 48014C3D  bl 0x82ff9ed0
	ctx.lr = 0x82FE5298;
	sub_82FF9ED0(ctx, base);
	// 82FE5298: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE529C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE52A0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE52A4: 481CB985  bl 0x831b0c28
	ctx.lr = 0x82FE52A8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE52A8 size=48
    let mut pc: u32 = 0x82FE52A8;
    'dispatch: loop {
        match pc {
            0x82FE52A8 => {
    //   block [0x82FE52A8..0x82FE52D8)
	// 82FE52A8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE52AC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE52B0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE52B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE52B8: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82FE52BC: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE52C0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE52C4: 480E281D  bl 0x830c7ae0
	ctx.lr = 0x82FE52C8;
	sub_830C7AE0(ctx, base);
	// 82FE52C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE52CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE52D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE52D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE52D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE52D8 size=8
    let mut pc: u32 = 0x82FE52D8;
    'dispatch: loop {
        match pc {
            0x82FE52D8 => {
    //   block [0x82FE52D8..0x82FE52E0)
	// 82FE52D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE52DC: 8213B500  lwz r16, -0x4b00(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE52E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE52E0 size=140
    let mut pc: u32 = 0x82FE52E0;
    'dispatch: loop {
        match pc {
            0x82FE52E0 => {
    //   block [0x82FE52E0..0x82FE536C)
	// 82FE52E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE52E4: 481C2E89  bl 0x831a816c
	ctx.lr = 0x82FE52E8;
	sub_831A8130(ctx, base);
	// 82FE52E8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE52EC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE52F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE52F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE52F8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE52FC: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE5300: 419A0048  beq cr6, 0x82fe5348
	if ctx.cr[6].eq {
	pc = 0x82FE5348; continue 'dispatch;
	}
	// 82FE5304: 4BFFC33D  bl 0x82fe1640
	ctx.lr = 0x82FE5308;
	sub_82FE1640(ctx, base);
	// 82FE5308: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE530C: 4182003C  beq 0x82fe5348
	if ctx.cr[0].eq {
	pc = 0x82FE5348; continue 'dispatch;
	}
	// 82FE5310: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82FE5314: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FE5318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE531C: 4BFFF59D  bl 0x82fe48b8
	ctx.lr = 0x82FE5320;
	sub_82FE48B8(ctx, base);
	// 82FE5320: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE5324: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5328: 41820014  beq 0x82fe533c
	if ctx.cr[0].eq {
	pc = 0x82FE533C; continue 'dispatch;
	}
	// 82FE532C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE5330: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE5334: 48024D6D  bl 0x8300a0a0
	ctx.lr = 0x82FE5338;
	sub_8300A0A0(ctx, base);
	// 82FE5338: 48000008  b 0x82fe5340
	pc = 0x82FE5340; continue 'dispatch;
	// 82FE533C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE5340: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE5344: 481C2E78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82FE5348: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE534C: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5350: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE5354: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5358: 48014B79  bl 0x82ff9ed0
	ctx.lr = 0x82FE535C;
	sub_82FF9ED0(ctx, base);
	// 82FE535C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE5360: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5364: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE5368: 481CB8C1  bl 0x831b0c28
	ctx.lr = 0x82FE536C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE536C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE536C size=48
    let mut pc: u32 = 0x82FE536C;
    'dispatch: loop {
        match pc {
            0x82FE536C => {
    //   block [0x82FE536C..0x82FE539C)
	// 82FE536C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE5370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE537C: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 82FE5380: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE5384: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE5388: 480E2759  bl 0x830c7ae0
	ctx.lr = 0x82FE538C;
	sub_830C7AE0(ctx, base);
	// 82FE538C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE53A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE53A0 size=8
    let mut pc: u32 = 0x82FE53A0;
    'dispatch: loop {
        match pc {
            0x82FE53A0 => {
    //   block [0x82FE53A0..0x82FE53A8)
	// 82FE53A0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE53A4: 8213B550  lwz r16, -0x4ab0(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19120 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE53A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE53A8 size=148
    let mut pc: u32 = 0x82FE53A8;
    'dispatch: loop {
        match pc {
            0x82FE53A8 => {
    //   block [0x82FE53A8..0x82FE543C)
	// 82FE53A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE53AC: 481C2DBD  bl 0x831a8168
	ctx.lr = 0x82FE53B0;
	sub_831A8130(ctx, base);
	// 82FE53B0: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE53B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE53B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE53BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE53C0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE53C4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE53C8: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE53CC: 419A004C  beq cr6, 0x82fe5418
	if ctx.cr[6].eq {
	pc = 0x82FE5418; continue 'dispatch;
	}
	// 82FE53D0: 4BFFC271  bl 0x82fe1640
	ctx.lr = 0x82FE53D4;
	sub_82FE1640(ctx, base);
	// 82FE53D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE53D8: 41820040  beq 0x82fe5418
	if ctx.cr[0].eq {
	pc = 0x82FE5418; continue 'dispatch;
	}
	// 82FE53DC: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82FE53E0: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82FE53E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE53E8: 4BFFF4D1  bl 0x82fe48b8
	ctx.lr = 0x82FE53EC;
	sub_82FE48B8(ctx, base);
	// 82FE53EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE53F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE53F4: 41820018  beq 0x82fe540c
	if ctx.cr[0].eq {
	pc = 0x82FE540C; continue 'dispatch;
	}
	// 82FE53F8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82FE53FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE5400: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE5404: 48025365  bl 0x8300a768
	ctx.lr = 0x82FE5408;
	sub_8300A768(ctx, base);
	// 82FE5408: 48000008  b 0x82fe5410
	pc = 0x82FE5410; continue 'dispatch;
	// 82FE540C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE5410: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE5414: 481C2DA4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FE5418: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE541C: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5420: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE5424: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5428: 48014AA9  bl 0x82ff9ed0
	ctx.lr = 0x82FE542C;
	sub_82FF9ED0(ctx, base);
	// 82FE542C: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE5430: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5434: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE5438: 481CB7F1  bl 0x831b0c28
	ctx.lr = 0x82FE543C;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE543C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE543C size=48
    let mut pc: u32 = 0x82FE543C;
    'dispatch: loop {
        match pc {
            0x82FE543C => {
    //   block [0x82FE543C..0x82FE546C)
	// 82FE543C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE5440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE544C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82FE5450: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE5454: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE5458: 480E2689  bl 0x830c7ae0
	ctx.lr = 0x82FE545C;
	sub_830C7AE0(ctx, base);
	// 82FE545C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE5464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5470 size=8
    let mut pc: u32 = 0x82FE5470;
    'dispatch: loop {
        match pc {
            0x82FE5470 => {
    //   block [0x82FE5470..0x82FE5478)
	// 82FE5470: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE5474: 8213B5A0  lwz r16, -0x4a60(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-19040 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5478 size=80
    let mut pc: u32 = 0x82FE5478;
    'dispatch: loop {
        match pc {
            0x82FE5478 => {
    //   block [0x82FE5478..0x82FE54C8)
	// 82FE5478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE547C: 481C2CF1  bl 0x831a816c
	ctx.lr = 0x82FE5480;
	sub_831A8130(ctx, base);
	// 82FE5480: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE5484: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5488: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE548C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82FE5490: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82FE5494: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82FE5498: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82FE549C: 4BFFF41D  bl 0x82fe48b8
	ctx.lr = 0x82FE54A0;
	sub_82FE48B8(ctx, base);
	// 82FE54A0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE54A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE54A8: 41820014  beq 0x82fe54bc
	if ctx.cr[0].eq {
	pc = 0x82FE54BC; continue 'dispatch;
	}
	// 82FE54AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82FE54B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE54B4: 48025B25  bl 0x8300afd8
	ctx.lr = 0x82FE54B8;
	sub_8300AFD8(ctx, base);
	// 82FE54B8: 48000008  b 0x82fe54c0
	pc = 0x82FE54C0; continue 'dispatch;
	// 82FE54BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE54C0: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE54C4: 481C2CF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE54C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE54C8 size=48
    let mut pc: u32 = 0x82FE54C8;
    'dispatch: loop {
        match pc {
            0x82FE54C8 => {
    //   block [0x82FE54C8..0x82FE54F8)
	// 82FE54C8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE54CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE54D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE54D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE54D8: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82FE54DC: 809F0094  lwz r4, 0x94(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE54E0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE54E4: 480E25FD  bl 0x830c7ae0
	ctx.lr = 0x82FE54E8;
	sub_830C7AE0(ctx, base);
	// 82FE54E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE54EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE54F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE54F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE54F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE54F8 size=28
    let mut pc: u32 = 0x82FE54F8;
    'dispatch: loop {
        match pc {
            0x82FE54F8 => {
    //   block [0x82FE54F8..0x82FE5514)
	// 82FE54F8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82FE54FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE5500: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82FE5504: 409A0008  bne cr6, 0x82fe550c
	if !ctx.cr[6].eq {
	pc = 0x82FE550C; continue 'dispatch;
	}
	// 82FE5508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE550C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FE5510: 4BFFF070  b 0x82fe4580
	sub_82FE4580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5518 size=8
    let mut pc: u32 = 0x82FE5518;
    'dispatch: loop {
        match pc {
            0x82FE5518 => {
    //   block [0x82FE5518..0x82FE5520)
	// 82FE5518: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE551C: 8213B5D8  lwz r16, -0x4a28(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18984 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5520 size=152
    let mut pc: u32 = 0x82FE5520;
    'dispatch: loop {
        match pc {
            0x82FE5520 => {
    //   block [0x82FE5520..0x82FE55B8)
	// 82FE5520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5524: 481C2C45  bl 0x831a8168
	ctx.lr = 0x82FE5528;
	sub_831A8130(ctx, base);
	// 82FE5528: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE552C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5530: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5534: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE5538: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE553C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE5540: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE5544: 419A0050  beq cr6, 0x82fe5594
	if ctx.cr[6].eq {
	pc = 0x82FE5594; continue 'dispatch;
	}
	// 82FE5548: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE554C: 4BFFC0F5  bl 0x82fe1640
	ctx.lr = 0x82FE5550;
	sub_82FE1640(ctx, base);
	// 82FE5550: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE5554: 41820040  beq 0x82fe5594
	if ctx.cr[0].eq {
	pc = 0x82FE5594; continue 'dispatch;
	}
	// 82FE5558: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82FE555C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82FE5560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5564: 4BFFF355  bl 0x82fe48b8
	ctx.lr = 0x82FE5568;
	sub_82FE48B8(ctx, base);
	// 82FE5568: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE556C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5570: 41820018  beq 0x82fe5588
	if ctx.cr[0].eq {
	pc = 0x82FE5588; continue 'dispatch;
	}
	// 82FE5574: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FE5578: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE557C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE5580: 48026ED9  bl 0x8300c458
	ctx.lr = 0x82FE5584;
	sub_8300C458(ctx, base);
	// 82FE5584: 48000008  b 0x82fe558c
	pc = 0x82FE558C; continue 'dispatch;
	// 82FE5588: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE558C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE5590: 481C2C28  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FE5594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5598: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE559C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE55A0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE55A4: 4801492D  bl 0x82ff9ed0
	ctx.lr = 0x82FE55A8;
	sub_82FF9ED0(ctx, base);
	// 82FE55A8: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE55AC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE55B0: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE55B4: 481CB675  bl 0x831b0c28
	ctx.lr = 0x82FE55B8;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE55B8 size=48
    let mut pc: u32 = 0x82FE55B8;
    'dispatch: loop {
        match pc {
            0x82FE55B8 => {
    //   block [0x82FE55B8..0x82FE55E8)
	// 82FE55B8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE55BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE55C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE55C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE55C8: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 82FE55CC: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE55D0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE55D4: 480E250D  bl 0x830c7ae0
	ctx.lr = 0x82FE55D8;
	sub_830C7AE0(ctx, base);
	// 82FE55D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE55DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE55E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE55E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE55E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE55E8 size=8
    let mut pc: u32 = 0x82FE55E8;
    'dispatch: loop {
        match pc {
            0x82FE55E8 => {
    //   block [0x82FE55E8..0x82FE55F0)
	// 82FE55E8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE55EC: 8213B628  lwz r16, -0x49d8(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18904 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE55F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE55F0 size=152
    let mut pc: u32 = 0x82FE55F0;
    'dispatch: loop {
        match pc {
            0x82FE55F0 => {
    //   block [0x82FE55F0..0x82FE5688)
	// 82FE55F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE55F4: 481C2B75  bl 0x831a8168
	ctx.lr = 0x82FE55F8;
	sub_831A8130(ctx, base);
	// 82FE55F8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 82FE55FC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5600: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE5604: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82FE5608: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE560C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82FE5610: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 82FE5614: 419A0050  beq cr6, 0x82fe5664
	if ctx.cr[6].eq {
	pc = 0x82FE5664; continue 'dispatch;
	}
	// 82FE5618: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82FE561C: 4BFFC025  bl 0x82fe1640
	ctx.lr = 0x82FE5620;
	sub_82FE1640(ctx, base);
	// 82FE5620: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82FE5624: 41820040  beq 0x82fe5664
	if ctx.cr[0].eq {
	pc = 0x82FE5664; continue 'dispatch;
	}
	// 82FE5628: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE562C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82FE5630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5634: 4BFFF285  bl 0x82fe48b8
	ctx.lr = 0x82FE5638;
	sub_82FE48B8(ctx, base);
	// 82FE5638: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE563C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5640: 41820018  beq 0x82fe5658
	if ctx.cr[0].eq {
	pc = 0x82FE5658; continue 'dispatch;
	}
	// 82FE5644: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82FE5648: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82FE564C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82FE5650: 480279A1  bl 0x8300cff0
	ctx.lr = 0x82FE5654;
	sub_8300CFF0(ctx, base);
	// 82FE5654: 48000008  b 0x82fe565c
	pc = 0x82FE565C; continue 'dispatch;
	// 82FE5658: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE565C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 82FE5660: 481C2B58  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82FE5664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5668: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE566C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82FE5670: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5674: 4801485D  bl 0x82ff9ed0
	ctx.lr = 0x82FE5678;
	sub_82FF9ED0(ctx, base);
	// 82FE5678: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 82FE567C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 82FE5680: 388BC620  addi r4, r11, -0x39e0
	ctx.r[4].s64 = ctx.r[11].s64 + -14816;
	// 82FE5684: 481CB5A5  bl 0x831b0c28
	ctx.lr = 0x82FE5688;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5688 size=48
    let mut pc: u32 = 0x82FE5688;
    'dispatch: loop {
        match pc {
            0x82FE5688 => {
    //   block [0x82FE5688..0x82FE56B8)
	// 82FE5688: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 82FE568C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5690: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5698: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82FE569C: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 82FE56A0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE56A4: 480E243D  bl 0x830c7ae0
	ctx.lr = 0x82FE56A8;
	sub_830C7AE0(ctx, base);
	// 82FE56A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE56AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE56B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE56B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE56B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE56B8 size=32
    let mut pc: u32 = 0x82FE56B8;
    'dispatch: loop {
        match pc {
            0x82FE56B8 => {
    //   block [0x82FE56B8..0x82FE56D8)
	// 82FE56B8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82FE56BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82FE56C0: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 82FE56C4: 409A0008  bne cr6, 0x82fe56cc
	if !ctx.cr[6].eq {
	pc = 0x82FE56CC; continue 'dispatch;
	}
	// 82FE56C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82FE56CC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82FE56D0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82FE56D4: 4BFFEFEC  b 0x82fe46c0
	sub_82FE46C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE56D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE56D8 size=8
    let mut pc: u32 = 0x82FE56D8;
    'dispatch: loop {
        match pc {
            0x82FE56D8 => {
    //   block [0x82FE56D8..0x82FE56E0)
	// 82FE56D8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE56DC: 8213B680  lwz r16, -0x4980(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18816 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE56E0 size=228
    let mut pc: u32 = 0x82FE56E0;
    'dispatch: loop {
        match pc {
            0x82FE56E0 => {
    //   block [0x82FE56E0..0x82FE57C4)
	// 82FE56E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE56E4: 481C2A81  bl 0x831a8164
	ctx.lr = 0x82FE56E8;
	sub_831A8130(ctx, base);
	// 82FE56E8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82FE56EC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE56F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82FE56F4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82FE56F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82FE56FC: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE5700: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 82FE5704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE5708: 409A0034  bne cr6, 0x82fe573c
	if !ctx.cr[6].eq {
	pc = 0x82FE573C; continue 'dispatch;
	}
	// 82FE570C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82FE5710: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5714: 4BFF2B85  bl 0x82fd8298
	ctx.lr = 0x82FE5718;
	sub_82FD8298(ctx, base);
	// 82FE5718: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82FE571C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82FE5720: 41820014  beq 0x82fe5734
	if ctx.cr[0].eq {
	pc = 0x82FE5734; continue 'dispatch;
	}
	// 82FE5724: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82FE5728: 80BE0090  lwz r5, 0x90(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE572C: 4BFFC795  bl 0x82fe1ec0
	ctx.lr = 0x82FE5730;
	sub_82FE1EC0(ctx, base);
	// 82FE5730: 48000008  b 0x82fe5738
	pc = 0x82FE5738; continue 'dispatch;
	// 82FE5734: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82FE5738: 907E006C  stw r3, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82FE573C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE5740: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE5744: 4BFFC815  bl 0x82fe1f58
	ctx.lr = 0x82FE5748;
	sub_82FE1F58(ctx, base);
	// 82FE5748: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE574C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE5750: 409A0054  bne cr6, 0x82fe57a4
	if !ctx.cr[6].eq {
	pc = 0x82FE57A4; continue 'dispatch;
	}
	// 82FE5754: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE5758: 809E0090  lwz r4, 0x90(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE575C: 4BFF2B3D  bl 0x82fd8298
	ctx.lr = 0x82FE5760;
	sub_82FD8298(ctx, base);
	// 82FE5760: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82FE5764: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82FE5768: 41820028  beq 0x82fe5790
	if ctx.cr[0].eq {
	pc = 0x82FE5790; continue 'dispatch;
	}
	// 82FE576C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5770: 80DE0090  lwz r6, 0x90(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5774: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82FE5778: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82FE577C: 48067075  bl 0x8304c7f0
	ctx.lr = 0x82FE5780;
	sub_8304C7F0(ctx, base);
	// 82FE5780: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 82FE5784: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 82FE5788: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE578C: 48000008  b 0x82fe5794
	pc = 0x82FE5794; continue 'dispatch;
	// 82FE5790: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82FE5794: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE5798: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE579C: 4BFFC7BD  bl 0x82fe1f58
	ctx.lr = 0x82FE57A0;
	sub_82FE1F58(ctx, base);
	// 82FE57A0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82FE57A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE57A8: 807E006C  lwz r3, 0x6c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82FE57AC: 4BFFC7AD  bl 0x82fe1f58
	ctx.lr = 0x82FE57B0;
	sub_82FE1F58(ctx, base);
	// 82FE57B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82FE57B4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82FE57B8: 48055999  bl 0x8303b150
	ctx.lr = 0x82FE57BC;
	sub_8303B150(ctx, base);
	// 82FE57BC: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 82FE57C0: 481C29F4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE57C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE57C4 size=48
    let mut pc: u32 = 0x82FE57C4;
    'dispatch: loop {
        match pc {
            0x82FE57C4 => {
    //   block [0x82FE57C4..0x82FE57F4)
	// 82FE57C4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE57C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE57CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE57D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE57D4: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE57D8: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE57DC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE57E0: 4BFF2B01  bl 0x82fd82e0
	ctx.lr = 0x82FE57E4;
	sub_82FD82E0(ctx, base);
	// 82FE57E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE57E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE57EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE57F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE57F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE57F4 size=48
    let mut pc: u32 = 0x82FE57F4;
    'dispatch: loop {
        match pc {
            0x82FE57F4 => {
    //   block [0x82FE57F4..0x82FE5824)
	// 82FE57F4: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 82FE57F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE57FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE5800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5804: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 82FE5808: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE580C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE5810: 4BFF2AD1  bl 0x82fd82e0
	ctx.lr = 0x82FE5814;
	sub_82FD82E0(ctx, base);
	// 82FE5814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE5818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE581C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE5820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82FE5828 size=8
    let mut pc: u32 = 0x82FE5828;
    'dispatch: loop {
        match pc {
            0x82FE5828 => {
    //   block [0x82FE5828..0x82FE5830)
	// 82FE5828: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 82FE582C: 8213B6D8  lwz r16, -0x4928(r19)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(-18728 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE5830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE5830 size=132
    let mut pc: u32 = 0x82FE5830;
    'dispatch: loop {
        match pc {
            0x82FE5830 => {
    //   block [0x82FE5830..0x82FE58B4)
	// 82FE5830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE5834: 481C2935  bl 0x831a8168
	ctx.lr = 0x82FE5838;
	sub_831A8130(ctx, base);
	// 82FE5838: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 82FE583C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE5840: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82FE5844: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82FE5848: 817D0070  lwz r11, 0x70(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE584C: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82FE5850: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82FE5854: 409A004C  bne cr6, 0x82fe58a0
	if !ctx.cr[6].eq {
	pc = 0x82FE58A0; continue 'dispatch;
	}
	// 82FE5858: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82FE585C: 809D0090  lwz r4, 0x90(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5860: 4BFF2A39  bl 0x82fd8298
	ctx.lr = 0x82FE5864;
	sub_82FD8298(ctx, base);
	// 82FE5864: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82FE5868: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82FE586C: 4182002C  beq 0x82fe5898
	if ctx.cr[0].eq {
	pc = 0x82FE5898; continue 'dispatch;
	}
	// 82FE5870: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82FE5874: 80DD0090  lwz r6, 0x90(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE5878: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82FE587C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82FE5880: 4BFFCC79  bl 0x82fe24f8
	ctx.lr = 0x82FE5884;
	sub_82FE24F8(ctx, base);
	// 82FE5884: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 82FE5888: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82FE588C: 396BB018  addi r11, r11, -0x4fe8
	ctx.r[11].s64 = ctx.r[11].s64 + -20456;
	// 82FE5890: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82FE5894: 48000008  b 0x82fe589c
	pc = 0x82FE589C; continue 'dispatch;
	// 82FE5898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82FE589C: 915D0070  stw r10, 0x70(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82FE58A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82FE58A4: 807D0070  lwz r3, 0x70(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 82FE58A8: 480558A9  bl 0x8303b150
	ctx.lr = 0x82FE58AC;
	sub_8303B150(ctx, base);
	// 82FE58AC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 82FE58B0: 481C2908  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82FE58B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82FE58B4 size=48
    let mut pc: u32 = 0x82FE58B4;
    'dispatch: loop {
        match pc {
            0x82FE58B4 => {
    //   block [0x82FE58B4..0x82FE58E4)
	// 82FE58B4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 82FE58B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82FE58BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82FE58C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82FE58C4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82FE58C8: 808B0090  lwz r4, 0x90(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 82FE58CC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82FE58D0: 4BFF2A11  bl 0x82fd82e0
	ctx.lr = 0x82FE58D4;
	sub_82FD82E0(ctx, base);
	// 82FE58D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82FE58D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82FE58DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82FE58E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


