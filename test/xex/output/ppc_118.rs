pub fn sub_827005D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827005D0 size=116
    let mut pc: u32 = 0x827005D0;
    'dispatch: loop {
        match pc {
            0x827005D0 => {
    //   block [0x827005D0..0x82700644)
	// 827005D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827005D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827005D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827005DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 827005E0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 827005E4: 390A6C88  addi r8, r10, 0x6c88
	ctx.r[8].s64 = ctx.r[10].s64 + 27784;
	// 827005E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827005EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827005F0: 38AAE23C  addi r5, r10, -0x1dc4
	ctx.r[5].s64 = ctx.r[10].s64 + -7620;
	// 827005F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827005F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827005FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700604: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82700608: 396BBF18  addi r11, r11, -0x40e8
	ctx.r[11].s64 = ctx.r[11].s64 + -16616;
	// 8270060C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700614: 386AE3BC  addi r3, r10, -0x1c44
	ctx.r[3].s64 = ctx.r[10].s64 + -7236;
	// 82700618: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270061C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700620: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82700624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700630: 4BD667F1  bl 0x82466e20
	ctx.lr = 0x82700634;
	sub_82466E20(ctx, base);
	// 82700634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270063C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700648 size=108
    let mut pc: u32 = 0x82700648;
    'dispatch: loop {
        match pc {
            0x82700648 => {
    //   block [0x82700648..0x827006B4)
	// 82700648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700654: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270065C: 38EB6D60  addi r7, r11, 0x6d60
	ctx.r[7].s64 = ctx.r[11].s64 + 28000;
	// 82700660: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82700664: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82700668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270066C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700678: 386AE3EC  addi r3, r10, -0x1c14
	ctx.r[3].s64 = ctx.r[10].s64 + -7188;
	// 8270067C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270068C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270069C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827006A0: 4BD66781  bl 0x82466e20
	ctx.lr = 0x827006A4;
	sub_82466E20(ctx, base);
	// 827006A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827006A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827006AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827006B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827006B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827006B8 size=112
    let mut pc: u32 = 0x827006B8;
    'dispatch: loop {
        match pc {
            0x827006B8 => {
    //   block [0x827006B8..0x82700728)
	// 827006B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827006BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827006C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827006C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827006C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827006CC: 38AAE23C  addi r5, r10, -0x1dc4
	ctx.r[5].s64 = ctx.r[10].s64 + -7620;
	// 827006D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827006D4: 390B6DA8  addi r8, r11, 0x6da8
	ctx.r[8].s64 = ctx.r[11].s64 + 28072;
	// 827006D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 827006DC: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 827006E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827006E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827006E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827006EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827006F0: 386AE41C  addi r3, r10, -0x1be4
	ctx.r[3].s64 = ctx.r[10].s64 + -7140;
	// 827006F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827006F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827006FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270070C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700714: 4BD6670D  bl 0x82466e20
	ctx.lr = 0x82700718;
	sub_82466E20(ctx, base);
	// 82700718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270071C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700728 size=112
    let mut pc: u32 = 0x82700728;
    'dispatch: loop {
        match pc {
            0x82700728 => {
    //   block [0x82700728..0x82700798)
	// 82700728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700734: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700738: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270073C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700744: 390B6E20  addi r8, r11, 0x6e20
	ctx.r[8].s64 = ctx.r[11].s64 + 28192;
	// 82700748: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270074C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82700750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700754: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270075C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700760: 386AE44C  addi r3, r10, -0x1bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -7092;
	// 82700764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270076C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270077C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700784: 4BD6669D  bl 0x82466e20
	ctx.lr = 0x82700788;
	sub_82466E20(ctx, base);
	// 82700788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700798 size=108
    let mut pc: u32 = 0x82700798;
    'dispatch: loop {
        match pc {
            0x82700798 => {
    //   block [0x82700798..0x82700804)
	// 82700798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827007A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827007A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827007A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827007AC: 38EB6E50  addi r7, r11, 0x6e50
	ctx.r[7].s64 = ctx.r[11].s64 + 28240;
	// 827007B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 827007B4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 827007B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827007BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827007C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827007C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827007C8: 386AE47C  addi r3, r10, -0x1b84
	ctx.r[3].s64 = ctx.r[10].s64 + -7044;
	// 827007CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827007D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827007D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827007D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827007DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827007E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827007E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827007E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827007EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827007F0: 4BD66631  bl 0x82466e20
	ctx.lr = 0x827007F4;
	sub_82466E20(ctx, base);
	// 827007F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827007F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827007FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700808 size=112
    let mut pc: u32 = 0x82700808;
    'dispatch: loop {
        match pc {
            0x82700808 => {
    //   block [0x82700808..0x82700878)
	// 82700808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700814: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700818: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270081C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700824: 390B6EC8  addi r8, r11, 0x6ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 28360;
	// 82700828: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270082C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82700830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700834: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270083C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700840: 386AE4AC  addi r3, r10, -0x1b54
	ctx.r[3].s64 = ctx.r[10].s64 + -6996;
	// 82700844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270084C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270085C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700864: 4BD665BD  bl 0x82466e20
	ctx.lr = 0x82700868;
	sub_82466E20(ctx, base);
	// 82700868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82700878 size=24
    let mut pc: u32 = 0x82700878;
    'dispatch: loop {
        match pc {
            0x82700878 => {
    //   block [0x82700878..0x82700890)
	// 82700878: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270087C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82700880: 394A9820  addi r10, r10, -0x67e0
	ctx.r[10].s64 = ctx.r[10].s64 + -26592;
	// 82700884: 816B6B94  lwz r11, 0x6b94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27540 as u32) ) } as u64;
	// 82700888: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8270088C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700890 size=116
    let mut pc: u32 = 0x82700890;
    'dispatch: loop {
        match pc {
            0x82700890 => {
    //   block [0x82700890..0x82700904)
	// 82700890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270089C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827008A0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827008A4: 390B9820  addi r8, r11, -0x67e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26592;
	// 827008A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827008AC: 392ABF74  addi r9, r10, -0x408c
	ctx.r[9].s64 = ctx.r[10].s64 + -16524;
	// 827008B0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827008B4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827008B8: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827008BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827008C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827008C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827008C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827008CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827008D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827008D4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827008D8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 827008DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827008E0: 386BE4DC  addi r3, r11, -0x1b24
	ctx.r[3].s64 = ctx.r[11].s64 + -6948;
	// 827008E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827008E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827008EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827008F0: 4BD66531  bl 0x82466e20
	ctx.lr = 0x827008F4;
	sub_82466E20(ctx, base);
	// 827008F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827008F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827008FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700908 size=112
    let mut pc: u32 = 0x82700908;
    'dispatch: loop {
        match pc {
            0x82700908 => {
    //   block [0x82700908..0x82700978)
	// 82700908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700914: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700918: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270091C: 38AAE4DC  addi r5, r10, -0x1b24
	ctx.r[5].s64 = ctx.r[10].s64 + -6948;
	// 82700920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700924: 390B6F10  addi r8, r11, 0x6f10
	ctx.r[8].s64 = ctx.r[11].s64 + 28432;
	// 82700928: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270092C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82700930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700934: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270093C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700940: 386AE50C  addi r3, r10, -0x1af4
	ctx.r[3].s64 = ctx.r[10].s64 + -6900;
	// 82700944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270094C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270095C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700964: 4BD664BD  bl 0x82466e20
	ctx.lr = 0x82700968;
	sub_82466E20(ctx, base);
	// 82700968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270096C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700978 size=112
    let mut pc: u32 = 0x82700978;
    'dispatch: loop {
        match pc {
            0x82700978 => {
    //   block [0x82700978..0x827009E8)
	// 82700978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700988: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270098C: 38AAE50C  addi r5, r10, -0x1af4
	ctx.r[5].s64 = ctx.r[10].s64 + -6900;
	// 82700990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700994: 390B6F40  addi r8, r11, 0x6f40
	ctx.r[8].s64 = ctx.r[11].s64 + 28480;
	// 82700998: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270099C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 827009A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827009A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827009A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827009AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827009B0: 386AE53C  addi r3, r10, -0x1ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -6852;
	// 827009B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827009B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827009BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827009C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827009C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827009C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827009CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827009D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827009D4: 4BD6644D  bl 0x82466e20
	ctx.lr = 0x827009D8;
	sub_82466E20(ctx, base);
	// 827009D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827009DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827009E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827009E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827009E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827009E8 size=112
    let mut pc: u32 = 0x827009E8;
    'dispatch: loop {
        match pc {
            0x827009E8 => {
    //   block [0x827009E8..0x82700A58)
	// 827009E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827009EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827009F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827009F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827009F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827009FC: 38AAE50C  addi r5, r10, -0x1af4
	ctx.r[5].s64 = ctx.r[10].s64 + -6900;
	// 82700A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700A04: 390B6FA0  addi r8, r11, 0x6fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 28576;
	// 82700A08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82700A0C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82700A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700A14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700A20: 386AE56C  addi r3, r10, -0x1a94
	ctx.r[3].s64 = ctx.r[10].s64 + -6804;
	// 82700A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700A44: 4BD663DD  bl 0x82466e20
	ctx.lr = 0x82700A48;
	sub_82466E20(ctx, base);
	// 82700A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700A58 size=112
    let mut pc: u32 = 0x82700A58;
    'dispatch: loop {
        match pc {
            0x82700A58 => {
    //   block [0x82700A58..0x82700AC8)
	// 82700A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700A68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700A6C: 38AAE50C  addi r5, r10, -0x1af4
	ctx.r[5].s64 = ctx.r[10].s64 + -6900;
	// 82700A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700A74: 390B6FD0  addi r8, r11, 0x6fd0
	ctx.r[8].s64 = ctx.r[11].s64 + 28624;
	// 82700A78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82700A7C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82700A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700A84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700A90: 386AE59C  addi r3, r10, -0x1a64
	ctx.r[3].s64 = ctx.r[10].s64 + -6756;
	// 82700A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700AB4: 4BD6636D  bl 0x82466e20
	ctx.lr = 0x82700AB8;
	sub_82466E20(ctx, base);
	// 82700AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700AC8 size=108
    let mut pc: u32 = 0x82700AC8;
    'dispatch: loop {
        match pc {
            0x82700AC8 => {
    //   block [0x82700AC8..0x82700B34)
	// 82700AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700AD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700ADC: 38EB7018  addi r7, r11, 0x7018
	ctx.r[7].s64 = ctx.r[11].s64 + 28696;
	// 82700AE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82700AE4: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82700AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700AF8: 386AE5CC  addi r3, r10, -0x1a34
	ctx.r[3].s64 = ctx.r[10].s64 + -6708;
	// 82700AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82700B20: 4BD66301  bl 0x82466e20
	ctx.lr = 0x82700B24;
	sub_82466E20(ctx, base);
	// 82700B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700B38 size=112
    let mut pc: u32 = 0x82700B38;
    'dispatch: loop {
        match pc {
            0x82700B38 => {
    //   block [0x82700B38..0x82700BA8)
	// 82700B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700B4C: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82700B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700B54: 390B7048  addi r8, r11, 0x7048
	ctx.r[8].s64 = ctx.r[11].s64 + 28744;
	// 82700B58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82700B5C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82700B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700B64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700B70: 386AE5FC  addi r3, r10, -0x1a04
	ctx.r[3].s64 = ctx.r[10].s64 + -6660;
	// 82700B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700B94: 4BD6628D  bl 0x82466e20
	ctx.lr = 0x82700B98;
	sub_82466E20(ctx, base);
	// 82700B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700BA8 size=116
    let mut pc: u32 = 0x82700BA8;
    'dispatch: loop {
        match pc {
            0x82700BA8 => {
    //   block [0x82700BA8..0x82700C1C)
	// 82700BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700BB4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 82700BB8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82700BBC: 390A7060  addi r8, r10, 0x7060
	ctx.r[8].s64 = ctx.r[10].s64 + 28768;
	// 82700BC0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700BC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82700BC8: 38AAEAAC  addi r5, r10, -0x1554
	ctx.r[5].s64 = ctx.r[10].s64 + -5460;
	// 82700BCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700BD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82700BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700BDC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82700BE0: 396BBF88  addi r11, r11, -0x4078
	ctx.r[11].s64 = ctx.r[11].s64 + -16504;
	// 82700BE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700BE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700BEC: 386AE62C  addi r3, r10, -0x19d4
	ctx.r[3].s64 = ctx.r[10].s64 + -6612;
	// 82700BF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82700BF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700BF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82700BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700C08: 4BD66219  bl 0x82466e20
	ctx.lr = 0x82700C0C;
	sub_82466E20(ctx, base);
	// 82700C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700C20 size=100
    let mut pc: u32 = 0x82700C20;
    'dispatch: loop {
        match pc {
            0x82700C20 => {
    //   block [0x82700C20..0x82700C84)
	// 82700C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700C2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700C34: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82700C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700C40: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82700C44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700C54: 386AE65C  addi r3, r10, -0x19a4
	ctx.r[3].s64 = ctx.r[10].s64 + -6564;
	// 82700C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700C60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700C68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700C70: 4BD661B1  bl 0x82466e20
	ctx.lr = 0x82700C74;
	sub_82466E20(ctx, base);
	// 82700C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700C88 size=100
    let mut pc: u32 = 0x82700C88;
    'dispatch: loop {
        match pc {
            0x82700C88 => {
    //   block [0x82700C88..0x82700CEC)
	// 82700C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700C9C: 38AAE6EC  addi r5, r10, -0x1914
	ctx.r[5].s64 = ctx.r[10].s64 + -6420;
	// 82700CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700CA8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82700CAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700CBC: 386AE68C  addi r3, r10, -0x1974
	ctx.r[3].s64 = ctx.r[10].s64 + -6516;
	// 82700CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700CC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700CC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700CD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700CD8: 4BD66149  bl 0x82466e20
	ctx.lr = 0x82700CDC;
	sub_82466E20(ctx, base);
	// 82700CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700CF0 size=100
    let mut pc: u32 = 0x82700CF0;
    'dispatch: loop {
        match pc {
            0x82700CF0 => {
    //   block [0x82700CF0..0x82700D54)
	// 82700CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700CFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700D04: 38AAE62C  addi r5, r10, -0x19d4
	ctx.r[5].s64 = ctx.r[10].s64 + -6612;
	// 82700D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700D10: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82700D14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700D24: 386AE6BC  addi r3, r10, -0x1944
	ctx.r[3].s64 = ctx.r[10].s64 + -6468;
	// 82700D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700D2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700D30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700D38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700D40: 4BD660E1  bl 0x82466e20
	ctx.lr = 0x82700D44;
	sub_82466E20(ctx, base);
	// 82700D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700D58 size=104
    let mut pc: u32 = 0x82700D58;
    'dispatch: loop {
        match pc {
            0x82700D58 => {
    //   block [0x82700D58..0x82700DC0)
	// 82700D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700D64: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82700D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700D6C: 392ABFEC  addi r9, r10, -0x4014
	ctx.r[9].s64 = ctx.r[10].s64 + -16404;
	// 82700D70: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700D78: 38AAE65C  addi r5, r10, -0x19a4
	ctx.r[5].s64 = ctx.r[10].s64 + -6564;
	// 82700D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700D8C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82700D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700D94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700D98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700DA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700DA4: 386AE6EC  addi r3, r10, -0x1914
	ctx.r[3].s64 = ctx.r[10].s64 + -6420;
	// 82700DA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82700DAC: 4BD66075  bl 0x82466e20
	ctx.lr = 0x82700DB0;
	sub_82466E20(ctx, base);
	// 82700DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700DC0 size=108
    let mut pc: u32 = 0x82700DC0;
    'dispatch: loop {
        match pc {
            0x82700DC0 => {
    //   block [0x82700DC0..0x82700E2C)
	// 82700DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700DCC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700DD4: 38EB71E4  addi r7, r11, 0x71e4
	ctx.r[7].s64 = ctx.r[11].s64 + 29156;
	// 82700DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82700DDC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82700DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82700DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700DF0: 386AE71C  addi r3, r10, -0x18e4
	ctx.r[3].s64 = ctx.r[10].s64 + -6372;
	// 82700DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82700DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82700E18: 4BD66009  bl 0x82466e20
	ctx.lr = 0x82700E1C;
	sub_82466E20(ctx, base);
	// 82700E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700E30 size=112
    let mut pc: u32 = 0x82700E30;
    'dispatch: loop {
        match pc {
            0x82700E30 => {
    //   block [0x82700E30..0x82700EA0)
	// 82700E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700E3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700E40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700E44: 38AAE6EC  addi r5, r10, -0x1914
	ctx.r[5].s64 = ctx.r[10].s64 + -6420;
	// 82700E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700E4C: 390B7218  addi r8, r11, 0x7218
	ctx.r[8].s64 = ctx.r[11].s64 + 29208;
	// 82700E50: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82700E54: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82700E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700E68: 386AE74C  addi r3, r10, -0x18b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6324;
	// 82700E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82700E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700E8C: 4BD65F95  bl 0x82466e20
	ctx.lr = 0x82700E90;
	sub_82466E20(ctx, base);
	// 82700E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82700EA0 size=24
    let mut pc: u32 = 0x82700EA0;
    'dispatch: loop {
        match pc {
            0x82700EA0 => {
    //   block [0x82700EA0..0x82700EB8)
	// 82700EA0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82700EA4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82700EA8: 394A9838  addi r10, r10, -0x67c8
	ctx.r[10].s64 = ctx.r[10].s64 + -26568;
	// 82700EAC: 816B7214  lwz r11, 0x7214(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29204 as u32) ) } as u64;
	// 82700EB0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82700EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700EB8 size=116
    let mut pc: u32 = 0x82700EB8;
    'dispatch: loop {
        match pc {
            0x82700EB8 => {
    //   block [0x82700EB8..0x82700F2C)
	// 82700EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700EC4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82700EC8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82700ECC: 390B9838  addi r8, r11, -0x67c8
	ctx.r[8].s64 = ctx.r[11].s64 + -26568;
	// 82700ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700ED4: 392AC050  addi r9, r10, -0x3fb0
	ctx.r[9].s64 = ctx.r[10].s64 + -16304;
	// 82700ED8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700EDC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82700EE0: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82700EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82700EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700EFC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82700F00: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82700F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82700F08: 386BE77C  addi r3, r11, -0x1884
	ctx.r[3].s64 = ctx.r[11].s64 + -6276;
	// 82700F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82700F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700F18: 4BD65F09  bl 0x82466e20
	ctx.lr = 0x82700F1C;
	sub_82466E20(ctx, base);
	// 82700F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700F30 size=100
    let mut pc: u32 = 0x82700F30;
    'dispatch: loop {
        match pc {
            0x82700F30 => {
    //   block [0x82700F30..0x82700F94)
	// 82700F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700F44: 38AAE77C  addi r5, r10, -0x1884
	ctx.r[5].s64 = ctx.r[10].s64 + -6276;
	// 82700F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700F50: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82700F54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700F64: 386AE7AC  addi r3, r10, -0x1854
	ctx.r[3].s64 = ctx.r[10].s64 + -6228;
	// 82700F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700F80: 4BD65EA1  bl 0x82466e20
	ctx.lr = 0x82700F84;
	sub_82466E20(ctx, base);
	// 82700F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82700F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82700F98 size=100
    let mut pc: u32 = 0x82700F98;
    'dispatch: loop {
        match pc {
            0x82700F98 => {
    //   block [0x82700F98..0x82700FFC)
	// 82700F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82700F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82700FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82700FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82700FAC: 38AAE80C  addi r5, r10, -0x17f4
	ctx.r[5].s64 = ctx.r[10].s64 + -6132;
	// 82700FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82700FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82700FB8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82700FBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82700FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82700FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82700FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82700FCC: 386AE7DC  addi r3, r10, -0x1824
	ctx.r[3].s64 = ctx.r[10].s64 + -6180;
	// 82700FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82700FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82700FD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82700FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82700FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82700FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82700FE8: 4BD65E39  bl 0x82466e20
	ctx.lr = 0x82700FEC;
	sub_82466E20(ctx, base);
	// 82700FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82700FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82700FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82700FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701000 size=112
    let mut pc: u32 = 0x82701000;
    'dispatch: loop {
        match pc {
            0x82701000 => {
    //   block [0x82701000..0x82701070)
	// 82701000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270100C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701010: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701014: 38AAE77C  addi r5, r10, -0x1884
	ctx.r[5].s64 = ctx.r[10].s64 + -6276;
	// 82701018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270101C: 390B72C0  addi r8, r11, 0x72c0
	ctx.r[8].s64 = ctx.r[11].s64 + 29376;
	// 82701020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82701024: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82701028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270102C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701038: 386AE80C  addi r3, r10, -0x17f4
	ctx.r[3].s64 = ctx.r[10].s64 + -6132;
	// 8270103C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270104C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270105C: 4BD65DC5  bl 0x82466e20
	ctx.lr = 0x82701060;
	sub_82466E20(ctx, base);
	// 82701060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270106C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701070 size=100
    let mut pc: u32 = 0x82701070;
    'dispatch: loop {
        match pc {
            0x82701070 => {
    //   block [0x82701070..0x827010D4)
	// 82701070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270107C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701084: 38AAE80C  addi r5, r10, -0x17f4
	ctx.r[5].s64 = ctx.r[10].s64 + -6132;
	// 82701088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270108C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701090: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 82701094: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270109C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827010A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827010A4: 386AE83C  addi r3, r10, -0x17c4
	ctx.r[3].s64 = ctx.r[10].s64 + -6084;
	// 827010A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827010AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827010B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827010B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827010B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827010BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827010C0: 4BD65D61  bl 0x82466e20
	ctx.lr = 0x827010C4;
	sub_82466E20(ctx, base);
	// 827010C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827010C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827010CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827010D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827010D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827010D8 size=100
    let mut pc: u32 = 0x827010D8;
    'dispatch: loop {
        match pc {
            0x827010D8 => {
    //   block [0x827010D8..0x8270113C)
	// 827010D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827010DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827010E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827010E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827010E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827010EC: 38AAE77C  addi r5, r10, -0x1884
	ctx.r[5].s64 = ctx.r[10].s64 + -6276;
	// 827010F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827010F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827010F8: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 827010FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270110C: 386AE86C  addi r3, r10, -0x1794
	ctx.r[3].s64 = ctx.r[10].s64 + -6036;
	// 82701110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701114: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270111C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82701124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701128: 4BD65CF9  bl 0x82466e20
	ctx.lr = 0x8270112C;
	sub_82466E20(ctx, base);
	// 8270112C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701140 size=100
    let mut pc: u32 = 0x82701140;
    'dispatch: loop {
        match pc {
            0x82701140 => {
    //   block [0x82701140..0x827011A4)
	// 82701140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270114C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701154: 38AAE7AC  addi r5, r10, -0x1854
	ctx.r[5].s64 = ctx.r[10].s64 + -6228;
	// 82701158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270115C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701160: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82701164: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270116C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701174: 386AE89C  addi r3, r10, -0x1764
	ctx.r[3].s64 = ctx.r[10].s64 + -5988;
	// 82701178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270117C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82701184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701190: 4BD65C91  bl 0x82466e20
	ctx.lr = 0x82701194;
	sub_82466E20(ctx, base);
	// 82701194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270119C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827011A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827011A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827011A8 size=100
    let mut pc: u32 = 0x827011A8;
    'dispatch: loop {
        match pc {
            0x827011A8 => {
    //   block [0x827011A8..0x8270120C)
	// 827011A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827011AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827011B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827011B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827011B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827011BC: 38AAE86C  addi r5, r10, -0x1794
	ctx.r[5].s64 = ctx.r[10].s64 + -6036;
	// 827011C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827011C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827011C8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 827011CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827011D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827011D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827011D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827011DC: 386AE8CC  addi r3, r10, -0x1734
	ctx.r[3].s64 = ctx.r[10].s64 + -5940;
	// 827011E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827011E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827011E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827011EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827011F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827011F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827011F8: 4BD65C29  bl 0x82466e20
	ctx.lr = 0x827011FC;
	sub_82466E20(ctx, base);
	// 827011FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701210 size=100
    let mut pc: u32 = 0x82701210;
    'dispatch: loop {
        match pc {
            0x82701210 => {
    //   block [0x82701210..0x82701274)
	// 82701210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270121C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701224: 38AAE7AC  addi r5, r10, -0x1854
	ctx.r[5].s64 = ctx.r[10].s64 + -6228;
	// 82701228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270122C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701230: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82701234: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270123C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701244: 386AE8FC  addi r3, r10, -0x1704
	ctx.r[3].s64 = ctx.r[10].s64 + -5892;
	// 82701248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270124C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82701254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270125C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701260: 4BD65BC1  bl 0x82466e20
	ctx.lr = 0x82701264;
	sub_82466E20(ctx, base);
	// 82701264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270126C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701278 size=112
    let mut pc: u32 = 0x82701278;
    'dispatch: loop {
        match pc {
            0x82701278 => {
    //   block [0x82701278..0x827012E8)
	// 82701278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270127C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701288: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270128C: 38AAE98C  addi r5, r10, -0x1674
	ctx.r[5].s64 = ctx.r[10].s64 + -5748;
	// 82701290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701294: 390B72F0  addi r8, r11, 0x72f0
	ctx.r[8].s64 = ctx.r[11].s64 + 29424;
	// 82701298: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270129C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 827012A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827012A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827012A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827012AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827012B0: 386AE92C  addi r3, r10, -0x16d4
	ctx.r[3].s64 = ctx.r[10].s64 + -5844;
	// 827012B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827012B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827012BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827012C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827012C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827012C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827012CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827012D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827012D4: 4BD65B4D  bl 0x82466e20
	ctx.lr = 0x827012D8;
	sub_82466E20(ctx, base);
	// 827012D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827012DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827012E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827012E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827012E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827012E8 size=112
    let mut pc: u32 = 0x827012E8;
    'dispatch: loop {
        match pc {
            0x827012E8 => {
    //   block [0x827012E8..0x82701358)
	// 827012E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827012EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827012F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827012F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827012F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827012FC: 38AAE9BC  addi r5, r10, -0x1644
	ctx.r[5].s64 = ctx.r[10].s64 + -5700;
	// 82701300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701304: 390B7320  addi r8, r11, 0x7320
	ctx.r[8].s64 = ctx.r[11].s64 + 29472;
	// 82701308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270130C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82701310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701314: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270131C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701320: 386AE95C  addi r3, r10, -0x16a4
	ctx.r[3].s64 = ctx.r[10].s64 + -5796;
	// 82701324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270132C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270133C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701344: 4BD65ADD  bl 0x82466e20
	ctx.lr = 0x82701348;
	sub_82466E20(ctx, base);
	// 82701348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270134C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701358 size=112
    let mut pc: u32 = 0x82701358;
    'dispatch: loop {
        match pc {
            0x82701358 => {
    //   block [0x82701358..0x827013C8)
	// 82701358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270135C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701368: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270136C: 38AAEAAC  addi r5, r10, -0x1554
	ctx.r[5].s64 = ctx.r[10].s64 + -5460;
	// 82701370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701374: 390B7338  addi r8, r11, 0x7338
	ctx.r[8].s64 = ctx.r[11].s64 + 29496;
	// 82701378: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270137C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82701380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270138C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701390: 386AE98C  addi r3, r10, -0x1674
	ctx.r[3].s64 = ctx.r[10].s64 + -5748;
	// 82701394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270139C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827013A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827013A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827013A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827013AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827013B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827013B4: 4BD65A6D  bl 0x82466e20
	ctx.lr = 0x827013B8;
	sub_82466E20(ctx, base);
	// 827013B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827013BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827013C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827013C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827013C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827013C8 size=112
    let mut pc: u32 = 0x827013C8;
    'dispatch: loop {
        match pc {
            0x827013C8 => {
    //   block [0x827013C8..0x82701438)
	// 827013C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827013CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827013D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827013D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827013D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827013DC: 38AAE98C  addi r5, r10, -0x1674
	ctx.r[5].s64 = ctx.r[10].s64 + -5748;
	// 827013E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827013E4: 390B7368  addi r8, r11, 0x7368
	ctx.r[8].s64 = ctx.r[11].s64 + 29544;
	// 827013E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827013EC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 827013F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827013F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827013F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827013FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701400: 386AE9BC  addi r3, r10, -0x1644
	ctx.r[3].s64 = ctx.r[10].s64 + -5700;
	// 82701404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270140C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270141C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701424: 4BD659FD  bl 0x82466e20
	ctx.lr = 0x82701428;
	sub_82466E20(ctx, base);
	// 82701428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270142C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701438 size=112
    let mut pc: u32 = 0x82701438;
    'dispatch: loop {
        match pc {
            0x82701438 => {
    //   block [0x82701438..0x827014A8)
	// 82701438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270143C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701448: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270144C: 38AAE9BC  addi r5, r10, -0x1644
	ctx.r[5].s64 = ctx.r[10].s64 + -5700;
	// 82701450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701454: 390B7380  addi r8, r11, 0x7380
	ctx.r[8].s64 = ctx.r[11].s64 + 29568;
	// 82701458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270145C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82701460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270146C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701470: 386AE9EC  addi r3, r10, -0x1614
	ctx.r[3].s64 = ctx.r[10].s64 + -5652;
	// 82701474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270147C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270148C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701494: 4BD6598D  bl 0x82466e20
	ctx.lr = 0x82701498;
	sub_82466E20(ctx, base);
	// 82701498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270149C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827014A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827014A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827014A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827014A8 size=112
    let mut pc: u32 = 0x827014A8;
    'dispatch: loop {
        match pc {
            0x827014A8 => {
    //   block [0x827014A8..0x82701518)
	// 827014A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827014AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827014B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827014B4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827014B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827014BC: 392AC07C  addi r9, r10, -0x3f84
	ctx.r[9].s64 = ctx.r[10].s64 + -16260;
	// 827014C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827014C4: 390B739C  addi r8, r11, 0x739c
	ctx.r[8].s64 = ctx.r[11].s64 + 29596;
	// 827014C8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 827014CC: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 827014D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827014D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827014D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827014DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827014E0: 386AEA1C  addi r3, r10, -0x15e4
	ctx.r[3].s64 = ctx.r[10].s64 + -5604;
	// 827014E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827014E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827014EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827014F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827014F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827014F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827014FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701504: 4BD6591D  bl 0x82466e20
	ctx.lr = 0x82701508;
	sub_82466E20(ctx, base);
	// 82701508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701518 size=116
    let mut pc: u32 = 0x82701518;
    'dispatch: loop {
        match pc {
            0x82701518 => {
    //   block [0x82701518..0x8270158C)
	// 82701518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701524: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 82701528: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8270152C: 390A73D0  addi r8, r10, 0x73d0
	ctx.r[8].s64 = ctx.r[10].s64 + 29648;
	// 82701530: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701534: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82701538: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 8270153C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701540: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270154C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82701550: 396BC090  addi r11, r11, -0x3f70
	ctx.r[11].s64 = ctx.r[11].s64 + -16240;
	// 82701554: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270155C: 386AEA4C  addi r3, r10, -0x15b4
	ctx.r[3].s64 = ctx.r[10].s64 + -5556;
	// 82701560: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82701564: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701568: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8270156C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701578: 4BD658A9  bl 0x82466e20
	ctx.lr = 0x8270157C;
	sub_82466E20(ctx, base);
	// 8270157C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82701590 size=48
    let mut pc: u32 = 0x82701590;
    'dispatch: loop {
        match pc {
            0x82701590 => {
    //   block [0x82701590..0x827015C0)
	// 82701590: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701594: 814B7480  lwz r10, 0x7480(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29824 as u32) ) } as u64;
	// 82701598: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270159C: 396B98B0  addi r11, r11, -0x6750
	ctx.r[11].s64 = ctx.r[11].s64 + -26448;
	// 827015A0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 827015A4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 827015A8: 814A747C  lwz r10, 0x747c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29820 as u32) ) } as u64;
	// 827015AC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 827015B0: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 827015B4: 814A7478  lwz r10, 0x7478(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29816 as u32) ) } as u64;
	// 827015B8: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 827015BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827015C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827015C0 size=116
    let mut pc: u32 = 0x827015C0;
    'dispatch: loop {
        match pc {
            0x827015C0 => {
    //   block [0x827015C0..0x82701634)
	// 827015C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827015C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827015C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827015CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827015D0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827015D4: 392BC160  addi r9, r11, -0x3ea0
	ctx.r[9].s64 = ctx.r[11].s64 + -16032;
	// 827015D8: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827015DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827015E0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 827015E4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 827015E8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827015EC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 827015F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827015F4: 396B98B0  addi r11, r11, -0x6750
	ctx.r[11].s64 = ctx.r[11].s64 + -26448;
	// 827015F8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 827015FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701600: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82701604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701608: 386AEA7C  addi r3, r10, -0x1584
	ctx.r[3].s64 = ctx.r[10].s64 + -5508;
	// 8270160C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82701610: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82701614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701618: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8270161C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82701620: 4BD65801  bl 0x82466e20
	ctx.lr = 0x82701624;
	sub_82466E20(ctx, base);
	// 82701624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270162C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701638 size=116
    let mut pc: u32 = 0x82701638;
    'dispatch: loop {
        match pc {
            0x82701638 => {
    //   block [0x82701638..0x827016AC)
	// 82701638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701644: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701648: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270164C: 390B7488  addi r8, r11, 0x7488
	ctx.r[8].s64 = ctx.r[11].s64 + 29832;
	// 82701650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701654: 392AC288  addi r9, r10, -0x3d78
	ctx.r[9].s64 = ctx.r[10].s64 + -15736;
	// 82701658: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270165C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82701660: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82701664: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270166C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270167C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82701680: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82701684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701688: 386BEAAC  addi r3, r11, -0x1554
	ctx.r[3].s64 = ctx.r[11].s64 + -5460;
	// 8270168C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701690: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701698: 4BD65789  bl 0x82466e20
	ctx.lr = 0x8270169C;
	sub_82466E20(ctx, base);
	// 8270169C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827016A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827016A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827016A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827016B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827016B0 size=112
    let mut pc: u32 = 0x827016B0;
    'dispatch: loop {
        match pc {
            0x827016B0 => {
    //   block [0x827016B0..0x82701720)
	// 827016B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827016B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827016B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827016BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827016C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827016C4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827016C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827016CC: 390B7518  addi r8, r11, 0x7518
	ctx.r[8].s64 = ctx.r[11].s64 + 29976;
	// 827016D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827016D4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 827016D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827016DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827016E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827016E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827016E8: 386AEADC  addi r3, r10, -0x1524
	ctx.r[3].s64 = ctx.r[10].s64 + -5412;
	// 827016EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827016F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827016F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827016F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827016FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270170C: 4BD65715  bl 0x82466e20
	ctx.lr = 0x82701710;
	sub_82466E20(ctx, base);
	// 82701710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270171C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701720 size=112
    let mut pc: u32 = 0x82701720;
    'dispatch: loop {
        match pc {
            0x82701720 => {
    //   block [0x82701720..0x82701790)
	// 82701720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270172C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701730: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701734: 38AACDFC  addi r5, r10, -0x3204
	ctx.r[5].s64 = ctx.r[10].s64 + -12804;
	// 82701738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270173C: 390B7530  addi r8, r11, 0x7530
	ctx.r[8].s64 = ctx.r[11].s64 + 30000;
	// 82701740: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701744: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82701748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270174C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701758: 386AEB0C  addi r3, r10, -0x14f4
	ctx.r[3].s64 = ctx.r[10].s64 + -5364;
	// 8270175C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270176C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270177C: 4BD656A5  bl 0x82466e20
	ctx.lr = 0x82701780;
	sub_82466E20(ctx, base);
	// 82701780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701790 size=108
    let mut pc: u32 = 0x82701790;
    'dispatch: loop {
        match pc {
            0x82701790 => {
    //   block [0x82701790..0x827017FC)
	// 82701790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270179C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827017A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827017A4: 38EB7548  addi r7, r11, 0x7548
	ctx.r[7].s64 = ctx.r[11].s64 + 30024;
	// 827017A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827017AC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 827017B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827017B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827017B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827017BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827017C0: 386AEB3C  addi r3, r10, -0x14c4
	ctx.r[3].s64 = ctx.r[10].s64 + -5316;
	// 827017C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827017C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827017CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827017D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827017D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827017D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827017DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827017E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827017E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827017E8: 4BD65639  bl 0x82466e20
	ctx.lr = 0x827017EC;
	sub_82466E20(ctx, base);
	// 827017EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827017F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827017F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827017F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701800 size=112
    let mut pc: u32 = 0x82701800;
    'dispatch: loop {
        match pc {
            0x82701800 => {
    //   block [0x82701800..0x82701870)
	// 82701800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270180C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701810: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701814: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82701818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270181C: 390B7560  addi r8, r11, 0x7560
	ctx.r[8].s64 = ctx.r[11].s64 + 30048;
	// 82701820: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82701824: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82701828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270182C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701838: 386AEB6C  addi r3, r10, -0x1494
	ctx.r[3].s64 = ctx.r[10].s64 + -5268;
	// 8270183C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270184C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270185C: 4BD655C5  bl 0x82466e20
	ctx.lr = 0x82701860;
	sub_82466E20(ctx, base);
	// 82701860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701870 size=108
    let mut pc: u32 = 0x82701870;
    'dispatch: loop {
        match pc {
            0x82701870 => {
    //   block [0x82701870..0x827018DC)
	// 82701870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270187C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701884: 38EB75A8  addi r7, r11, 0x75a8
	ctx.r[7].s64 = ctx.r[11].s64 + 30120;
	// 82701888: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270188C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82701890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270189C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827018A0: 386AEB9C  addi r3, r10, -0x1464
	ctx.r[3].s64 = ctx.r[10].s64 + -5220;
	// 827018A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827018A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827018AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827018B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827018B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827018B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827018BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827018C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827018C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827018C8: 4BD65559  bl 0x82466e20
	ctx.lr = 0x827018CC;
	sub_82466E20(ctx, base);
	// 827018CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827018D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827018D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827018D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827018E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827018E0 size=112
    let mut pc: u32 = 0x827018E0;
    'dispatch: loop {
        match pc {
            0x827018E0 => {
    //   block [0x827018E0..0x82701950)
	// 827018E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827018E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827018E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827018EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827018F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827018F4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827018F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827018FC: 390B75C0  addi r8, r11, 0x75c0
	ctx.r[8].s64 = ctx.r[11].s64 + 30144;
	// 82701900: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82701904: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82701908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270190C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701918: 386AEBCC  addi r3, r10, -0x1434
	ctx.r[3].s64 = ctx.r[10].s64 + -5172;
	// 8270191C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270192C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270193C: 4BD654E5  bl 0x82466e20
	ctx.lr = 0x82701940;
	sub_82466E20(ctx, base);
	// 82701940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701950 size=112
    let mut pc: u32 = 0x82701950;
    'dispatch: loop {
        match pc {
            0x82701950 => {
    //   block [0x82701950..0x827019C0)
	// 82701950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270195C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82701960: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701964: 392AC2E0  addi r9, r10, -0x3d20
	ctx.r[9].s64 = ctx.r[10].s64 + -15648;
	// 82701968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270196C: 390B75F8  addi r8, r11, 0x75f8
	ctx.r[8].s64 = ctx.r[11].s64 + 30200;
	// 82701970: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82701974: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82701978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270197C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701988: 386AEBFC  addi r3, r10, -0x1404
	ctx.r[3].s64 = ctx.r[10].s64 + -5124;
	// 8270198C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701990: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270199C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827019A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827019A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827019A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827019AC: 4BD65475  bl 0x82466e20
	ctx.lr = 0x827019B0;
	sub_82466E20(ctx, base);
	// 827019B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827019B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827019B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827019BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827019C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827019C0 size=116
    let mut pc: u32 = 0x827019C0;
    'dispatch: loop {
        match pc {
            0x827019C0 => {
    //   block [0x827019C0..0x82701A34)
	// 827019C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827019C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827019C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827019CC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827019D0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827019D4: 390B76A0  addi r8, r11, 0x76a0
	ctx.r[8].s64 = ctx.r[11].s64 + 30368;
	// 827019D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827019DC: 392AC2B4  addi r9, r10, -0x3d4c
	ctx.r[9].s64 = ctx.r[10].s64 + -15692;
	// 827019E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827019E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827019E8: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 827019EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827019F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827019F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827019F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827019FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701A04: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82701A08: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82701A0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701A10: 386BEC2C  addi r3, r11, -0x13d4
	ctx.r[3].s64 = ctx.r[11].s64 + -5076;
	// 82701A14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701A18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701A20: 4BD65401  bl 0x82466e20
	ctx.lr = 0x82701A24;
	sub_82466E20(ctx, base);
	// 82701A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701A38 size=112
    let mut pc: u32 = 0x82701A38;
    'dispatch: loop {
        match pc {
            0x82701A38 => {
    //   block [0x82701A38..0x82701AA8)
	// 82701A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701A44: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82701A48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701A4C: 392AC30C  addi r9, r10, -0x3cf4
	ctx.r[9].s64 = ctx.r[10].s64 + -15604;
	// 82701A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701A54: 390B76C0  addi r8, r11, 0x76c0
	ctx.r[8].s64 = ctx.r[11].s64 + 30400;
	// 82701A58: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82701A5C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82701A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701A70: 386AEC5C  addi r3, r10, -0x13a4
	ctx.r[3].s64 = ctx.r[10].s64 + -5028;
	// 82701A74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82701A78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82701A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701A94: 4BD6538D  bl 0x82466e20
	ctx.lr = 0x82701A98;
	sub_82466E20(ctx, base);
	// 82701A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701AA8 size=112
    let mut pc: u32 = 0x82701AA8;
    'dispatch: loop {
        match pc {
            0x82701AA8 => {
    //   block [0x82701AA8..0x82701B18)
	// 82701AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701AB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701ABC: 38AADE4C  addi r5, r10, -0x21b4
	ctx.r[5].s64 = ctx.r[10].s64 + -8628;
	// 82701AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701AC4: 390B7720  addi r8, r11, 0x7720
	ctx.r[8].s64 = ctx.r[11].s64 + 30496;
	// 82701AC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701ACC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82701AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701AD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701AE0: 386AEC8C  addi r3, r10, -0x1374
	ctx.r[3].s64 = ctx.r[10].s64 + -4980;
	// 82701AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701B04: 4BD6531D  bl 0x82466e20
	ctx.lr = 0x82701B08;
	sub_82466E20(ctx, base);
	// 82701B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701B18 size=112
    let mut pc: u32 = 0x82701B18;
    'dispatch: loop {
        match pc {
            0x82701B18 => {
    //   block [0x82701B18..0x82701B88)
	// 82701B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701B28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701B2C: 38AADD5C  addi r5, r10, -0x22a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8868;
	// 82701B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701B34: 390B7738  addi r8, r11, 0x7738
	ctx.r[8].s64 = ctx.r[11].s64 + 30520;
	// 82701B38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82701B3C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82701B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701B50: 386AECBC  addi r3, r10, -0x1344
	ctx.r[3].s64 = ctx.r[10].s64 + -4932;
	// 82701B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701B74: 4BD652AD  bl 0x82466e20
	ctx.lr = 0x82701B78;
	sub_82466E20(ctx, base);
	// 82701B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701B88 size=112
    let mut pc: u32 = 0x82701B88;
    'dispatch: loop {
        match pc {
            0x82701B88 => {
    //   block [0x82701B88..0x82701BF8)
	// 82701B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701B98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701B9C: 38AADD5C  addi r5, r10, -0x22a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8868;
	// 82701BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701BA4: 390B7780  addi r8, r11, 0x7780
	ctx.r[8].s64 = ctx.r[11].s64 + 30592;
	// 82701BA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82701BAC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82701BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701BC0: 386AECEC  addi r3, r10, -0x1314
	ctx.r[3].s64 = ctx.r[10].s64 + -4884;
	// 82701BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701BE4: 4BD6523D  bl 0x82466e20
	ctx.lr = 0x82701BE8;
	sub_82466E20(ctx, base);
	// 82701BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701BF8 size=112
    let mut pc: u32 = 0x82701BF8;
    'dispatch: loop {
        match pc {
            0x82701BF8 => {
    //   block [0x82701BF8..0x82701C68)
	// 82701BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701C0C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82701C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701C14: 390B77E0  addi r8, r11, 0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30688;
	// 82701C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82701C1C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82701C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701C30: 386AED1C  addi r3, r10, -0x12e4
	ctx.r[3].s64 = ctx.r[10].s64 + -4836;
	// 82701C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701C54: 4BD651CD  bl 0x82466e20
	ctx.lr = 0x82701C58;
	sub_82466E20(ctx, base);
	// 82701C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701C68 size=112
    let mut pc: u32 = 0x82701C68;
    'dispatch: loop {
        match pc {
            0x82701C68 => {
    //   block [0x82701C68..0x82701CD8)
	// 82701C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701C7C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82701C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701C84: 390B7840  addi r8, r11, 0x7840
	ctx.r[8].s64 = ctx.r[11].s64 + 30784;
	// 82701C88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82701C8C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82701C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701C98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701CA0: 386AED4C  addi r3, r10, -0x12b4
	ctx.r[3].s64 = ctx.r[10].s64 + -4788;
	// 82701CA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701CC4: 4BD6515D  bl 0x82466e20
	ctx.lr = 0x82701CC8;
	sub_82466E20(ctx, base);
	// 82701CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701CD8 size=112
    let mut pc: u32 = 0x82701CD8;
    'dispatch: loop {
        match pc {
            0x82701CD8 => {
    //   block [0x82701CD8..0x82701D48)
	// 82701CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701CE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701CE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701CEC: 38AADD5C  addi r5, r10, -0x22a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8868;
	// 82701CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701CF4: 390B78A0  addi r8, r11, 0x78a0
	ctx.r[8].s64 = ctx.r[11].s64 + 30880;
	// 82701CF8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82701CFC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82701D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701D04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701D08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701D10: 386AED7C  addi r3, r10, -0x1284
	ctx.r[3].s64 = ctx.r[10].s64 + -4740;
	// 82701D14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701D34: 4BD650ED  bl 0x82466e20
	ctx.lr = 0x82701D38;
	sub_82466E20(ctx, base);
	// 82701D38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701D48 size=112
    let mut pc: u32 = 0x82701D48;
    'dispatch: loop {
        match pc {
            0x82701D48 => {
    //   block [0x82701D48..0x82701DB8)
	// 82701D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701D54: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 82701D58: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 82701D5C: 38EA7960  addi r7, r10, 0x7960
	ctx.r[7].s64 = ctx.r[10].s64 + 31072;
	// 82701D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701D64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82701D68: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82701D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701D70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82701D74: 396BC320  addi r11, r11, -0x3ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -15584;
	// 82701D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82701D7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701D84: 386AEDAC  addi r3, r10, -0x1254
	ctx.r[3].s64 = ctx.r[10].s64 + -4692;
	// 82701D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701D8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82701D90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701D94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82701D98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701D9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701DA4: 4BD6507D  bl 0x82466e20
	ctx.lr = 0x82701DA8;
	sub_82466E20(ctx, base);
	// 82701DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701DB8 size=112
    let mut pc: u32 = 0x82701DB8;
    'dispatch: loop {
        match pc {
            0x82701DB8 => {
    //   block [0x82701DB8..0x82701E28)
	// 82701DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701DC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701DC8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701DCC: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701DD4: 390B7B28  addi r8, r11, 0x7b28
	ctx.r[8].s64 = ctx.r[11].s64 + 31528;
	// 82701DD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701DDC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82701DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701DF0: 386AEDDC  addi r3, r10, -0x1224
	ctx.r[3].s64 = ctx.r[10].s64 + -4644;
	// 82701DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701E04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82701E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701E14: 4BD6500D  bl 0x82466e20
	ctx.lr = 0x82701E18;
	sub_82466E20(ctx, base);
	// 82701E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701E28 size=112
    let mut pc: u32 = 0x82701E28;
    'dispatch: loop {
        match pc {
            0x82701E28 => {
    //   block [0x82701E28..0x82701E98)
	// 82701E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701E34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701E38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701E3C: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701E44: 390B7B40  addi r8, r11, 0x7b40
	ctx.r[8].s64 = ctx.r[11].s64 + 31552;
	// 82701E48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701E4C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82701E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701E54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701E58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701E60: 386AEE0C  addi r3, r10, -0x11f4
	ctx.r[3].s64 = ctx.r[10].s64 + -4596;
	// 82701E64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701E74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82701E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701E84: 4BD64F9D  bl 0x82466e20
	ctx.lr = 0x82701E88;
	sub_82466E20(ctx, base);
	// 82701E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701E98 size=112
    let mut pc: u32 = 0x82701E98;
    'dispatch: loop {
        match pc {
            0x82701E98 => {
    //   block [0x82701E98..0x82701F08)
	// 82701E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701EA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701EA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701EAC: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701EB4: 390B7B58  addi r8, r11, 0x7b58
	ctx.r[8].s64 = ctx.r[11].s64 + 31576;
	// 82701EB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82701EBC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82701EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701ED0: 386AEE3C  addi r3, r10, -0x11c4
	ctx.r[3].s64 = ctx.r[10].s64 + -4548;
	// 82701ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701EF4: 4BD64F2D  bl 0x82466e20
	ctx.lr = 0x82701EF8;
	sub_82466E20(ctx, base);
	// 82701EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701F08 size=108
    let mut pc: u32 = 0x82701F08;
    'dispatch: loop {
        match pc {
            0x82701F08 => {
    //   block [0x82701F08..0x82701F74)
	// 82701F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701F14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701F1C: 38EB7B88  addi r7, r11, 0x7b88
	ctx.r[7].s64 = ctx.r[11].s64 + 31624;
	// 82701F20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82701F24: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82701F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701F2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82701F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701F38: 386AEE6C  addi r3, r10, -0x1194
	ctx.r[3].s64 = ctx.r[10].s64 + -4500;
	// 82701F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82701F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82701F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82701F60: 4BD64EC1  bl 0x82466e20
	ctx.lr = 0x82701F64;
	sub_82466E20(ctx, base);
	// 82701F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701F78 size=112
    let mut pc: u32 = 0x82701F78;
    'dispatch: loop {
        match pc {
            0x82701F78 => {
    //   block [0x82701F78..0x82701FE8)
	// 82701F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701F84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701F88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701F8C: 38AACEBC  addi r5, r10, -0x3144
	ctx.r[5].s64 = ctx.r[10].s64 + -12612;
	// 82701F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701F94: 390B7BB8  addi r8, r11, 0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 31672;
	// 82701F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82701F9C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82701FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82701FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82701FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82701FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82701FB0: 386AEE9C  addi r3, r10, -0x1164
	ctx.r[3].s64 = ctx.r[10].s64 + -4452;
	// 82701FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82701FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82701FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82701FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82701FC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82701FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82701FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82701FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82701FD4: 4BD64E4D  bl 0x82466e20
	ctx.lr = 0x82701FD8;
	sub_82466E20(ctx, base);
	// 82701FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82701FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82701FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82701FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82701FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82701FE8 size=108
    let mut pc: u32 = 0x82701FE8;
    'dispatch: loop {
        match pc {
            0x82701FE8 => {
    //   block [0x82701FE8..0x82702054)
	// 82701FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82701FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82701FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82701FF4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82701FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82701FFC: 38EB7BD0  addi r7, r11, 0x7bd0
	ctx.r[7].s64 = ctx.r[11].s64 + 31696;
	// 82702000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82702004: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82702008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270200C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702018: 386AEECC  addi r3, r10, -0x1134
	ctx.r[3].s64 = ctx.r[10].s64 + -4404;
	// 8270201C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270203C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702040: 4BD64DE1  bl 0x82466e20
	ctx.lr = 0x82702044;
	sub_82466E20(ctx, base);
	// 82702044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270204C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702058 size=108
    let mut pc: u32 = 0x82702058;
    'dispatch: loop {
        match pc {
            0x82702058 => {
    //   block [0x82702058..0x827020C4)
	// 82702058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270205C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702064: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270206C: 38EB7C00  addi r7, r11, 0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + 31744;
	// 82702070: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702074: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82702078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270207C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702088: 386AEEFC  addi r3, r10, -0x1104
	ctx.r[3].s64 = ctx.r[10].s64 + -4356;
	// 8270208C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270209C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827020A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827020A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827020A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827020AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827020B0: 4BD64D71  bl 0x82466e20
	ctx.lr = 0x827020B4;
	sub_82466E20(ctx, base);
	// 827020B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827020B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827020BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827020C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827020C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827020C8 size=112
    let mut pc: u32 = 0x827020C8;
    'dispatch: loop {
        match pc {
            0x827020C8 => {
    //   block [0x827020C8..0x82702138)
	// 827020C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827020CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827020D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827020D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827020D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827020DC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827020E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827020E4: 390B7C48  addi r8, r11, 0x7c48
	ctx.r[8].s64 = ctx.r[11].s64 + 31816;
	// 827020E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827020EC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 827020F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827020F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827020F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827020FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702100: 386AEF2C  addi r3, r10, -0x10d4
	ctx.r[3].s64 = ctx.r[10].s64 + -4308;
	// 82702104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270210C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270211C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702124: 4BD64CFD  bl 0x82466e20
	ctx.lr = 0x82702128;
	sub_82466E20(ctx, base);
	// 82702128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270212C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702138 size=112
    let mut pc: u32 = 0x82702138;
    'dispatch: loop {
        match pc {
            0x82702138 => {
    //   block [0x82702138..0x827021A8)
	// 82702138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270213C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702144: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702148: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270214C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82702150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702154: 390B7C90  addi r8, r11, 0x7c90
	ctx.r[8].s64 = ctx.r[11].s64 + 31888;
	// 82702158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270215C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82702160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702164: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270216C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702170: 386AEF5C  addi r3, r10, -0x10a4
	ctx.r[3].s64 = ctx.r[10].s64 + -4260;
	// 82702174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270217C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270218C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702194: 4BD64C8D  bl 0x82466e20
	ctx.lr = 0x82702198;
	sub_82466E20(ctx, base);
	// 82702198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827021A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827021A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827021A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827021A8 size=108
    let mut pc: u32 = 0x827021A8;
    'dispatch: loop {
        match pc {
            0x827021A8 => {
    //   block [0x827021A8..0x82702214)
	// 827021A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827021AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827021B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827021B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827021B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827021BC: 38EB7D20  addi r7, r11, 0x7d20
	ctx.r[7].s64 = ctx.r[11].s64 + 32032;
	// 827021C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 827021C4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 827021C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827021CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827021D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827021D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827021D8: 386AEF8C  addi r3, r10, -0x1074
	ctx.r[3].s64 = ctx.r[10].s64 + -4212;
	// 827021DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827021E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827021E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827021E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827021EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827021F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827021F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827021F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827021FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702200: 4BD64C21  bl 0x82466e20
	ctx.lr = 0x82702204;
	sub_82466E20(ctx, base);
	// 82702204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270220C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702218 size=108
    let mut pc: u32 = 0x82702218;
    'dispatch: loop {
        match pc {
            0x82702218 => {
    //   block [0x82702218..0x82702284)
	// 82702218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702224: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270222C: 38EB7D68  addi r7, r11, 0x7d68
	ctx.r[7].s64 = ctx.r[11].s64 + 32104;
	// 82702230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82702234: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82702238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270223C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702248: 386AEFBC  addi r3, r10, -0x1044
	ctx.r[3].s64 = ctx.r[10].s64 + -4164;
	// 8270224C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270225C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270226C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702270: 4BD64BB1  bl 0x82466e20
	ctx.lr = 0x82702274;
	sub_82466E20(ctx, base);
	// 82702274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270227C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702288 size=108
    let mut pc: u32 = 0x82702288;
    'dispatch: loop {
        match pc {
            0x82702288 => {
    //   block [0x82702288..0x827022F4)
	// 82702288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702294: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270229C: 38EB7D98  addi r7, r11, 0x7d98
	ctx.r[7].s64 = ctx.r[11].s64 + 32152;
	// 827022A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827022A4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 827022A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827022AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827022B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827022B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827022B8: 386AEFEC  addi r3, r10, -0x1014
	ctx.r[3].s64 = ctx.r[10].s64 + -4116;
	// 827022BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827022C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827022C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827022C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827022CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827022D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827022D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827022D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827022DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827022E0: 4BD64B41  bl 0x82466e20
	ctx.lr = 0x827022E4;
	sub_82466E20(ctx, base);
	// 827022E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827022E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827022EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827022F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827022F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827022F8 size=112
    let mut pc: u32 = 0x827022F8;
    'dispatch: loop {
        match pc {
            0x827022F8 => {
    //   block [0x827022F8..0x82702368)
	// 827022F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827022FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702308: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270230C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702314: 390B7DC8  addi r8, r11, 0x7dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 32200;
	// 82702318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270231C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82702320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702324: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270232C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702330: 386AF01C  addi r3, r10, -0xfe4
	ctx.r[3].s64 = ctx.r[10].s64 + -4068;
	// 82702334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270233C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270234C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702354: 4BD64ACD  bl 0x82466e20
	ctx.lr = 0x82702358;
	sub_82466E20(ctx, base);
	// 82702358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270235C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702368 size=112
    let mut pc: u32 = 0x82702368;
    'dispatch: loop {
        match pc {
            0x82702368 => {
    //   block [0x82702368..0x827023D8)
	// 82702368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702374: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702378: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 8270237C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702384: 390B7DF8  addi r8, r11, 0x7df8
	ctx.r[8].s64 = ctx.r[11].s64 + 32248;
	// 82702388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270238C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82702390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702394: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270239C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827023A0: 386AF04C  addi r3, r10, -0xfb4
	ctx.r[3].s64 = ctx.r[10].s64 + -4020;
	// 827023A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827023A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827023AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827023B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827023B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827023B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827023BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827023C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827023C4: 4BD64A5D  bl 0x82466e20
	ctx.lr = 0x827023C8;
	sub_82466E20(ctx, base);
	// 827023C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827023CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827023D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827023D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827023D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827023D8 size=112
    let mut pc: u32 = 0x827023D8;
    'dispatch: loop {
        match pc {
            0x827023D8 => {
    //   block [0x827023D8..0x82702448)
	// 827023D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827023DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827023E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827023E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827023E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827023EC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827023F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827023F4: 390B7E10  addi r8, r11, 0x7e10
	ctx.r[8].s64 = ctx.r[11].s64 + 32272;
	// 827023F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827023FC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82702400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702404: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270240C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702410: 386AF07C  addi r3, r10, -0xf84
	ctx.r[3].s64 = ctx.r[10].s64 + -3972;
	// 82702414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270241C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270242C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702434: 4BD649ED  bl 0x82466e20
	ctx.lr = 0x82702438;
	sub_82466E20(ctx, base);
	// 82702438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270243C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702448 size=108
    let mut pc: u32 = 0x82702448;
    'dispatch: loop {
        match pc {
            0x82702448 => {
    //   block [0x82702448..0x827024B4)
	// 82702448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270244C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702454: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270245C: 38EB7E28  addi r7, r11, 0x7e28
	ctx.r[7].s64 = ctx.r[11].s64 + 32296;
	// 82702460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82702464: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82702468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270246C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702478: 386AF0AC  addi r3, r10, -0xf54
	ctx.r[3].s64 = ctx.r[10].s64 + -3924;
	// 8270247C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270248C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270249C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827024A0: 4BD64981  bl 0x82466e20
	ctx.lr = 0x827024A4;
	sub_82466E20(ctx, base);
	// 827024A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827024A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827024AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827024B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827024B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827024B8 size=112
    let mut pc: u32 = 0x827024B8;
    'dispatch: loop {
        match pc {
            0x827024B8 => {
    //   block [0x827024B8..0x82702528)
	// 827024B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827024BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827024C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827024C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827024C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827024CC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827024D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827024D4: 390B7E58  addi r8, r11, 0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + 32344;
	// 827024D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827024DC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 827024E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827024E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827024E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827024EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827024F0: 386AF0DC  addi r3, r10, -0xf24
	ctx.r[3].s64 = ctx.r[10].s64 + -3876;
	// 827024F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827024F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827024FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270250C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702514: 4BD6490D  bl 0x82466e20
	ctx.lr = 0x82702518;
	sub_82466E20(ctx, base);
	// 82702518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270251C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702528 size=108
    let mut pc: u32 = 0x82702528;
    'dispatch: loop {
        match pc {
            0x82702528 => {
    //   block [0x82702528..0x82702594)
	// 82702528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702534: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 82702538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270253C: 38EB7E70  addi r7, r11, 0x7e70
	ctx.r[7].s64 = ctx.r[11].s64 + 32368;
	// 82702540: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82702544: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82702548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270254C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702558: 386AF10C  addi r3, r10, -0xef4
	ctx.r[3].s64 = ctx.r[10].s64 + -3828;
	// 8270255C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270256C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270257C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702580: 4BD648A1  bl 0x82466e20
	ctx.lr = 0x82702584;
	sub_82466E20(ctx, base);
	// 82702584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270258C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702598 size=112
    let mut pc: u32 = 0x82702598;
    'dispatch: loop {
        match pc {
            0x82702598 => {
    //   block [0x82702598..0x82702608)
	// 82702598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827025A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827025A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827025A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 827025AC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827025B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827025B4: 390B7F60  addi r8, r11, 0x7f60
	ctx.r[8].s64 = ctx.r[11].s64 + 32608;
	// 827025B8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 827025BC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 827025C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827025C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827025C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827025CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827025D0: 386AF13C  addi r3, r10, -0xec4
	ctx.r[3].s64 = ctx.r[10].s64 + -3780;
	// 827025D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827025D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827025DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827025E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827025E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827025E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827025EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827025F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827025F4: 4BD6482D  bl 0x82466e20
	ctx.lr = 0x827025F8;
	sub_82466E20(ctx, base);
	// 827025F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827025FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702608 size=108
    let mut pc: u32 = 0x82702608;
    'dispatch: loop {
        match pc {
            0x82702608 => {
    //   block [0x82702608..0x82702674)
	// 82702608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702614: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270261C: 38EB8110  addi r7, r11, -0x7ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -32496;
	// 82702620: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82702624: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82702628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270262C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702638: 386AF16C  addi r3, r10, -0xe94
	ctx.r[3].s64 = ctx.r[10].s64 + -3732;
	// 8270263C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270265C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702660: 4BD647C1  bl 0x82466e20
	ctx.lr = 0x82702664;
	sub_82466E20(ctx, base);
	// 82702664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270266C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702678 size=112
    let mut pc: u32 = 0x82702678;
    'dispatch: loop {
        match pc {
            0x82702678 => {
    //   block [0x82702678..0x827026E8)
	// 82702678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270267C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702684: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702688: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270268C: 38AADD8C  addi r5, r10, -0x2274
	ctx.r[5].s64 = ctx.r[10].s64 + -8820;
	// 82702690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702694: 390B82A8  addi r8, r11, -0x7d58
	ctx.r[8].s64 = ctx.r[11].s64 + -32088;
	// 82702698: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8270269C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 827026A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827026A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827026A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827026AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827026B0: 386AF19C  addi r3, r10, -0xe64
	ctx.r[3].s64 = ctx.r[10].s64 + -3684;
	// 827026B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827026B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827026BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827026C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827026C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827026C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827026CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827026D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827026D4: 4BD6474D  bl 0x82466e20
	ctx.lr = 0x827026D8;
	sub_82466E20(ctx, base);
	// 827026D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827026DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827026E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827026E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827026E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827026E8 size=100
    let mut pc: u32 = 0x827026E8;
    'dispatch: loop {
        match pc {
            0x827026E8 => {
    //   block [0x827026E8..0x8270274C)
	// 827026E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827026EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827026F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827026F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827026F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827026FC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702708: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8270270C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270271C: 386AF1CC  addi r3, r10, -0xe34
	ctx.r[3].s64 = ctx.r[10].s64 + -3636;
	// 82702720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270272C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702738: 4BD646E9  bl 0x82466e20
	ctx.lr = 0x8270273C;
	sub_82466E20(ctx, base);
	// 8270273C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702750 size=112
    let mut pc: u32 = 0x82702750;
    'dispatch: loop {
        match pc {
            0x82702750 => {
    //   block [0x82702750..0x827027C0)
	// 82702750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270275C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702760: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702764: 38AAF1CC  addi r5, r10, -0xe34
	ctx.r[5].s64 = ctx.r[10].s64 + -3636;
	// 82702768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270276C: 390B8500  addi r8, r11, -0x7b00
	ctx.r[8].s64 = ctx.r[11].s64 + -31488;
	// 82702770: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82702774: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82702778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270277C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702788: 386AF1FC  addi r3, r10, -0xe04
	ctx.r[3].s64 = ctx.r[10].s64 + -3588;
	// 8270278C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270279C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827027A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827027A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827027A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827027AC: 4BD64675  bl 0x82466e20
	ctx.lr = 0x827027B0;
	sub_82466E20(ctx, base);
	// 827027B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827027B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827027B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827027BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827027C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827027C0 size=100
    let mut pc: u32 = 0x827027C0;
    'dispatch: loop {
        match pc {
            0x827027C0 => {
    //   block [0x827027C0..0x82702824)
	// 827027C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827027C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827027C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827027CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827027D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827027D4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827027D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827027DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827027E0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 827027E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827027E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827027EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827027F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827027F4: 386AF22C  addi r3, r10, -0xdd4
	ctx.r[3].s64 = ctx.r[10].s64 + -3540;
	// 827027F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827027FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82702804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270280C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702810: 4BD64611  bl 0x82466e20
	ctx.lr = 0x82702814;
	sub_82466E20(ctx, base);
	// 82702814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270281C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702828 size=108
    let mut pc: u32 = 0x82702828;
    'dispatch: loop {
        match pc {
            0x82702828 => {
    //   block [0x82702828..0x82702894)
	// 82702828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702834: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270283C: 38EB8578  addi r7, r11, -0x7a88
	ctx.r[7].s64 = ctx.r[11].s64 + -31368;
	// 82702840: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702844: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82702848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270284C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702858: 386AF25C  addi r3, r10, -0xda4
	ctx.r[3].s64 = ctx.r[10].s64 + -3492;
	// 8270285C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270286C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270287C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702880: 4BD645A1  bl 0x82466e20
	ctx.lr = 0x82702884;
	sub_82466E20(ctx, base);
	// 82702884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270288C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702898 size=112
    let mut pc: u32 = 0x82702898;
    'dispatch: loop {
        match pc {
            0x82702898 => {
    //   block [0x82702898..0x82702908)
	// 82702898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270289C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827028A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827028A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827028A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827028AC: 38AAF22C  addi r5, r10, -0xdd4
	ctx.r[5].s64 = ctx.r[10].s64 + -3540;
	// 827028B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827028B4: 390B85C0  addi r8, r11, -0x7a40
	ctx.r[8].s64 = ctx.r[11].s64 + -31296;
	// 827028B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827028BC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 827028C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827028C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827028C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827028CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827028D0: 386AF28C  addi r3, r10, -0xd74
	ctx.r[3].s64 = ctx.r[10].s64 + -3444;
	// 827028D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827028D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827028DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827028E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827028E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827028E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827028EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827028F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827028F4: 4BD6452D  bl 0x82466e20
	ctx.lr = 0x827028F8;
	sub_82466E20(ctx, base);
	// 827028F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827028FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702908 size=100
    let mut pc: u32 = 0x82702908;
    'dispatch: loop {
        match pc {
            0x82702908 => {
    //   block [0x82702908..0x8270296C)
	// 82702908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702914: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270291C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702928: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8270292C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270293C: 386AF2BC  addi r3, r10, -0xd44
	ctx.r[3].s64 = ctx.r[10].s64 + -3396;
	// 82702940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702948: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270294C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702958: 4BD644C9  bl 0x82466e20
	ctx.lr = 0x8270295C;
	sub_82466E20(ctx, base);
	// 8270295C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702970 size=100
    let mut pc: u32 = 0x82702970;
    'dispatch: loop {
        match pc {
            0x82702970 => {
    //   block [0x82702970..0x827029D4)
	// 82702970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270297C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702984: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270298C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702990: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82702994: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270299C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827029A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827029A4: 386AF2EC  addi r3, r10, -0xd14
	ctx.r[3].s64 = ctx.r[10].s64 + -3348;
	// 827029A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827029AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827029B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827029B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827029B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827029BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827029C0: 4BD64461  bl 0x82466e20
	ctx.lr = 0x827029C4;
	sub_82466E20(ctx, base);
	// 827029C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827029C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827029CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827029D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827029D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827029D8 size=112
    let mut pc: u32 = 0x827029D8;
    'dispatch: loop {
        match pc {
            0x827029D8 => {
    //   block [0x827029D8..0x82702A48)
	// 827029D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827029DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827029E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827029E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827029E8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827029EC: 38AAF2BC  addi r5, r10, -0xd44
	ctx.r[5].s64 = ctx.r[10].s64 + -3396;
	// 827029F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827029F4: 390B85F0  addi r8, r11, -0x7a10
	ctx.r[8].s64 = ctx.r[11].s64 + -31248;
	// 827029F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 827029FC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82702A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702A04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702A10: 386AF31C  addi r3, r10, -0xce4
	ctx.r[3].s64 = ctx.r[10].s64 + -3300;
	// 82702A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702A34: 4BD643ED  bl 0x82466e20
	ctx.lr = 0x82702A38;
	sub_82466E20(ctx, base);
	// 82702A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702A48 size=112
    let mut pc: u32 = 0x82702A48;
    'dispatch: loop {
        match pc {
            0x82702A48 => {
    //   block [0x82702A48..0x82702AB8)
	// 82702A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702A54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702A58: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702A5C: 38AAF2EC  addi r5, r10, -0xd14
	ctx.r[5].s64 = ctx.r[10].s64 + -3348;
	// 82702A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702A64: 390B8650  addi r8, r11, -0x79b0
	ctx.r[8].s64 = ctx.r[11].s64 + -31152;
	// 82702A68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82702A6C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82702A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702A80: 386AF34C  addi r3, r10, -0xcb4
	ctx.r[3].s64 = ctx.r[10].s64 + -3252;
	// 82702A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702AA4: 4BD6437D  bl 0x82466e20
	ctx.lr = 0x82702AA8;
	sub_82466E20(ctx, base);
	// 82702AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702AB8 size=100
    let mut pc: u32 = 0x82702AB8;
    'dispatch: loop {
        match pc {
            0x82702AB8 => {
    //   block [0x82702AB8..0x82702B1C)
	// 82702AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702AC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702ACC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702AD8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 82702ADC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702AEC: 386AF37C  addi r3, r10, -0xc84
	ctx.r[3].s64 = ctx.r[10].s64 + -3204;
	// 82702AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702AF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82702AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702B08: 4BD64319  bl 0x82466e20
	ctx.lr = 0x82702B0C;
	sub_82466E20(ctx, base);
	// 82702B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702B20 size=112
    let mut pc: u32 = 0x82702B20;
    'dispatch: loop {
        match pc {
            0x82702B20 => {
    //   block [0x82702B20..0x82702B90)
	// 82702B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702B2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702B30: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702B34: 38AAF37C  addi r5, r10, -0xc84
	ctx.r[5].s64 = ctx.r[10].s64 + -3204;
	// 82702B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702B3C: 390B86B0  addi r8, r11, -0x7950
	ctx.r[8].s64 = ctx.r[11].s64 + -31056;
	// 82702B40: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82702B44: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82702B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702B4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702B58: 386AF3AC  addi r3, r10, -0xc54
	ctx.r[3].s64 = ctx.r[10].s64 + -3156;
	// 82702B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702B7C: 4BD642A5  bl 0x82466e20
	ctx.lr = 0x82702B80;
	sub_82466E20(ctx, base);
	// 82702B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702B90 size=108
    let mut pc: u32 = 0x82702B90;
    'dispatch: loop {
        match pc {
            0x82702B90 => {
    //   block [0x82702B90..0x82702BFC)
	// 82702B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702B9C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702BA4: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 82702BA8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82702BAC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82702BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702BC0: 386AF3DC  addi r3, r10, -0xc24
	ctx.r[3].s64 = ctx.r[10].s64 + -3108;
	// 82702BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702BE8: 4BD64239  bl 0x82466e20
	ctx.lr = 0x82702BEC;
	sub_82466E20(ctx, base);
	// 82702BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702C00 size=108
    let mut pc: u32 = 0x82702C00;
    'dispatch: loop {
        match pc {
            0x82702C00 => {
    //   block [0x82702C00..0x82702C6C)
	// 82702C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702C0C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702C14: 38EB8890  addi r7, r11, -0x7770
	ctx.r[7].s64 = ctx.r[11].s64 + -30576;
	// 82702C18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702C1C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82702C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702C30: 386AF40C  addi r3, r10, -0xbf4
	ctx.r[3].s64 = ctx.r[10].s64 + -3060;
	// 82702C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702C58: 4BD641C9  bl 0x82466e20
	ctx.lr = 0x82702C5C;
	sub_82466E20(ctx, base);
	// 82702C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702C70 size=108
    let mut pc: u32 = 0x82702C70;
    'dispatch: loop {
        match pc {
            0x82702C70 => {
    //   block [0x82702C70..0x82702CDC)
	// 82702C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702C7C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702C80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702C84: 38EB88D8  addi r7, r11, -0x7728
	ctx.r[7].s64 = ctx.r[11].s64 + -30504;
	// 82702C88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82702C8C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82702C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702C98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702CA0: 386AF43C  addi r3, r10, -0xbc4
	ctx.r[3].s64 = ctx.r[10].s64 + -3012;
	// 82702CA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702CA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702CC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702CC8: 4BD64159  bl 0x82466e20
	ctx.lr = 0x82702CCC;
	sub_82466E20(ctx, base);
	// 82702CCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702CE0 size=108
    let mut pc: u32 = 0x82702CE0;
    'dispatch: loop {
        match pc {
            0x82702CE0 => {
    //   block [0x82702CE0..0x82702D4C)
	// 82702CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702CEC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702CF4: 38EB89B0  addi r7, r11, -0x7650
	ctx.r[7].s64 = ctx.r[11].s64 + -30288;
	// 82702CF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82702CFC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82702D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702D04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702D08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702D10: 386AF46C  addi r3, r10, -0xb94
	ctx.r[3].s64 = ctx.r[10].s64 + -2964;
	// 82702D14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702D38: 4BD640E9  bl 0x82466e20
	ctx.lr = 0x82702D3C;
	sub_82466E20(ctx, base);
	// 82702D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702D50 size=100
    let mut pc: u32 = 0x82702D50;
    'dispatch: loop {
        match pc {
            0x82702D50 => {
    //   block [0x82702D50..0x82702DB4)
	// 82702D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702D5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702D64: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702D70: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82702D74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702D84: 386AF49C  addi r3, r10, -0xb64
	ctx.r[3].s64 = ctx.r[10].s64 + -2916;
	// 82702D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702D90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82702D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82702D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702DA0: 4BD64081  bl 0x82466e20
	ctx.lr = 0x82702DA4;
	sub_82466E20(ctx, base);
	// 82702DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702DB8 size=112
    let mut pc: u32 = 0x82702DB8;
    'dispatch: loop {
        match pc {
            0x82702DB8 => {
    //   block [0x82702DB8..0x82702E28)
	// 82702DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702DC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702DC8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702DCC: 38AAF49C  addi r5, r10, -0xb64
	ctx.r[5].s64 = ctx.r[10].s64 + -2916;
	// 82702DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702DD4: 390B89C8  addi r8, r11, -0x7638
	ctx.r[8].s64 = ctx.r[11].s64 + -30264;
	// 82702DD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82702DDC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82702DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702DF0: 386AF4CC  addi r3, r10, -0xb34
	ctx.r[3].s64 = ctx.r[10].s64 + -2868;
	// 82702DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702E14: 4BD6400D  bl 0x82466e20
	ctx.lr = 0x82702E18;
	sub_82466E20(ctx, base);
	// 82702E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702E28 size=108
    let mut pc: u32 = 0x82702E28;
    'dispatch: loop {
        match pc {
            0x82702E28 => {
    //   block [0x82702E28..0x82702E94)
	// 82702E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702E34: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702E3C: 38EB8A10  addi r7, r11, -0x75f0
	ctx.r[7].s64 = ctx.r[11].s64 + -30192;
	// 82702E40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702E44: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82702E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702E4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702E58: 386AF4FC  addi r3, r10, -0xb04
	ctx.r[3].s64 = ctx.r[10].s64 + -2820;
	// 82702E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702E80: 4BD63FA1  bl 0x82466e20
	ctx.lr = 0x82702E84;
	sub_82466E20(ctx, base);
	// 82702E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702E98 size=112
    let mut pc: u32 = 0x82702E98;
    'dispatch: loop {
        match pc {
            0x82702E98 => {
    //   block [0x82702E98..0x82702F08)
	// 82702E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702EA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702EA8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702EAC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82702EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702EB4: 390B8A58  addi r8, r11, -0x75a8
	ctx.r[8].s64 = ctx.r[11].s64 + -30120;
	// 82702EB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82702EBC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82702EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702ED0: 386AF52C  addi r3, r10, -0xad4
	ctx.r[3].s64 = ctx.r[10].s64 + -2772;
	// 82702ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702EF4: 4BD63F2D  bl 0x82466e20
	ctx.lr = 0x82702EF8;
	sub_82466E20(ctx, base);
	// 82702EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702F08 size=108
    let mut pc: u32 = 0x82702F08;
    'dispatch: loop {
        match pc {
            0x82702F08 => {
    //   block [0x82702F08..0x82702F74)
	// 82702F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702F14: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702F1C: 38EB8A70  addi r7, r11, -0x7590
	ctx.r[7].s64 = ctx.r[11].s64 + -30096;
	// 82702F20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82702F24: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82702F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702F2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82702F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702F38: 386AF55C  addi r3, r10, -0xaa4
	ctx.r[3].s64 = ctx.r[10].s64 + -2724;
	// 82702F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82702F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82702F60: 4BD63EC1  bl 0x82466e20
	ctx.lr = 0x82702F64;
	sub_82466E20(ctx, base);
	// 82702F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702F78 size=112
    let mut pc: u32 = 0x82702F78;
    'dispatch: loop {
        match pc {
            0x82702F78 => {
    //   block [0x82702F78..0x82702FE8)
	// 82702F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702F84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702F88: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82702F8C: 38AAF52C  addi r5, r10, -0xad4
	ctx.r[5].s64 = ctx.r[10].s64 + -2772;
	// 82702F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82702F94: 390B8AB8  addi r8, r11, -0x7548
	ctx.r[8].s64 = ctx.r[11].s64 + -30024;
	// 82702F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82702F9C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82702FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82702FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82702FB0: 386AF58C  addi r3, r10, -0xa74
	ctx.r[3].s64 = ctx.r[10].s64 + -2676;
	// 82702FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82702FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82702FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82702FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82702FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82702FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82702FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82702FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82702FD4: 4BD63E4D  bl 0x82466e20
	ctx.lr = 0x82702FD8;
	sub_82466E20(ctx, base);
	// 82702FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82702FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82702FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82702FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82702FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82702FE8 size=100
    let mut pc: u32 = 0x82702FE8;
    'dispatch: loop {
        match pc {
            0x82702FE8 => {
    //   block [0x82702FE8..0x8270304C)
	// 82702FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82702FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82702FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82702FF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82702FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82702FFC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703008: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8270300C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270301C: 386AF5BC  addi r3, r10, -0xa44
	ctx.r[3].s64 = ctx.r[10].s64 + -2628;
	// 82703020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270302C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703038: 4BD63DE9  bl 0x82466e20
	ctx.lr = 0x8270303C;
	sub_82466E20(ctx, base);
	// 8270303C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703050 size=112
    let mut pc: u32 = 0x82703050;
    'dispatch: loop {
        match pc {
            0x82703050 => {
    //   block [0x82703050..0x827030C0)
	// 82703050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270305C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703060: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703064: 38AAF5BC  addi r5, r10, -0xa44
	ctx.r[5].s64 = ctx.r[10].s64 + -2628;
	// 82703068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270306C: 390B8AD0  addi r8, r11, -0x7530
	ctx.r[8].s64 = ctx.r[11].s64 + -30000;
	// 82703070: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82703074: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82703078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270307C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703088: 386AF5EC  addi r3, r10, -0xa14
	ctx.r[3].s64 = ctx.r[10].s64 + -2580;
	// 8270308C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270309C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827030A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827030A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827030A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827030AC: 4BD63D75  bl 0x82466e20
	ctx.lr = 0x827030B0;
	sub_82466E20(ctx, base);
	// 827030B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827030B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827030B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827030BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827030C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827030C0 size=108
    let mut pc: u32 = 0x827030C0;
    'dispatch: loop {
        match pc {
            0x827030C0 => {
    //   block [0x827030C0..0x8270312C)
	// 827030C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827030C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827030C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827030CC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827030D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827030D4: 38EB8B78  addi r7, r11, -0x7488
	ctx.r[7].s64 = ctx.r[11].s64 + -29832;
	// 827030D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827030DC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 827030E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827030E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827030E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827030EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827030F0: 386AF61C  addi r3, r10, -0x9e4
	ctx.r[3].s64 = ctx.r[10].s64 + -2532;
	// 827030F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827030F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827030FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270310C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82703118: 4BD63D09  bl 0x82466e20
	ctx.lr = 0x8270311C;
	sub_82466E20(ctx, base);
	// 8270311C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703130 size=112
    let mut pc: u32 = 0x82703130;
    'dispatch: loop {
        match pc {
            0x82703130 => {
    //   block [0x82703130..0x827031A0)
	// 82703130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270313C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703140: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703144: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270314C: 390B8BA8  addi r8, r11, -0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + -29784;
	// 82703150: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703154: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82703158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270315C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703168: 386AF64C  addi r3, r10, -0x9b4
	ctx.r[3].s64 = ctx.r[10].s64 + -2484;
	// 8270316C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270317C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270318C: 4BD63C95  bl 0x82466e20
	ctx.lr = 0x82703190;
	sub_82466E20(ctx, base);
	// 82703190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827031A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827031A0 size=112
    let mut pc: u32 = 0x827031A0;
    'dispatch: loop {
        match pc {
            0x827031A0 => {
    //   block [0x827031A0..0x82703210)
	// 827031A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827031A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827031A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827031AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827031B0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827031B4: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 827031B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827031BC: 390B8BF0  addi r8, r11, -0x7410
	ctx.r[8].s64 = ctx.r[11].s64 + -29712;
	// 827031C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827031C4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 827031C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827031CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827031D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827031D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827031D8: 386AF67C  addi r3, r10, -0x984
	ctx.r[3].s64 = ctx.r[10].s64 + -2436;
	// 827031DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827031E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827031E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827031E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827031EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827031F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827031F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827031F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827031FC: 4BD63C25  bl 0x82466e20
	ctx.lr = 0x82703200;
	sub_82466E20(ctx, base);
	// 82703200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703210 size=100
    let mut pc: u32 = 0x82703210;
    'dispatch: loop {
        match pc {
            0x82703210 => {
    //   block [0x82703210..0x82703274)
	// 82703210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270321C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703224: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270322C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703230: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82703234: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703244: 386AF6AC  addi r3, r10, -0x954
	ctx.r[3].s64 = ctx.r[10].s64 + -2388;
	// 82703248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270324C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82703254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270325C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703260: 4BD63BC1  bl 0x82466e20
	ctx.lr = 0x82703264;
	sub_82466E20(ctx, base);
	// 82703264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270326C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703278 size=112
    let mut pc: u32 = 0x82703278;
    'dispatch: loop {
        match pc {
            0x82703278 => {
    //   block [0x82703278..0x827032E8)
	// 82703278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703288: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270328C: 38AAF6AC  addi r5, r10, -0x954
	ctx.r[5].s64 = ctx.r[10].s64 + -2388;
	// 82703290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703294: 390B8C38  addi r8, r11, -0x73c8
	ctx.r[8].s64 = ctx.r[11].s64 + -29640;
	// 82703298: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270329C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 827032A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827032A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827032A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827032AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827032B0: 386AF6DC  addi r3, r10, -0x924
	ctx.r[3].s64 = ctx.r[10].s64 + -2340;
	// 827032B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827032B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827032BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827032C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827032C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827032C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827032CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827032D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827032D4: 4BD63B4D  bl 0x82466e20
	ctx.lr = 0x827032D8;
	sub_82466E20(ctx, base);
	// 827032D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827032DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827032E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827032E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827032E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827032E8 size=112
    let mut pc: u32 = 0x827032E8;
    'dispatch: loop {
        match pc {
            0x827032E8 => {
    //   block [0x827032E8..0x82703358)
	// 827032E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827032EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827032F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827032F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827032F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827032FC: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703304: 390B8C80  addi r8, r11, -0x7380
	ctx.r[8].s64 = ctx.r[11].s64 + -29568;
	// 82703308: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270330C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82703310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703314: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270331C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703320: 386AF70C  addi r3, r10, -0x8f4
	ctx.r[3].s64 = ctx.r[10].s64 + -2292;
	// 82703324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270332C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270333C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703344: 4BD63ADD  bl 0x82466e20
	ctx.lr = 0x82703348;
	sub_82466E20(ctx, base);
	// 82703348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270334C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703358 size=112
    let mut pc: u32 = 0x82703358;
    'dispatch: loop {
        match pc {
            0x82703358 => {
    //   block [0x82703358..0x827033C8)
	// 82703358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270335C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703368: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270336C: 38AAC37C  addi r5, r10, -0x3c84
	ctx.r[5].s64 = ctx.r[10].s64 + -15492;
	// 82703370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82703374: 390B8C98  addi r8, r11, -0x7368
	ctx.r[8].s64 = ctx.r[11].s64 + -29544;
	// 82703378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270337C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82703380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270338C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703390: 386AF73C  addi r3, r10, -0x8c4
	ctx.r[3].s64 = ctx.r[10].s64 + -2244;
	// 82703394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827033A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827033A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827033A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827033AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 827033B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827033B4: 4BD63A6D  bl 0x82466e20
	ctx.lr = 0x827033B8;
	sub_82466E20(ctx, base);
	// 827033B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827033BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827033C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827033C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827033C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827033C8 size=112
    let mut pc: u32 = 0x827033C8;
    'dispatch: loop {
        match pc {
            0x827033C8 => {
    //   block [0x827033C8..0x82703438)
	// 827033C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827033CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827033D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827033D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827033D8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827033DC: 38AAF70C  addi r5, r10, -0x8f4
	ctx.r[5].s64 = ctx.r[10].s64 + -2292;
	// 827033E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 827033E4: 390B8CB0  addi r8, r11, -0x7350
	ctx.r[8].s64 = ctx.r[11].s64 + -29520;
	// 827033E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827033EC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 827033F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827033F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827033F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827033FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703400: 386AF76C  addi r3, r10, -0x894
	ctx.r[3].s64 = ctx.r[10].s64 + -2196;
	// 82703404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270341C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82703420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703424: 4BD639FD  bl 0x82466e20
	ctx.lr = 0x82703428;
	sub_82466E20(ctx, base);
	// 82703428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703438 size=72
    let mut pc: u32 = 0x82703438;
    'dispatch: loop {
        match pc {
            0x82703438 => {
    //   block [0x82703438..0x82703480)
	// 82703438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703444: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703448: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8270344C: 38CBB360  addi r6, r11, -0x4ca0
	ctx.r[6].s64 = ctx.r[11].s64 + -19616;
	// 82703450: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703454: 388BC378  addi r4, r11, -0x3c88
	ctx.r[4].s64 = ctx.r[11].s64 + -15496;
	// 82703458: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270345C: 386BF79C  addi r3, r11, -0x864
	ctx.r[3].s64 = ctx.r[11].s64 + -2148;
	// 82703460: 4BD78629  bl 0x8247ba88
	ctx.lr = 0x82703464;
	sub_8247BA88(ctx, base);
	// 82703464: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82703468: 386BCF60  addi r3, r11, -0x30a0
	ctx.r[3].s64 = ctx.r[11].s64 + -12448;
	// 8270346C: 4BE2F6CD  bl 0x82532b38
	ctx.lr = 0x82703470;
	sub_82532B38(ctx, base);
	// 82703470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82703474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703480 size=108
    let mut pc: u32 = 0x82703480;
    'dispatch: loop {
        match pc {
            0x82703480 => {
    //   block [0x82703480..0x827034EC)
	// 82703480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270348C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703494: 38EBCD94  addi r7, r11, -0x326c
	ctx.r[7].s64 = ctx.r[11].s64 + -12908;
	// 82703498: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270349C: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 827034A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827034A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827034A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827034AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827034B0: 386AF7B4  addi r3, r10, -0x84c
	ctx.r[3].s64 = ctx.r[10].s64 + -2124;
	// 827034B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827034B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827034BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827034C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827034C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827034C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827034CC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 827034D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827034D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827034D8: 4BD63949  bl 0x82466e20
	ctx.lr = 0x827034DC;
	sub_82466E20(ctx, base);
	// 827034DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827034E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827034E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827034E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827034F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827034F0 size=112
    let mut pc: u32 = 0x827034F0;
    'dispatch: loop {
        match pc {
            0x827034F0 => {
    //   block [0x827034F0..0x82703560)
	// 827034F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827034F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827034F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827034FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703500: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703504: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82703508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270350C: 390BCDC4  addi r8, r11, -0x323c
	ctx.r[8].s64 = ctx.r[11].s64 + -12860;
	// 82703510: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82703514: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 82703518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270351C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703528: 386AF7E4  addi r3, r10, -0x81c
	ctx.r[3].s64 = ctx.r[10].s64 + -2076;
	// 8270352C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270353C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703544: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82703548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270354C: 4BD638D5  bl 0x82466e20
	ctx.lr = 0x82703550;
	sub_82466E20(ctx, base);
	// 82703550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270355C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82703560 size=24
    let mut pc: u32 = 0x82703560;
    'dispatch: loop {
        match pc {
            0x82703560 => {
    //   block [0x82703560..0x82703578)
	// 82703560: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703564: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82703568: 394A9F1C  addi r10, r10, -0x60e4
	ctx.r[10].s64 = ctx.r[10].s64 + -24804;
	// 8270356C: 816B9D30  lwz r11, -0x62d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25296 as u32) ) } as u64;
	// 82703570: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82703574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703578 size=112
    let mut pc: u32 = 0x82703578;
    'dispatch: loop {
        match pc {
            0x82703578 => {
    //   block [0x82703578..0x827035E8)
	// 82703578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270357C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703584: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82703588: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270358C: 392ACF18  addi r9, r10, -0x30e8
	ctx.r[9].s64 = ctx.r[10].s64 + -12520;
	// 82703590: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703594: 390B9F1C  addi r8, r11, -0x60e4
	ctx.r[8].s64 = ctx.r[11].s64 + -24804;
	// 82703598: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8270359C: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 827035A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827035A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827035A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827035AC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 827035B0: 386AF814  addi r3, r10, -0x7ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2028;
	// 827035B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827035B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 827035BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827035C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827035C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827035C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827035CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827035D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827035D4: 4BD6384D  bl 0x82466e20
	ctx.lr = 0x827035D8;
	sub_82466E20(ctx, base);
	// 827035D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827035DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827035E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827035E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827035E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827035E8 size=112
    let mut pc: u32 = 0x827035E8;
    'dispatch: loop {
        match pc {
            0x827035E8 => {
    //   block [0x827035E8..0x82703658)
	// 827035E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827035EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827035F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827035F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827035F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827035FC: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703600: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703604: 390BCF40  addi r8, r11, -0x30c0
	ctx.r[8].s64 = ctx.r[11].s64 + -12480;
	// 82703608: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270360C: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 82703610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703614: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703620: 386AF844  addi r3, r10, -0x7bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1980;
	// 82703624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270362C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270363C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703644: 4BD637DD  bl 0x82466e20
	ctx.lr = 0x82703648;
	sub_82466E20(ctx, base);
	// 82703648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270364C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703658 size=108
    let mut pc: u32 = 0x82703658;
    'dispatch: loop {
        match pc {
            0x82703658 => {
    //   block [0x82703658..0x827036C4)
	// 82703658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703664: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270366C: 38EBCF70  addi r7, r11, -0x3090
	ctx.r[7].s64 = ctx.r[11].s64 + -12432;
	// 82703670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82703674: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 82703678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270367C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82703684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703688: 386AF874  addi r3, r10, -0x78c
	ctx.r[3].s64 = ctx.r[10].s64 + -1932;
	// 8270368C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82703690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270369C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827036A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827036A4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 827036A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827036AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827036B0: 4BD63771  bl 0x82466e20
	ctx.lr = 0x827036B4;
	sub_82466E20(ctx, base);
	// 827036B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827036B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827036BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827036C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827036C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827036C8 size=112
    let mut pc: u32 = 0x827036C8;
    'dispatch: loop {
        match pc {
            0x827036C8 => {
    //   block [0x827036C8..0x82703738)
	// 827036C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827036CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827036D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827036D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827036D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827036DC: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 827036E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827036E4: 390BCF88  addi r8, r11, -0x3078
	ctx.r[8].s64 = ctx.r[11].s64 + -12408;
	// 827036E8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 827036EC: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 827036F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827036F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827036F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827036FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703700: 386AF8A4  addi r3, r10, -0x75c
	ctx.r[3].s64 = ctx.r[10].s64 + -1884;
	// 82703704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270370C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270371C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82703720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703724: 4BD636FD  bl 0x82466e20
	ctx.lr = 0x82703728;
	sub_82466E20(ctx, base);
	// 82703728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270372C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703738 size=100
    let mut pc: u32 = 0x82703738;
    'dispatch: loop {
        match pc {
            0x82703738 => {
    //   block [0x82703738..0x8270379C)
	// 82703738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270373C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703744: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270374C: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703750: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703758: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 8270375C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270376C: 386AF8D4  addi r3, r10, -0x72c
	ctx.r[3].s64 = ctx.r[10].s64 + -1836;
	// 82703770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703778: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270377C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703780: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703784: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82703788: 4BD63699  bl 0x82466e20
	ctx.lr = 0x8270378C;
	sub_82466E20(ctx, base);
	// 8270378C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827037A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827037A0 size=112
    let mut pc: u32 = 0x827037A0;
    'dispatch: loop {
        match pc {
            0x827037A0 => {
    //   block [0x827037A0..0x82703810)
	// 827037A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827037A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827037A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827037AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827037B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827037B4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 827037B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827037BC: 390BD048  addi r8, r11, -0x2fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -12216;
	// 827037C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827037C4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 827037C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827037CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827037D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827037D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827037D8: 386AF904  addi r3, r10, -0x6fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1788;
	// 827037DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827037E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827037E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827037E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827037EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827037F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827037F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 827037F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827037FC: 4BD63625  bl 0x82466e20
	ctx.lr = 0x82703800;
	sub_82466E20(ctx, base);
	// 82703800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270380C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703810 size=112
    let mut pc: u32 = 0x82703810;
    'dispatch: loop {
        match pc {
            0x82703810 => {
    //   block [0x82703810..0x82703880)
	// 82703810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270381C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703820: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703824: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270382C: 390BD060  addi r8, r11, -0x2fa0
	ctx.r[8].s64 = ctx.r[11].s64 + -12192;
	// 82703830: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82703834: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 82703838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270383C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703848: 386AF934  addi r3, r10, -0x6cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1740;
	// 8270384C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270385C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703864: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 82703868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270386C: 4BD635B5  bl 0x82466e20
	ctx.lr = 0x82703870;
	sub_82466E20(ctx, base);
	// 82703870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270387C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703880 size=112
    let mut pc: u32 = 0x82703880;
    'dispatch: loop {
        match pc {
            0x82703880 => {
    //   block [0x82703880..0x827038F0)
	// 82703880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270388C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703890: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703894: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703898: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270389C: 390BD090  addi r8, r11, -0x2f70
	ctx.r[8].s64 = ctx.r[11].s64 + -12144;
	// 827038A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827038A4: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 827038A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827038AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827038B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827038B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827038B8: 386AF964  addi r3, r10, -0x69c
	ctx.r[3].s64 = ctx.r[10].s64 + -1692;
	// 827038BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827038C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827038C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827038C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827038CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827038D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827038D4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827038D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827038DC: 4BD63545  bl 0x82466e20
	ctx.lr = 0x827038E0;
	sub_82466E20(ctx, base);
	// 827038E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827038E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827038E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827038EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827038F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827038F0 size=112
    let mut pc: u32 = 0x827038F0;
    'dispatch: loop {
        match pc {
            0x827038F0 => {
    //   block [0x827038F0..0x82703960)
	// 827038F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827038F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827038F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827038FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703900: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703904: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270390C: 390BD0C0  addi r8, r11, -0x2f40
	ctx.r[8].s64 = ctx.r[11].s64 + -12096;
	// 82703910: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82703914: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 82703918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270391C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703928: 386AF994  addi r3, r10, -0x66c
	ctx.r[3].s64 = ctx.r[10].s64 + -1644;
	// 8270392C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270393C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703944: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82703948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270394C: 4BD634D5  bl 0x82466e20
	ctx.lr = 0x82703950;
	sub_82466E20(ctx, base);
	// 82703950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270395C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703960 size=112
    let mut pc: u32 = 0x82703960;
    'dispatch: loop {
        match pc {
            0x82703960 => {
    //   block [0x82703960..0x827039D0)
	// 82703960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270396C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703970: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703974: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270397C: 390BD0F0  addi r8, r11, -0x2f10
	ctx.r[8].s64 = ctx.r[11].s64 + -12048;
	// 82703980: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82703984: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 82703988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270398C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703998: 386AF9C4  addi r3, r10, -0x63c
	ctx.r[3].s64 = ctx.r[10].s64 + -1596;
	// 8270399C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827039A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827039A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827039A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827039AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827039B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827039B4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 827039B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827039BC: 4BD63465  bl 0x82466e20
	ctx.lr = 0x827039C0;
	sub_82466E20(ctx, base);
	// 827039C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827039C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827039C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827039CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827039D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827039D0 size=112
    let mut pc: u32 = 0x827039D0;
    'dispatch: loop {
        match pc {
            0x827039D0 => {
    //   block [0x827039D0..0x82703A40)
	// 827039D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827039D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827039D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827039DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827039E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827039E4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 827039E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827039EC: 390BD108  addi r8, r11, -0x2ef8
	ctx.r[8].s64 = ctx.r[11].s64 + -12024;
	// 827039F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827039F4: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 827039F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827039FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703A08: 386AF9F4  addi r3, r10, -0x60c
	ctx.r[3].s64 = ctx.r[10].s64 + -1548;
	// 82703A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703A24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82703A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703A2C: 4BD633F5  bl 0x82466e20
	ctx.lr = 0x82703A30;
	sub_82466E20(ctx, base);
	// 82703A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703A40 size=112
    let mut pc: u32 = 0x82703A40;
    'dispatch: loop {
        match pc {
            0x82703A40 => {
    //   block [0x82703A40..0x82703AB0)
	// 82703A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703A4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703A50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703A54: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703A58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703A5C: 390BD120  addi r8, r11, -0x2ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -12000;
	// 82703A60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703A64: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 82703A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703A6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703A78: 386AFA24  addi r3, r10, -0x5dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1500;
	// 82703A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703A94: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703A9C: 4BD63385  bl 0x82466e20
	ctx.lr = 0x82703AA0;
	sub_82466E20(ctx, base);
	// 82703AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703AB0 size=112
    let mut pc: u32 = 0x82703AB0;
    'dispatch: loop {
        match pc {
            0x82703AB0 => {
    //   block [0x82703AB0..0x82703B20)
	// 82703AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703ABC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703AC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703AC4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703AC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703ACC: 390BD168  addi r8, r11, -0x2e98
	ctx.r[8].s64 = ctx.r[11].s64 + -11928;
	// 82703AD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703AD4: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 82703AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703ADC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703AE8: 386AFA54  addi r3, r10, -0x5ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1452;
	// 82703AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703B04: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703B0C: 4BD63315  bl 0x82466e20
	ctx.lr = 0x82703B10;
	sub_82466E20(ctx, base);
	// 82703B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703B20 size=112
    let mut pc: u32 = 0x82703B20;
    'dispatch: loop {
        match pc {
            0x82703B20 => {
    //   block [0x82703B20..0x82703B90)
	// 82703B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703B2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703B30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703B34: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703B3C: 390BD1B0  addi r8, r11, -0x2e50
	ctx.r[8].s64 = ctx.r[11].s64 + -11856;
	// 82703B40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82703B44: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 82703B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703B4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703B58: 386AFA84  addi r3, r10, -0x57c
	ctx.r[3].s64 = ctx.r[10].s64 + -1404;
	// 82703B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703B74: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82703B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703B7C: 4BD632A5  bl 0x82466e20
	ctx.lr = 0x82703B80;
	sub_82466E20(ctx, base);
	// 82703B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703B90 size=112
    let mut pc: u32 = 0x82703B90;
    'dispatch: loop {
        match pc {
            0x82703B90 => {
    //   block [0x82703B90..0x82703C00)
	// 82703B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703BA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703BA4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703BAC: 390BD1C8  addi r8, r11, -0x2e38
	ctx.r[8].s64 = ctx.r[11].s64 + -11832;
	// 82703BB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82703BB4: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 82703BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703BBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703BC8: 386AFAB4  addi r3, r10, -0x54c
	ctx.r[3].s64 = ctx.r[10].s64 + -1356;
	// 82703BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703BE4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82703BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703BEC: 4BD63235  bl 0x82466e20
	ctx.lr = 0x82703BF0;
	sub_82466E20(ctx, base);
	// 82703BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703C00 size=112
    let mut pc: u32 = 0x82703C00;
    'dispatch: loop {
        match pc {
            0x82703C00 => {
    //   block [0x82703C00..0x82703C70)
	// 82703C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703C0C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703C10: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703C14: 396BD1F8  addi r11, r11, -0x2e08
	ctx.r[11].s64 = ctx.r[11].s64 + -11784;
	// 82703C18: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703C20: 390B0078  addi r8, r11, 0x78
	ctx.r[8].s64 = ctx.r[11].s64 + 120;
	// 82703C24: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82703C28: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 82703C2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703C30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703C34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703C38: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82703C3C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82703C40: 386AFAE4  addi r3, r10, -0x51c
	ctx.r[3].s64 = ctx.r[10].s64 + -1308;
	// 82703C44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82703C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703C50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82703C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703C58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82703C5C: 4BD631C5  bl 0x82466e20
	ctx.lr = 0x82703C60;
	sub_82466E20(ctx, base);
	// 82703C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703C70 size=112
    let mut pc: u32 = 0x82703C70;
    'dispatch: loop {
        match pc {
            0x82703C70 => {
    //   block [0x82703C70..0x82703CE0)
	// 82703C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703C7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703C80: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703C84: 396BD288  addi r11, r11, -0x2d78
	ctx.r[11].s64 = ctx.r[11].s64 + -11640;
	// 82703C88: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703C8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703C90: 390B0090  addi r8, r11, 0x90
	ctx.r[8].s64 = ctx.r[11].s64 + 144;
	// 82703C94: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82703C98: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 82703C9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703CA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703CA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703CA8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82703CAC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82703CB0: 386AFB14  addi r3, r10, -0x4ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1260;
	// 82703CB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82703CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703CC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82703CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703CC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82703CCC: 4BD63155  bl 0x82466e20
	ctx.lr = 0x82703CD0;
	sub_82466E20(ctx, base);
	// 82703CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82703CE0 size=24
    let mut pc: u32 = 0x82703CE0;
    'dispatch: loop {
        match pc {
            0x82703CE0 => {
    //   block [0x82703CE0..0x82703CF8)
	// 82703CE0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703CE4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82703CE8: 394A9F38  addi r10, r10, -0x60c8
	ctx.r[10].s64 = ctx.r[10].s64 + -24776;
	// 82703CEC: 816B9D38  lwz r11, -0x62c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25288 as u32) ) } as u64;
	// 82703CF0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82703CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703CF8 size=116
    let mut pc: u32 = 0x82703CF8;
    'dispatch: loop {
        match pc {
            0x82703CF8 => {
    //   block [0x82703CF8..0x82703D6C)
	// 82703CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703D04: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703D08: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703D0C: 392BD344  addi r9, r11, -0x2cbc
	ctx.r[9].s64 = ctx.r[11].s64 + -11452;
	// 82703D10: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703D14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703D18: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82703D1C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82703D20: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82703D24: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 82703D28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703D2C: 396B9F38  addi r11, r11, -0x60c8
	ctx.r[11].s64 = ctx.r[11].s64 + -24776;
	// 82703D30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82703D34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703D38: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82703D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703D40: 386AFB44  addi r3, r10, -0x4bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1212;
	// 82703D44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82703D48: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82703D4C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82703D50: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82703D54: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82703D58: 4BD630C9  bl 0x82466e20
	ctx.lr = 0x82703D5C;
	sub_82466E20(ctx, base);
	// 82703D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703D70 size=112
    let mut pc: u32 = 0x82703D70;
    'dispatch: loop {
        match pc {
            0x82703D70 => {
    //   block [0x82703D70..0x82703DE0)
	// 82703D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703D7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703D80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703D84: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703D88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703D8C: 390BD380  addi r8, r11, -0x2c80
	ctx.r[8].s64 = ctx.r[11].s64 + -11392;
	// 82703D90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82703D94: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 82703D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703D9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703DA8: 386AFB74  addi r3, r10, -0x48c
	ctx.r[3].s64 = ctx.r[10].s64 + -1164;
	// 82703DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703DC4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82703DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703DCC: 4BD63055  bl 0x82466e20
	ctx.lr = 0x82703DD0;
	sub_82466E20(ctx, base);
	// 82703DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703DE0 size=112
    let mut pc: u32 = 0x82703DE0;
    'dispatch: loop {
        match pc {
            0x82703DE0 => {
    //   block [0x82703DE0..0x82703E50)
	// 82703DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703DEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703DF0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703DF4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703DF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703DFC: 390BD3E0  addi r8, r11, -0x2c20
	ctx.r[8].s64 = ctx.r[11].s64 + -11296;
	// 82703E00: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82703E04: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 82703E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703E0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703E18: 386AFBA4  addi r3, r10, -0x45c
	ctx.r[3].s64 = ctx.r[10].s64 + -1116;
	// 82703E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703E34: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82703E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703E3C: 4BD62FE5  bl 0x82466e20
	ctx.lr = 0x82703E40;
	sub_82466E20(ctx, base);
	// 82703E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703E50 size=112
    let mut pc: u32 = 0x82703E50;
    'dispatch: loop {
        match pc {
            0x82703E50 => {
    //   block [0x82703E50..0x82703EC0)
	// 82703E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703E60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703E64: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703E68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703E6C: 390BD488  addi r8, r11, -0x2b78
	ctx.r[8].s64 = ctx.r[11].s64 + -11128;
	// 82703E70: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82703E74: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 82703E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703E7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703E88: 386AFBD4  addi r3, r10, -0x42c
	ctx.r[3].s64 = ctx.r[10].s64 + -1068;
	// 82703E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703EA4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82703EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703EAC: 4BD62F75  bl 0x82466e20
	ctx.lr = 0x82703EB0;
	sub_82466E20(ctx, base);
	// 82703EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703EC0 size=112
    let mut pc: u32 = 0x82703EC0;
    'dispatch: loop {
        match pc {
            0x82703EC0 => {
    //   block [0x82703EC0..0x82703F30)
	// 82703EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703ECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703ED0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703ED4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703EDC: 390BD500  addi r8, r11, -0x2b00
	ctx.r[8].s64 = ctx.r[11].s64 + -11008;
	// 82703EE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82703EE4: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 82703EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703EEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703EF8: 386AFC04  addi r3, r10, -0x3fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1020;
	// 82703EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703F14: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82703F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703F1C: 4BD62F05  bl 0x82466e20
	ctx.lr = 0x82703F20;
	sub_82466E20(ctx, base);
	// 82703F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703F30 size=112
    let mut pc: u32 = 0x82703F30;
    'dispatch: loop {
        match pc {
            0x82703F30 => {
    //   block [0x82703F30..0x82703FA0)
	// 82703F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703F40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703F44: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703F48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703F4C: 390BD548  addi r8, r11, -0x2ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -10936;
	// 82703F50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82703F54: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 82703F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703F5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703F68: 386AFC34  addi r3, r10, -0x3cc
	ctx.r[3].s64 = ctx.r[10].s64 + -972;
	// 82703F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703F84: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82703F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703F8C: 4BD62E95  bl 0x82466e20
	ctx.lr = 0x82703F90;
	sub_82466E20(ctx, base);
	// 82703F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82703F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82703F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82703F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82703FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82703FA0 size=112
    let mut pc: u32 = 0x82703FA0;
    'dispatch: loop {
        match pc {
            0x82703FA0 => {
    //   block [0x82703FA0..0x82704010)
	// 82703FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82703FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82703FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82703FAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703FB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82703FB4: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82703FB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82703FBC: 390BD5D8  addi r8, r11, -0x2a28
	ctx.r[8].s64 = ctx.r[11].s64 + -10792;
	// 82703FC0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82703FC4: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 82703FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82703FCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82703FD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82703FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82703FD8: 386AFC64  addi r3, r10, -0x39c
	ctx.r[3].s64 = ctx.r[10].s64 + -924;
	// 82703FDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82703FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82703FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82703FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82703FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82703FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82703FF4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82703FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82703FFC: 4BD62E25  bl 0x82466e20
	ctx.lr = 0x82704000;
	sub_82466E20(ctx, base);
	// 82704000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704010 size=112
    let mut pc: u32 = 0x82704010;
    'dispatch: loop {
        match pc {
            0x82704010 => {
    //   block [0x82704010..0x82704080)
	// 82704010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270401C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704020: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704024: 38AAF814  addi r5, r10, -0x7ec
	ctx.r[5].s64 = ctx.r[10].s64 + -2028;
	// 82704028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270402C: 390BD638  addi r8, r11, -0x29c8
	ctx.r[8].s64 = ctx.r[11].s64 + -10696;
	// 82704030: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82704034: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 82704038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270403C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704048: 386AFC94  addi r3, r10, -0x36c
	ctx.r[3].s64 = ctx.r[10].s64 + -876;
	// 8270404C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270405C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704064: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82704068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270406C: 4BD62DB5  bl 0x82466e20
	ctx.lr = 0x82704070;
	sub_82466E20(ctx, base);
	// 82704070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270407C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704080 size=112
    let mut pc: u32 = 0x82704080;
    'dispatch: loop {
        match pc {
            0x82704080 => {
    //   block [0x82704080..0x827040F0)
	// 82704080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270408C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704090: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704094: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270409C: 390BD698  addi r8, r11, -0x2968
	ctx.r[8].s64 = ctx.r[11].s64 + -10600;
	// 827040A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827040A4: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 827040A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827040AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827040B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827040B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827040B8: 386AFCC4  addi r3, r10, -0x33c
	ctx.r[3].s64 = ctx.r[10].s64 + -828;
	// 827040BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827040C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827040C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827040C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827040CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827040D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827040D4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827040D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827040DC: 4BD62D45  bl 0x82466e20
	ctx.lr = 0x827040E0;
	sub_82466E20(ctx, base);
	// 827040E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827040E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827040E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827040EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827040F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827040F0 size=112
    let mut pc: u32 = 0x827040F0;
    'dispatch: loop {
        match pc {
            0x827040F0 => {
    //   block [0x827040F0..0x82704160)
	// 827040F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827040F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827040F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827040FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704100: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704104: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270410C: 390BD6C8  addi r8, r11, -0x2938
	ctx.r[8].s64 = ctx.r[11].s64 + -10552;
	// 82704110: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82704114: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 82704118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270411C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704128: 386AFCF4  addi r3, r10, -0x30c
	ctx.r[3].s64 = ctx.r[10].s64 + -780;
	// 8270412C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270413C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704144: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82704148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270414C: 4BD62CD5  bl 0x82466e20
	ctx.lr = 0x82704150;
	sub_82466E20(ctx, base);
	// 82704150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704160 size=100
    let mut pc: u32 = 0x82704160;
    'dispatch: loop {
        match pc {
            0x82704160 => {
    //   block [0x82704160..0x827041C4)
	// 82704160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270416C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704174: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704178: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270417C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704180: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 82704184: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270418C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704194: 386AFD24  addi r3, r10, -0x2dc
	ctx.r[3].s64 = ctx.r[10].s64 + -732;
	// 82704198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270419C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827041A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827041A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827041A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827041AC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 827041B0: 4BD62C71  bl 0x82466e20
	ctx.lr = 0x827041B4;
	sub_82466E20(ctx, base);
	// 827041B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827041B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827041BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827041C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827041C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827041C8 size=112
    let mut pc: u32 = 0x827041C8;
    'dispatch: loop {
        match pc {
            0x827041C8 => {
    //   block [0x827041C8..0x82704238)
	// 827041C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827041CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827041D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827041D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827041D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827041DC: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 827041E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827041E4: 390BD710  addi r8, r11, -0x28f0
	ctx.r[8].s64 = ctx.r[11].s64 + -10480;
	// 827041E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827041EC: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 827041F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827041F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827041F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827041FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704200: 386AFD54  addi r3, r10, -0x2ac
	ctx.r[3].s64 = ctx.r[10].s64 + -684;
	// 82704204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270420C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270421C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82704220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704224: 4BD62BFD  bl 0x82466e20
	ctx.lr = 0x82704228;
	sub_82466E20(ctx, base);
	// 82704228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270422C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704238 size=100
    let mut pc: u32 = 0x82704238;
    'dispatch: loop {
        match pc {
            0x82704238 => {
    //   block [0x82704238..0x8270429C)
	// 82704238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270423C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704244: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270424C: 38AAFC94  addi r5, r10, -0x36c
	ctx.r[5].s64 = ctx.r[10].s64 + -876;
	// 82704250: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82704254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704258: 388AB3BC  addi r4, r10, -0x4c44
	ctx.r[4].s64 = ctx.r[10].s64 + -19524;
	// 8270425C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270426C: 386AFD84  addi r3, r10, -0x27c
	ctx.r[3].s64 = ctx.r[10].s64 + -636;
	// 82704270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704278: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270427C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704280: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704284: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82704288: 4BD62B99  bl 0x82466e20
	ctx.lr = 0x8270428C;
	sub_82466E20(ctx, base);
	// 8270428C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827042A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827042A0 size=108
    let mut pc: u32 = 0x827042A0;
    'dispatch: loop {
        match pc {
            0x827042A0 => {
    //   block [0x827042A0..0x8270430C)
	// 827042A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827042A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827042A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827042AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827042B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827042B4: 392BD748  addi r9, r11, -0x28b8
	ctx.r[9].s64 = ctx.r[11].s64 + -10424;
	// 827042B8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 827042BC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 827042C0: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 827042C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827042C8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827042CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827042D0: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 827042D4: 386AFDB4  addi r3, r10, -0x24c
	ctx.r[3].s64 = ctx.r[10].s64 + -588;
	// 827042D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827042DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827042E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827042E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827042E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827042EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827042F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827042F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827042F8: 4BD62B29  bl 0x82466e20
	ctx.lr = 0x827042FC;
	sub_82466E20(ctx, base);
	// 827042FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704310 size=112
    let mut pc: u32 = 0x82704310;
    'dispatch: loop {
        match pc {
            0x82704310 => {
    //   block [0x82704310..0x82704380)
	// 82704310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270431C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704320: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704324: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704328: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270432C: 390BD808  addi r8, r11, -0x27f8
	ctx.r[8].s64 = ctx.r[11].s64 + -10232;
	// 82704330: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82704334: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 82704338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270433C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704348: 386AFDE4  addi r3, r10, -0x21c
	ctx.r[3].s64 = ctx.r[10].s64 + -540;
	// 8270434C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270435C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704364: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 82704368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270436C: 4BD62AB5  bl 0x82466e20
	ctx.lr = 0x82704370;
	sub_82466E20(ctx, base);
	// 82704370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270437C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704380 size=100
    let mut pc: u32 = 0x82704380;
    'dispatch: loop {
        match pc {
            0x82704380 => {
    //   block [0x82704380..0x827043E4)
	// 82704380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270438C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704394: 38AA04E4  addi r5, r10, 0x4e4
	ctx.r[5].s64 = ctx.r[10].s64 + 1252;
	// 82704398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270439C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827043A0: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 827043A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827043A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827043AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827043B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827043B4: 386AFE14  addi r3, r10, -0x1ec
	ctx.r[3].s64 = ctx.r[10].s64 + -492;
	// 827043B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827043BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827043C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827043C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827043C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827043CC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827043D0: 4BD62A51  bl 0x82466e20
	ctx.lr = 0x827043D4;
	sub_82466E20(ctx, base);
	// 827043D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827043D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827043DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827043E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827043E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827043E8 size=112
    let mut pc: u32 = 0x827043E8;
    'dispatch: loop {
        match pc {
            0x827043E8 => {
    //   block [0x827043E8..0x82704458)
	// 827043E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827043EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827043F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827043F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827043F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827043FC: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82704400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704404: 390BD884  addi r8, r11, -0x277c
	ctx.r[8].s64 = ctx.r[11].s64 + -10108;
	// 82704408: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270440C: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 82704410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270441C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704420: 386AFE44  addi r3, r10, -0x1bc
	ctx.r[3].s64 = ctx.r[10].s64 + -444;
	// 82704424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270442C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270443C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82704440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704444: 4BD629DD  bl 0x82466e20
	ctx.lr = 0x82704448;
	sub_82466E20(ctx, base);
	// 82704448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270444C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704458 size=92
    let mut pc: u32 = 0x82704458;
    'dispatch: loop {
        match pc {
            0x82704458 => {
    //   block [0x82704458..0x827044B4)
	// 82704458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704460: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704464: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82704468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8270446C: 4BD92C5D  bl 0x824970c8
	ctx.lr = 0x82704470;
	sub_824970C8(ctx, base);
	// 82704470: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704474: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82704478: 394BC6B8  addi r10, r11, -0x3948
	ctx.r[10].s64 = ctx.r[11].s64 + -14664;
	// 8270447C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704480: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704484: 396BFE74  addi r11, r11, -0x18c
	ctx.r[11].s64 = ctx.r[11].s64 + -396;
	// 82704488: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270448C: 394810D8  addi r10, r8, 0x10d8
	ctx.r[10].s64 = ctx.r[8].s64 + 4312;
	// 82704490: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704494: 394910F0  addi r10, r9, 0x10f0
	ctx.r[10].s64 = ctx.r[9].s64 + 4336;
	// 82704498: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8270449C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827044A0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827044A4: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 827044A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827044AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827044B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827044B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827044B8 size=116
    let mut pc: u32 = 0x827044B8;
    'dispatch: loop {
        match pc {
            0x827044B8 => {
    //   block [0x827044B8..0x8270452C)
	// 827044B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827044BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827044C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827044C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827044C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827044CC: 392ADA18  addi r9, r10, -0x25e8
	ctx.r[9].s64 = ctx.r[10].s64 + -9704;
	// 827044D0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827044D4: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 827044D8: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827044DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827044E0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 827044E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827044E8: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 827044EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827044F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 827044F4: 396BDA40  addi r11, r11, -0x25c0
	ctx.r[11].s64 = ctx.r[11].s64 + -9664;
	// 827044F8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827044FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704500: 386AFE84  addi r3, r10, -0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + -380;
	// 82704504: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82704508: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8270450C: 38C00310  li r6, 0x310
	ctx.r[6].s64 = 784;
	// 82704510: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82704514: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704518: 4BD62909  bl 0x82466e20
	ctx.lr = 0x8270451C;
	sub_82466E20(ctx, base);
	// 8270451C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704530 size=108
    let mut pc: u32 = 0x82704530;
    'dispatch: loop {
        match pc {
            0x82704530 => {
    //   block [0x82704530..0x8270459C)
	// 82704530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270453C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704544: 38EBE0A0  addi r7, r11, -0x1f60
	ctx.r[7].s64 = ctx.r[11].s64 + -8032;
	// 82704548: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270454C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 82704550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704554: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270455C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704560: 386AFEB4  addi r3, r10, -0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + -332;
	// 82704564: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270456C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270457C: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 82704580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704588: 4BD62899  bl 0x82466e20
	ctx.lr = 0x8270458C;
	sub_82466E20(ctx, base);
	// 8270458C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827045A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827045A0 size=112
    let mut pc: u32 = 0x827045A0;
    'dispatch: loop {
        match pc {
            0x827045A0 => {
    //   block [0x827045A0..0x82704610)
	// 827045A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827045A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827045A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827045AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827045B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827045B4: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827045B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827045BC: 390BE0D0  addi r8, r11, -0x1f30
	ctx.r[8].s64 = ctx.r[11].s64 + -7984;
	// 827045C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827045C4: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 827045C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827045CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827045D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827045D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827045D8: 386AFEE4  addi r3, r10, -0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + -284;
	// 827045DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827045E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827045E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827045E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827045EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827045F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827045F4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 827045F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827045FC: 4BD62825  bl 0x82466e20
	ctx.lr = 0x82704600;
	sub_82466E20(ctx, base);
	// 82704600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270460C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704610 size=108
    let mut pc: u32 = 0x82704610;
    'dispatch: loop {
        match pc {
            0x82704610 => {
    //   block [0x82704610..0x8270467C)
	// 82704610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270461C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704624: 392BE138  addi r9, r11, -0x1ec8
	ctx.r[9].s64 = ctx.r[11].s64 + -7880;
	// 82704628: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8270462C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82704630: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 82704634: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704638: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270463C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82704640: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 82704644: 386AFF14  addi r3, r10, -0xec
	ctx.r[3].s64 = ctx.r[10].s64 + -236;
	// 82704648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270464C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704654: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704658: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270465C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704660: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704664: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704668: 4BD627B9  bl 0x82466e20
	ctx.lr = 0x8270466C;
	sub_82466E20(ctx, base);
	// 8270466C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704680 size=112
    let mut pc: u32 = 0x82704680;
    'dispatch: loop {
        match pc {
            0x82704680 => {
    //   block [0x82704680..0x827046F0)
	// 82704680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270468C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704690: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704694: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270469C: 390BE198  addi r8, r11, -0x1e68
	ctx.r[8].s64 = ctx.r[11].s64 + -7784;
	// 827046A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827046A4: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 827046A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827046AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827046B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827046B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827046B8: 386AFF44  addi r3, r10, -0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + -188;
	// 827046BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827046C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827046C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827046C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827046CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827046D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827046D4: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 827046D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827046DC: 4BD62745  bl 0x82466e20
	ctx.lr = 0x827046E0;
	sub_82466E20(ctx, base);
	// 827046E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827046E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827046E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827046EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827046F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827046F0 size=108
    let mut pc: u32 = 0x827046F0;
    'dispatch: loop {
        match pc {
            0x827046F0 => {
    //   block [0x827046F0..0x8270475C)
	// 827046F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827046F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827046F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827046FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704704: 392BE200  addi r9, r11, -0x1e00
	ctx.r[9].s64 = ctx.r[11].s64 + -7680;
	// 82704708: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8270470C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82704710: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 82704714: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704718: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270471C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82704720: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82704724: 386AFF74  addi r3, r10, -0x8c
	ctx.r[3].s64 = ctx.r[10].s64 + -140;
	// 82704728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270472C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704734: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704738: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270473C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704740: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704744: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704748: 4BD626D9  bl 0x82466e20
	ctx.lr = 0x8270474C;
	sub_82466E20(ctx, base);
	// 8270474C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704760 size=112
    let mut pc: u32 = 0x82704760;
    'dispatch: loop {
        match pc {
            0x82704760 => {
    //   block [0x82704760..0x827047D0)
	// 82704760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270476C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704770: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704774: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270477C: 390BE278  addi r8, r11, -0x1d88
	ctx.r[8].s64 = ctx.r[11].s64 + -7560;
	// 82704780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704784: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 82704788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270478C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704798: 386AFFA4  addi r3, r10, -0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + -92;
	// 8270479C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827047A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827047A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827047A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827047AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827047B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827047B4: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 827047B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827047BC: 4BD62665  bl 0x82466e20
	ctx.lr = 0x827047C0;
	sub_82466E20(ctx, base);
	// 827047C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827047C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827047C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827047CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827047D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827047D0 size=112
    let mut pc: u32 = 0x827047D0;
    'dispatch: loop {
        match pc {
            0x827047D0 => {
    //   block [0x827047D0..0x82704840)
	// 827047D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827047D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827047D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827047DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827047E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827047E4: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 827047E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827047EC: 390BE2C4  addi r8, r11, -0x1d3c
	ctx.r[8].s64 = ctx.r[11].s64 + -7484;
	// 827047F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827047F4: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 827047F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827047FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704808: 386AFFD4  addi r3, r10, -0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + -44;
	// 8270480C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270481C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704824: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82704828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270482C: 4BD625F5  bl 0x82466e20
	ctx.lr = 0x82704830;
	sub_82466E20(ctx, base);
	// 82704830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270483C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704840 size=100
    let mut pc: u32 = 0x82704840;
    'dispatch: loop {
        match pc {
            0x82704840 => {
    //   block [0x82704840..0x827048A4)
	// 82704840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270484C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704854: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82704858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704860: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 82704864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270486C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704874: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82704878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270487C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704880: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270488C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82704890: 4BD62591  bl 0x82466e20
	ctx.lr = 0x82704894;
	sub_82466E20(ctx, base);
	// 82704894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827048A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827048A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827048A8 size=24
    let mut pc: u32 = 0x827048A8;
    'dispatch: loop {
        match pc {
            0x827048A8 => {
    //   block [0x827048A8..0x827048C0)
	// 827048A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827048AC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827048B0: 394AA288  addi r10, r10, -0x5d78
	ctx.r[10].s64 = ctx.r[10].s64 + -23928;
	// 827048B4: 816BA274  lwz r11, -0x5d8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23948 as u32) ) } as u64;
	// 827048B8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 827048BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827048C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827048C0 size=116
    let mut pc: u32 = 0x827048C0;
    'dispatch: loop {
        match pc {
            0x827048C0 => {
    //   block [0x827048C0..0x82704934)
	// 827048C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827048C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827048C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827048CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827048D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827048D4: 392AE370  addi r9, r10, -0x1c90
	ctx.r[9].s64 = ctx.r[10].s64 + -7312;
	// 827048D8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827048DC: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 827048E0: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827048E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827048E8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 827048EC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827048F0: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 827048F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827048F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 827048FC: 396BA288  addi r11, r11, -0x5d78
	ctx.r[11].s64 = ctx.r[11].s64 + -23928;
	// 82704900: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704908: 386A0034  addi r3, r10, 0x34
	ctx.r[3].s64 = ctx.r[10].s64 + 52;
	// 8270490C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82704910: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82704914: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82704918: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8270491C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704920: 4BD62501  bl 0x82466e20
	ctx.lr = 0x82704924;
	sub_82466E20(ctx, base);
	// 82704924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270492C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704938 size=112
    let mut pc: u32 = 0x82704938;
    'dispatch: loop {
        match pc {
            0x82704938 => {
    //   block [0x82704938..0x827049A8)
	// 82704938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270493C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704940: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704944: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704948: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8270494C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704950: 396BE418  addi r11, r11, -0x1be8
	ctx.r[11].s64 = ctx.r[11].s64 + -7144;
	// 82704954: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82704958: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8270495C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82704960: 4BD9E559  bl 0x824a2eb8
	ctx.lr = 0x82704964;
	sub_824A2EB8(ctx, base);
	// 82704964: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704968: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 8270496C: 394BBFB8  addi r10, r11, -0x4048
	ctx.r[10].s64 = ctx.r[11].s64 + -16456;
	// 82704970: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704974: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704978: 396B0064  addi r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 + 100;
	// 8270497C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82704980: 394813D0  addi r10, r8, 0x13d0
	ctx.r[10].s64 = ctx.r[8].s64 + 5072;
	// 82704984: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704988: 394913B8  addi r10, r9, 0x13b8
	ctx.r[10].s64 = ctx.r[9].s64 + 5048;
	// 8270498C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82704990: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82704994: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82704998: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8270499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827049A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827049A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827049A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827049A8 size=24
    let mut pc: u32 = 0x827049A8;
    'dispatch: loop {
        match pc {
            0x827049A8 => {
    //   block [0x827049A8..0x827049C0)
	// 827049A8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827049AC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827049B0: 394AA388  addi r10, r10, -0x5c78
	ctx.r[10].s64 = ctx.r[10].s64 + -23672;
	// 827049B4: 816BA380  lwz r11, -0x5c80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23680 as u32) ) } as u64;
	// 827049B8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 827049BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827049C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827049C0 size=116
    let mut pc: u32 = 0x827049C0;
    'dispatch: loop {
        match pc {
            0x827049C0 => {
    //   block [0x827049C0..0x82704A34)
	// 827049C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827049C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827049C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827049CC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827049D0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827049D4: 390BA388  addi r8, r11, -0x5c78
	ctx.r[8].s64 = ctx.r[11].s64 + -23672;
	// 827049D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827049DC: 392AE400  addi r9, r10, -0x1c00
	ctx.r[9].s64 = ctx.r[10].s64 + -7168;
	// 827049E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827049E4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 827049E8: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827049EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827049F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827049F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827049F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827049FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704A04: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704A08: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 82704A0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82704A10: 386B0074  addi r3, r11, 0x74
	ctx.r[3].s64 = ctx.r[11].s64 + 116;
	// 82704A14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704A18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704A1C: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82704A20: 4BD62401  bl 0x82466e20
	ctx.lr = 0x82704A24;
	sub_82466E20(ctx, base);
	// 82704A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704A38 size=112
    let mut pc: u32 = 0x82704A38;
    'dispatch: loop {
        match pc {
            0x82704A38 => {
    //   block [0x82704A38..0x82704AA8)
	// 82704A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704A44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704A48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704A4C: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82704A50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704A54: 390BE448  addi r8, r11, -0x1bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7096;
	// 82704A58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704A5C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 82704A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704A70: 386A00A4  addi r3, r10, 0xa4
	ctx.r[3].s64 = ctx.r[10].s64 + 164;
	// 82704A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704A8C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82704A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704A94: 4BD6238D  bl 0x82466e20
	ctx.lr = 0x82704A98;
	sub_82466E20(ctx, base);
	// 82704A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704AA8 size=100
    let mut pc: u32 = 0x82704AA8;
    'dispatch: loop {
        match pc {
            0x82704AA8 => {
    //   block [0x82704AA8..0x82704B0C)
	// 82704AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704ABC: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 82704AC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704AC8: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 82704ACC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704ADC: 386A00D4  addi r3, r10, 0xd4
	ctx.r[3].s64 = ctx.r[10].s64 + 212;
	// 82704AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704AE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704AE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704AF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704AF4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82704AF8: 4BD62329  bl 0x82466e20
	ctx.lr = 0x82704AFC;
	sub_82466E20(ctx, base);
	// 82704AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704B10 size=100
    let mut pc: u32 = 0x82704B10;
    'dispatch: loop {
        match pc {
            0x82704B10 => {
    //   block [0x82704B10..0x82704B74)
	// 82704B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704B1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704B24: 38AA00D4  addi r5, r10, 0xd4
	ctx.r[5].s64 = ctx.r[10].s64 + 212;
	// 82704B28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704B30: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 82704B34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704B44: 386A0104  addi r3, r10, 0x104
	ctx.r[3].s64 = ctx.r[10].s64 + 260;
	// 82704B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704B4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704B50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704B5C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82704B60: 4BD622C1  bl 0x82466e20
	ctx.lr = 0x82704B64;
	sub_82466E20(ctx, base);
	// 82704B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704B78 size=100
    let mut pc: u32 = 0x82704B78;
    'dispatch: loop {
        match pc {
            0x82704B78 => {
    //   block [0x82704B78..0x82704BDC)
	// 82704B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704B84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704B8C: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 82704B90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704B98: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 82704B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704BAC: 386A0134  addi r3, r10, 0x134
	ctx.r[3].s64 = ctx.r[10].s64 + 308;
	// 82704BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704BB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704BB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82704BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704BC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704BC4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82704BC8: 4BD62259  bl 0x82466e20
	ctx.lr = 0x82704BCC;
	sub_82466E20(ctx, base);
	// 82704BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704BE0 size=92
    let mut pc: u32 = 0x82704BE0;
    'dispatch: loop {
        match pc {
            0x82704BE0 => {
    //   block [0x82704BE0..0x82704C3C)
	// 82704BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704BE8: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704BEC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82704BF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82704BF4: 4BD8CA55  bl 0x82491648
	ctx.lr = 0x82704BF8;
	sub_82491648(ctx, base);
	// 82704BF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704BFC: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82704C00: 394BC630  addi r10, r11, -0x39d0
	ctx.r[10].s64 = ctx.r[11].s64 + -14800;
	// 82704C04: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704C08: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704C0C: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 82704C10: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82704C14: 39481598  addi r10, r8, 0x1598
	ctx.r[10].s64 = ctx.r[8].s64 + 5528;
	// 82704C18: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704C1C: 39491580  addi r10, r9, 0x1580
	ctx.r[10].s64 = ctx.r[9].s64 + 5504;
	// 82704C20: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82704C24: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82704C28: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82704C2C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82704C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704C40 size=112
    let mut pc: u32 = 0x82704C40;
    'dispatch: loop {
        match pc {
            0x82704C40 => {
    //   block [0x82704C40..0x82704CB0)
	// 82704C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704C4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704C50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704C54: 38AA0E18  addi r5, r10, 0xe18
	ctx.r[5].s64 = ctx.r[10].s64 + 3608;
	// 82704C58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704C5C: 390BE460  addi r8, r11, -0x1ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -7072;
	// 82704C60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704C64: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 82704C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704C6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704C70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704C78: 386A0174  addi r3, r10, 0x174
	ctx.r[3].s64 = ctx.r[10].s64 + 372;
	// 82704C7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704C94: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 82704C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704C9C: 4BD62185  bl 0x82466e20
	ctx.lr = 0x82704CA0;
	sub_82466E20(ctx, base);
	// 82704CA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704CB0 size=108
    let mut pc: u32 = 0x82704CB0;
    'dispatch: loop {
        match pc {
            0x82704CB0 => {
    //   block [0x82704CB0..0x82704D1C)
	// 82704CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704CBC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704CC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704CC4: 38EBE580  addi r7, r11, -0x1a80
	ctx.r[7].s64 = ctx.r[11].s64 + -6784;
	// 82704CC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82704CCC: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 82704CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704CD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704CE0: 386A01A4  addi r3, r10, 0x1a4
	ctx.r[3].s64 = ctx.r[10].s64 + 420;
	// 82704CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704CFC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82704D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704D08: 4BD62119  bl 0x82466e20
	ctx.lr = 0x82704D0C;
	sub_82466E20(ctx, base);
	// 82704D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704D20 size=108
    let mut pc: u32 = 0x82704D20;
    'dispatch: loop {
        match pc {
            0x82704D20 => {
    //   block [0x82704D20..0x82704D8C)
	// 82704D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704D2C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704D30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704D34: 38EBE5C8  addi r7, r11, -0x1a38
	ctx.r[7].s64 = ctx.r[11].s64 + -6712;
	// 82704D38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82704D3C: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 82704D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704D44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704D50: 386A01D4  addi r3, r10, 0x1d4
	ctx.r[3].s64 = ctx.r[10].s64 + 468;
	// 82704D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704D6C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82704D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704D78: 4BD620A9  bl 0x82466e20
	ctx.lr = 0x82704D7C;
	sub_82466E20(ctx, base);
	// 82704D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704D90 size=108
    let mut pc: u32 = 0x82704D90;
    'dispatch: loop {
        match pc {
            0x82704D90 => {
    //   block [0x82704D90..0x82704DFC)
	// 82704D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704D9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704DA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704DA4: 38EBE628  addi r7, r11, -0x19d8
	ctx.r[7].s64 = ctx.r[11].s64 + -6616;
	// 82704DA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82704DAC: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 82704DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704DB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704DC0: 386A0204  addi r3, r10, 0x204
	ctx.r[3].s64 = ctx.r[10].s64 + 516;
	// 82704DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704DDC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82704DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704DE8: 4BD62039  bl 0x82466e20
	ctx.lr = 0x82704DEC;
	sub_82466E20(ctx, base);
	// 82704DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704E00 size=92
    let mut pc: u32 = 0x82704E00;
    'dispatch: loop {
        match pc {
            0x82704E00 => {
    //   block [0x82704E00..0x82704E5C)
	// 82704E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704E08: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704E0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82704E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82704E14: 4BD89A75  bl 0x8248e888
	ctx.lr = 0x82704E18;
	sub_8248E888(ctx, base);
	// 82704E18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82704E1C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82704E20: 394BC4A8  addi r10, r11, -0x3b58
	ctx.r[10].s64 = ctx.r[11].s64 + -15192;
	// 82704E24: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82704E28: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82704E2C: 396B0234  addi r11, r11, 0x234
	ctx.r[11].s64 = ctx.r[11].s64 + 564;
	// 82704E30: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82704E34: 39481740  addi r10, r8, 0x1740
	ctx.r[10].s64 = ctx.r[8].s64 + 5952;
	// 82704E38: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82704E3C: 39491728  addi r10, r9, 0x1728
	ctx.r[10].s64 = ctx.r[9].s64 + 5928;
	// 82704E40: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82704E44: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82704E48: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82704E4C: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 82704E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704E60 size=116
    let mut pc: u32 = 0x82704E60;
    'dispatch: loop {
        match pc {
            0x82704E60 => {
    //   block [0x82704E60..0x82704ED4)
	// 82704E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704E6C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704E70: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704E74: 392BE568  addi r9, r11, -0x1a98
	ctx.r[9].s64 = ctx.r[11].s64 + -6808;
	// 82704E78: 38AA0DE8  addi r5, r10, 0xde8
	ctx.r[5].s64 = ctx.r[10].s64 + 3560;
	// 82704E7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704E80: 38E90288  addi r7, r9, 0x288
	ctx.r[7].s64 = ctx.r[9].s64 + 648;
	// 82704E84: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82704E88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704E8C: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 82704E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704E94: 396BE658  addi r11, r11, -0x19a8
	ctx.r[11].s64 = ctx.r[11].s64 + -6568;
	// 82704E98: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82704E9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704EA0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82704EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704EA8: 386A0244  addi r3, r10, 0x244
	ctx.r[3].s64 = ctx.r[10].s64 + 580;
	// 82704EAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82704EB0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82704EB4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 82704EB8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82704EBC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82704EC0: 4BD61F61  bl 0x82466e20
	ctx.lr = 0x82704EC4;
	sub_82466E20(ctx, base);
	// 82704EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704ED8 size=112
    let mut pc: u32 = 0x82704ED8;
    'dispatch: loop {
        match pc {
            0x82704ED8 => {
    //   block [0x82704ED8..0x82704F48)
	// 82704ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82704EE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704EEC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82704EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704EF4: 390BE838  addi r8, r11, -0x17c8
	ctx.r[8].s64 = ctx.r[11].s64 + -6088;
	// 82704EF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82704EFC: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 82704F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704F04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82704F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704F10: 386A0274  addi r3, r10, 0x274
	ctx.r[3].s64 = ctx.r[10].s64 + 628;
	// 82704F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82704F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704F2C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82704F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704F34: 4BD61EED  bl 0x82466e20
	ctx.lr = 0x82704F38;
	sub_82466E20(ctx, base);
	// 82704F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704F48 size=108
    let mut pc: u32 = 0x82704F48;
    'dispatch: loop {
        match pc {
            0x82704F48 => {
    //   block [0x82704F48..0x82704FB4)
	// 82704F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704F54: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704F5C: 38EBE850  addi r7, r11, -0x17b0
	ctx.r[7].s64 = ctx.r[11].s64 + -6064;
	// 82704F60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82704F64: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 82704F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704F78: 386A02A4  addi r3, r10, 0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + 676;
	// 82704F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82704F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82704F94: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82704F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82704F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82704FA0: 4BD61E81  bl 0x82466e20
	ctx.lr = 0x82704FA4;
	sub_82466E20(ctx, base);
	// 82704FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82704FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82704FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82704FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82704FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82704FB8 size=108
    let mut pc: u32 = 0x82704FB8;
    'dispatch: loop {
        match pc {
            0x82704FB8 => {
    //   block [0x82704FB8..0x82705024)
	// 82704FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82704FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82704FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82704FC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82704FC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82704FCC: 38EBE868  addi r7, r11, -0x1798
	ctx.r[7].s64 = ctx.r[11].s64 + -6040;
	// 82704FD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82704FD4: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 82704FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82704FDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82704FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82704FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82704FE8: 386A02D4  addi r3, r10, 0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + 724;
	// 82704FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82704FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82704FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82704FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82704FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705004: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82705008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270500C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705010: 4BD61E11  bl 0x82466e20
	ctx.lr = 0x82705014;
	sub_82466E20(ctx, base);
	// 82705014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270501C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705028 size=112
    let mut pc: u32 = 0x82705028;
    'dispatch: loop {
        match pc {
            0x82705028 => {
    //   block [0x82705028..0x82705098)
	// 82705028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270502C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705038: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270503C: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 82705040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705044: 390BE8B0  addi r8, r11, -0x1750
	ctx.r[8].s64 = ctx.r[11].s64 + -5968;
	// 82705048: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270504C: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 82705050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705054: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270505C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705060: 386A0304  addi r3, r10, 0x304
	ctx.r[3].s64 = ctx.r[10].s64 + 772;
	// 82705064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270507C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82705080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705084: 4BD61D9D  bl 0x82466e20
	ctx.lr = 0x82705088;
	sub_82466E20(ctx, base);
	// 82705088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270508C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705098 size=92
    let mut pc: u32 = 0x82705098;
    'dispatch: loop {
        match pc {
            0x82705098 => {
    //   block [0x82705098..0x827050F4)
	// 82705098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270509C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827050A0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827050A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827050A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827050AC: 4BD961E5  bl 0x8249b290
	ctx.lr = 0x827050B0;
	sub_8249B290(ctx, base);
	// 827050B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827050B4: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 827050B8: 394BC6C4  addi r10, r11, -0x393c
	ctx.r[10].s64 = ctx.r[11].s64 + -14652;
	// 827050BC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827050C0: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 827050C4: 396B0334  addi r11, r11, 0x334
	ctx.r[11].s64 = ctx.r[11].s64 + 820;
	// 827050C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827050CC: 394817E8  addi r10, r8, 0x17e8
	ctx.r[10].s64 = ctx.r[8].s64 + 6120;
	// 827050D0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827050D4: 39491800  addi r10, r9, 0x1800
	ctx.r[10].s64 = ctx.r[9].s64 + 6144;
	// 827050D8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827050DC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827050E0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827050E4: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 827050E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827050EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827050F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827050F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827050F8 size=48
    let mut pc: u32 = 0x827050F8;
    'dispatch: loop {
        match pc {
            0x827050F8 => {
    //   block [0x827050F8..0x82705128)
	// 827050F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827050FC: 814BA55C  lwz r10, -0x5aa4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23204 as u32) ) } as u64;
	// 82705100: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705104: 396BA560  addi r11, r11, -0x5aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -23200;
	// 82705108: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8270510C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82705110: 814AA558  lwz r10, -0x5aa8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23208 as u32) ) } as u64;
	// 82705114: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82705118: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270511C: 814AA554  lwz r10, -0x5aac(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23212 as u32) ) } as u64;
	// 82705120: 914B0380  stw r10, 0x380(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(896 as u32), ctx.r[10].u32 ) };
	// 82705124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705128 size=116
    let mut pc: u32 = 0x82705128;
    'dispatch: loop {
        match pc {
            0x82705128 => {
    //   block [0x82705128..0x8270519C)
	// 82705128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270512C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705134: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705138: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270513C: 392BE9D8  addi r9, r11, -0x1628
	ctx.r[9].s64 = ctx.r[11].s64 + -5672;
	// 82705140: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705144: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705148: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8270514C: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 82705150: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705154: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 82705158: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270515C: 396BA560  addi r11, r11, -0x5aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -23200;
	// 82705160: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82705164: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705168: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8270516C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705170: 386A0344  addi r3, r10, 0x344
	ctx.r[3].s64 = ctx.r[10].s64 + 836;
	// 82705174: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82705178: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8270517C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82705180: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82705184: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705188: 4BD61C99  bl 0x82466e20
	ctx.lr = 0x8270518C;
	sub_82466E20(ctx, base);
	// 8270518C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827051A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827051A0 size=108
    let mut pc: u32 = 0x827051A0;
    'dispatch: loop {
        match pc {
            0x827051A0 => {
    //   block [0x827051A0..0x8270520C)
	// 827051A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827051A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827051A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827051AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827051B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827051B4: 392BEB48  addi r9, r11, -0x14b8
	ctx.r[9].s64 = ctx.r[11].s64 + -5304;
	// 827051B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 827051BC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 827051C0: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 827051C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827051C8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827051CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827051D0: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 827051D4: 386A0374  addi r3, r10, 0x374
	ctx.r[3].s64 = ctx.r[10].s64 + 884;
	// 827051D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827051DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827051E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827051E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827051E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827051EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827051F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827051F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827051F8: 4BD61C29  bl 0x82466e20
	ctx.lr = 0x827051FC;
	sub_82466E20(ctx, base);
	// 827051FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705210 size=112
    let mut pc: u32 = 0x82705210;
    'dispatch: loop {
        match pc {
            0x82705210 => {
    //   block [0x82705210..0x82705280)
	// 82705210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270521C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705220: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705224: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705228: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270522C: 390BEBA8  addi r8, r11, -0x1458
	ctx.r[8].s64 = ctx.r[11].s64 + -5208;
	// 82705230: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705234: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 82705238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270523C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705248: 386A03A4  addi r3, r10, 0x3a4
	ctx.r[3].s64 = ctx.r[10].s64 + 932;
	// 8270524C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270525C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705264: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 82705268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270526C: 4BD61BB5  bl 0x82466e20
	ctx.lr = 0x82705270;
	sub_82466E20(ctx, base);
	// 82705270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270527C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705280 size=108
    let mut pc: u32 = 0x82705280;
    'dispatch: loop {
        match pc {
            0x82705280 => {
    //   block [0x82705280..0x827052EC)
	// 82705280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270528C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705294: 38EBEBF8  addi r7, r11, -0x1408
	ctx.r[7].s64 = ctx.r[11].s64 + -5128;
	// 82705298: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8270529C: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 827052A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827052A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827052A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827052AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827052B0: 386A03D4  addi r3, r10, 0x3d4
	ctx.r[3].s64 = ctx.r[10].s64 + 980;
	// 827052B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827052B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827052BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827052C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827052C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827052C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827052CC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 827052D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827052D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827052D8: 4BD61B49  bl 0x82466e20
	ctx.lr = 0x827052DC;
	sub_82466E20(ctx, base);
	// 827052DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827052E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827052E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827052E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827052F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827052F0 size=92
    let mut pc: u32 = 0x827052F0;
    'dispatch: loop {
        match pc {
            0x827052F0 => {
    //   block [0x827052F0..0x8270534C)
	// 827052F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827052F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827052F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827052FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705304: 4BDA64A5  bl 0x824ab7a8
	ctx.lr = 0x82705308;
	sub_824AB7A8(ctx, base);
	// 82705308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270530C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705310: 394BC250  addi r10, r11, -0x3db0
	ctx.r[10].s64 = ctx.r[11].s64 + -15792;
	// 82705314: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705318: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 8270531C: 396B0404  addi r11, r11, 0x404
	ctx.r[11].s64 = ctx.r[11].s64 + 1028;
	// 82705320: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705324: 394818A8  addi r10, r8, 0x18a8
	ctx.r[10].s64 = ctx.r[8].s64 + 6312;
	// 82705328: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270532C: 394918C0  addi r10, r9, 0x18c0
	ctx.r[10].s64 = ctx.r[9].s64 + 6336;
	// 82705330: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705334: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705338: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270533C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82705340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705350 size=112
    let mut pc: u32 = 0x82705350;
    'dispatch: loop {
        match pc {
            0x82705350 => {
    //   block [0x82705350..0x827053C0)
	// 82705350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270535C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705360: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705364: 396BEC88  addi r11, r11, -0x1378
	ctx.r[11].s64 = ctx.r[11].s64 + -4984;
	// 82705368: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 8270536C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705370: 390B00D8  addi r8, r11, 0xd8
	ctx.r[8].s64 = ctx.r[11].s64 + 216;
	// 82705374: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82705378: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 8270537C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705380: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705388: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8270538C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82705390: 386A0414  addi r3, r10, 0x414
	ctx.r[3].s64 = ctx.r[10].s64 + 1044;
	// 82705394: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270539C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827053A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 827053A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827053A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 827053AC: 4BD61A75  bl 0x82466e20
	ctx.lr = 0x827053B0;
	sub_82466E20(ctx, base);
	// 827053B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827053B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827053B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827053BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827053C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827053C0 size=108
    let mut pc: u32 = 0x827053C0;
    'dispatch: loop {
        match pc {
            0x827053C0 => {
    //   block [0x827053C0..0x8270542C)
	// 827053C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827053C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827053C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827053CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827053D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827053D4: 38EBED94  addi r7, r11, -0x126c
	ctx.r[7].s64 = ctx.r[11].s64 + -4716;
	// 827053D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827053DC: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 827053E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827053E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827053E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827053EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827053F0: 386A0444  addi r3, r10, 0x444
	ctx.r[3].s64 = ctx.r[10].s64 + 1092;
	// 827053F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827053F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827053FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270540C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82705410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705418: 4BD61A09  bl 0x82466e20
	ctx.lr = 0x8270541C;
	sub_82466E20(ctx, base);
	// 8270541C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705430 size=92
    let mut pc: u32 = 0x82705430;
    'dispatch: loop {
        match pc {
            0x82705430 => {
    //   block [0x82705430..0x8270548C)
	// 82705430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705438: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270543C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705444: 4BD8C52D  bl 0x82491970
	ctx.lr = 0x82705448;
	sub_82491970(ctx, base);
	// 82705448: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270544C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705450: 394BC68C  addi r10, r11, -0x3974
	ctx.r[10].s64 = ctx.r[11].s64 + -14708;
	// 82705454: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705458: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 8270545C: 396B0474  addi r11, r11, 0x474
	ctx.r[11].s64 = ctx.r[11].s64 + 1140;
	// 82705460: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705464: 39481928  addi r10, r8, 0x1928
	ctx.r[10].s64 = ctx.r[8].s64 + 6440;
	// 82705468: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270546C: 39491910  addi r10, r9, 0x1910
	ctx.r[10].s64 = ctx.r[9].s64 + 6416;
	// 82705470: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705474: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705478: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270547C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 82705480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705490 size=112
    let mut pc: u32 = 0x82705490;
    'dispatch: loop {
        match pc {
            0x82705490 => {
    //   block [0x82705490..0x82705500)
	// 82705490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270549C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827054A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827054A4: 38AA0E18  addi r5, r10, 0xe18
	ctx.r[5].s64 = ctx.r[10].s64 + 3608;
	// 827054A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827054AC: 390BEDAC  addi r8, r11, -0x1254
	ctx.r[8].s64 = ctx.r[11].s64 + -4692;
	// 827054B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827054B4: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 827054B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827054BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827054C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827054C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827054C8: 386A0484  addi r3, r10, 0x484
	ctx.r[3].s64 = ctx.r[10].s64 + 1156;
	// 827054CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827054D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827054D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827054D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827054DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827054E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827054E4: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 827054E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827054EC: 4BD61935  bl 0x82466e20
	ctx.lr = 0x827054F0;
	sub_82466E20(ctx, base);
	// 827054F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827054F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827054F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827054FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705500 size=100
    let mut pc: u32 = 0x82705500;
    'dispatch: loop {
        match pc {
            0x82705500 => {
    //   block [0x82705500..0x82705564)
	// 82705500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270550C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705514: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 82705518: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270551C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705520: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 82705524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270552C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705534: 386A04B4  addi r3, r10, 0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1204;
	// 82705538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270553C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705540: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82705544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705548: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270554C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82705550: 4BD618D1  bl 0x82466e20
	ctx.lr = 0x82705554;
	sub_82466E20(ctx, base);
	// 82705554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270555C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705568 size=100
    let mut pc: u32 = 0x82705568;
    'dispatch: loop {
        match pc {
            0x82705568 => {
    //   block [0x82705568..0x827055CC)
	// 82705568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705574: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270557C: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 82705580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705588: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 8270558C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270559C: 386A04E4  addi r3, r10, 0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1252;
	// 827055A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827055A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827055A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827055AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827055B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827055B4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827055B8: 4BD61869  bl 0x82466e20
	ctx.lr = 0x827055BC;
	sub_82466E20(ctx, base);
	// 827055BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827055C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827055C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827055C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827055D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827055D0 size=24
    let mut pc: u32 = 0x827055D0;
    'dispatch: loop {
        match pc {
            0x827055D0 => {
    //   block [0x827055D0..0x827055E8)
	// 827055D0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827055D4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827055D8: 394AAA04  addi r10, r10, -0x55fc
	ctx.r[10].s64 = ctx.r[10].s64 + -22012;
	// 827055DC: 816BAA00  lwz r11, -0x5600(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22016 as u32) ) } as u64;
	// 827055E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827055E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827055E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827055E8 size=116
    let mut pc: u32 = 0x827055E8;
    'dispatch: loop {
        match pc {
            0x827055E8 => {
    //   block [0x827055E8..0x8270565C)
	// 827055E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827055EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827055F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827055F4: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827055F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827055FC: 390BAA04  addi r8, r11, -0x55fc
	ctx.r[8].s64 = ctx.r[11].s64 + -22012;
	// 82705600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705604: 392AEE3C  addi r9, r10, -0x11c4
	ctx.r[9].s64 = ctx.r[10].s64 + -4548;
	// 82705608: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270560C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82705610: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705614: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270561C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270562C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705630: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 82705634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705638: 386B0514  addi r3, r11, 0x514
	ctx.r[3].s64 = ctx.r[11].s64 + 1300;
	// 8270563C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82705640: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705644: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82705648: 4BD617D9  bl 0x82466e20
	ctx.lr = 0x8270564C;
	sub_82466E20(ctx, base);
	// 8270564C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705660 size=108
    let mut pc: u32 = 0x82705660;
    'dispatch: loop {
        match pc {
            0x82705660 => {
    //   block [0x82705660..0x827056CC)
	// 82705660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270566C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705670: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705674: 38EBEE50  addi r7, r11, -0x11b0
	ctx.r[7].s64 = ctx.r[11].s64 + -4528;
	// 82705678: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270567C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 82705680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705684: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270568C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705690: 386A0544  addi r3, r10, 0x544
	ctx.r[3].s64 = ctx.r[10].s64 + 1348;
	// 82705694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270569C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827056A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827056A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827056A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827056AC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 827056B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827056B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827056B8: 4BD61769  bl 0x82466e20
	ctx.lr = 0x827056BC;
	sub_82466E20(ctx, base);
	// 827056BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827056C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827056C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827056C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827056D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827056D0 size=92
    let mut pc: u32 = 0x827056D0;
    'dispatch: loop {
        match pc {
            0x827056D0 => {
    //   block [0x827056D0..0x8270572C)
	// 827056D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827056D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827056D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827056DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827056E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827056E4: 4BDA7595  bl 0x824acc78
	ctx.lr = 0x827056E8;
	sub_824ACC78(ctx, base);
	// 827056E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827056EC: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 827056F0: 394BC1E8  addi r10, r11, -0x3e18
	ctx.r[10].s64 = ctx.r[11].s64 + -15896;
	// 827056F4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827056F8: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 827056FC: 396B0574  addi r11, r11, 0x574
	ctx.r[11].s64 = ctx.r[11].s64 + 1396;
	// 82705700: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705704: 39481AD8  addi r10, r8, 0x1ad8
	ctx.r[10].s64 = ctx.r[8].s64 + 6872;
	// 82705708: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270570C: 39491AF0  addi r10, r9, 0x1af0
	ctx.r[10].s64 = ctx.r[9].s64 + 6896;
	// 82705710: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705714: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705718: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270571C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82705720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705730 size=112
    let mut pc: u32 = 0x82705730;
    'dispatch: loop {
        match pc {
            0x82705730 => {
    //   block [0x82705730..0x827057A0)
	// 82705730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270573C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705740: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705744: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 82705748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270574C: 390BEE80  addi r8, r11, -0x1180
	ctx.r[8].s64 = ctx.r[11].s64 + -4480;
	// 82705750: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82705754: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 82705758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270575C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705768: 386A0584  addi r3, r10, 0x584
	ctx.r[3].s64 = ctx.r[10].s64 + 1412;
	// 8270576C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270577C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705784: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 82705788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270578C: 4BD61695  bl 0x82466e20
	ctx.lr = 0x82705790;
	sub_82466E20(ctx, base);
	// 82705790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270579C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827057A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827057A0 size=108
    let mut pc: u32 = 0x827057A0;
    'dispatch: loop {
        match pc {
            0x827057A0 => {
    //   block [0x827057A0..0x8270580C)
	// 827057A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827057A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827057A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827057AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827057B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827057B4: 38EBEF10  addi r7, r11, -0x10f0
	ctx.r[7].s64 = ctx.r[11].s64 + -4336;
	// 827057B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 827057BC: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 827057C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827057C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827057C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827057CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827057D0: 386A05B4  addi r3, r10, 0x5b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1460;
	// 827057D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827057D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827057DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827057E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827057E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827057E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827057EC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827057F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827057F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827057F8: 4BD61629  bl 0x82466e20
	ctx.lr = 0x827057FC;
	sub_82466E20(ctx, base);
	// 827057FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705810 size=92
    let mut pc: u32 = 0x82705810;
    'dispatch: loop {
        match pc {
            0x82705810 => {
    //   block [0x82705810..0x8270586C)
	// 82705810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705818: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270581C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705824: 4BDA79E5  bl 0x824ad208
	ctx.lr = 0x82705828;
	sub_824AD208(ctx, base);
	// 82705828: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270582C: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705830: 394BC2C0  addi r10, r11, -0x3d40
	ctx.r[10].s64 = ctx.r[11].s64 + -15680;
	// 82705834: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705838: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 8270583C: 396B05E4  addi r11, r11, 0x5e4
	ctx.r[11].s64 = ctx.r[11].s64 + 1508;
	// 82705840: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705844: 39481B40  addi r10, r8, 0x1b40
	ctx.r[10].s64 = ctx.r[8].s64 + 6976;
	// 82705848: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270584C: 39491B58  addi r10, r9, 0x1b58
	ctx.r[10].s64 = ctx.r[9].s64 + 7000;
	// 82705850: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705854: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705858: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270585C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82705860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705870 size=112
    let mut pc: u32 = 0x82705870;
    'dispatch: loop {
        match pc {
            0x82705870 => {
    //   block [0x82705870..0x827058E0)
	// 82705870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270587C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705880: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705884: 38AA0004  addi r5, r10, 4
	ctx.r[5].s64 = ctx.r[10].s64 + 4;
	// 82705888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270588C: 390BEF58  addi r8, r11, -0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + -4264;
	// 82705890: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82705894: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 82705898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270589C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827058A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827058A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827058A8: 386A05F4  addi r3, r10, 0x5f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1524;
	// 827058AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827058B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827058B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827058B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827058BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827058C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827058C4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827058C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827058CC: 4BD61555  bl 0x82466e20
	ctx.lr = 0x827058D0;
	sub_82466E20(ctx, base);
	// 827058D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827058D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827058D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827058DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827058E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827058E0 size=24
    let mut pc: u32 = 0x827058E0;
    'dispatch: loop {
        match pc {
            0x827058E0 => {
    //   block [0x827058E0..0x827058F8)
	// 827058E0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827058E4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827058E8: 394AAA60  addi r10, r10, -0x55a0
	ctx.r[10].s64 = ctx.r[10].s64 + -21920;
	// 827058EC: 816BAA40  lwz r11, -0x55c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21952 as u32) ) } as u64;
	// 827058F0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 827058F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827058F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827058F8 size=116
    let mut pc: u32 = 0x827058F8;
    'dispatch: loop {
        match pc {
            0x827058F8 => {
    //   block [0x827058F8..0x8270596C)
	// 827058F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827058FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705904: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705908: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270590C: 390BAA60  addi r8, r11, -0x55a0
	ctx.r[8].s64 = ctx.r[11].s64 + -21920;
	// 82705910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705914: 392AF028  addi r9, r10, -0xfd8
	ctx.r[9].s64 = ctx.r[10].s64 + -4056;
	// 82705918: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270591C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82705920: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705924: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270592C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270593C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705940: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 82705944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705948: 386B0624  addi r3, r11, 0x624
	ctx.r[3].s64 = ctx.r[11].s64 + 1572;
	// 8270594C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82705950: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705954: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82705958: 4BD614C9  bl 0x82466e20
	ctx.lr = 0x8270595C;
	sub_82466E20(ctx, base);
	// 8270595C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705970 size=100
    let mut pc: u32 = 0x82705970;
    'dispatch: loop {
        match pc {
            0x82705970 => {
    //   block [0x82705970..0x827059D4)
	// 82705970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270597C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705984: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 82705988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270598C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705990: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 82705994: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270599C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827059A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827059A4: 386A0654  addi r3, r10, 0x654
	ctx.r[3].s64 = ctx.r[10].s64 + 1620;
	// 827059A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827059AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827059B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827059B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827059B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827059BC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827059C0: 4BD61461  bl 0x82466e20
	ctx.lr = 0x827059C4;
	sub_82466E20(ctx, base);
	// 827059C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827059C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827059CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827059D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827059D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827059D8 size=112
    let mut pc: u32 = 0x827059D8;
    'dispatch: loop {
        match pc {
            0x827059D8 => {
    //   block [0x827059D8..0x82705A48)
	// 827059D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827059DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827059E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827059E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827059E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827059EC: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 827059F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827059F4: 390BF098  addi r8, r11, -0xf68
	ctx.r[8].s64 = ctx.r[11].s64 + -3944;
	// 827059F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827059FC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 82705A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705A04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705A10: 386A0684  addi r3, r10, 0x684
	ctx.r[3].s64 = ctx.r[10].s64 + 1668;
	// 82705A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705A2C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705A34: 4BD613ED  bl 0x82466e20
	ctx.lr = 0x82705A38;
	sub_82466E20(ctx, base);
	// 82705A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705A48 size=112
    let mut pc: u32 = 0x82705A48;
    'dispatch: loop {
        match pc {
            0x82705A48 => {
    //   block [0x82705A48..0x82705AB8)
	// 82705A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705A54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705A58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705A5C: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82705A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705A64: 390BF0F4  addi r8, r11, -0xf0c
	ctx.r[8].s64 = ctx.r[11].s64 + -3852;
	// 82705A68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705A6C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 82705A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705A80: 386A06B4  addi r3, r10, 0x6b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1716;
	// 82705A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705A9C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82705AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705AA4: 4BD6137D  bl 0x82466e20
	ctx.lr = 0x82705AA8;
	sub_82466E20(ctx, base);
	// 82705AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705AB8 size=92
    let mut pc: u32 = 0x82705AB8;
    'dispatch: loop {
        match pc {
            0x82705AB8 => {
    //   block [0x82705AB8..0x82705B14)
	// 82705AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705AC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82705AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82705ACC: 4BDA7DFD  bl 0x824ad8c8
	ctx.lr = 0x82705AD0;
	sub_824AD8C8(ctx, base);
	// 82705AD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82705AD4: 3D008249  lis r8, -0x7db7
	ctx.r[8].s64 = -2109145088;
	// 82705AD8: 394BC344  addi r10, r11, -0x3cbc
	ctx.r[10].s64 = ctx.r[11].s64 + -15548;
	// 82705ADC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705AE0: 3D208249  lis r9, -0x7db7
	ctx.r[9].s64 = -2109145088;
	// 82705AE4: 396B06E4  addi r11, r11, 0x6e4
	ctx.r[11].s64 = ctx.r[11].s64 + 1764;
	// 82705AE8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82705AEC: 39481CF8  addi r10, r8, 0x1cf8
	ctx.r[10].s64 = ctx.r[8].s64 + 7416;
	// 82705AF0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82705AF4: 39491CE0  addi r10, r9, 0x1ce0
	ctx.r[10].s64 = ctx.r[9].s64 + 7392;
	// 82705AF8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82705AFC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82705B00: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82705B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705B18 size=112
    let mut pc: u32 = 0x82705B18;
    'dispatch: loop {
        match pc {
            0x82705B18 => {
    //   block [0x82705B18..0x82705B88)
	// 82705B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705B24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705B28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705B2C: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705B30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705B34: 390BF110  addi r8, r11, -0xef0
	ctx.r[8].s64 = ctx.r[11].s64 + -3824;
	// 82705B38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82705B3C: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 82705B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705B50: 386A06F4  addi r3, r10, 0x6f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1780;
	// 82705B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705B6C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705B74: 4BD612AD  bl 0x82466e20
	ctx.lr = 0x82705B78;
	sub_82466E20(ctx, base);
	// 82705B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705B88 size=108
    let mut pc: u32 = 0x82705B88;
    'dispatch: loop {
        match pc {
            0x82705B88 => {
    //   block [0x82705B88..0x82705BF4)
	// 82705B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705B94: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705B9C: 38EBF158  addi r7, r11, -0xea8
	ctx.r[7].s64 = ctx.r[11].s64 + -3752;
	// 82705BA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82705BA4: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 82705BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705BAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82705BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705BB8: 386A0724  addi r3, r10, 0x724
	ctx.r[3].s64 = ctx.r[10].s64 + 1828;
	// 82705BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705BD4: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82705BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705BE0: 4BD61241  bl 0x82466e20
	ctx.lr = 0x82705BE4;
	sub_82466E20(ctx, base);
	// 82705BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705BF8 size=112
    let mut pc: u32 = 0x82705BF8;
    'dispatch: loop {
        match pc {
            0x82705BF8 => {
    //   block [0x82705BF8..0x82705C68)
	// 82705BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705C08: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705C0C: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705C14: 390BF188  addi r8, r11, -0xe78
	ctx.r[8].s64 = ctx.r[11].s64 + -3704;
	// 82705C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705C1C: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 82705C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705C30: 386A0754  addi r3, r10, 0x754
	ctx.r[3].s64 = ctx.r[10].s64 + 1876;
	// 82705C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705C4C: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 82705C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705C54: 4BD611CD  bl 0x82466e20
	ctx.lr = 0x82705C58;
	sub_82466E20(ctx, base);
	// 82705C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82705C68 size=24
    let mut pc: u32 = 0x82705C68;
    'dispatch: loop {
        match pc {
            0x82705C68 => {
    //   block [0x82705C68..0x82705C80)
	// 82705C68: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705C6C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82705C70: 394AAC10  addi r10, r10, -0x53f0
	ctx.r[10].s64 = ctx.r[10].s64 + -21488;
	// 82705C74: 816BABF8  lwz r11, -0x5408(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21512 as u32) ) } as u64;
	// 82705C78: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82705C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705C80 size=116
    let mut pc: u32 = 0x82705C80;
    'dispatch: loop {
        match pc {
            0x82705C80 => {
    //   block [0x82705C80..0x82705CF4)
	// 82705C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705C8C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82705C90: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82705C94: 390BAC10  addi r8, r11, -0x53f0
	ctx.r[8].s64 = ctx.r[11].s64 + -21488;
	// 82705C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705C9C: 392AF1FC  addi r9, r10, -0xe04
	ctx.r[9].s64 = ctx.r[10].s64 + -3588;
	// 82705CA0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705CA4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82705CA8: 38AA0C78  addi r5, r10, 0xc78
	ctx.r[5].s64 = ctx.r[10].s64 + 3192;
	// 82705CAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705CB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705CB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705CB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705CC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705CC4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82705CC8: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 82705CCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705CD0: 386B0784  addi r3, r11, 0x784
	ctx.r[3].s64 = ctx.r[11].s64 + 1924;
	// 82705CD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82705CD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705CDC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82705CE0: 4BD61141  bl 0x82466e20
	ctx.lr = 0x82705CE4;
	sub_82466E20(ctx, base);
	// 82705CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705CF8 size=116
    let mut pc: u32 = 0x82705CF8;
    'dispatch: loop {
        match pc {
            0x82705CF8 => {
    //   block [0x82705CF8..0x82705D6C)
	// 82705CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705D04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82705D08: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82705D0C: 390AF228  addi r8, r10, -0xdd8
	ctx.r[8].s64 = ctx.r[10].s64 + -3544;
	// 82705D10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82705D14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705D18: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705D1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705D20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82705D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705D2C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 82705D30: 396BF2D0  addi r11, r11, -0xd30
	ctx.r[11].s64 = ctx.r[11].s64 + -3376;
	// 82705D34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705D3C: 386A07B4  addi r3, r10, 0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1972;
	// 82705D40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82705D44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705D48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82705D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705D54: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 82705D58: 4BD610C9  bl 0x82466e20
	ctx.lr = 0x82705D5C;
	sub_82466E20(ctx, base);
	// 82705D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705D70 size=112
    let mut pc: u32 = 0x82705D70;
    'dispatch: loop {
        match pc {
            0x82705D70 => {
    //   block [0x82705D70..0x82705DE0)
	// 82705D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705D7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705D80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705D84: 38AA0C48  addi r5, r10, 0xc48
	ctx.r[5].s64 = ctx.r[10].s64 + 3144;
	// 82705D88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705D8C: 390BF308  addi r8, r11, -0xcf8
	ctx.r[8].s64 = ctx.r[11].s64 + -3320;
	// 82705D90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82705D94: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 82705D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705D9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705DA8: 386A07E4  addi r3, r10, 0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2020;
	// 82705DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705DC4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705DCC: 4BD61055  bl 0x82466e20
	ctx.lr = 0x82705DD0;
	sub_82466E20(ctx, base);
	// 82705DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705DE0 size=108
    let mut pc: u32 = 0x82705DE0;
    'dispatch: loop {
        match pc {
            0x82705DE0 => {
    //   block [0x82705DE0..0x82705E4C)
	// 82705DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705DEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705DF4: 38EBF338  addi r7, r11, -0xcc8
	ctx.r[7].s64 = ctx.r[11].s64 + -3272;
	// 82705DF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82705DFC: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 82705E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705E04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705E08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82705E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705E10: 386A0814  addi r3, r10, 0x814
	ctx.r[3].s64 = ctx.r[10].s64 + 2068;
	// 82705E14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705E2C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82705E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705E38: 4BD60FE9  bl 0x82466e20
	ctx.lr = 0x82705E3C;
	sub_82466E20(ctx, base);
	// 82705E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705E50 size=112
    let mut pc: u32 = 0x82705E50;
    'dispatch: loop {
        match pc {
            0x82705E50 => {
    //   block [0x82705E50..0x82705EC0)
	// 82705E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705E60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705E64: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82705E68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705E6C: 390BF368  addi r8, r11, -0xc98
	ctx.r[8].s64 = ctx.r[11].s64 + -3224;
	// 82705E70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82705E74: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 82705E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705E7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82705E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705E88: 386A0844  addi r3, r10, 0x844
	ctx.r[3].s64 = ctx.r[10].s64 + 2116;
	// 82705E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82705E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705EA4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82705EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705EAC: 4BD60F75  bl 0x82466e20
	ctx.lr = 0x82705EB0;
	sub_82466E20(ctx, base);
	// 82705EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705EC0 size=100
    let mut pc: u32 = 0x82705EC0;
    'dispatch: loop {
        match pc {
            0x82705EC0 => {
    //   block [0x82705EC0..0x82705F24)
	// 82705EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82705ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705ED4: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82705ED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705EE0: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 82705EE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705EF4: 386A0874  addi r3, r10, 0x874
	ctx.r[3].s64 = ctx.r[10].s64 + 2164;
	// 82705EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705EFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705F00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82705F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705F08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705F0C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82705F10: 4BD60F11  bl 0x82466e20
	ctx.lr = 0x82705F14;
	sub_82466E20(ctx, base);
	// 82705F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705F28 size=100
    let mut pc: u32 = 0x82705F28;
    'dispatch: loop {
        match pc {
            0x82705F28 => {
    //   block [0x82705F28..0x82705F8C)
	// 82705F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705F34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705F3C: 38AA0E48  addi r5, r10, 0xe48
	ctx.r[5].s64 = ctx.r[10].s64 + 3656;
	// 82705F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705F48: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 82705F4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705F5C: 386A08A4  addi r3, r10, 0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + 2212;
	// 82705F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705F68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82705F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705F70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82705F74: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82705F78: 4BD60EA9  bl 0x82466e20
	ctx.lr = 0x82705F7C;
	sub_82466E20(ctx, base);
	// 82705F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82705F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82705F90 size=108
    let mut pc: u32 = 0x82705F90;
    'dispatch: loop {
        match pc {
            0x82705F90 => {
    //   block [0x82705F90..0x82705FFC)
	// 82705F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82705F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82705F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82705F9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82705FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82705FA4: 38EBF3D8  addi r7, r11, -0xc28
	ctx.r[7].s64 = ctx.r[11].s64 + -3112;
	// 82705FA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82705FAC: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 82705FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82705FB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82705FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82705FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82705FC0: 386A08D4  addi r3, r10, 0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2260;
	// 82705FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82705FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82705FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82705FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82705FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82705FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82705FDC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82705FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82705FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82705FE8: 4BD60E39  bl 0x82466e20
	ctx.lr = 0x82705FEC;
	sub_82466E20(ctx, base);
	// 82705FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82705FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82705FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82705FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706000 size=112
    let mut pc: u32 = 0x82706000;
    'dispatch: loop {
        match pc {
            0x82706000 => {
    //   block [0x82706000..0x82706070)
	// 82706000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270600C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706010: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706014: 38AA0E48  addi r5, r10, 0xe48
	ctx.r[5].s64 = ctx.r[10].s64 + 3656;
	// 82706018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270601C: 390BF408  addi r8, r11, -0xbf8
	ctx.r[8].s64 = ctx.r[11].s64 + -3064;
	// 82706020: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82706024: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 82706028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270602C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706038: 386A0904  addi r3, r10, 0x904
	ctx.r[3].s64 = ctx.r[10].s64 + 2308;
	// 8270603C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270604C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706054: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82706058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270605C: 4BD60DC5  bl 0x82466e20
	ctx.lr = 0x82706060;
	sub_82466E20(ctx, base);
	// 82706060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270606C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706070 size=108
    let mut pc: u32 = 0x82706070;
    'dispatch: loop {
        match pc {
            0x82706070 => {
    //   block [0x82706070..0x827060DC)
	// 82706070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270607C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706084: 392BF4E8  addi r9, r11, -0xb18
	ctx.r[9].s64 = ctx.r[11].s64 + -2840;
	// 82706088: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8270608C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82706090: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 82706094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706098: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270609C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827060A0: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 827060A4: 386A0934  addi r3, r10, 0x934
	ctx.r[3].s64 = ctx.r[10].s64 + 2356;
	// 827060A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827060AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827060B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827060B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827060B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827060BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827060C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827060C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827060C8: 4BD60D59  bl 0x82466e20
	ctx.lr = 0x827060CC;
	sub_82466E20(ctx, base);
	// 827060CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827060D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827060D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827060D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827060E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827060E0 size=112
    let mut pc: u32 = 0x827060E0;
    'dispatch: loop {
        match pc {
            0x827060E0 => {
    //   block [0x827060E0..0x82706150)
	// 827060E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827060E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827060E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827060EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827060F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827060F4: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827060F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827060FC: 390BF590  addi r8, r11, -0xa70
	ctx.r[8].s64 = ctx.r[11].s64 + -2672;
	// 82706100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706104: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 82706108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270610C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706118: 386A0964  addi r3, r10, 0x964
	ctx.r[3].s64 = ctx.r[10].s64 + 2404;
	// 8270611C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270612C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706134: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 82706138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270613C: 4BD60CE5  bl 0x82466e20
	ctx.lr = 0x82706140;
	sub_82466E20(ctx, base);
	// 82706140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270614C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706150 size=108
    let mut pc: u32 = 0x82706150;
    'dispatch: loop {
        match pc {
            0x82706150 => {
    //   block [0x82706150..0x827061BC)
	// 82706150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270615C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706164: 392BF624  addi r9, r11, -0x9dc
	ctx.r[9].s64 = ctx.r[11].s64 + -2524;
	// 82706168: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8270616C: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 82706170: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 82706174: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706178: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270617C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706180: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 82706184: 386A0994  addi r3, r10, 0x994
	ctx.r[3].s64 = ctx.r[10].s64 + 2452;
	// 82706188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270618C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706190: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706194: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706198: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270619C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827061A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827061A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827061A8: 4BD60C79  bl 0x82466e20
	ctx.lr = 0x827061AC;
	sub_82466E20(ctx, base);
	// 827061AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827061B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827061B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827061B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827061C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827061C0 size=112
    let mut pc: u32 = 0x827061C0;
    'dispatch: loop {
        match pc {
            0x827061C0 => {
    //   block [0x827061C0..0x82706230)
	// 827061C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827061C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827061C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827061CC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827061D0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827061D4: 392BF5F8  addi r9, r11, -0xa08
	ctx.r[9].s64 = ctx.r[11].s64 + -2568;
	// 827061D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827061DC: 390900E8  addi r8, r9, 0xe8
	ctx.r[8].s64 = ctx.r[9].s64 + 232;
	// 827061E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827061E4: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 827061E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827061EC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827061F0: 38C00140  li r6, 0x140
	ctx.r[6].s64 = 320;
	// 827061F4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827061F8: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 827061FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706200: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706204: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706208: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270620C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706210: 386B09C4  addi r3, r11, 0x9c4
	ctx.r[3].s64 = ctx.r[11].s64 + 2500;
	// 82706214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706218: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270621C: 4BD60C05  bl 0x82466e20
	ctx.lr = 0x82706220;
	sub_82466E20(ctx, base);
	// 82706220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270622C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82706230 size=24
    let mut pc: u32 = 0x82706230;
    'dispatch: loop {
        match pc {
            0x82706230 => {
    //   block [0x82706230..0x82706248)
	// 82706230: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82706234: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82706238: 394AAEC0  addi r10, r10, -0x5140
	ctx.r[10].s64 = ctx.r[10].s64 + -20800;
	// 8270623C: 816BAEA8  lwz r11, -0x5158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20824 as u32) ) } as u64;
	// 82706240: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82706244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706248 size=112
    let mut pc: u32 = 0x82706248;
    'dispatch: loop {
        match pc {
            0x82706248 => {
    //   block [0x82706248..0x827062B8)
	// 82706248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270624C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706254: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82706258: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270625C: 392AF754  addi r9, r10, -0x8ac
	ctx.r[9].s64 = ctx.r[10].s64 + -2220;
	// 82706260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706264: 390BAEC0  addi r8, r11, -0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + -20800;
	// 82706268: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8270626C: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 82706270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706274: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270627C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82706280: 386A09F4  addi r3, r10, 0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2548;
	// 82706284: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706288: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270628C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270629C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827062A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827062A4: 4BD60B7D  bl 0x82466e20
	ctx.lr = 0x827062A8;
	sub_82466E20(ctx, base);
	// 827062A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827062AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827062B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827062B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827062B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827062B8 size=112
    let mut pc: u32 = 0x827062B8;
    'dispatch: loop {
        match pc {
            0x827062B8 => {
    //   block [0x827062B8..0x82706328)
	// 827062B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827062BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827062C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827062C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827062C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827062CC: 392BF818  addi r9, r11, -0x7e8
	ctx.r[9].s64 = ctx.r[11].s64 + -2024;
	// 827062D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827062D4: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 827062D8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827062DC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827062E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827062E4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827062E8: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 827062EC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827062F0: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 827062F4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827062F8: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827062FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706300: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706304: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706308: 386B0A24  addi r3, r11, 0xa24
	ctx.r[3].s64 = ctx.r[11].s64 + 2596;
	// 8270630C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706310: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706314: 4BD60B0D  bl 0x82466e20
	ctx.lr = 0x82706318;
	sub_82466E20(ctx, base);
	// 82706318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270631C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706328 size=108
    let mut pc: u32 = 0x82706328;
    'dispatch: loop {
        match pc {
            0x82706328 => {
    //   block [0x82706328..0x82706394)
	// 82706328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706334: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270633C: 38EBF848  addi r7, r11, -0x7b8
	ctx.r[7].s64 = ctx.r[11].s64 + -1976;
	// 82706340: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82706344: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 82706348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270634C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82706354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706358: 386A0A54  addi r3, r10, 0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + 2644;
	// 8270635C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270636C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706374: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82706378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270637C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706380: 4BD60AA1  bl 0x82466e20
	ctx.lr = 0x82706384;
	sub_82466E20(ctx, base);
	// 82706384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270638C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706398 size=108
    let mut pc: u32 = 0x82706398;
    'dispatch: loop {
        match pc {
            0x82706398 => {
    //   block [0x82706398..0x82706404)
	// 82706398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270639C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827063A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827063A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827063A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827063AC: 38EBF8A8  addi r7, r11, -0x758
	ctx.r[7].s64 = ctx.r[11].s64 + -1880;
	// 827063B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 827063B4: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 827063B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827063BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827063C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827063C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827063C8: 386A0A84  addi r3, r10, 0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + 2692;
	// 827063CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827063D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827063D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827063D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827063DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827063E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827063E4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827063E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827063EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827063F0: 4BD60A31  bl 0x82466e20
	ctx.lr = 0x827063F4;
	sub_82466E20(ctx, base);
	// 827063F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827063F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827063FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706408 size=112
    let mut pc: u32 = 0x82706408;
    'dispatch: loop {
        match pc {
            0x82706408 => {
    //   block [0x82706408..0x82706478)
	// 82706408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270640C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706414: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706418: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270641C: 38AA0874  addi r5, r10, 0x874
	ctx.r[5].s64 = ctx.r[10].s64 + 2164;
	// 82706420: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706424: 390BF920  addi r8, r11, -0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + -1760;
	// 82706428: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8270642C: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 82706430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706434: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270643C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706440: 386A0AB4  addi r3, r10, 0xab4
	ctx.r[3].s64 = ctx.r[10].s64 + 2740;
	// 82706444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270644C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270645C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82706460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706464: 4BD609BD  bl 0x82466e20
	ctx.lr = 0x82706468;
	sub_82466E20(ctx, base);
	// 82706468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270646C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706478 size=100
    let mut pc: u32 = 0x82706478;
    'dispatch: loop {
        match pc {
            0x82706478 => {
    //   block [0x82706478..0x827064DC)
	// 82706478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270647C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706484: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270648C: 38AA00D4  addi r5, r10, 0xd4
	ctx.r[5].s64 = ctx.r[10].s64 + 212;
	// 82706490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706498: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 8270649C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827064A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827064A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827064A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827064AC: 386A0AE4  addi r3, r10, 0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + 2788;
	// 827064B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827064B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827064B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827064BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827064C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827064C4: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 827064C8: 4BD60959  bl 0x82466e20
	ctx.lr = 0x827064CC;
	sub_82466E20(ctx, base);
	// 827064CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827064D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827064D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827064D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827064E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827064E0 size=108
    let mut pc: u32 = 0x827064E0;
    'dispatch: loop {
        match pc {
            0x827064E0 => {
    //   block [0x827064E0..0x8270654C)
	// 827064E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827064E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827064E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827064EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827064F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827064F4: 38EBF9CC  addi r7, r11, -0x634
	ctx.r[7].s64 = ctx.r[11].s64 + -1588;
	// 827064F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827064FC: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 82706500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706504: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270650C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706510: 386A0B14  addi r3, r10, 0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + 2836;
	// 82706514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270651C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270652C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82706530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706538: 4BD608E9  bl 0x82466e20
	ctx.lr = 0x8270653C;
	sub_82466E20(ctx, base);
	// 8270653C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706550 size=112
    let mut pc: u32 = 0x82706550;
    'dispatch: loop {
        match pc {
            0x82706550 => {
    //   block [0x82706550..0x827065C0)
	// 82706550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270655C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706560: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706564: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270656C: 390BF9FC  addi r8, r11, -0x604
	ctx.r[8].s64 = ctx.r[11].s64 + -1540;
	// 82706570: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706574: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 82706578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270657C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706588: 386A0B44  addi r3, r10, 0xb44
	ctx.r[3].s64 = ctx.r[10].s64 + 2884;
	// 8270658C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270659C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827065A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827065A4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 827065A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827065AC: 4BD60875  bl 0x82466e20
	ctx.lr = 0x827065B0;
	sub_82466E20(ctx, base);
	// 827065B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827065B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827065B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827065BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827065C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827065C0 size=100
    let mut pc: u32 = 0x827065C0;
    'dispatch: loop {
        match pc {
            0x827065C0 => {
    //   block [0x827065C0..0x82706624)
	// 827065C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827065C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827065C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827065CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827065D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827065D4: 38AA0BD8  addi r5, r10, 0xbd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3032;
	// 827065D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827065DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827065E0: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 827065E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827065E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827065EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827065F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827065F4: 386A0B74  addi r3, r10, 0xb74
	ctx.r[3].s64 = ctx.r[10].s64 + 2932;
	// 827065F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827065FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706600: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82706604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270660C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82706610: 4BD60811  bl 0x82466e20
	ctx.lr = 0x82706614;
	sub_82466E20(ctx, base);
	// 82706614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270661C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706628 size=108
    let mut pc: u32 = 0x82706628;
    'dispatch: loop {
        match pc {
            0x82706628 => {
    //   block [0x82706628..0x82706694)
	// 82706628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270662C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706634: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270663C: 38EBFF98  addi r7, r11, -0x68
	ctx.r[7].s64 = ctx.r[11].s64 + -104;
	// 82706640: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82706644: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 82706648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270664C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82706654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706658: 386A0BA8  addi r3, r10, 0xba8
	ctx.r[3].s64 = ctx.r[10].s64 + 2984;
	// 8270665C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270666C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706674: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82706678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270667C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706680: 4BD607A1  bl 0x82466e20
	ctx.lr = 0x82706684;
	sub_82466E20(ctx, base);
	// 82706684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270668C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82706698 size=24
    let mut pc: u32 = 0x82706698;
    'dispatch: loop {
        match pc {
            0x82706698 => {
    //   block [0x82706698..0x827066B0)
	// 82706698: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270669C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827066A0: 394AB320  addi r10, r10, -0x4ce0
	ctx.r[10].s64 = ctx.r[10].s64 + -19680;
	// 827066A4: 816BB318  lwz r11, -0x4ce8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19688 as u32) ) } as u64;
	// 827066A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 827066AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827066B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827066B0 size=116
    let mut pc: u32 = 0x827066B0;
    'dispatch: loop {
        match pc {
            0x827066B0 => {
    //   block [0x827066B0..0x82706724)
	// 827066B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827066B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827066B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827066BC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827066C0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827066C4: 390BB320  addi r8, r11, -0x4ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -19680;
	// 827066C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827066CC: 392A0038  addi r9, r10, 0x38
	ctx.r[9].s64 = ctx.r[10].s64 + 56;
	// 827066D0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827066D4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 827066D8: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827066DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827066E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827066E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827066E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827066EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827066F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827066F4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827066F8: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 827066FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706700: 386B0BD8  addi r3, r11, 0xbd8
	ctx.r[3].s64 = ctx.r[11].s64 + 3032;
	// 82706704: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706708: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270670C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82706710: 4BD60711  bl 0x82466e20
	ctx.lr = 0x82706714;
	sub_82466E20(ctx, base);
	// 82706714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270671C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706728 size=96
    let mut pc: u32 = 0x82706728;
    'dispatch: loop {
        match pc {
            0x82706728 => {
    //   block [0x82706728..0x82706788)
	// 82706728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706730: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706734: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82706738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8270673C: 4BD8814D  bl 0x8248e888
	ctx.lr = 0x82706740;
	sub_8248E888(ctx, base);
	// 82706740: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82706744: 3CE0824A  lis r7, -0x7db6
	ctx.r[7].s64 = -2109079552;
	// 82706748: 394BC4E8  addi r10, r11, -0x3b18
	ctx.r[10].s64 = ctx.r[11].s64 + -15128;
	// 8270674C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706750: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 82706754: 392BD8C4  addi r9, r11, -0x273c
	ctx.r[9].s64 = ctx.r[11].s64 + -10044;
	// 82706758: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270675C: 396B0C08  addi r11, r11, 0xc08
	ctx.r[11].s64 = ctx.r[11].s64 + 3080;
	// 82706760: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706764: 3947AEC0  addi r10, r7, -0x5140
	ctx.r[10].s64 = ctx.r[7].s64 + -20800;
	// 82706768: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270676C: 3948AEA8  addi r10, r8, -0x5158
	ctx.r[10].s64 = ctx.r[8].s64 + -20824;
	// 82706770: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706774: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82706778: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 8270677C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706788 size=100
    let mut pc: u32 = 0x82706788;
    'dispatch: loop {
        match pc {
            0x82706788 => {
    //   block [0x82706788..0x827067EC)
	// 82706788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270678C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706794: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270679C: 38AA0244  addi r5, r10, 0x244
	ctx.r[5].s64 = ctx.r[10].s64 + 580;
	// 827067A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827067A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827067A8: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 827067AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827067B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827067B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827067B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827067BC: 386A0C18  addi r3, r10, 0xc18
	ctx.r[3].s64 = ctx.r[10].s64 + 3096;
	// 827067C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827067C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827067C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827067CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827067D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827067D4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 827067D8: 4BD60649  bl 0x82466e20
	ctx.lr = 0x827067DC;
	sub_82466E20(ctx, base);
	// 827067DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827067E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827067E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827067E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827067F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827067F0 size=112
    let mut pc: u32 = 0x827067F0;
    'dispatch: loop {
        match pc {
            0x827067F0 => {
    //   block [0x827067F0..0x82706860)
	// 827067F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827067F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827067F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827067FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82706800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706804: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82706808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270680C: 390B0068  addi r8, r11, 0x68
	ctx.r[8].s64 = ctx.r[11].s64 + 104;
	// 82706810: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82706814: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 82706818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270681C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706828: 386A0C48  addi r3, r10, 0xc48
	ctx.r[3].s64 = ctx.r[10].s64 + 3144;
	// 8270682C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270683C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706844: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82706848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270684C: 4BD605D5  bl 0x82466e20
	ctx.lr = 0x82706850;
	sub_82466E20(ctx, base);
	// 82706850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270685C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706860 size=112
    let mut pc: u32 = 0x82706860;
    'dispatch: loop {
        match pc {
            0x82706860 => {
    //   block [0x82706860..0x827068D0)
	// 82706860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270686C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706870: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706874: 38AA0514  addi r5, r10, 0x514
	ctx.r[5].s64 = ctx.r[10].s64 + 1300;
	// 82706878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270687C: 390B00C8  addi r8, r11, 0xc8
	ctx.r[8].s64 = ctx.r[11].s64 + 200;
	// 82706880: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706884: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 82706888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270688C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706898: 386A0C78  addi r3, r10, 0xc78
	ctx.r[3].s64 = ctx.r[10].s64 + 3192;
	// 8270689C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827068A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827068A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827068A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827068AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827068B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827068B4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 827068B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827068BC: 4BD60565  bl 0x82466e20
	ctx.lr = 0x827068C0;
	sub_82466E20(ctx, base);
	// 827068C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827068C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827068C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827068CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827068D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827068D0 size=92
    let mut pc: u32 = 0x827068D0;
    'dispatch: loop {
        match pc {
            0x827068D0 => {
    //   block [0x827068D0..0x8270692C)
	// 827068D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827068D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827068D8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827068DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827068E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827068E4: 4BDB1BF5  bl 0x824b84d8
	ctx.lr = 0x827068E8;
	sub_824B84D8(ctx, base);
	// 827068E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827068EC: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 827068F0: 394BC2D8  addi r10, r11, -0x3d28
	ctx.r[10].s64 = ctx.r[11].s64 + -15656;
	// 827068F4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827068F8: 3D20824A  lis r9, -0x7db6
	ctx.r[9].s64 = -2109079552;
	// 827068FC: 396B0CA8  addi r11, r11, 0xca8
	ctx.r[11].s64 = ctx.r[11].s64 + 3240;
	// 82706900: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706904: 3948B2E8  addi r10, r8, -0x4d18
	ctx.r[10].s64 = ctx.r[8].s64 + -19736;
	// 82706908: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270690C: 3949B2D0  addi r10, r9, -0x4d30
	ctx.r[10].s64 = ctx.r[9].s64 + -19760;
	// 82706910: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706914: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82706918: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270691C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82706920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706930 size=112
    let mut pc: u32 = 0x82706930;
    'dispatch: loop {
        match pc {
            0x82706930 => {
    //   block [0x82706930..0x827069A0)
	// 82706930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270693C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706940: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706944: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270694C: 390B00F8  addi r8, r11, 0xf8
	ctx.r[8].s64 = ctx.r[11].s64 + 248;
	// 82706950: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706954: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 82706958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270695C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706968: 386A0CB8  addi r3, r10, 0xcb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3256;
	// 8270696C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270697C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706984: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 82706988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270698C: 4BD60495  bl 0x82466e20
	ctx.lr = 0x82706990;
	sub_82466E20(ctx, base);
	// 82706990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270699C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827069A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827069A0 size=100
    let mut pc: u32 = 0x827069A0;
    'dispatch: loop {
        match pc {
            0x827069A0 => {
    //   block [0x827069A0..0x82706A04)
	// 827069A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827069A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827069A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827069AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827069B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827069B4: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827069B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827069BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827069C0: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 827069C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827069C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827069CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827069D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827069D4: 386A0CE8  addi r3, r10, 0xce8
	ctx.r[3].s64 = ctx.r[10].s64 + 3304;
	// 827069D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827069DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827069E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 827069E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827069E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 827069EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 827069F0: 4BD60431  bl 0x82466e20
	ctx.lr = 0x827069F4;
	sub_82466E20(ctx, base);
	// 827069F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827069F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827069FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706A08 size=108
    let mut pc: u32 = 0x82706A08;
    'dispatch: loop {
        match pc {
            0x82706A08 => {
    //   block [0x82706A08..0x82706A74)
	// 82706A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706A14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706A1C: 392B0138  addi r9, r11, 0x138
	ctx.r[9].s64 = ctx.r[11].s64 + 312;
	// 82706A20: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82706A24: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 82706A28: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 82706A2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706A30: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706A34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706A38: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 82706A3C: 386A0D18  addi r3, r10, 0xd18
	ctx.r[3].s64 = ctx.r[10].s64 + 3352;
	// 82706A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706A44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706A48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706A4C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706A50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706A54: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706A58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82706A5C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706A60: 4BD603C1  bl 0x82466e20
	ctx.lr = 0x82706A64;
	sub_82466E20(ctx, base);
	// 82706A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706A78 size=112
    let mut pc: u32 = 0x82706A78;
    'dispatch: loop {
        match pc {
            0x82706A78 => {
    //   block [0x82706A78..0x82706AE8)
	// 82706A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706A84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706A88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706A8C: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706A90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706A94: 390B01F8  addi r8, r11, 0x1f8
	ctx.r[8].s64 = ctx.r[11].s64 + 504;
	// 82706A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706A9C: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 82706AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706AA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706AB0: 386A0D48  addi r3, r10, 0xd48
	ctx.r[3].s64 = ctx.r[10].s64 + 3400;
	// 82706AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706ACC: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82706AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706AD4: 4BD6034D  bl 0x82466e20
	ctx.lr = 0x82706AD8;
	sub_82466E20(ctx, base);
	// 82706AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706AE8 size=112
    let mut pc: u32 = 0x82706AE8;
    'dispatch: loop {
        match pc {
            0x82706AE8 => {
    //   block [0x82706AE8..0x82706B58)
	// 82706AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706AF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706AF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706AFC: 38AA0624  addi r5, r10, 0x624
	ctx.r[5].s64 = ctx.r[10].s64 + 1572;
	// 82706B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706B04: 390B0244  addi r8, r11, 0x244
	ctx.r[8].s64 = ctx.r[11].s64 + 580;
	// 82706B08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706B0C: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 82706B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706B14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706B20: 386A0D78  addi r3, r10, 0xd78
	ctx.r[3].s64 = ctx.r[10].s64 + 3448;
	// 82706B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706B3C: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 82706B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706B44: 4BD602DD  bl 0x82466e20
	ctx.lr = 0x82706B48;
	sub_82466E20(ctx, base);
	// 82706B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706B58 size=92
    let mut pc: u32 = 0x82706B58;
    'dispatch: loop {
        match pc {
            0x82706B58 => {
    //   block [0x82706B58..0x82706BB4)
	// 82706B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706B60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706B64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82706B68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82706B6C: 4BDB373D  bl 0x824ba2a8
	ctx.lr = 0x82706B70;
	sub_824BA2A8(ctx, base);
	// 82706B70: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82706B74: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 82706B78: 394BC14C  addi r10, r11, -0x3eb4
	ctx.r[10].s64 = ctx.r[11].s64 + -16052;
	// 82706B7C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706B80: 3D20824A  lis r9, -0x7db6
	ctx.r[9].s64 = -2109079552;
	// 82706B84: 396B0DA8  addi r11, r11, 0xda8
	ctx.r[11].s64 = ctx.r[11].s64 + 3496;
	// 82706B88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706B8C: 3948B828  addi r10, r8, -0x47d8
	ctx.r[10].s64 = ctx.r[8].s64 + -18392;
	// 82706B90: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82706B94: 3949B810  addi r10, r9, -0x47f0
	ctx.r[10].s64 = ctx.r[9].s64 + -18416;
	// 82706B98: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706B9C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82706BA0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82706BA4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82706BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706BB8 size=112
    let mut pc: u32 = 0x82706BB8;
    'dispatch: loop {
        match pc {
            0x82706BB8 => {
    //   block [0x82706BB8..0x82706C28)
	// 82706BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706BC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706BC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706BCC: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82706BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706BD4: 390B0298  addi r8, r11, 0x298
	ctx.r[8].s64 = ctx.r[11].s64 + 664;
	// 82706BD8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82706BDC: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 82706BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706BE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706BF0: 386A0DB8  addi r3, r10, 0xdb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3512;
	// 82706BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706C0C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82706C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706C14: 4BD6020D  bl 0x82466e20
	ctx.lr = 0x82706C18;
	sub_82466E20(ctx, base);
	// 82706C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706C28 size=116
    let mut pc: u32 = 0x82706C28;
    'dispatch: loop {
        match pc {
            0x82706C28 => {
    //   block [0x82706C28..0x82706C9C)
	// 82706C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706C34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706C38: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82706C3C: 390B03B8  addi r8, r11, 0x3b8
	ctx.r[8].s64 = ctx.r[11].s64 + 952;
	// 82706C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706C44: 392A03A4  addi r9, r10, 0x3a4
	ctx.r[9].s64 = ctx.r[10].s64 + 932;
	// 82706C48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82706C4C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82706C50: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82706C54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706C5C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706C6C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706C70: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 82706C74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82706C78: 386B0DE8  addi r3, r11, 0xde8
	ctx.r[3].s64 = ctx.r[11].s64 + 3560;
	// 82706C7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706C80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706C84: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82706C88: 4BD60199  bl 0x82466e20
	ctx.lr = 0x82706C8C;
	sub_82466E20(ctx, base);
	// 82706C8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706CA0 size=112
    let mut pc: u32 = 0x82706CA0;
    'dispatch: loop {
        match pc {
            0x82706CA0 => {
    //   block [0x82706CA0..0x82706D10)
	// 82706CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706CAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706CB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706CB4: 38AA0EB8  addi r5, r10, 0xeb8
	ctx.r[5].s64 = ctx.r[10].s64 + 3768;
	// 82706CB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706CBC: 390B0448  addi r8, r11, 0x448
	ctx.r[8].s64 = ctx.r[11].s64 + 1096;
	// 82706CC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706CC4: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 82706CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706CCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706CD8: 386A0E18  addi r3, r10, 0xe18
	ctx.r[3].s64 = ctx.r[10].s64 + 3608;
	// 82706CDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706CF4: 38C00150  li r6, 0x150
	ctx.r[6].s64 = 336;
	// 82706CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706CFC: 4BD60125  bl 0x82466e20
	ctx.lr = 0x82706D00;
	sub_82466E20(ctx, base);
	// 82706D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706D10 size=104
    let mut pc: u32 = 0x82706D10;
    'dispatch: loop {
        match pc {
            0x82706D10 => {
    //   block [0x82706D10..0x82706D78)
	// 82706D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706D1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82706D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706D24: 392A0480  addi r9, r10, 0x480
	ctx.r[9].s64 = ctx.r[10].s64 + 1152;
	// 82706D28: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706D30: 38AA0CE8  addi r5, r10, 0xce8
	ctx.r[5].s64 = ctx.r[10].s64 + 3304;
	// 82706D34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706D44: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 82706D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706D4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706D50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82706D54: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82706D58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82706D5C: 386A0E48  addi r3, r10, 0xe48
	ctx.r[3].s64 = ctx.r[10].s64 + 3656;
	// 82706D60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82706D64: 4BD600BD  bl 0x82466e20
	ctx.lr = 0x82706D68;
	sub_82466E20(ctx, base);
	// 82706D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


