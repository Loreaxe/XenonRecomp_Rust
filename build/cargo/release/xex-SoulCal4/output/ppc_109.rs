pub fn sub_826705D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826705D8 size=112
    let mut pc: u32 = 0x826705D8;
    'dispatch: loop {
        match pc {
            0x826705D8 => {
    //   block [0x826705D8..0x82670648)
	// 826705D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826705DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826705E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826705E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826705E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826705EC: 38AA1A28  addi r5, r10, 0x1a28
	ctx.r[5].s64 = ctx.r[10].s64 + 6696;
	// 826705F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826705F4: 390BFEE8  addi r8, r11, -0x118
	ctx.r[8].s64 = ctx.r[11].s64 + -280;
	// 826705F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826705FC: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82670600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670604: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267060C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670610: 386A1A58  addi r3, r10, 0x1a58
	ctx.r[3].s64 = ctx.r[10].s64 + 6744;
	// 82670614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267061C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670634: 4BDF67ED  bl 0x82466e20
	ctx.lr = 0x82670638;
	sub_82466E20(ctx, base);
	// 82670638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670648 size=112
    let mut pc: u32 = 0x82670648;
    'dispatch: loop {
        match pc {
            0x82670648 => {
    //   block [0x82670648..0x826706B8)
	// 82670648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670654: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670658: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267065C: 38AA1A58  addi r5, r10, 0x1a58
	ctx.r[5].s64 = ctx.r[10].s64 + 6744;
	// 82670660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670664: 390BFF18  addi r8, r11, -0xe8
	ctx.r[8].s64 = ctx.r[11].s64 + -232;
	// 82670668: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267066C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82670670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267067C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670680: 386A1A88  addi r3, r10, 0x1a88
	ctx.r[3].s64 = ctx.r[10].s64 + 6792;
	// 82670684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267068C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267069C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826706A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826706A4: 4BDF677D  bl 0x82466e20
	ctx.lr = 0x826706A8;
	sub_82466E20(ctx, base);
	// 826706A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826706AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826706B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826706B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826706B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826706B8 size=112
    let mut pc: u32 = 0x826706B8;
    'dispatch: loop {
        match pc {
            0x826706B8 => {
    //   block [0x826706B8..0x82670728)
	// 826706B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826706BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826706C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826706C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826706C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826706CC: 38AA1A58  addi r5, r10, 0x1a58
	ctx.r[5].s64 = ctx.r[10].s64 + 6744;
	// 826706D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826706D4: 390BFF78  addi r8, r11, -0x88
	ctx.r[8].s64 = ctx.r[11].s64 + -136;
	// 826706D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826706DC: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826706E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826706E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826706E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826706EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826706F0: 386A1AB8  addi r3, r10, 0x1ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 6840;
	// 826706F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826706F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826706FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267070C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670714: 4BDF670D  bl 0x82466e20
	ctx.lr = 0x82670718;
	sub_82466E20(ctx, base);
	// 82670718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267071C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670728 size=112
    let mut pc: u32 = 0x82670728;
    'dispatch: loop {
        match pc {
            0x82670728 => {
    //   block [0x82670728..0x82670798)
	// 82670728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670738: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267073C: 38AA1A58  addi r5, r10, 0x1a58
	ctx.r[5].s64 = ctx.r[10].s64 + 6744;
	// 82670740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670744: 390BFFA8  addi r8, r11, -0x58
	ctx.r[8].s64 = ctx.r[11].s64 + -88;
	// 82670748: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267074C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82670750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670754: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267075C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670760: 386A1AE8  addi r3, r10, 0x1ae8
	ctx.r[3].s64 = ctx.r[10].s64 + 6888;
	// 82670764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267076C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267077C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670784: 4BDF669D  bl 0x82466e20
	ctx.lr = 0x82670788;
	sub_82466E20(ctx, base);
	// 82670788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670798 size=112
    let mut pc: u32 = 0x82670798;
    'dispatch: loop {
        match pc {
            0x82670798 => {
    //   block [0x82670798..0x82670808)
	// 82670798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826707A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826707A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826707A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826707AC: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 826707B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826707B4: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 826707B8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826707BC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826707C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826707C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826707C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826707CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826707D0: 386A1B18  addi r3, r10, 0x1b18
	ctx.r[3].s64 = ctx.r[10].s64 + 6936;
	// 826707D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826707D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826707DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826707E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826707E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826707E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826707EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826707F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826707F4: 4BDF662D  bl 0x82466e20
	ctx.lr = 0x826707F8;
	sub_82466E20(ctx, base);
	// 826707F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826707FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670808 size=116
    let mut pc: u32 = 0x82670808;
    'dispatch: loop {
        match pc {
            0x82670808 => {
    //   block [0x82670808..0x8267087C)
	// 82670808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670814: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82670818: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8267081C: 390A0080  addi r8, r10, 0x80
	ctx.r[8].s64 = ctx.r[10].s64 + 128;
	// 82670820: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670824: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82670828: 38AA1FC8  addi r5, r10, 0x1fc8
	ctx.r[5].s64 = ctx.r[10].s64 + 8136;
	// 8267082C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670830: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82670834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267083C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82670840: 396B18F0  addi r11, r11, 0x18f0
	ctx.r[11].s64 = ctx.r[11].s64 + 6384;
	// 82670844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670848: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267084C: 386A1B48  addi r3, r10, 0x1b48
	ctx.r[3].s64 = ctx.r[10].s64 + 6984;
	// 82670850: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82670854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670858: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267085C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670868: 4BDF65B9  bl 0x82466e20
	ctx.lr = 0x8267086C;
	sub_82466E20(ctx, base);
	// 8267086C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670880 size=100
    let mut pc: u32 = 0x82670880;
    'dispatch: loop {
        match pc {
            0x82670880 => {
    //   block [0x82670880..0x826708E4)
	// 82670880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267088C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670894: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82670898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267089C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826708A0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826708A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826708A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826708AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826708B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826708B4: 386A1B78  addi r3, r10, 0x1b78
	ctx.r[3].s64 = ctx.r[10].s64 + 7032;
	// 826708B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826708BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826708C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826708C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826708C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826708CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826708D0: 4BDF6551  bl 0x82466e20
	ctx.lr = 0x826708D4;
	sub_82466E20(ctx, base);
	// 826708D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826708D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826708DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826708E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826708E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826708E8 size=100
    let mut pc: u32 = 0x826708E8;
    'dispatch: loop {
        match pc {
            0x826708E8 => {
    //   block [0x826708E8..0x8267094C)
	// 826708E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826708EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826708F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826708F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826708F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826708FC: 38AA1C08  addi r5, r10, 0x1c08
	ctx.r[5].s64 = ctx.r[10].s64 + 7176;
	// 82670900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670908: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8267090C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267091C: 386A1BA8  addi r3, r10, 0x1ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 7080;
	// 82670920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267092C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670938: 4BDF64E9  bl 0x82466e20
	ctx.lr = 0x8267093C;
	sub_82466E20(ctx, base);
	// 8267093C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670950 size=100
    let mut pc: u32 = 0x82670950;
    'dispatch: loop {
        match pc {
            0x82670950 => {
    //   block [0x82670950..0x826709B4)
	// 82670950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267095C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670964: 38AA1B48  addi r5, r10, 0x1b48
	ctx.r[5].s64 = ctx.r[10].s64 + 6984;
	// 82670968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267096C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670970: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82670974: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267097C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670984: 386A1BD8  addi r3, r10, 0x1bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 7128;
	// 82670988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267098C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670990: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670998: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267099C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826709A0: 4BDF6481  bl 0x82466e20
	ctx.lr = 0x826709A4;
	sub_82466E20(ctx, base);
	// 826709A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826709A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826709AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826709B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826709B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826709B8 size=104
    let mut pc: u32 = 0x826709B8;
    'dispatch: loop {
        match pc {
            0x826709B8 => {
    //   block [0x826709B8..0x82670A20)
	// 826709B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826709BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826709C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826709C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826709C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826709CC: 392A195C  addi r9, r10, 0x195c
	ctx.r[9].s64 = ctx.r[10].s64 + 6492;
	// 826709D0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826709D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826709D8: 38AA1B78  addi r5, r10, 0x1b78
	ctx.r[5].s64 = ctx.r[10].s64 + 7032;
	// 826709DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826709E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826709E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826709E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826709EC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826709F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826709F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826709F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826709FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670A00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670A04: 386A1C08  addi r3, r10, 0x1c08
	ctx.r[3].s64 = ctx.r[10].s64 + 7176;
	// 82670A08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82670A0C: 4BDF6415  bl 0x82466e20
	ctx.lr = 0x82670A10;
	sub_82466E20(ctx, base);
	// 82670A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670A20 size=108
    let mut pc: u32 = 0x82670A20;
    'dispatch: loop {
        match pc {
            0x82670A20 => {
    //   block [0x82670A20..0x82670A8C)
	// 82670A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670A2C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670A34: 38EB0234  addi r7, r11, 0x234
	ctx.r[7].s64 = ctx.r[11].s64 + 564;
	// 82670A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82670A3C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82670A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670A44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82670A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670A50: 386A1C38  addi r3, r10, 0x1c38
	ctx.r[3].s64 = ctx.r[10].s64 + 7224;
	// 82670A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82670A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82670A78: 4BDF63A9  bl 0x82466e20
	ctx.lr = 0x82670A7C;
	sub_82466E20(ctx, base);
	// 82670A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670A90 size=112
    let mut pc: u32 = 0x82670A90;
    'dispatch: loop {
        match pc {
            0x82670A90 => {
    //   block [0x82670A90..0x82670B00)
	// 82670A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670A9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670AA0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670AA4: 38AA1C08  addi r5, r10, 0x1c08
	ctx.r[5].s64 = ctx.r[10].s64 + 7176;
	// 82670AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670AAC: 390B0268  addi r8, r11, 0x268
	ctx.r[8].s64 = ctx.r[11].s64 + 616;
	// 82670AB0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82670AB4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82670AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670ABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670AC8: 386A1C68  addi r3, r10, 0x1c68
	ctx.r[3].s64 = ctx.r[10].s64 + 7272;
	// 82670ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670AEC: 4BDF6335  bl 0x82466e20
	ctx.lr = 0x82670AF0;
	sub_82466E20(ctx, base);
	// 82670AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670B00 size=116
    let mut pc: u32 = 0x82670B00;
    'dispatch: loop {
        match pc {
            0x82670B00 => {
    //   block [0x82670B00..0x82670B74)
	// 82670B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670B0C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670B10: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82670B14: 390B0310  addi r8, r11, 0x310
	ctx.r[8].s64 = ctx.r[11].s64 + 784;
	// 82670B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670B1C: 392A19C0  addi r9, r10, 0x19c0
	ctx.r[9].s64 = ctx.r[10].s64 + 6592;
	// 82670B20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670B24: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82670B28: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82670B2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670B34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670B44: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82670B48: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82670B4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82670B50: 386B1C98  addi r3, r11, 0x1c98
	ctx.r[3].s64 = ctx.r[11].s64 + 7320;
	// 82670B54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82670B58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670B60: 4BDF62C1  bl 0x82466e20
	ctx.lr = 0x82670B64;
	sub_82466E20(ctx, base);
	// 82670B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670B78 size=112
    let mut pc: u32 = 0x82670B78;
    'dispatch: loop {
        match pc {
            0x82670B78 => {
    //   block [0x82670B78..0x82670BE8)
	// 82670B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670B88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670B8C: 38AA1D58  addi r5, r10, 0x1d58
	ctx.r[5].s64 = ctx.r[10].s64 + 7512;
	// 82670B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670B94: 390B0328  addi r8, r11, 0x328
	ctx.r[8].s64 = ctx.r[11].s64 + 808;
	// 82670B98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82670B9C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82670BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670BA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670BB0: 386A1CC8  addi r3, r10, 0x1cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 7368;
	// 82670BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670BD4: 4BDF624D  bl 0x82466e20
	ctx.lr = 0x82670BD8;
	sub_82466E20(ctx, base);
	// 82670BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670BE8 size=100
    let mut pc: u32 = 0x82670BE8;
    'dispatch: loop {
        match pc {
            0x82670BE8 => {
    //   block [0x82670BE8..0x82670C4C)
	// 82670BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670BF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670BFC: 38AA1D28  addi r5, r10, 0x1d28
	ctx.r[5].s64 = ctx.r[10].s64 + 7464;
	// 82670C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670C08: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82670C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670C1C: 386A1CF8  addi r3, r10, 0x1cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 7416;
	// 82670C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670C24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670C28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670C30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670C38: 4BDF61E9  bl 0x82466e20
	ctx.lr = 0x82670C3C;
	sub_82466E20(ctx, base);
	// 82670C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670C50 size=112
    let mut pc: u32 = 0x82670C50;
    'dispatch: loop {
        match pc {
            0x82670C50 => {
    //   block [0x82670C50..0x82670CC0)
	// 82670C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670C5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670C60: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670C64: 38AA1D58  addi r5, r10, 0x1d58
	ctx.r[5].s64 = ctx.r[10].s64 + 7512;
	// 82670C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670C6C: 390B0340  addi r8, r11, 0x340
	ctx.r[8].s64 = ctx.r[11].s64 + 832;
	// 82670C70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670C74: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82670C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670C7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670C88: 386A1D28  addi r3, r10, 0x1d28
	ctx.r[3].s64 = ctx.r[10].s64 + 7464;
	// 82670C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670CAC: 4BDF6175  bl 0x82466e20
	ctx.lr = 0x82670CB0;
	sub_82466E20(ctx, base);
	// 82670CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670CC0 size=112
    let mut pc: u32 = 0x82670CC0;
    'dispatch: loop {
        match pc {
            0x82670CC0 => {
    //   block [0x82670CC0..0x82670D30)
	// 82670CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670CD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670CD4: 38AA1C98  addi r5, r10, 0x1c98
	ctx.r[5].s64 = ctx.r[10].s64 + 7320;
	// 82670CD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82670CDC: 390B0370  addi r8, r11, 0x370
	ctx.r[8].s64 = ctx.r[11].s64 + 880;
	// 82670CE0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82670CE4: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82670CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670CF8: 386A1D58  addi r3, r10, 0x1d58
	ctx.r[3].s64 = ctx.r[10].s64 + 7512;
	// 82670CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670D1C: 4BDF6105  bl 0x82466e20
	ctx.lr = 0x82670D20;
	sub_82466E20(ctx, base);
	// 82670D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670D30 size=100
    let mut pc: u32 = 0x82670D30;
    'dispatch: loop {
        match pc {
            0x82670D30 => {
    //   block [0x82670D30..0x82670D94)
	// 82670D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670D44: 38AA1D58  addi r5, r10, 0x1d58
	ctx.r[5].s64 = ctx.r[10].s64 + 7512;
	// 82670D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670D50: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82670D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670D64: 386A1D88  addi r3, r10, 0x1d88
	ctx.r[3].s64 = ctx.r[10].s64 + 7560;
	// 82670D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670D70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670D78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670D80: 4BDF60A1  bl 0x82466e20
	ctx.lr = 0x82670D84;
	sub_82466E20(ctx, base);
	// 82670D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670D98 size=100
    let mut pc: u32 = 0x82670D98;
    'dispatch: loop {
        match pc {
            0x82670D98 => {
    //   block [0x82670D98..0x82670DFC)
	// 82670D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670DAC: 38AA1CC8  addi r5, r10, 0x1cc8
	ctx.r[5].s64 = ctx.r[10].s64 + 7368;
	// 82670DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670DB8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82670DBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670DCC: 386A1DB8  addi r3, r10, 0x1db8
	ctx.r[3].s64 = ctx.r[10].s64 + 7608;
	// 82670DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670DD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670DD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670DE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670DE8: 4BDF6039  bl 0x82466e20
	ctx.lr = 0x82670DEC;
	sub_82466E20(ctx, base);
	// 82670DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670E00 size=100
    let mut pc: u32 = 0x82670E00;
    'dispatch: loop {
        match pc {
            0x82670E00 => {
    //   block [0x82670E00..0x82670E64)
	// 82670E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670E0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670E14: 38AA1D88  addi r5, r10, 0x1d88
	ctx.r[5].s64 = ctx.r[10].s64 + 7560;
	// 82670E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670E20: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82670E24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670E34: 386A1DE8  addi r3, r10, 0x1de8
	ctx.r[3].s64 = ctx.r[10].s64 + 7656;
	// 82670E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670E3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670E40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670E48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670E50: 4BDF5FD1  bl 0x82466e20
	ctx.lr = 0x82670E54;
	sub_82466E20(ctx, base);
	// 82670E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670E68 size=100
    let mut pc: u32 = 0x82670E68;
    'dispatch: loop {
        match pc {
            0x82670E68 => {
    //   block [0x82670E68..0x82670ECC)
	// 82670E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670E7C: 38AA1CC8  addi r5, r10, 0x1cc8
	ctx.r[5].s64 = ctx.r[10].s64 + 7368;
	// 82670E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670E88: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82670E8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670E9C: 386A1E18  addi r3, r10, 0x1e18
	ctx.r[3].s64 = ctx.r[10].s64 + 7704;
	// 82670EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670EA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670EA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670EB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670EB8: 4BDF5F69  bl 0x82466e20
	ctx.lr = 0x82670EBC;
	sub_82466E20(ctx, base);
	// 82670EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670ED0 size=112
    let mut pc: u32 = 0x82670ED0;
    'dispatch: loop {
        match pc {
            0x82670ED0 => {
    //   block [0x82670ED0..0x82670F40)
	// 82670ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670EE0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670EE4: 38AA1EA8  addi r5, r10, 0x1ea8
	ctx.r[5].s64 = ctx.r[10].s64 + 7848;
	// 82670EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670EEC: 390B0418  addi r8, r11, 0x418
	ctx.r[8].s64 = ctx.r[11].s64 + 1048;
	// 82670EF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670EF4: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82670EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670EFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670F08: 386A1E48  addi r3, r10, 0x1e48
	ctx.r[3].s64 = ctx.r[10].s64 + 7752;
	// 82670F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670F2C: 4BDF5EF5  bl 0x82466e20
	ctx.lr = 0x82670F30;
	sub_82466E20(ctx, base);
	// 82670F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670F40 size=112
    let mut pc: u32 = 0x82670F40;
    'dispatch: loop {
        match pc {
            0x82670F40 => {
    //   block [0x82670F40..0x82670FB0)
	// 82670F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670F50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670F54: 38AA1ED8  addi r5, r10, 0x1ed8
	ctx.r[5].s64 = ctx.r[10].s64 + 7896;
	// 82670F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670F5C: 390B0448  addi r8, r11, 0x448
	ctx.r[8].s64 = ctx.r[11].s64 + 1096;
	// 82670F60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82670F64: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82670F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670F78: 386A1E78  addi r3, r10, 0x1e78
	ctx.r[3].s64 = ctx.r[10].s64 + 7800;
	// 82670F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670F9C: 4BDF5E85  bl 0x82466e20
	ctx.lr = 0x82670FA0;
	sub_82466E20(ctx, base);
	// 82670FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670FB0 size=112
    let mut pc: u32 = 0x82670FB0;
    'dispatch: loop {
        match pc {
            0x82670FB0 => {
    //   block [0x82670FB0..0x82671020)
	// 82670FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670FBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670FC0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670FC4: 38AA1FC8  addi r5, r10, 0x1fc8
	ctx.r[5].s64 = ctx.r[10].s64 + 8136;
	// 82670FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670FCC: 390B0460  addi r8, r11, 0x460
	ctx.r[8].s64 = ctx.r[11].s64 + 1120;
	// 82670FD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670FD4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82670FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670FDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670FE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670FE8: 386A1EA8  addi r3, r10, 0x1ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 7848;
	// 82670FEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267100C: 4BDF5E15  bl 0x82466e20
	ctx.lr = 0x82671010;
	sub_82466E20(ctx, base);
	// 82671010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671020 size=112
    let mut pc: u32 = 0x82671020;
    'dispatch: loop {
        match pc {
            0x82671020 => {
    //   block [0x82671020..0x82671090)
	// 82671020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267102C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671030: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671034: 38AA1EA8  addi r5, r10, 0x1ea8
	ctx.r[5].s64 = ctx.r[10].s64 + 7848;
	// 82671038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267103C: 390B0490  addi r8, r11, 0x490
	ctx.r[8].s64 = ctx.r[11].s64 + 1168;
	// 82671040: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671044: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82671048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267104C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671058: 386A1ED8  addi r3, r10, 0x1ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 7896;
	// 8267105C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267106C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267107C: 4BDF5DA5  bl 0x82466e20
	ctx.lr = 0x82671080;
	sub_82466E20(ctx, base);
	// 82671080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267108C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671090 size=112
    let mut pc: u32 = 0x82671090;
    'dispatch: loop {
        match pc {
            0x82671090 => {
    //   block [0x82671090..0x82671100)
	// 82671090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267109C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826710A0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826710A4: 38AA1ED8  addi r5, r10, 0x1ed8
	ctx.r[5].s64 = ctx.r[10].s64 + 7896;
	// 826710A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826710AC: 390B04A8  addi r8, r11, 0x4a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1192;
	// 826710B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826710B4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826710B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826710BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826710C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826710C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826710C8: 386A1F08  addi r3, r10, 0x1f08
	ctx.r[3].s64 = ctx.r[10].s64 + 7944;
	// 826710CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826710D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826710D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826710D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826710DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826710E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826710E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826710E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826710EC: 4BDF5D35  bl 0x82466e20
	ctx.lr = 0x826710F0;
	sub_82466E20(ctx, base);
	// 826710F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826710F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826710F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826710FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671100 size=112
    let mut pc: u32 = 0x82671100;
    'dispatch: loop {
        match pc {
            0x82671100 => {
    //   block [0x82671100..0x82671170)
	// 82671100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267110C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82671110: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671114: 392A19EC  addi r9, r10, 0x19ec
	ctx.r[9].s64 = ctx.r[10].s64 + 6636;
	// 82671118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267111C: 390B04C4  addi r8, r11, 0x4c4
	ctx.r[8].s64 = ctx.r[11].s64 + 1220;
	// 82671120: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82671124: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82671128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267112C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671138: 386A1F38  addi r3, r10, 0x1f38
	ctx.r[3].s64 = ctx.r[10].s64 + 7992;
	// 8267113C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82671140: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82671144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267114C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267115C: 4BDF5CC5  bl 0x82466e20
	ctx.lr = 0x82671160;
	sub_82466E20(ctx, base);
	// 82671160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267116C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671170 size=112
    let mut pc: u32 = 0x82671170;
    'dispatch: loop {
        match pc {
            0x82671170 => {
    //   block [0x82671170..0x826711E0)
	// 82671170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267117C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671180: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671184: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267118C: 390B04F8  addi r8, r11, 0x4f8
	ctx.r[8].s64 = ctx.r[11].s64 + 1272;
	// 82671190: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82671194: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82671198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267119C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826711A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826711A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826711A8: 386A1F68  addi r3, r10, 0x1f68
	ctx.r[3].s64 = ctx.r[10].s64 + 8040;
	// 826711AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826711B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826711B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826711B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826711BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826711C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826711C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826711C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826711CC: 4BDF5C55  bl 0x82466e20
	ctx.lr = 0x826711D0;
	sub_82466E20(ctx, base);
	// 826711D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826711D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826711D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826711DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826711E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826711E0 size=48
    let mut pc: u32 = 0x826711E0;
    'dispatch: loop {
        match pc {
            0x826711E0 => {
    //   block [0x826711E0..0x82671210)
	// 826711E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826711E4: 814B0590  lwz r10, 0x590(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1424 as u32) ) } as u64;
	// 826711E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826711EC: 396B25A8  addi r11, r11, 0x25a8
	ctx.r[11].s64 = ctx.r[11].s64 + 9640;
	// 826711F0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826711F4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826711F8: 814A058C  lwz r10, 0x58c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1420 as u32) ) } as u64;
	// 826711FC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82671200: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82671204: 814A0588  lwz r10, 0x588(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82671208: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 8267120C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671210 size=116
    let mut pc: u32 = 0x82671210;
    'dispatch: loop {
        match pc {
            0x82671210 => {
    //   block [0x82671210..0x82671284)
	// 82671210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267121C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82671220: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671224: 392B1AE0  addi r9, r11, 0x1ae0
	ctx.r[9].s64 = ctx.r[11].s64 + 6880;
	// 82671228: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8267122C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671230: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82671234: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82671238: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267123C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82671240: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671244: 396B25A8  addi r11, r11, 0x25a8
	ctx.r[11].s64 = ctx.r[11].s64 + 9640;
	// 82671248: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267124C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671250: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82671254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671258: 386A1F98  addi r3, r10, 0x1f98
	ctx.r[3].s64 = ctx.r[10].s64 + 8088;
	// 8267125C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82671260: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82671264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671268: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267126C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82671270: 4BDF5BB1  bl 0x82466e20
	ctx.lr = 0x82671274;
	sub_82466E20(ctx, base);
	// 82671274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267127C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671288 size=116
    let mut pc: u32 = 0x82671288;
    'dispatch: loop {
        match pc {
            0x82671288 => {
    //   block [0x82671288..0x826712FC)
	// 82671288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671294: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671298: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267129C: 390B0598  addi r8, r11, 0x598
	ctx.r[8].s64 = ctx.r[11].s64 + 1432;
	// 826712A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826712A4: 392A1BC8  addi r9, r10, 0x1bc8
	ctx.r[9].s64 = ctx.r[10].s64 + 7112;
	// 826712A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826712AC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826712B0: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826712B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826712B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826712BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826712C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826712C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826712C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826712CC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826712D0: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826712D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826712D8: 386B1FC8  addi r3, r11, 0x1fc8
	ctx.r[3].s64 = ctx.r[11].s64 + 8136;
	// 826712DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826712E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826712E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826712E8: 4BDF5B39  bl 0x82466e20
	ctx.lr = 0x826712EC;
	sub_82466E20(ctx, base);
	// 826712EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826712F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826712F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826712F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671300 size=112
    let mut pc: u32 = 0x82671300;
    'dispatch: loop {
        match pc {
            0x82671300 => {
    //   block [0x82671300..0x82671370)
	// 82671300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267130C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671310: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671314: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267131C: 390B0628  addi r8, r11, 0x628
	ctx.r[8].s64 = ctx.r[11].s64 + 1576;
	// 82671320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671324: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82671328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267132C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671338: 386A1FF8  addi r3, r10, 0x1ff8
	ctx.r[3].s64 = ctx.r[10].s64 + 8184;
	// 8267133C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267134C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267135C: 4BDF5AC5  bl 0x82466e20
	ctx.lr = 0x82671360;
	sub_82466E20(ctx, base);
	// 82671360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267136C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671370 size=112
    let mut pc: u32 = 0x82671370;
    'dispatch: loop {
        match pc {
            0x82671370 => {
    //   block [0x82671370..0x826713E0)
	// 82671370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267137C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671380: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671384: 38AA07F8  addi r5, r10, 0x7f8
	ctx.r[5].s64 = ctx.r[10].s64 + 2040;
	// 82671388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267138C: 390B0640  addi r8, r11, 0x640
	ctx.r[8].s64 = ctx.r[11].s64 + 1600;
	// 82671390: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671394: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82671398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267139C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826713A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826713A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826713A8: 386A2028  addi r3, r10, 0x2028
	ctx.r[3].s64 = ctx.r[10].s64 + 8232;
	// 826713AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826713B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826713B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826713B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826713BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826713C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826713C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826713C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826713CC: 4BDF5A55  bl 0x82466e20
	ctx.lr = 0x826713D0;
	sub_82466E20(ctx, base);
	// 826713D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826713D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826713D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826713DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826713E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826713E0 size=108
    let mut pc: u32 = 0x826713E0;
    'dispatch: loop {
        match pc {
            0x826713E0 => {
    //   block [0x826713E0..0x8267144C)
	// 826713E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826713E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826713E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826713EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826713F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826713F4: 38EB0658  addi r7, r11, 0x658
	ctx.r[7].s64 = ctx.r[11].s64 + 1624;
	// 826713F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826713FC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82671400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671404: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267140C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671410: 386A2058  addi r3, r10, 0x2058
	ctx.r[3].s64 = ctx.r[10].s64 + 8280;
	// 82671414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267141C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267142C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671438: 4BDF59E9  bl 0x82466e20
	ctx.lr = 0x8267143C;
	sub_82466E20(ctx, base);
	// 8267143C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671450 size=112
    let mut pc: u32 = 0x82671450;
    'dispatch: loop {
        match pc {
            0x82671450 => {
    //   block [0x82671450..0x826714C0)
	// 82671450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267145C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671460: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671464: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267146C: 390B0670  addi r8, r11, 0x670
	ctx.r[8].s64 = ctx.r[11].s64 + 1648;
	// 82671470: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82671474: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82671478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267147C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671488: 386A2088  addi r3, r10, 0x2088
	ctx.r[3].s64 = ctx.r[10].s64 + 8328;
	// 8267148C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267149C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826714A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826714A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826714A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826714AC: 4BDF5975  bl 0x82466e20
	ctx.lr = 0x826714B0;
	sub_82466E20(ctx, base);
	// 826714B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826714B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826714B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826714BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826714C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826714C0 size=108
    let mut pc: u32 = 0x826714C0;
    'dispatch: loop {
        match pc {
            0x826714C0 => {
    //   block [0x826714C0..0x8267152C)
	// 826714C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826714C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826714C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826714CC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826714D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826714D4: 38EB06B8  addi r7, r11, 0x6b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1720;
	// 826714D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826714DC: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826714E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826714E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826714E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826714EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826714F0: 386A20B8  addi r3, r10, 0x20b8
	ctx.r[3].s64 = ctx.r[10].s64 + 8376;
	// 826714F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826714F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826714FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267150C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671518: 4BDF5909  bl 0x82466e20
	ctx.lr = 0x8267151C;
	sub_82466E20(ctx, base);
	// 8267151C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671530 size=112
    let mut pc: u32 = 0x82671530;
    'dispatch: loop {
        match pc {
            0x82671530 => {
    //   block [0x82671530..0x826715A0)
	// 82671530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267153C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671540: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671544: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267154C: 390B06D0  addi r8, r11, 0x6d0
	ctx.r[8].s64 = ctx.r[11].s64 + 1744;
	// 82671550: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82671554: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82671558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267155C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671568: 386A20E8  addi r3, r10, 0x20e8
	ctx.r[3].s64 = ctx.r[10].s64 + 8424;
	// 8267156C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267157C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267158C: 4BDF5895  bl 0x82466e20
	ctx.lr = 0x82671590;
	sub_82466E20(ctx, base);
	// 82671590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267159C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826715A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826715A0 size=112
    let mut pc: u32 = 0x826715A0;
    'dispatch: loop {
        match pc {
            0x826715A0 => {
    //   block [0x826715A0..0x82671610)
	// 826715A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826715A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826715A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826715AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826715B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826715B4: 38AA21A8  addi r5, r10, 0x21a8
	ctx.r[5].s64 = ctx.r[10].s64 + 8616;
	// 826715B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826715BC: 390B0700  addi r8, r11, 0x700
	ctx.r[8].s64 = ctx.r[11].s64 + 1792;
	// 826715C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826715C4: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 826715C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826715CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826715D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826715D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826715D8: 386A2118  addi r3, r10, 0x2118
	ctx.r[3].s64 = ctx.r[10].s64 + 8472;
	// 826715DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826715E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826715E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826715E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826715EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826715F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826715F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826715F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826715FC: 4BDF5825  bl 0x82466e20
	ctx.lr = 0x82671600;
	sub_82466E20(ctx, base);
	// 82671600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267160C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671610 size=108
    let mut pc: u32 = 0x82671610;
    'dispatch: loop {
        match pc {
            0x82671610 => {
    //   block [0x82671610..0x8267167C)
	// 82671610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267161C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82671624: 38EB0778  addi r7, r11, 0x778
	ctx.r[7].s64 = ctx.r[11].s64 + 1912;
	// 82671628: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267162C: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82671630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267163C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671640: 386A2148  addi r3, r10, 0x2148
	ctx.r[3].s64 = ctx.r[10].s64 + 8520;
	// 82671644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267164C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267165C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671668: 4BDF57B9  bl 0x82466e20
	ctx.lr = 0x8267166C;
	sub_82466E20(ctx, base);
	// 8267166C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671680 size=108
    let mut pc: u32 = 0x82671680;
    'dispatch: loop {
        match pc {
            0x82671680 => {
    //   block [0x82671680..0x826716EC)
	// 82671680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267168C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82671694: 38EB07C0  addi r7, r11, 0x7c0
	ctx.r[7].s64 = ctx.r[11].s64 + 1984;
	// 82671698: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267169C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 826716A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826716A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826716A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826716AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826716B0: 386A2178  addi r3, r10, 0x2178
	ctx.r[3].s64 = ctx.r[10].s64 + 8568;
	// 826716B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826716B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826716BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826716C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826716C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826716C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826716CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826716D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826716D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826716D8: 4BDF5749  bl 0x82466e20
	ctx.lr = 0x826716DC;
	sub_82466E20(ctx, base);
	// 826716DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826716E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826716E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826716E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826716F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826716F0 size=116
    let mut pc: u32 = 0x826716F0;
    'dispatch: loop {
        match pc {
            0x826716F0 => {
    //   block [0x826716F0..0x82671764)
	// 826716F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826716F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826716F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826716FC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82671700: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82671704: 390A0808  addi r8, r10, 0x808
	ctx.r[8].s64 = ctx.r[10].s64 + 2056;
	// 82671708: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267170C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82671710: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82671714: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671718: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267171C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671724: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82671728: 396B1BDC  addi r11, r11, 0x1bdc
	ctx.r[11].s64 = ctx.r[11].s64 + 7132;
	// 8267172C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671734: 386A21A8  addi r3, r10, 0x21a8
	ctx.r[3].s64 = ctx.r[10].s64 + 8616;
	// 82671738: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267173C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671740: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82671744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267174C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671750: 4BDF56D1  bl 0x82466e20
	ctx.lr = 0x82671754;
	sub_82466E20(ctx, base);
	// 82671754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267175C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671768 size=108
    let mut pc: u32 = 0x82671768;
    'dispatch: loop {
        match pc {
            0x82671768 => {
    //   block [0x82671768..0x826717D4)
	// 82671768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267176C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671774: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267177C: 38EB08E0  addi r7, r11, 0x8e0
	ctx.r[7].s64 = ctx.r[11].s64 + 2272;
	// 82671780: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671784: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 82671788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267178C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671798: 386A21D8  addi r3, r10, 0x21d8
	ctx.r[3].s64 = ctx.r[10].s64 + 8664;
	// 8267179C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826717A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826717A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826717A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826717AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826717B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826717B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826717B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826717BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826717C0: 4BDF5661  bl 0x82466e20
	ctx.lr = 0x826717C4;
	sub_82466E20(ctx, base);
	// 826717C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826717C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826717CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826717D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826717D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826717D8 size=108
    let mut pc: u32 = 0x826717D8;
    'dispatch: loop {
        match pc {
            0x826717D8 => {
    //   block [0x826717D8..0x82671844)
	// 826717D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826717DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826717E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826717E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826717E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826717EC: 38EB0910  addi r7, r11, 0x910
	ctx.r[7].s64 = ctx.r[11].s64 + 2320;
	// 826717F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826717F4: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 826717F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826717FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671808: 386A2208  addi r3, r10, 0x2208
	ctx.r[3].s64 = ctx.r[10].s64 + 8712;
	// 8267180C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267181C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267182C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671830: 4BDF55F1  bl 0x82466e20
	ctx.lr = 0x82671834;
	sub_82466E20(ctx, base);
	// 82671834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267183C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671848 size=112
    let mut pc: u32 = 0x82671848;
    'dispatch: loop {
        match pc {
            0x82671848 => {
    //   block [0x82671848..0x826718B8)
	// 82671848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267184C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671854: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671858: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267185C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82671860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671864: 390B0940  addi r8, r11, 0x940
	ctx.r[8].s64 = ctx.r[11].s64 + 2368;
	// 82671868: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8267186C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82671870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267187C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671880: 386A2238  addi r3, r10, 0x2238
	ctx.r[3].s64 = ctx.r[10].s64 + 8760;
	// 82671884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267188C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267189C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826718A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826718A4: 4BDF557D  bl 0x82466e20
	ctx.lr = 0x826718A8;
	sub_82466E20(ctx, base);
	// 826718A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826718AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826718B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826718B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826718B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826718B8 size=112
    let mut pc: u32 = 0x826718B8;
    'dispatch: loop {
        match pc {
            0x826718B8 => {
    //   block [0x826718B8..0x82671928)
	// 826718B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826718BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826718C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826718C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826718C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826718CC: 38AA11E8  addi r5, r10, 0x11e8
	ctx.r[5].s64 = ctx.r[10].s64 + 4584;
	// 826718D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826718D4: 390B0A18  addi r8, r11, 0xa18
	ctx.r[8].s64 = ctx.r[11].s64 + 2584;
	// 826718D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826718DC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826718E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826718E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826718E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826718EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826718F0: 386A2268  addi r3, r10, 0x2268
	ctx.r[3].s64 = ctx.r[10].s64 + 8808;
	// 826718F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826718F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826718FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267190C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671914: 4BDF550D  bl 0x82466e20
	ctx.lr = 0x82671918;
	sub_82466E20(ctx, base);
	// 82671918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267191C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671928 size=112
    let mut pc: u32 = 0x82671928;
    'dispatch: loop {
        match pc {
            0x82671928 => {
    //   block [0x82671928..0x82671998)
	// 82671928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267192C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671938: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267193C: 38AA11E8  addi r5, r10, 0x11e8
	ctx.r[5].s64 = ctx.r[10].s64 + 4584;
	// 82671940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671944: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 82671948: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267194C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82671950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267195C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671960: 386A2298  addi r3, r10, 0x2298
	ctx.r[3].s64 = ctx.r[10].s64 + 8856;
	// 82671964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267196C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267197C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671984: 4BDF549D  bl 0x82466e20
	ctx.lr = 0x82671988;
	sub_82466E20(ctx, base);
	// 82671988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267198C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671998 size=112
    let mut pc: u32 = 0x82671998;
    'dispatch: loop {
        match pc {
            0x82671998 => {
    //   block [0x82671998..0x82671A08)
	// 82671998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826719A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826719A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826719A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826719AC: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 826719B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826719B4: 390B0AC0  addi r8, r11, 0xac0
	ctx.r[8].s64 = ctx.r[11].s64 + 2752;
	// 826719B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826719BC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826719C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826719C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826719C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826719CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826719D0: 386A22C8  addi r3, r10, 0x22c8
	ctx.r[3].s64 = ctx.r[10].s64 + 8904;
	// 826719D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826719D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826719DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826719E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826719E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826719E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826719EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826719F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826719F4: 4BDF542D  bl 0x82466e20
	ctx.lr = 0x826719F8;
	sub_82466E20(ctx, base);
	// 826719F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826719FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671A08 size=112
    let mut pc: u32 = 0x82671A08;
    'dispatch: loop {
        match pc {
            0x82671A08 => {
    //   block [0x82671A08..0x82671A78)
	// 82671A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671A18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671A1C: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 82671A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671A24: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 82671A28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82671A2C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82671A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671A40: 386A22F8  addi r3, r10, 0x22f8
	ctx.r[3].s64 = ctx.r[10].s64 + 8952;
	// 82671A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671A64: 4BDF53BD  bl 0x82466e20
	ctx.lr = 0x82671A68;
	sub_82466E20(ctx, base);
	// 82671A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671A78 size=112
    let mut pc: u32 = 0x82671A78;
    'dispatch: loop {
        match pc {
            0x82671A78 => {
    //   block [0x82671A78..0x82671AE8)
	// 82671A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671A88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671A8C: 38AA11E8  addi r5, r10, 0x11e8
	ctx.r[5].s64 = ctx.r[10].s64 + 4584;
	// 82671A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671A94: 390B0B80  addi r8, r11, 0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + 2944;
	// 82671A98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82671A9C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82671AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671AA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671AB0: 386A2328  addi r3, r10, 0x2328
	ctx.r[3].s64 = ctx.r[10].s64 + 9000;
	// 82671AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671AD4: 4BDF534D  bl 0x82466e20
	ctx.lr = 0x82671AD8;
	sub_82466E20(ctx, base);
	// 82671AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671AE8 size=108
    let mut pc: u32 = 0x82671AE8;
    'dispatch: loop {
        match pc {
            0x82671AE8 => {
    //   block [0x82671AE8..0x82671B54)
	// 82671AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671AF4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671AFC: 38EB0C40  addi r7, r11, 0xc40
	ctx.r[7].s64 = ctx.r[11].s64 + 3136;
	// 82671B00: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82671B04: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82671B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671B0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671B18: 386A2358  addi r3, r10, 0x2358
	ctx.r[3].s64 = ctx.r[10].s64 + 9048;
	// 82671B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671B40: 4BDF52E1  bl 0x82466e20
	ctx.lr = 0x82671B44;
	sub_82466E20(ctx, base);
	// 82671B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671B58 size=112
    let mut pc: u32 = 0x82671B58;
    'dispatch: loop {
        match pc {
            0x82671B58 => {
    //   block [0x82671B58..0x82671BC8)
	// 82671B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671B64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671B68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671B6C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671B74: 390B0DD8  addi r8, r11, 0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + 3544;
	// 82671B78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671B7C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82671B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671B90: 386A2388  addi r3, r10, 0x2388
	ctx.r[3].s64 = ctx.r[10].s64 + 9096;
	// 82671B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671BB4: 4BDF526D  bl 0x82466e20
	ctx.lr = 0x82671BB8;
	sub_82466E20(ctx, base);
	// 82671BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671BC8 size=112
    let mut pc: u32 = 0x82671BC8;
    'dispatch: loop {
        match pc {
            0x82671BC8 => {
    //   block [0x82671BC8..0x82671C38)
	// 82671BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671BD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671BD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671BDC: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671BE4: 390B0DF0  addi r8, r11, 0xdf0
	ctx.r[8].s64 = ctx.r[11].s64 + 3568;
	// 82671BE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671BEC: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82671BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671BF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671C00: 386A23B8  addi r3, r10, 0x23b8
	ctx.r[3].s64 = ctx.r[10].s64 + 9144;
	// 82671C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671C14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82671C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671C24: 4BDF51FD  bl 0x82466e20
	ctx.lr = 0x82671C28;
	sub_82466E20(ctx, base);
	// 82671C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671C38 size=112
    let mut pc: u32 = 0x82671C38;
    'dispatch: loop {
        match pc {
            0x82671C38 => {
    //   block [0x82671C38..0x82671CA8)
	// 82671C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671C44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671C48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671C4C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671C54: 390B0E08  addi r8, r11, 0xe08
	ctx.r[8].s64 = ctx.r[11].s64 + 3592;
	// 82671C58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82671C5C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82671C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671C64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671C70: 386A23E8  addi r3, r10, 0x23e8
	ctx.r[3].s64 = ctx.r[10].s64 + 9192;
	// 82671C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671C94: 4BDF518D  bl 0x82466e20
	ctx.lr = 0x82671C98;
	sub_82466E20(ctx, base);
	// 82671C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671CA8 size=108
    let mut pc: u32 = 0x82671CA8;
    'dispatch: loop {
        match pc {
            0x82671CA8 => {
    //   block [0x82671CA8..0x82671D14)
	// 82671CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671CB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671CBC: 38EB0E38  addi r7, r11, 0xe38
	ctx.r[7].s64 = ctx.r[11].s64 + 3640;
	// 82671CC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671CC4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82671CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671CD8: 386A2418  addi r3, r10, 0x2418
	ctx.r[3].s64 = ctx.r[10].s64 + 9240;
	// 82671CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671D00: 4BDF5121  bl 0x82466e20
	ctx.lr = 0x82671D04;
	sub_82466E20(ctx, base);
	// 82671D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671D18 size=112
    let mut pc: u32 = 0x82671D18;
    'dispatch: loop {
        match pc {
            0x82671D18 => {
    //   block [0x82671D18..0x82671D88)
	// 82671D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671D24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671D28: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671D2C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671D34: 390B0E68  addi r8, r11, 0xe68
	ctx.r[8].s64 = ctx.r[11].s64 + 3688;
	// 82671D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671D3C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82671D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671D44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671D50: 386A2448  addi r3, r10, 0x2448
	ctx.r[3].s64 = ctx.r[10].s64 + 9288;
	// 82671D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82671D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671D74: 4BDF50AD  bl 0x82466e20
	ctx.lr = 0x82671D78;
	sub_82466E20(ctx, base);
	// 82671D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671D88 size=108
    let mut pc: u32 = 0x82671D88;
    'dispatch: loop {
        match pc {
            0x82671D88 => {
    //   block [0x82671D88..0x82671DF4)
	// 82671D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671D94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671D9C: 38EB0E80  addi r7, r11, 0xe80
	ctx.r[7].s64 = ctx.r[11].s64 + 3712;
	// 82671DA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671DA4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82671DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671DB8: 386A2478  addi r3, r10, 0x2478
	ctx.r[3].s64 = ctx.r[10].s64 + 9336;
	// 82671DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671DE0: 4BDF5041  bl 0x82466e20
	ctx.lr = 0x82671DE4;
	sub_82466E20(ctx, base);
	// 82671DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671DF8 size=108
    let mut pc: u32 = 0x82671DF8;
    'dispatch: loop {
        match pc {
            0x82671DF8 => {
    //   block [0x82671DF8..0x82671E64)
	// 82671DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671E04: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671E0C: 38EB0EB0  addi r7, r11, 0xeb0
	ctx.r[7].s64 = ctx.r[11].s64 + 3760;
	// 82671E10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82671E14: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82671E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671E1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671E28: 386A24A8  addi r3, r10, 0x24a8
	ctx.r[3].s64 = ctx.r[10].s64 + 9384;
	// 82671E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671E50: 4BDF4FD1  bl 0x82466e20
	ctx.lr = 0x82671E54;
	sub_82466E20(ctx, base);
	// 82671E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671E68 size=112
    let mut pc: u32 = 0x82671E68;
    'dispatch: loop {
        match pc {
            0x82671E68 => {
    //   block [0x82671E68..0x82671ED8)
	// 82671E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671E78: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671E7C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671E84: 390B0EF8  addi r8, r11, 0xef8
	ctx.r[8].s64 = ctx.r[11].s64 + 3832;
	// 82671E88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82671E8C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 82671E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671E94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671EA0: 386A24D8  addi r3, r10, 0x24d8
	ctx.r[3].s64 = ctx.r[10].s64 + 9432;
	// 82671EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671EC4: 4BDF4F5D  bl 0x82466e20
	ctx.lr = 0x82671EC8;
	sub_82466E20(ctx, base);
	// 82671EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671ED8 size=112
    let mut pc: u32 = 0x82671ED8;
    'dispatch: loop {
        match pc {
            0x82671ED8 => {
    //   block [0x82671ED8..0x82671F48)
	// 82671ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671EE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671EE8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671EEC: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 82671EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671EF4: 390B0F40  addi r8, r11, 0xf40
	ctx.r[8].s64 = ctx.r[11].s64 + 3904;
	// 82671EF8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82671EFC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82671F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671F04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671F10: 386A2508  addi r3, r10, 0x2508
	ctx.r[3].s64 = ctx.r[10].s64 + 9480;
	// 82671F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671F34: 4BDF4EED  bl 0x82466e20
	ctx.lr = 0x82671F38;
	sub_82466E20(ctx, base);
	// 82671F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671F48 size=108
    let mut pc: u32 = 0x82671F48;
    'dispatch: loop {
        match pc {
            0x82671F48 => {
    //   block [0x82671F48..0x82671FB4)
	// 82671F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671F54: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671F5C: 38EB0FD0  addi r7, r11, 0xfd0
	ctx.r[7].s64 = ctx.r[11].s64 + 4048;
	// 82671F60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82671F64: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 82671F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671F78: 386A2538  addi r3, r10, 0x2538
	ctx.r[3].s64 = ctx.r[10].s64 + 9528;
	// 82671F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671FA0: 4BDF4E81  bl 0x82466e20
	ctx.lr = 0x82671FA4;
	sub_82466E20(ctx, base);
	// 82671FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671FB8 size=108
    let mut pc: u32 = 0x82671FB8;
    'dispatch: loop {
        match pc {
            0x82671FB8 => {
    //   block [0x82671FB8..0x82672024)
	// 82671FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671FC4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671FCC: 38EB1018  addi r7, r11, 0x1018
	ctx.r[7].s64 = ctx.r[11].s64 + 4120;
	// 82671FD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671FD4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82671FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671FDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671FE8: 386A2568  addi r3, r10, 0x2568
	ctx.r[3].s64 = ctx.r[10].s64 + 9576;
	// 82671FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267200C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672010: 4BDF4E11  bl 0x82466e20
	ctx.lr = 0x82672014;
	sub_82466E20(ctx, base);
	// 82672014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267201C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672028 size=108
    let mut pc: u32 = 0x82672028;
    'dispatch: loop {
        match pc {
            0x82672028 => {
    //   block [0x82672028..0x82672094)
	// 82672028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672034: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267203C: 38EB1048  addi r7, r11, 0x1048
	ctx.r[7].s64 = ctx.r[11].s64 + 4168;
	// 82672040: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82672044: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82672048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267204C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672058: 386A2598  addi r3, r10, 0x2598
	ctx.r[3].s64 = ctx.r[10].s64 + 9624;
	// 8267205C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267206C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267207C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672080: 4BDF4DA1  bl 0x82466e20
	ctx.lr = 0x82672084;
	sub_82466E20(ctx, base);
	// 82672084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267208C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672098 size=112
    let mut pc: u32 = 0x82672098;
    'dispatch: loop {
        match pc {
            0x82672098 => {
    //   block [0x82672098..0x82672108)
	// 82672098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267209C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826720A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826720A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826720A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826720AC: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826720B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826720B4: 390B1078  addi r8, r11, 0x1078
	ctx.r[8].s64 = ctx.r[11].s64 + 4216;
	// 826720B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826720BC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826720C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826720C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826720C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826720CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826720D0: 386A25C8  addi r3, r10, 0x25c8
	ctx.r[3].s64 = ctx.r[10].s64 + 9672;
	// 826720D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826720D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826720DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826720E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826720E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826720E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826720EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826720F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826720F4: 4BDF4D2D  bl 0x82466e20
	ctx.lr = 0x826720F8;
	sub_82466E20(ctx, base);
	// 826720F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826720FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672108 size=112
    let mut pc: u32 = 0x82672108;
    'dispatch: loop {
        match pc {
            0x82672108 => {
    //   block [0x82672108..0x82672178)
	// 82672108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267210C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672118: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267211C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672124: 390B10A8  addi r8, r11, 0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4264;
	// 82672128: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267212C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82672130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267213C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672140: 386A25F8  addi r3, r10, 0x25f8
	ctx.r[3].s64 = ctx.r[10].s64 + 9720;
	// 82672144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267214C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267215C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672164: 4BDF4CBD  bl 0x82466e20
	ctx.lr = 0x82672168;
	sub_82466E20(ctx, base);
	// 82672168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267216C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672178 size=112
    let mut pc: u32 = 0x82672178;
    'dispatch: loop {
        match pc {
            0x82672178 => {
    //   block [0x82672178..0x826721E8)
	// 82672178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672188: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267218C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672194: 390B10C0  addi r8, r11, 0x10c0
	ctx.r[8].s64 = ctx.r[11].s64 + 4288;
	// 82672198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267219C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826721A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826721A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826721A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826721AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826721B0: 386A2628  addi r3, r10, 0x2628
	ctx.r[3].s64 = ctx.r[10].s64 + 9768;
	// 826721B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826721B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826721BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826721C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826721C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826721C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826721CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826721D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826721D4: 4BDF4C4D  bl 0x82466e20
	ctx.lr = 0x826721D8;
	sub_82466E20(ctx, base);
	// 826721D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826721DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826721E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826721E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826721E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826721E8 size=108
    let mut pc: u32 = 0x826721E8;
    'dispatch: loop {
        match pc {
            0x826721E8 => {
    //   block [0x826721E8..0x82672254)
	// 826721E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826721EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826721F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826721F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826721F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826721FC: 38EB10D8  addi r7, r11, 0x10d8
	ctx.r[7].s64 = ctx.r[11].s64 + 4312;
	// 82672200: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82672204: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82672208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267220C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672218: 386A2658  addi r3, r10, 0x2658
	ctx.r[3].s64 = ctx.r[10].s64 + 9816;
	// 8267221C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267222C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672240: 4BDF4BE1  bl 0x82466e20
	ctx.lr = 0x82672244;
	sub_82466E20(ctx, base);
	// 82672244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672258 size=112
    let mut pc: u32 = 0x82672258;
    'dispatch: loop {
        match pc {
            0x82672258 => {
    //   block [0x82672258..0x826722C8)
	// 82672258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672264: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672268: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267226C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672274: 390B1108  addi r8, r11, 0x1108
	ctx.r[8].s64 = ctx.r[11].s64 + 4360;
	// 82672278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267227C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82672280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672290: 386A2688  addi r3, r10, 0x2688
	ctx.r[3].s64 = ctx.r[10].s64 + 9864;
	// 82672294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826722A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826722A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826722A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826722AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826722B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826722B4: 4BDF4B6D  bl 0x82466e20
	ctx.lr = 0x826722B8;
	sub_82466E20(ctx, base);
	// 826722B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826722BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826722C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826722C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826722C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826722C8 size=108
    let mut pc: u32 = 0x826722C8;
    'dispatch: loop {
        match pc {
            0x826722C8 => {
    //   block [0x826722C8..0x82672334)
	// 826722C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826722CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826722D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826722D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826722D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826722DC: 38EB1120  addi r7, r11, 0x1120
	ctx.r[7].s64 = ctx.r[11].s64 + 4384;
	// 826722E0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826722E4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826722E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826722EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826722F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826722F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826722F8: 386A26B8  addi r3, r10, 0x26b8
	ctx.r[3].s64 = ctx.r[10].s64 + 9912;
	// 826722FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267230C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267231C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672320: 4BDF4B01  bl 0x82466e20
	ctx.lr = 0x82672324;
	sub_82466E20(ctx, base);
	// 82672324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267232C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672338 size=112
    let mut pc: u32 = 0x82672338;
    'dispatch: loop {
        match pc {
            0x82672338 => {
    //   block [0x82672338..0x826723A8)
	// 82672338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672344: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672348: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267234C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672354: 390B11F8  addi r8, r11, 0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + 4600;
	// 82672358: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8267235C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82672360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672370: 386A26E8  addi r3, r10, 0x26e8
	ctx.r[3].s64 = ctx.r[10].s64 + 9960;
	// 82672374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267237C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267238C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672394: 4BDF4A8D  bl 0x82466e20
	ctx.lr = 0x82672398;
	sub_82466E20(ctx, base);
	// 82672398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826723A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826723A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826723A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826723A8 size=108
    let mut pc: u32 = 0x826723A8;
    'dispatch: loop {
        match pc {
            0x826723A8 => {
    //   block [0x826723A8..0x82672414)
	// 826723A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826723AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826723B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826723B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826723B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826723BC: 38EB13A8  addi r7, r11, 0x13a8
	ctx.r[7].s64 = ctx.r[11].s64 + 5032;
	// 826723C0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826723C4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826723C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826723CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826723D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826723D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826723D8: 386A2718  addi r3, r10, 0x2718
	ctx.r[3].s64 = ctx.r[10].s64 + 10008;
	// 826723DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826723E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826723E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826723E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826723EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826723F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826723F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826723F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826723FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672400: 4BDF4A21  bl 0x82466e20
	ctx.lr = 0x82672404;
	sub_82466E20(ctx, base);
	// 82672404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267240C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672418 size=112
    let mut pc: u32 = 0x82672418;
    'dispatch: loop {
        match pc {
            0x82672418 => {
    //   block [0x82672418..0x82672488)
	// 82672418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672428: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267242C: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 82672430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672434: 390B1540  addi r8, r11, 0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + 5440;
	// 82672438: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 8267243C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82672440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267244C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672450: 386A2748  addi r3, r10, 0x2748
	ctx.r[3].s64 = ctx.r[10].s64 + 10056;
	// 82672454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267245C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267246C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672474: 4BDF49AD  bl 0x82466e20
	ctx.lr = 0x82672478;
	sub_82466E20(ctx, base);
	// 82672478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267247C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672488 size=100
    let mut pc: u32 = 0x82672488;
    'dispatch: loop {
        match pc {
            0x82672488 => {
    //   block [0x82672488..0x826724EC)
	// 82672488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267249C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826724A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826724A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826724A8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826724AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826724B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826724B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826724B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826724BC: 386A2778  addi r3, r10, 0x2778
	ctx.r[3].s64 = ctx.r[10].s64 + 10104;
	// 826724C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826724C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826724C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826724CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826724D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826724D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826724D8: 4BDF4949  bl 0x82466e20
	ctx.lr = 0x826724DC;
	sub_82466E20(ctx, base);
	// 826724DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826724E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826724E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826724E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826724F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826724F0 size=112
    let mut pc: u32 = 0x826724F0;
    'dispatch: loop {
        match pc {
            0x826724F0 => {
    //   block [0x826724F0..0x82672560)
	// 826724F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826724F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826724F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826724FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672500: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672504: 38AA2778  addi r5, r10, 0x2778
	ctx.r[5].s64 = ctx.r[10].s64 + 10104;
	// 82672508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267250C: 390B17B0  addi r8, r11, 0x17b0
	ctx.r[8].s64 = ctx.r[11].s64 + 6064;
	// 82672510: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82672514: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82672518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267251C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672528: 386A27A8  addi r3, r10, 0x27a8
	ctx.r[3].s64 = ctx.r[10].s64 + 10152;
	// 8267252C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267254C: 4BDF48D5  bl 0x82466e20
	ctx.lr = 0x82672550;
	sub_82466E20(ctx, base);
	// 82672550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672560 size=100
    let mut pc: u32 = 0x82672560;
    'dispatch: loop {
        match pc {
            0x82672560 => {
    //   block [0x82672560..0x826725C4)
	// 82672560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267256C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672574: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672580: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82672584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267258C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672594: 386A27D8  addi r3, r10, 0x27d8
	ctx.r[3].s64 = ctx.r[10].s64 + 10200;
	// 82672598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267259C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826725A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826725A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826725A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826725AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826725B0: 4BDF4871  bl 0x82466e20
	ctx.lr = 0x826725B4;
	sub_82466E20(ctx, base);
	// 826725B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826725B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826725BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826725C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826725C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826725C8 size=108
    let mut pc: u32 = 0x826725C8;
    'dispatch: loop {
        match pc {
            0x826725C8 => {
    //   block [0x826725C8..0x82672634)
	// 826725C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826725CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826725D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826725D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826725D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826725DC: 38EB1828  addi r7, r11, 0x1828
	ctx.r[7].s64 = ctx.r[11].s64 + 6184;
	// 826725E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826725E4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826725E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826725EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826725F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826725F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826725F8: 386A2808  addi r3, r10, 0x2808
	ctx.r[3].s64 = ctx.r[10].s64 + 10248;
	// 826725FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267260C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267261C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672620: 4BDF4801  bl 0x82466e20
	ctx.lr = 0x82672624;
	sub_82466E20(ctx, base);
	// 82672624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267262C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672638 size=112
    let mut pc: u32 = 0x82672638;
    'dispatch: loop {
        match pc {
            0x82672638 => {
    //   block [0x82672638..0x826726A8)
	// 82672638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267263C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672648: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267264C: 38AA27D8  addi r5, r10, 0x27d8
	ctx.r[5].s64 = ctx.r[10].s64 + 10200;
	// 82672650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672654: 390B1870  addi r8, r11, 0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + 6256;
	// 82672658: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267265C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82672660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267266C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672670: 386A2838  addi r3, r10, 0x2838
	ctx.r[3].s64 = ctx.r[10].s64 + 10296;
	// 82672674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267268C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672694: 4BDF478D  bl 0x82466e20
	ctx.lr = 0x82672698;
	sub_82466E20(ctx, base);
	// 82672698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267269C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826726A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826726A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826726A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826726A8 size=100
    let mut pc: u32 = 0x826726A8;
    'dispatch: loop {
        match pc {
            0x826726A8 => {
    //   block [0x826726A8..0x8267270C)
	// 826726A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826726AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826726B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826726B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826726B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826726BC: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826726C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826726C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826726C8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826726CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826726D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826726D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826726D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826726DC: 386A2868  addi r3, r10, 0x2868
	ctx.r[3].s64 = ctx.r[10].s64 + 10344;
	// 826726E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826726E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826726E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826726EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826726F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826726F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826726F8: 4BDF4729  bl 0x82466e20
	ctx.lr = 0x826726FC;
	sub_82466E20(ctx, base);
	// 826726FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672710 size=100
    let mut pc: u32 = 0x82672710;
    'dispatch: loop {
        match pc {
            0x82672710 => {
    //   block [0x82672710..0x82672774)
	// 82672710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267271C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672724: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672730: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82672734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267273C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672744: 386A2898  addi r3, r10, 0x2898
	ctx.r[3].s64 = ctx.r[10].s64 + 10392;
	// 82672748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267274C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672750: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267275C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672760: 4BDF46C1  bl 0x82466e20
	ctx.lr = 0x82672764;
	sub_82466E20(ctx, base);
	// 82672764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267276C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672778 size=112
    let mut pc: u32 = 0x82672778;
    'dispatch: loop {
        match pc {
            0x82672778 => {
    //   block [0x82672778..0x826727E8)
	// 82672778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267277C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672788: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267278C: 38AA2868  addi r5, r10, 0x2868
	ctx.r[5].s64 = ctx.r[10].s64 + 10344;
	// 82672790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672794: 390B18A0  addi r8, r11, 0x18a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6304;
	// 82672798: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267279C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826727A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826727A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826727A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826727AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826727B0: 386A28C8  addi r3, r10, 0x28c8
	ctx.r[3].s64 = ctx.r[10].s64 + 10440;
	// 826727B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826727B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826727BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826727C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826727C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826727C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826727CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826727D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826727D4: 4BDF464D  bl 0x82466e20
	ctx.lr = 0x826727D8;
	sub_82466E20(ctx, base);
	// 826727D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826727DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826727E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826727E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826727E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826727E8 size=112
    let mut pc: u32 = 0x826727E8;
    'dispatch: loop {
        match pc {
            0x826727E8 => {
    //   block [0x826727E8..0x82672858)
	// 826727E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826727EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826727F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826727F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826727F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826727FC: 38AA2898  addi r5, r10, 0x2898
	ctx.r[5].s64 = ctx.r[10].s64 + 10392;
	// 82672800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672804: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 82672808: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267280C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82672810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267281C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672820: 386A28F8  addi r3, r10, 0x28f8
	ctx.r[3].s64 = ctx.r[10].s64 + 10488;
	// 82672824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267282C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267283C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672844: 4BDF45DD  bl 0x82466e20
	ctx.lr = 0x82672848;
	sub_82466E20(ctx, base);
	// 82672848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267284C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672858 size=100
    let mut pc: u32 = 0x82672858;
    'dispatch: loop {
        match pc {
            0x82672858 => {
    //   block [0x82672858..0x826728BC)
	// 82672858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672864: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267286C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672878: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8267287C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267288C: 386A2928  addi r3, r10, 0x2928
	ctx.r[3].s64 = ctx.r[10].s64 + 10536;
	// 82672890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267289C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826728A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826728A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826728A8: 4BDF4579  bl 0x82466e20
	ctx.lr = 0x826728AC;
	sub_82466E20(ctx, base);
	// 826728AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826728B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826728B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826728B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826728C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826728C0 size=112
    let mut pc: u32 = 0x826728C0;
    'dispatch: loop {
        match pc {
            0x826728C0 => {
    //   block [0x826728C0..0x82672930)
	// 826728C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826728C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826728C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826728CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826728D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826728D4: 38AA2928  addi r5, r10, 0x2928
	ctx.r[5].s64 = ctx.r[10].s64 + 10536;
	// 826728D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826728DC: 390B1960  addi r8, r11, 0x1960
	ctx.r[8].s64 = ctx.r[11].s64 + 6496;
	// 826728E0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826728E4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826728E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826728EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826728F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826728F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826728F8: 386A2958  addi r3, r10, 0x2958
	ctx.r[3].s64 = ctx.r[10].s64 + 10584;
	// 826728FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267291C: 4BDF4505  bl 0x82466e20
	ctx.lr = 0x82672920;
	sub_82466E20(ctx, base);
	// 82672920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672930 size=108
    let mut pc: u32 = 0x82672930;
    'dispatch: loop {
        match pc {
            0x82672930 => {
    //   block [0x82672930..0x8267299C)
	// 82672930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267293C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672944: 38EB1A50  addi r7, r11, 0x1a50
	ctx.r[7].s64 = ctx.r[11].s64 + 6736;
	// 82672948: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8267294C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82672950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267295C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672960: 386A2988  addi r3, r10, 0x2988
	ctx.r[3].s64 = ctx.r[10].s64 + 10632;
	// 82672964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267296C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267297C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672988: 4BDF4499  bl 0x82466e20
	ctx.lr = 0x8267298C;
	sub_82466E20(ctx, base);
	// 8267298C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826729A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826729A0 size=108
    let mut pc: u32 = 0x826729A0;
    'dispatch: loop {
        match pc {
            0x826729A0 => {
    //   block [0x826729A0..0x82672A0C)
	// 826729A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826729A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826729A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826729AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826729B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826729B4: 38EB1B40  addi r7, r11, 0x1b40
	ctx.r[7].s64 = ctx.r[11].s64 + 6976;
	// 826729B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826729BC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826729C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826729C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826729C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826729CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826729D0: 386A29B8  addi r3, r10, 0x29b8
	ctx.r[3].s64 = ctx.r[10].s64 + 10680;
	// 826729D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826729D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826729DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826729E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826729E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826729E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826729EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826729F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826729F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826729F8: 4BDF4429  bl 0x82466e20
	ctx.lr = 0x826729FC;
	sub_82466E20(ctx, base);
	// 826729FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672A10 size=108
    let mut pc: u32 = 0x82672A10;
    'dispatch: loop {
        match pc {
            0x82672A10 => {
    //   block [0x82672A10..0x82672A7C)
	// 82672A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672A1C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672A24: 38EB1B88  addi r7, r11, 0x1b88
	ctx.r[7].s64 = ctx.r[11].s64 + 7048;
	// 82672A28: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82672A2C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82672A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672A40: 386A29E8  addi r3, r10, 0x29e8
	ctx.r[3].s64 = ctx.r[10].s64 + 10728;
	// 82672A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672A68: 4BDF43B9  bl 0x82466e20
	ctx.lr = 0x82672A6C;
	sub_82466E20(ctx, base);
	// 82672A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672A80 size=108
    let mut pc: u32 = 0x82672A80;
    'dispatch: loop {
        match pc {
            0x82672A80 => {
    //   block [0x82672A80..0x82672AEC)
	// 82672A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672A8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672A94: 38EB1C60  addi r7, r11, 0x1c60
	ctx.r[7].s64 = ctx.r[11].s64 + 7264;
	// 82672A98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82672A9C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82672AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672AA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672AB0: 386A2A18  addi r3, r10, 0x2a18
	ctx.r[3].s64 = ctx.r[10].s64 + 10776;
	// 82672AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672AD8: 4BDF4349  bl 0x82466e20
	ctx.lr = 0x82672ADC;
	sub_82466E20(ctx, base);
	// 82672ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672AF0 size=100
    let mut pc: u32 = 0x82672AF0;
    'dispatch: loop {
        match pc {
            0x82672AF0 => {
    //   block [0x82672AF0..0x82672B54)
	// 82672AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672AFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672B04: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672B10: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82672B14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672B24: 386A2A48  addi r3, r10, 0x2a48
	ctx.r[3].s64 = ctx.r[10].s64 + 10824;
	// 82672B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672B2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672B30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672B38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82672B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672B40: 4BDF42E1  bl 0x82466e20
	ctx.lr = 0x82672B44;
	sub_82466E20(ctx, base);
	// 82672B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672B58 size=112
    let mut pc: u32 = 0x82672B58;
    'dispatch: loop {
        match pc {
            0x82672B58 => {
    //   block [0x82672B58..0x82672BC8)
	// 82672B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672B64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672B6C: 38AA2A48  addi r5, r10, 0x2a48
	ctx.r[5].s64 = ctx.r[10].s64 + 10824;
	// 82672B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672B74: 390B1C78  addi r8, r11, 0x1c78
	ctx.r[8].s64 = ctx.r[11].s64 + 7288;
	// 82672B78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82672B7C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82672B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672B90: 386A2A78  addi r3, r10, 0x2a78
	ctx.r[3].s64 = ctx.r[10].s64 + 10872;
	// 82672B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672BB4: 4BDF426D  bl 0x82466e20
	ctx.lr = 0x82672BB8;
	sub_82466E20(ctx, base);
	// 82672BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672BC8 size=108
    let mut pc: u32 = 0x82672BC8;
    'dispatch: loop {
        match pc {
            0x82672BC8 => {
    //   block [0x82672BC8..0x82672C34)
	// 82672BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672BD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672BDC: 38EB1CC0  addi r7, r11, 0x1cc0
	ctx.r[7].s64 = ctx.r[11].s64 + 7360;
	// 82672BE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82672BE4: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82672BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672BEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672BF8: 386A2AA8  addi r3, r10, 0x2aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 10920;
	// 82672BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672C20: 4BDF4201  bl 0x82466e20
	ctx.lr = 0x82672C24;
	sub_82466E20(ctx, base);
	// 82672C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672C38 size=112
    let mut pc: u32 = 0x82672C38;
    'dispatch: loop {
        match pc {
            0x82672C38 => {
    //   block [0x82672C38..0x82672CA8)
	// 82672C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672C44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672C48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672C4C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672C54: 390B1D08  addi r8, r11, 0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + 7432;
	// 82672C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82672C5C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82672C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672C64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672C70: 386A2AD8  addi r3, r10, 0x2ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 10968;
	// 82672C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672C94: 4BDF418D  bl 0x82466e20
	ctx.lr = 0x82672C98;
	sub_82466E20(ctx, base);
	// 82672C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672CA8 size=108
    let mut pc: u32 = 0x82672CA8;
    'dispatch: loop {
        match pc {
            0x82672CA8 => {
    //   block [0x82672CA8..0x82672D14)
	// 82672CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672CB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672CBC: 38EB1D20  addi r7, r11, 0x1d20
	ctx.r[7].s64 = ctx.r[11].s64 + 7456;
	// 82672CC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82672CC4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82672CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672CD8: 386A2B08  addi r3, r10, 0x2b08
	ctx.r[3].s64 = ctx.r[10].s64 + 11016;
	// 82672CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672D00: 4BDF4121  bl 0x82466e20
	ctx.lr = 0x82672D04;
	sub_82466E20(ctx, base);
	// 82672D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672D18 size=112
    let mut pc: u32 = 0x82672D18;
    'dispatch: loop {
        match pc {
            0x82672D18 => {
    //   block [0x82672D18..0x82672D88)
	// 82672D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672D24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672D28: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672D2C: 38AA2AD8  addi r5, r10, 0x2ad8
	ctx.r[5].s64 = ctx.r[10].s64 + 10968;
	// 82672D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672D34: 390B1D68  addi r8, r11, 0x1d68
	ctx.r[8].s64 = ctx.r[11].s64 + 7528;
	// 82672D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82672D3C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82672D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672D44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672D50: 386A2B38  addi r3, r10, 0x2b38
	ctx.r[3].s64 = ctx.r[10].s64 + 11064;
	// 82672D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672D74: 4BDF40AD  bl 0x82466e20
	ctx.lr = 0x82672D78;
	sub_82466E20(ctx, base);
	// 82672D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672D88 size=100
    let mut pc: u32 = 0x82672D88;
    'dispatch: loop {
        match pc {
            0x82672D88 => {
    //   block [0x82672D88..0x82672DEC)
	// 82672D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672D94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672D9C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672DA8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82672DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672DBC: 386A2B68  addi r3, r10, 0x2b68
	ctx.r[3].s64 = ctx.r[10].s64 + 11112;
	// 82672DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672DC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672DC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82672DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672DD8: 4BDF4049  bl 0x82466e20
	ctx.lr = 0x82672DDC;
	sub_82466E20(ctx, base);
	// 82672DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672DF0 size=112
    let mut pc: u32 = 0x82672DF0;
    'dispatch: loop {
        match pc {
            0x82672DF0 => {
    //   block [0x82672DF0..0x82672E60)
	// 82672DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672DFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672E00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672E04: 38AA2B68  addi r5, r10, 0x2b68
	ctx.r[5].s64 = ctx.r[10].s64 + 11112;
	// 82672E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672E0C: 390B1D80  addi r8, r11, 0x1d80
	ctx.r[8].s64 = ctx.r[11].s64 + 7552;
	// 82672E10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82672E14: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82672E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672E1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672E28: 386A2B98  addi r3, r10, 0x2b98
	ctx.r[3].s64 = ctx.r[10].s64 + 11160;
	// 82672E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672E4C: 4BDF3FD5  bl 0x82466e20
	ctx.lr = 0x82672E50;
	sub_82466E20(ctx, base);
	// 82672E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672E60 size=108
    let mut pc: u32 = 0x82672E60;
    'dispatch: loop {
        match pc {
            0x82672E60 => {
    //   block [0x82672E60..0x82672ECC)
	// 82672E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672E6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672E74: 38EB1E28  addi r7, r11, 0x1e28
	ctx.r[7].s64 = ctx.r[11].s64 + 7720;
	// 82672E78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82672E7C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82672E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672E90: 386A2BC8  addi r3, r10, 0x2bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 11208;
	// 82672E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672EB8: 4BDF3F69  bl 0x82466e20
	ctx.lr = 0x82672EBC;
	sub_82466E20(ctx, base);
	// 82672EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672ED0 size=112
    let mut pc: u32 = 0x82672ED0;
    'dispatch: loop {
        match pc {
            0x82672ED0 => {
    //   block [0x82672ED0..0x82672F40)
	// 82672ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672EE0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672EE4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672EEC: 390B1E58  addi r8, r11, 0x1e58
	ctx.r[8].s64 = ctx.r[11].s64 + 7768;
	// 82672EF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82672EF4: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82672EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672EFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672F08: 386A2BF8  addi r3, r10, 0x2bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 11256;
	// 82672F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672F2C: 4BDF3EF5  bl 0x82466e20
	ctx.lr = 0x82672F30;
	sub_82466E20(ctx, base);
	// 82672F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672F40 size=112
    let mut pc: u32 = 0x82672F40;
    'dispatch: loop {
        match pc {
            0x82672F40 => {
    //   block [0x82672F40..0x82672FB0)
	// 82672F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672F50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672F54: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672F5C: 390B1EA0  addi r8, r11, 0x1ea0
	ctx.r[8].s64 = ctx.r[11].s64 + 7840;
	// 82672F60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82672F64: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82672F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672F78: 386A2C28  addi r3, r10, 0x2c28
	ctx.r[3].s64 = ctx.r[10].s64 + 11304;
	// 82672F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672F9C: 4BDF3E85  bl 0x82466e20
	ctx.lr = 0x82672FA0;
	sub_82466E20(ctx, base);
	// 82672FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672FB0 size=100
    let mut pc: u32 = 0x82672FB0;
    'dispatch: loop {
        match pc {
            0x82672FB0 => {
    //   block [0x82672FB0..0x82673014)
	// 82672FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672FBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672FC4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672FD0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82672FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672FE4: 386A2C58  addi r3, r10, 0x2c58
	ctx.r[3].s64 = ctx.r[10].s64 + 11352;
	// 82672FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672FEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672FF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672FF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82672FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673000: 4BDF3E21  bl 0x82466e20
	ctx.lr = 0x82673004;
	sub_82466E20(ctx, base);
	// 82673004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673018 size=112
    let mut pc: u32 = 0x82673018;
    'dispatch: loop {
        match pc {
            0x82673018 => {
    //   block [0x82673018..0x82673088)
	// 82673018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673028: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267302C: 38AA2C58  addi r5, r10, 0x2c58
	ctx.r[5].s64 = ctx.r[10].s64 + 11352;
	// 82673030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673034: 390B1EE8  addi r8, r11, 0x1ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 7912;
	// 82673038: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267303C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82673040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267304C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673050: 386A2C88  addi r3, r10, 0x2c88
	ctx.r[3].s64 = ctx.r[10].s64 + 11400;
	// 82673054: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267305C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267306C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673074: 4BDF3DAD  bl 0x82466e20
	ctx.lr = 0x82673078;
	sub_82466E20(ctx, base);
	// 82673078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267307C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673088 size=112
    let mut pc: u32 = 0x82673088;
    'dispatch: loop {
        match pc {
            0x82673088 => {
    //   block [0x82673088..0x826730F8)
	// 82673088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673098: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267309C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826730A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826730A4: 390B1F30  addi r8, r11, 0x1f30
	ctx.r[8].s64 = ctx.r[11].s64 + 7984;
	// 826730A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826730AC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826730B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826730B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826730B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826730BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826730C0: 386A2CB8  addi r3, r10, 0x2cb8
	ctx.r[3].s64 = ctx.r[10].s64 + 11448;
	// 826730C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826730C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826730CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826730D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826730D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826730D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826730DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826730E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826730E4: 4BDF3D3D  bl 0x82466e20
	ctx.lr = 0x826730E8;
	sub_82466E20(ctx, base);
	// 826730E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826730EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826730F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826730F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826730F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826730F8 size=112
    let mut pc: u32 = 0x826730F8;
    'dispatch: loop {
        match pc {
            0x826730F8 => {
    //   block [0x826730F8..0x82673168)
	// 826730F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826730FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673108: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267310C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82673110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673114: 390B1F48  addi r8, r11, 0x1f48
	ctx.r[8].s64 = ctx.r[11].s64 + 8008;
	// 82673118: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267311C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82673120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267312C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673130: 386A2CE8  addi r3, r10, 0x2ce8
	ctx.r[3].s64 = ctx.r[10].s64 + 11496;
	// 82673134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267313C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82673148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267314C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673154: 4BDF3CCD  bl 0x82466e20
	ctx.lr = 0x82673158;
	sub_82466E20(ctx, base);
	// 82673158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267315C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673168 size=112
    let mut pc: u32 = 0x82673168;
    'dispatch: loop {
        match pc {
            0x82673168 => {
    //   block [0x82673168..0x826731D8)
	// 82673168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267316C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673178: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267317C: 38AA2CB8  addi r5, r10, 0x2cb8
	ctx.r[5].s64 = ctx.r[10].s64 + 11448;
	// 82673180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673184: 390B1F60  addi r8, r11, 0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + 8032;
	// 82673188: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267318C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82673190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267319C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826731A0: 386A2D18  addi r3, r10, 0x2d18
	ctx.r[3].s64 = ctx.r[10].s64 + 11544;
	// 826731A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826731A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826731AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826731B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826731B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826731B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826731BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826731C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826731C4: 4BDF3C5D  bl 0x82466e20
	ctx.lr = 0x826731C8;
	sub_82466E20(ctx, base);
	// 826731C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826731CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826731D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826731D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826731D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826731D8 size=72
    let mut pc: u32 = 0x826731D8;
    'dispatch: loop {
        match pc {
            0x826731D8 => {
    //   block [0x826731D8..0x82673220)
	// 826731D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826731DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826731E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826731E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826731E8: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 826731EC: 38CB1250  addi r6, r11, 0x1250
	ctx.r[6].s64 = ctx.r[11].s64 + 4688;
	// 826731F0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826731F4: 388B1C08  addi r4, r11, 0x1c08
	ctx.r[4].s64 = ctx.r[11].s64 + 7176;
	// 826731F8: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826731FC: 386B2D48  addi r3, r11, 0x2d48
	ctx.r[3].s64 = ctx.r[11].s64 + 11592;
	// 82673200: 4BE08889  bl 0x8247ba88
	ctx.lr = 0x82673204;
	sub_8247BA88(ctx, base);
	// 82673204: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82673208: 386BCE10  addi r3, r11, -0x31f0
	ctx.r[3].s64 = ctx.r[11].s64 + -12784;
	// 8267320C: 4BEBF92D  bl 0x82532b38
	ctx.lr = 0x82673210;
	sub_82532B38(ctx, base);
	// 82673210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82673214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267321C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673220 size=108
    let mut pc: u32 = 0x82673220;
    'dispatch: loop {
        match pc {
            0x82673220 => {
    //   block [0x82673220..0x8267328C)
	// 82673220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267322C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673234: 38EB2890  addi r7, r11, 0x2890
	ctx.r[7].s64 = ctx.r[11].s64 + 10384;
	// 82673238: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8267323C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82673240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267324C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673250: 386A2D60  addi r3, r10, 0x2d60
	ctx.r[3].s64 = ctx.r[10].s64 + 11616;
	// 82673254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267325C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267326C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673278: 4BDF3BA9  bl 0x82466e20
	ctx.lr = 0x8267327C;
	sub_82466E20(ctx, base);
	// 8267327C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673290 size=108
    let mut pc: u32 = 0x82673290;
    'dispatch: loop {
        match pc {
            0x82673290 => {
    //   block [0x82673290..0x826732FC)
	// 82673290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267329C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826732A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826732A4: 38EB2908  addi r7, r11, 0x2908
	ctx.r[7].s64 = ctx.r[11].s64 + 10504;
	// 826732A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826732AC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826732B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826732B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826732B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826732BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826732C0: 386A2D90  addi r3, r10, 0x2d90
	ctx.r[3].s64 = ctx.r[10].s64 + 11664;
	// 826732C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826732C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826732CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826732D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826732D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826732D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826732DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826732E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826732E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826732E8: 4BDF3B39  bl 0x82466e20
	ctx.lr = 0x826732EC;
	sub_82466E20(ctx, base);
	// 826732EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826732F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826732F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826732F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673300 size=108
    let mut pc: u32 = 0x82673300;
    'dispatch: loop {
        match pc {
            0x82673300 => {
    //   block [0x82673300..0x8267336C)
	// 82673300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267330C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673310: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673314: 38EB2938  addi r7, r11, 0x2938
	ctx.r[7].s64 = ctx.r[11].s64 + 10552;
	// 82673318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267331C: 388AAEA0  addi r4, r10, -0x5160
	ctx.r[4].s64 = ctx.r[10].s64 + -20832;
	// 82673320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267332C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673330: 386A2DC0  addi r3, r10, 0x2dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 11712;
	// 82673334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267333C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267334C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673358: 4BDF3AC9  bl 0x82466e20
	ctx.lr = 0x8267335C;
	sub_82466E20(ctx, base);
	// 8267335C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673370 size=108
    let mut pc: u32 = 0x82673370;
    'dispatch: loop {
        match pc {
            0x82673370 => {
    //   block [0x82673370..0x826733DC)
	// 82673370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267337C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673380: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673384: 38EB2968  addi r7, r11, 0x2968
	ctx.r[7].s64 = ctx.r[11].s64 + 10600;
	// 82673388: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267338C: 388AAE2C  addi r4, r10, -0x51d4
	ctx.r[4].s64 = ctx.r[10].s64 + -20948;
	// 82673390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826733A0: 386A2DF0  addi r3, r10, 0x2df0
	ctx.r[3].s64 = ctx.r[10].s64 + 11760;
	// 826733A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826733A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826733AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826733B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826733B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826733B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826733BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826733C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826733C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826733C8: 4BDF3A59  bl 0x82466e20
	ctx.lr = 0x826733CC;
	sub_82466E20(ctx, base);
	// 826733CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826733D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826733D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826733D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826733E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826733E0 size=112
    let mut pc: u32 = 0x826733E0;
    'dispatch: loop {
        match pc {
            0x826733E0 => {
    //   block [0x826733E0..0x82673450)
	// 826733E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826733E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826733E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826733EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826733F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826733F4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826733F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826733FC: 390B2998  addi r8, r11, 0x2998
	ctx.r[8].s64 = ctx.r[11].s64 + 10648;
	// 82673400: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82673404: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82673408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267340C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673418: 386A2E20  addi r3, r10, 0x2e20
	ctx.r[3].s64 = ctx.r[10].s64 + 11808;
	// 8267341C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267342C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267343C: 4BDF39E5  bl 0x82466e20
	ctx.lr = 0x82673440;
	sub_82466E20(ctx, base);
	// 82673440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267344C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673450 size=108
    let mut pc: u32 = 0x82673450;
    'dispatch: loop {
        match pc {
            0x82673450 => {
    //   block [0x82673450..0x826734BC)
	// 82673450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267345C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673460: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673464: 38EB29F8  addi r7, r11, 0x29f8
	ctx.r[7].s64 = ctx.r[11].s64 + 10744;
	// 82673468: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267346C: 388AAE70  addi r4, r10, -0x5190
	ctx.r[4].s64 = ctx.r[10].s64 + -20880;
	// 82673470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673474: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673478: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267347C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673480: 386A2E50  addi r3, r10, 0x2e50
	ctx.r[3].s64 = ctx.r[10].s64 + 11856;
	// 82673484: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267348C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267349C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826734A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826734A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826734A8: 4BDF3979  bl 0x82466e20
	ctx.lr = 0x826734AC;
	sub_82466E20(ctx, base);
	// 826734AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826734B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826734B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826734B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826734C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826734C0 size=112
    let mut pc: u32 = 0x826734C0;
    'dispatch: loop {
        match pc {
            0x826734C0 => {
    //   block [0x826734C0..0x82673530)
	// 826734C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826734C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826734C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826734CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826734D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826734D4: 38AA2E20  addi r5, r10, 0x2e20
	ctx.r[5].s64 = ctx.r[10].s64 + 11808;
	// 826734D8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826734DC: 390B2A58  addi r8, r11, 0x2a58
	ctx.r[8].s64 = ctx.r[11].s64 + 10840;
	// 826734E0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826734E4: 388AAF30  addi r4, r10, -0x50d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20688;
	// 826734E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826734EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826734F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826734F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826734F8: 386A2E80  addi r3, r10, 0x2e80
	ctx.r[3].s64 = ctx.r[10].s64 + 11904;
	// 826734FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267350C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267351C: 4BDF3905  bl 0x82466e20
	ctx.lr = 0x82673520;
	sub_82466E20(ctx, base);
	// 82673520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267352C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673530 size=112
    let mut pc: u32 = 0x82673530;
    'dispatch: loop {
        match pc {
            0x82673530 => {
    //   block [0x82673530..0x826735A0)
	// 82673530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267353C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673540: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673544: 38AA2E20  addi r5, r10, 0x2e20
	ctx.r[5].s64 = ctx.r[10].s64 + 11808;
	// 82673548: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267354C: 390B2AE8  addi r8, r11, 0x2ae8
	ctx.r[8].s64 = ctx.r[11].s64 + 10984;
	// 82673550: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82673554: 388AAF18  addi r4, r10, -0x50e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20712;
	// 82673558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267355C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673568: 386A2EB0  addi r3, r10, 0x2eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 11952;
	// 8267356C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267357C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267358C: 4BDF3895  bl 0x82466e20
	ctx.lr = 0x82673590;
	sub_82466E20(ctx, base);
	// 82673590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267359C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826735A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826735A0 size=108
    let mut pc: u32 = 0x826735A0;
    'dispatch: loop {
        match pc {
            0x826735A0 => {
    //   block [0x826735A0..0x8267360C)
	// 826735A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826735A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826735A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826735AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826735B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826735B4: 38EB2B00  addi r7, r11, 0x2b00
	ctx.r[7].s64 = ctx.r[11].s64 + 11008;
	// 826735B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826735BC: 388AAE40  addi r4, r10, -0x51c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20928;
	// 826735C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826735C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826735C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826735CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826735D0: 386A2EE0  addi r3, r10, 0x2ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 12000;
	// 826735D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826735D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826735DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826735E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826735E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826735E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826735EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826735F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826735F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826735F8: 4BDF3829  bl 0x82466e20
	ctx.lr = 0x826735FC;
	sub_82466E20(ctx, base);
	// 826735FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673610 size=112
    let mut pc: u32 = 0x82673610;
    'dispatch: loop {
        match pc {
            0x82673610 => {
    //   block [0x82673610..0x82673680)
	// 82673610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267361C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673620: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673624: 38AA2E20  addi r5, r10, 0x2e20
	ctx.r[5].s64 = ctx.r[10].s64 + 11808;
	// 82673628: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267362C: 390B2B60  addi r8, r11, 0x2b60
	ctx.r[8].s64 = ctx.r[11].s64 + 11104;
	// 82673630: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82673634: 388AAEF8  addi r4, r10, -0x5108
	ctx.r[4].s64 = ctx.r[10].s64 + -20744;
	// 82673638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267363C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673648: 386A2F10  addi r3, r10, 0x2f10
	ctx.r[3].s64 = ctx.r[10].s64 + 12048;
	// 8267364C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267365C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267366C: 4BDF37B5  bl 0x82466e20
	ctx.lr = 0x82673670;
	sub_82466E20(ctx, base);
	// 82673670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267367C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673680 size=108
    let mut pc: u32 = 0x82673680;
    'dispatch: loop {
        match pc {
            0x82673680 => {
    //   block [0x82673680..0x826736EC)
	// 82673680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267368C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673694: 38EB2C08  addi r7, r11, 0x2c08
	ctx.r[7].s64 = ctx.r[11].s64 + 11272;
	// 82673698: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267369C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826736A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826736A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826736A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826736AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826736B0: 386A2F40  addi r3, r10, 0x2f40
	ctx.r[3].s64 = ctx.r[10].s64 + 12096;
	// 826736B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826736B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826736BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826736C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826736C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826736C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826736CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826736D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826736D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826736D8: 4BDF3749  bl 0x82466e20
	ctx.lr = 0x826736DC;
	sub_82466E20(ctx, base);
	// 826736DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826736E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826736E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826736E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826736F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826736F0 size=100
    let mut pc: u32 = 0x826736F0;
    'dispatch: loop {
        match pc {
            0x826736F0 => {
    //   block [0x826736F0..0x82673754)
	// 826736F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826736F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826736F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826736FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673704: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82673708: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267370C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673710: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 82673714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267371C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673724: 386A2F70  addi r3, r10, 0x2f70
	ctx.r[3].s64 = ctx.r[10].s64 + 12144;
	// 82673728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267372C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673730: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82673734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673740: 4BDF36E1  bl 0x82466e20
	ctx.lr = 0x82673744;
	sub_82466E20(ctx, base);
	// 82673744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267374C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673758 size=112
    let mut pc: u32 = 0x82673758;
    'dispatch: loop {
        match pc {
            0x82673758 => {
    //   block [0x82673758..0x826737C8)
	// 82673758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267375C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673768: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267376C: 38AA2F70  addi r5, r10, 0x2f70
	ctx.r[5].s64 = ctx.r[10].s64 + 12144;
	// 82673770: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673774: 390B2C68  addi r8, r11, 0x2c68
	ctx.r[8].s64 = ctx.r[11].s64 + 11368;
	// 82673778: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267377C: 388AAF4C  addi r4, r10, -0x50b4
	ctx.r[4].s64 = ctx.r[10].s64 + -20660;
	// 82673780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267378C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673790: 386A2FA0  addi r3, r10, 0x2fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 12192;
	// 82673794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267379C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826737A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826737A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826737A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826737AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826737B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826737B4: 4BDF366D  bl 0x82466e20
	ctx.lr = 0x826737B8;
	sub_82466E20(ctx, base);
	// 826737B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826737BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826737C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826737C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826737C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826737C8 size=96
    let mut pc: u32 = 0x826737C8;
    'dispatch: loop {
        match pc {
            0x826737C8 => {
    //   block [0x826737C8..0x82673828)
	// 826737C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826737CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826737D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826737D4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826737D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826737DC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826737E0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826737E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826737E8: 386A2FD0  addi r3, r10, 0x2fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 12240;
	// 826737EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826737F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826737F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826737F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826737FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673808: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267380C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82673814: 4BDF360D  bl 0x82466e20
	ctx.lr = 0x82673818;
	sub_82466E20(ctx, base);
	// 82673818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267381C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673828 size=112
    let mut pc: u32 = 0x82673828;
    'dispatch: loop {
        match pc {
            0x82673828 => {
    //   block [0x82673828..0x82673898)
	// 82673828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267382C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673834: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673838: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267383C: 38AA2FD0  addi r5, r10, 0x2fd0
	ctx.r[5].s64 = ctx.r[10].s64 + 12240;
	// 82673840: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673844: 390B2CE0  addi r8, r11, 0x2ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 11488;
	// 82673848: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267384C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82673850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673854: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267385C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673860: 386A3000  addi r3, r10, 0x3000
	ctx.r[3].s64 = ctx.r[10].s64 + 12288;
	// 82673864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267386C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267387C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673884: 4BDF359D  bl 0x82466e20
	ctx.lr = 0x82673888;
	sub_82466E20(ctx, base);
	// 82673888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267388C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673898 size=108
    let mut pc: u32 = 0x82673898;
    'dispatch: loop {
        match pc {
            0x82673898 => {
    //   block [0x82673898..0x82673904)
	// 82673898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267389C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826738A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826738A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826738A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826738AC: 38EB2D10  addi r7, r11, 0x2d10
	ctx.r[7].s64 = ctx.r[11].s64 + 11536;
	// 826738B0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826738B4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826738B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826738BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826738C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826738C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826738C8: 386A3030  addi r3, r10, 0x3030
	ctx.r[3].s64 = ctx.r[10].s64 + 12336;
	// 826738CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826738D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826738D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826738D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826738DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826738E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826738E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826738E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826738EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826738F0: 4BDF3531  bl 0x82466e20
	ctx.lr = 0x826738F4;
	sub_82466E20(ctx, base);
	// 826738F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826738F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826738FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673908 size=108
    let mut pc: u32 = 0x82673908;
    'dispatch: loop {
        match pc {
            0x82673908 => {
    //   block [0x82673908..0x82673974)
	// 82673908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267390C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673914: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673918: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267391C: 38EB2DB8  addi r7, r11, 0x2db8
	ctx.r[7].s64 = ctx.r[11].s64 + 11704;
	// 82673920: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673924: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82673928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267392C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673938: 386A3060  addi r3, r10, 0x3060
	ctx.r[3].s64 = ctx.r[10].s64 + 12384;
	// 8267393C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267394C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267395C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673960: 4BDF34C1  bl 0x82466e20
	ctx.lr = 0x82673964;
	sub_82466E20(ctx, base);
	// 82673964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267396C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673978 size=108
    let mut pc: u32 = 0x82673978;
    'dispatch: loop {
        match pc {
            0x82673978 => {
    //   block [0x82673978..0x826739E4)
	// 82673978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673984: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673988: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8267398C: 38EB2DE8  addi r7, r11, 0x2de8
	ctx.r[7].s64 = ctx.r[11].s64 + 11752;
	// 82673990: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673994: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82673998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267399C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826739A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826739A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826739A8: 386A3090  addi r3, r10, 0x3090
	ctx.r[3].s64 = ctx.r[10].s64 + 12432;
	// 826739AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826739B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826739B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826739B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826739BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826739C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826739C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826739C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826739CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826739D0: 4BDF3451  bl 0x82466e20
	ctx.lr = 0x826739D4;
	sub_82466E20(ctx, base);
	// 826739D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826739D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826739DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826739E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826739E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826739E8 size=28
    let mut pc: u32 = 0x826739E8;
    'dispatch: loop {
        match pc {
            0x826739E8 => {
    //   block [0x826739E8..0x82673A04)
	// 826739E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826739EC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826739F0: 394A6BA0  addi r10, r10, 0x6ba0
	ctx.r[10].s64 = ctx.r[10].s64 + 27552;
	// 826739F4: 816B2E18  lwz r11, 0x2e18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11800 as u32) ) } as u64;
	// 826739F8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826739FC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82673A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673A08 size=112
    let mut pc: u32 = 0x82673A08;
    'dispatch: loop {
        match pc {
            0x82673A08 => {
    //   block [0x82673A08..0x82673A78)
	// 82673A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673A14: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673A18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673A1C: 392A30F0  addi r9, r10, 0x30f0
	ctx.r[9].s64 = ctx.r[10].s64 + 12528;
	// 82673A20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673A24: 390B6BA0  addi r8, r11, 0x6ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 27552;
	// 82673A28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82673A2C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82673A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673A40: 386A30C0  addi r3, r10, 0x30c0
	ctx.r[3].s64 = ctx.r[10].s64 + 12480;
	// 82673A44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82673A48: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82673A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673A64: 4BDF33BD  bl 0x82466e20
	ctx.lr = 0x82673A68;
	sub_82466E20(ctx, base);
	// 82673A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673A78 size=108
    let mut pc: u32 = 0x82673A78;
    'dispatch: loop {
        match pc {
            0x82673A78 => {
    //   block [0x82673A78..0x82673AE4)
	// 82673A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673A84: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673A88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673A8C: 38EB2E24  addi r7, r11, 0x2e24
	ctx.r[7].s64 = ctx.r[11].s64 + 11812;
	// 82673A90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673A94: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82673A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673A9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673AA8: 386A30F0  addi r3, r10, 0x30f0
	ctx.r[3].s64 = ctx.r[10].s64 + 12528;
	// 82673AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673AD0: 4BDF3351  bl 0x82466e20
	ctx.lr = 0x82673AD4;
	sub_82466E20(ctx, base);
	// 82673AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673AE8 size=108
    let mut pc: u32 = 0x82673AE8;
    'dispatch: loop {
        match pc {
            0x82673AE8 => {
    //   block [0x82673AE8..0x82673B54)
	// 82673AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673AF4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673AF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673AFC: 38EB2E54  addi r7, r11, 0x2e54
	ctx.r[7].s64 = ctx.r[11].s64 + 11860;
	// 82673B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673B04: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82673B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673B0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673B18: 386A3120  addi r3, r10, 0x3120
	ctx.r[3].s64 = ctx.r[10].s64 + 12576;
	// 82673B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673B40: 4BDF32E1  bl 0x82466e20
	ctx.lr = 0x82673B44;
	sub_82466E20(ctx, base);
	// 82673B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673B58 size=108
    let mut pc: u32 = 0x82673B58;
    'dispatch: loop {
        match pc {
            0x82673B58 => {
    //   block [0x82673B58..0x82673BC4)
	// 82673B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673B64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673B68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673B6C: 38EB2E88  addi r7, r11, 0x2e88
	ctx.r[7].s64 = ctx.r[11].s64 + 11912;
	// 82673B70: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82673B74: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82673B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673B7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673B88: 386A3150  addi r3, r10, 0x3150
	ctx.r[3].s64 = ctx.r[10].s64 + 12624;
	// 82673B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673BB0: 4BDF3271  bl 0x82466e20
	ctx.lr = 0x82673BB4;
	sub_82466E20(ctx, base);
	// 82673BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673BC8 size=108
    let mut pc: u32 = 0x82673BC8;
    'dispatch: loop {
        match pc {
            0x82673BC8 => {
    //   block [0x82673BC8..0x82673C34)
	// 82673BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673BD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673BD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82673BDC: 38EB2F48  addi r7, r11, 0x2f48
	ctx.r[7].s64 = ctx.r[11].s64 + 12104;
	// 82673BE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82673BE4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82673BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673BEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673BF8: 386A3180  addi r3, r10, 0x3180
	ctx.r[3].s64 = ctx.r[10].s64 + 12672;
	// 82673BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673C20: 4BDF3201  bl 0x82466e20
	ctx.lr = 0x82673C24;
	sub_82466E20(ctx, base);
	// 82673C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673C38 size=108
    let mut pc: u32 = 0x82673C38;
    'dispatch: loop {
        match pc {
            0x82673C38 => {
    //   block [0x82673C38..0x82673CA4)
	// 82673C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673C44: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673C4C: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 82673C50: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82673C54: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82673C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673C5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673C60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673C68: 386A31B0  addi r3, r10, 0x31b0
	ctx.r[3].s64 = ctx.r[10].s64 + 12720;
	// 82673C6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673C8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673C90: 4BDF3191  bl 0x82466e20
	ctx.lr = 0x82673C94;
	sub_82466E20(ctx, base);
	// 82673C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673CA8 size=108
    let mut pc: u32 = 0x82673CA8;
    'dispatch: loop {
        match pc {
            0x82673CA8 => {
    //   block [0x82673CA8..0x82673D14)
	// 82673CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673CB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673CBC: 38EB2FD8  addi r7, r11, 0x2fd8
	ctx.r[7].s64 = ctx.r[11].s64 + 12248;
	// 82673CC0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82673CC4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82673CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673CD8: 386A31E0  addi r3, r10, 0x31e0
	ctx.r[3].s64 = ctx.r[10].s64 + 12768;
	// 82673CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673D00: 4BDF3121  bl 0x82466e20
	ctx.lr = 0x82673D04;
	sub_82466E20(ctx, base);
	// 82673D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673D18 size=108
    let mut pc: u32 = 0x82673D18;
    'dispatch: loop {
        match pc {
            0x82673D18 => {
    //   block [0x82673D18..0x82673D84)
	// 82673D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673D24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673D2C: 38EB3068  addi r7, r11, 0x3068
	ctx.r[7].s64 = ctx.r[11].s64 + 12392;
	// 82673D30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673D34: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82673D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673D48: 386A3210  addi r3, r10, 0x3210
	ctx.r[3].s64 = ctx.r[10].s64 + 12816;
	// 82673D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673D70: 4BDF30B1  bl 0x82466e20
	ctx.lr = 0x82673D74;
	sub_82466E20(ctx, base);
	// 82673D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673D88 size=108
    let mut pc: u32 = 0x82673D88;
    'dispatch: loop {
        match pc {
            0x82673D88 => {
    //   block [0x82673D88..0x82673DF4)
	// 82673D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673D94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673D9C: 38EB3098  addi r7, r11, 0x3098
	ctx.r[7].s64 = ctx.r[11].s64 + 12440;
	// 82673DA0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82673DA4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82673DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673DB8: 386A3240  addi r3, r10, 0x3240
	ctx.r[3].s64 = ctx.r[10].s64 + 12864;
	// 82673DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673DE0: 4BDF3041  bl 0x82466e20
	ctx.lr = 0x82673DE4;
	sub_82466E20(ctx, base);
	// 82673DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82673DF8 size=24
    let mut pc: u32 = 0x82673DF8;
    'dispatch: loop {
        match pc {
            0x82673DF8 => {
    //   block [0x82673DF8..0x82673E10)
	// 82673DF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673DFC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82673E00: 394A6C60  addi r10, r10, 0x6c60
	ctx.r[10].s64 = ctx.r[10].s64 + 27744;
	// 82673E04: 816B2E84  lwz r11, 0x2e84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11908 as u32) ) } as u64;
	// 82673E08: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82673E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673E10 size=112
    let mut pc: u32 = 0x82673E10;
    'dispatch: loop {
        match pc {
            0x82673E10 => {
    //   block [0x82673E10..0x82673E80)
	// 82673E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673E1C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673E20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673E24: 392A3144  addi r9, r10, 0x3144
	ctx.r[9].s64 = ctx.r[10].s64 + 12612;
	// 82673E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673E2C: 390B6C60  addi r8, r11, 0x6c60
	ctx.r[8].s64 = ctx.r[11].s64 + 27744;
	// 82673E30: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82673E34: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82673E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673E3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673E48: 386A3270  addi r3, r10, 0x3270
	ctx.r[3].s64 = ctx.r[10].s64 + 12912;
	// 82673E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82673E50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82673E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673E6C: 4BDF2FB5  bl 0x82466e20
	ctx.lr = 0x82673E70;
	sub_82466E20(ctx, base);
	// 82673E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673E80 size=108
    let mut pc: u32 = 0x82673E80;
    'dispatch: loop {
        match pc {
            0x82673E80 => {
    //   block [0x82673E80..0x82673EEC)
	// 82673E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673E8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673E94: 38EB3144  addi r7, r11, 0x3144
	ctx.r[7].s64 = ctx.r[11].s64 + 12612;
	// 82673E98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673E9C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82673EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673EB0: 386A32A0  addi r3, r10, 0x32a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12960;
	// 82673EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673ED8: 4BDF2F49  bl 0x82466e20
	ctx.lr = 0x82673EDC;
	sub_82466E20(ctx, base);
	// 82673EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673EF0 size=112
    let mut pc: u32 = 0x82673EF0;
    'dispatch: loop {
        match pc {
            0x82673EF0 => {
    //   block [0x82673EF0..0x82673F60)
	// 82673EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673EFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82673F00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673F04: 392A3188  addi r9, r10, 0x3188
	ctx.r[9].s64 = ctx.r[10].s64 + 12680;
	// 82673F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673F0C: 390B3178  addi r8, r11, 0x3178
	ctx.r[8].s64 = ctx.r[11].s64 + 12664;
	// 82673F10: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82673F14: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82673F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673F1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673F20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82673F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673F28: 386A32D0  addi r3, r10, 0x32d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13008;
	// 82673F2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82673F30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82673F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673F4C: 4BDF2ED5  bl 0x82466e20
	ctx.lr = 0x82673F50;
	sub_82466E20(ctx, base);
	// 82673F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673F60 size=108
    let mut pc: u32 = 0x82673F60;
    'dispatch: loop {
        match pc {
            0x82673F60 => {
    //   block [0x82673F60..0x82673FCC)
	// 82673F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673F6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673F74: 38EB3208  addi r7, r11, 0x3208
	ctx.r[7].s64 = ctx.r[11].s64 + 12808;
	// 82673F78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82673F7C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82673F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673F90: 386A3300  addi r3, r10, 0x3300
	ctx.r[3].s64 = ctx.r[10].s64 + 13056;
	// 82673F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82673F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82673F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82673FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82673FB8: 4BDF2E69  bl 0x82466e20
	ctx.lr = 0x82673FBC;
	sub_82466E20(ctx, base);
	// 82673FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673FD0 size=108
    let mut pc: u32 = 0x82673FD0;
    'dispatch: loop {
        match pc {
            0x82673FD0 => {
    //   block [0x82673FD0..0x8267403C)
	// 82673FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82673FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673FDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82673FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673FE4: 38EB3220  addi r7, r11, 0x3220
	ctx.r[7].s64 = ctx.r[11].s64 + 12832;
	// 82673FE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82673FEC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82673FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82673FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674000: 386A3330  addi r3, r10, 0x3330
	ctx.r[3].s64 = ctx.r[10].s64 + 13104;
	// 82674004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267400C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267401C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674028: 4BDF2DF9  bl 0x82466e20
	ctx.lr = 0x8267402C;
	sub_82466E20(ctx, base);
	// 8267402C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82674040 size=24
    let mut pc: u32 = 0x82674040;
    'dispatch: loop {
        match pc {
            0x82674040 => {
    //   block [0x82674040..0x82674058)
	// 82674040: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674044: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82674048: 394A6CD8  addi r10, r10, 0x6cd8
	ctx.r[10].s64 = ctx.r[10].s64 + 27864;
	// 8267404C: 816B3174  lwz r11, 0x3174(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12660 as u32) ) } as u64;
	// 82674050: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82674054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674058 size=112
    let mut pc: u32 = 0x82674058;
    'dispatch: loop {
        match pc {
            0x82674058 => {
    //   block [0x82674058..0x826740C8)
	// 82674058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674064: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82674068: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267406C: 392A31C4  addi r9, r10, 0x31c4
	ctx.r[9].s64 = ctx.r[10].s64 + 12740;
	// 82674070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674074: 390B6CD8  addi r8, r11, 0x6cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 27864;
	// 82674078: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267407C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82674080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267408C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674090: 386A3360  addi r3, r10, 0x3360
	ctx.r[3].s64 = ctx.r[10].s64 + 13152;
	// 82674094: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82674098: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267409C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826740A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826740A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826740A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826740AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826740B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826740B4: 4BDF2D6D  bl 0x82466e20
	ctx.lr = 0x826740B8;
	sub_82466E20(ctx, base);
	// 826740B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826740BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826740C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826740C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826740C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826740C8 size=108
    let mut pc: u32 = 0x826740C8;
    'dispatch: loop {
        match pc {
            0x826740C8 => {
    //   block [0x826740C8..0x82674134)
	// 826740C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826740CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826740D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826740D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826740D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826740DC: 38EB3250  addi r7, r11, 0x3250
	ctx.r[7].s64 = ctx.r[11].s64 + 12880;
	// 826740E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826740E4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826740E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826740EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826740F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826740F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826740F8: 386A3390  addi r3, r10, 0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + 13200;
	// 826740FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267410C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267411C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674120: 4BDF2D01  bl 0x82466e20
	ctx.lr = 0x82674124;
	sub_82466E20(ctx, base);
	// 82674124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267412C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674138 size=108
    let mut pc: u32 = 0x82674138;
    'dispatch: loop {
        match pc {
            0x82674138 => {
    //   block [0x82674138..0x826741A4)
	// 82674138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674144: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267414C: 38EB3268  addi r7, r11, 0x3268
	ctx.r[7].s64 = ctx.r[11].s64 + 12904;
	// 82674150: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82674154: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82674158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267415C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674168: 386A33C0  addi r3, r10, 0x33c0
	ctx.r[3].s64 = ctx.r[10].s64 + 13248;
	// 8267416C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267417C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267418C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674190: 4BDF2C91  bl 0x82466e20
	ctx.lr = 0x82674194;
	sub_82466E20(ctx, base);
	// 82674194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267419C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826741A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826741A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826741A8 size=108
    let mut pc: u32 = 0x826741A8;
    'dispatch: loop {
        match pc {
            0x826741A8 => {
    //   block [0x826741A8..0x82674214)
	// 826741A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826741AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826741B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826741B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826741B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826741BC: 38EB32B0  addi r7, r11, 0x32b0
	ctx.r[7].s64 = ctx.r[11].s64 + 12976;
	// 826741C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826741C4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826741C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826741CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826741D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826741D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826741D8: 386A33F0  addi r3, r10, 0x33f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13296;
	// 826741DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826741E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826741E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826741E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826741EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826741F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826741F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826741F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826741FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674200: 4BDF2C21  bl 0x82466e20
	ctx.lr = 0x82674204;
	sub_82466E20(ctx, base);
	// 82674204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267420C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674218 size=108
    let mut pc: u32 = 0x82674218;
    'dispatch: loop {
        match pc {
            0x82674218 => {
    //   block [0x82674218..0x82674284)
	// 82674218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674224: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267422C: 38EB32E0  addi r7, r11, 0x32e0
	ctx.r[7].s64 = ctx.r[11].s64 + 13024;
	// 82674230: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82674234: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82674238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267423C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674248: 386A3420  addi r3, r10, 0x3420
	ctx.r[3].s64 = ctx.r[10].s64 + 13344;
	// 8267424C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267425C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267426C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674270: 4BDF2BB1  bl 0x82466e20
	ctx.lr = 0x82674274;
	sub_82466E20(ctx, base);
	// 82674274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267427C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674288 size=108
    let mut pc: u32 = 0x82674288;
    'dispatch: loop {
        match pc {
            0x82674288 => {
    //   block [0x82674288..0x826742F4)
	// 82674288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267428C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674294: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267429C: 38EB3400  addi r7, r11, 0x3400
	ctx.r[7].s64 = ctx.r[11].s64 + 13312;
	// 826742A0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826742A4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826742A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826742AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826742B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826742B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826742B8: 386A3450  addi r3, r10, 0x3450
	ctx.r[3].s64 = ctx.r[10].s64 + 13392;
	// 826742BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826742C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826742C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826742C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826742CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826742D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826742D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826742D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826742DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826742E0: 4BDF2B41  bl 0x82466e20
	ctx.lr = 0x826742E4;
	sub_82466E20(ctx, base);
	// 826742E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826742E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826742EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826742F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826742F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826742F8 size=108
    let mut pc: u32 = 0x826742F8;
    'dispatch: loop {
        match pc {
            0x826742F8 => {
    //   block [0x826742F8..0x82674364)
	// 826742F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826742FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674304: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267430C: 38EB3490  addi r7, r11, 0x3490
	ctx.r[7].s64 = ctx.r[11].s64 + 13456;
	// 82674310: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82674314: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82674318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267431C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674328: 386A3480  addi r3, r10, 0x3480
	ctx.r[3].s64 = ctx.r[10].s64 + 13440;
	// 8267432C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267433C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267434C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674350: 4BDF2AD1  bl 0x82466e20
	ctx.lr = 0x82674354;
	sub_82466E20(ctx, base);
	// 82674354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267435C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674368 size=108
    let mut pc: u32 = 0x82674368;
    'dispatch: loop {
        match pc {
            0x82674368 => {
    //   block [0x82674368..0x826743D4)
	// 82674368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267436C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674374: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267437C: 38EB3550  addi r7, r11, 0x3550
	ctx.r[7].s64 = ctx.r[11].s64 + 13648;
	// 82674380: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82674384: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82674388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267438C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674398: 386A34B0  addi r3, r10, 0x34b0
	ctx.r[3].s64 = ctx.r[10].s64 + 13488;
	// 8267439C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826743A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826743A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826743A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826743AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826743B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826743B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826743B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826743BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826743C0: 4BDF2A61  bl 0x82466e20
	ctx.lr = 0x826743C4;
	sub_82466E20(ctx, base);
	// 826743C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826743C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826743CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826743D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826743D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826743D8 size=108
    let mut pc: u32 = 0x826743D8;
    'dispatch: loop {
        match pc {
            0x826743D8 => {
    //   block [0x826743D8..0x82674444)
	// 826743D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826743DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826743E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826743E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826743E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826743EC: 38EB3628  addi r7, r11, 0x3628
	ctx.r[7].s64 = ctx.r[11].s64 + 13864;
	// 826743F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826743F4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826743F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826743FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674408: 386A34E0  addi r3, r10, 0x34e0
	ctx.r[3].s64 = ctx.r[10].s64 + 13536;
	// 8267440C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267441C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267442C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674430: 4BDF29F1  bl 0x82466e20
	ctx.lr = 0x82674434;
	sub_82466E20(ctx, base);
	// 82674434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267443C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674448 size=108
    let mut pc: u32 = 0x82674448;
    'dispatch: loop {
        match pc {
            0x82674448 => {
    //   block [0x82674448..0x826744B4)
	// 82674448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267444C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674454: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267445C: 38EB36E8  addi r7, r11, 0x36e8
	ctx.r[7].s64 = ctx.r[11].s64 + 14056;
	// 82674460: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82674464: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82674468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267446C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674478: 386A3510  addi r3, r10, 0x3510
	ctx.r[3].s64 = ctx.r[10].s64 + 13584;
	// 8267447C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267448C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267449C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826744A0: 4BDF2981  bl 0x82466e20
	ctx.lr = 0x826744A4;
	sub_82466E20(ctx, base);
	// 826744A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826744A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826744AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826744B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826744B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826744B8 size=108
    let mut pc: u32 = 0x826744B8;
    'dispatch: loop {
        match pc {
            0x826744B8 => {
    //   block [0x826744B8..0x82674524)
	// 826744B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826744BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826744C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826744C4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826744C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826744CC: 38EB3790  addi r7, r11, 0x3790
	ctx.r[7].s64 = ctx.r[11].s64 + 14224;
	// 826744D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826744D4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826744D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826744DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826744E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826744E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826744E8: 386A3540  addi r3, r10, 0x3540
	ctx.r[3].s64 = ctx.r[10].s64 + 13632;
	// 826744EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826744F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826744F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826744F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826744FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267450C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674510: 4BDF2911  bl 0x82466e20
	ctx.lr = 0x82674514;
	sub_82466E20(ctx, base);
	// 82674514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267451C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674528 size=108
    let mut pc: u32 = 0x82674528;
    'dispatch: loop {
        match pc {
            0x82674528 => {
    //   block [0x82674528..0x82674594)
	// 82674528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267452C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674534: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267453C: 38EB37D8  addi r7, r11, 0x37d8
	ctx.r[7].s64 = ctx.r[11].s64 + 14296;
	// 82674540: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674544: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82674548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267454C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674558: 386A3570  addi r3, r10, 0x3570
	ctx.r[3].s64 = ctx.r[10].s64 + 13680;
	// 8267455C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267456C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267457C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674580: 4BDF28A1  bl 0x82466e20
	ctx.lr = 0x82674584;
	sub_82466E20(ctx, base);
	// 82674584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267458C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674598 size=108
    let mut pc: u32 = 0x82674598;
    'dispatch: loop {
        match pc {
            0x82674598 => {
    //   block [0x82674598..0x82674604)
	// 82674598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826745A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826745A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826745A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826745AC: 38EB3838  addi r7, r11, 0x3838
	ctx.r[7].s64 = ctx.r[11].s64 + 14392;
	// 826745B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826745B4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826745B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826745BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826745C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826745C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826745C8: 386A35A0  addi r3, r10, 0x35a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13728;
	// 826745CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826745D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826745D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826745D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826745DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826745E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826745E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826745E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826745EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826745F0: 4BDF2831  bl 0x82466e20
	ctx.lr = 0x826745F4;
	sub_82466E20(ctx, base);
	// 826745F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826745F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826745FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674608 size=108
    let mut pc: u32 = 0x82674608;
    'dispatch: loop {
        match pc {
            0x82674608 => {
    //   block [0x82674608..0x82674674)
	// 82674608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674614: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267461C: 38EB3868  addi r7, r11, 0x3868
	ctx.r[7].s64 = ctx.r[11].s64 + 14440;
	// 82674620: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82674624: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82674628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267462C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674638: 386A35D0  addi r3, r10, 0x35d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13776;
	// 8267463C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267464C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267465C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674660: 4BDF27C1  bl 0x82466e20
	ctx.lr = 0x82674664;
	sub_82466E20(ctx, base);
	// 82674664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267466C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674678 size=108
    let mut pc: u32 = 0x82674678;
    'dispatch: loop {
        match pc {
            0x82674678 => {
    //   block [0x82674678..0x826746E4)
	// 82674678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674684: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267468C: 38EB3970  addi r7, r11, 0x3970
	ctx.r[7].s64 = ctx.r[11].s64 + 14704;
	// 82674690: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674694: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82674698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267469C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826746A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826746A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826746A8: 386A3600  addi r3, r10, 0x3600
	ctx.r[3].s64 = ctx.r[10].s64 + 13824;
	// 826746AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826746B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826746B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826746B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826746BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826746C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826746C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826746C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826746CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826746D0: 4BDF2751  bl 0x82466e20
	ctx.lr = 0x826746D4;
	sub_82466E20(ctx, base);
	// 826746D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826746D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826746DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826746E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826746E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826746E8 size=108
    let mut pc: u32 = 0x826746E8;
    'dispatch: loop {
        match pc {
            0x826746E8 => {
    //   block [0x826746E8..0x82674754)
	// 826746E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826746EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826746F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826746F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826746F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826746FC: 38EB39D0  addi r7, r11, 0x39d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14800;
	// 82674700: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82674704: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82674708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267470C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674718: 386A3630  addi r3, r10, 0x3630
	ctx.r[3].s64 = ctx.r[10].s64 + 13872;
	// 8267471C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267472C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267473C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674740: 4BDF26E1  bl 0x82466e20
	ctx.lr = 0x82674744;
	sub_82466E20(ctx, base);
	// 82674744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267474C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674758 size=108
    let mut pc: u32 = 0x82674758;
    'dispatch: loop {
        match pc {
            0x82674758 => {
    //   block [0x82674758..0x826747C4)
	// 82674758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267475C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674764: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267476C: 38EB3AC0  addi r7, r11, 0x3ac0
	ctx.r[7].s64 = ctx.r[11].s64 + 15040;
	// 82674770: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82674774: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82674778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267477C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674788: 386A3660  addi r3, r10, 0x3660
	ctx.r[3].s64 = ctx.r[10].s64 + 13920;
	// 8267478C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267479C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826747A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826747A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826747A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826747AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826747B0: 4BDF2671  bl 0x82466e20
	ctx.lr = 0x826747B4;
	sub_82466E20(ctx, base);
	// 826747B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826747B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826747BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826747C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826747C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826747C8 size=108
    let mut pc: u32 = 0x826747C8;
    'dispatch: loop {
        match pc {
            0x826747C8 => {
    //   block [0x826747C8..0x82674834)
	// 826747C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826747CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826747D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826747D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826747D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826747DC: 38EB3B98  addi r7, r11, 0x3b98
	ctx.r[7].s64 = ctx.r[11].s64 + 15256;
	// 826747E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826747E4: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826747E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826747EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826747F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826747F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826747F8: 386A3690  addi r3, r10, 0x3690
	ctx.r[3].s64 = ctx.r[10].s64 + 13968;
	// 826747FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267480C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267481C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674820: 4BDF2601  bl 0x82466e20
	ctx.lr = 0x82674824;
	sub_82466E20(ctx, base);
	// 82674824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674838 size=108
    let mut pc: u32 = 0x82674838;
    'dispatch: loop {
        match pc {
            0x82674838 => {
    //   block [0x82674838..0x826748A4)
	// 82674838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674844: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267484C: 38EB3BC8  addi r7, r11, 0x3bc8
	ctx.r[7].s64 = ctx.r[11].s64 + 15304;
	// 82674850: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674854: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82674858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267485C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674868: 386A36C0  addi r3, r10, 0x36c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14016;
	// 8267486C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267487C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267488C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674890: 4BDF2591  bl 0x82466e20
	ctx.lr = 0x82674894;
	sub_82466E20(ctx, base);
	// 82674894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267489C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826748A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826748A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826748A8 size=108
    let mut pc: u32 = 0x826748A8;
    'dispatch: loop {
        match pc {
            0x826748A8 => {
    //   block [0x826748A8..0x82674914)
	// 826748A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826748AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826748B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826748B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826748B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826748BC: 38EB3BE0  addi r7, r11, 0x3be0
	ctx.r[7].s64 = ctx.r[11].s64 + 15328;
	// 826748C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826748C4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826748C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826748CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826748D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826748D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826748D8: 386A36F0  addi r3, r10, 0x36f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14064;
	// 826748DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826748E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826748E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826748E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826748EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826748F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826748F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826748F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826748FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674900: 4BDF2521  bl 0x82466e20
	ctx.lr = 0x82674904;
	sub_82466E20(ctx, base);
	// 82674904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267490C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674918 size=112
    let mut pc: u32 = 0x82674918;
    'dispatch: loop {
        match pc {
            0x82674918 => {
    //   block [0x82674918..0x82674988)
	// 82674918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674924: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674928: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267492C: 38AA36F0  addi r5, r10, 0x36f0
	ctx.r[5].s64 = ctx.r[10].s64 + 14064;
	// 82674930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674934: 390B3C40  addi r8, r11, 0x3c40
	ctx.r[8].s64 = ctx.r[11].s64 + 15424;
	// 82674938: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267493C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82674940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267494C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674950: 386A3720  addi r3, r10, 0x3720
	ctx.r[3].s64 = ctx.r[10].s64 + 14112;
	// 82674954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267495C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267496C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674974: 4BDF24AD  bl 0x82466e20
	ctx.lr = 0x82674978;
	sub_82466E20(ctx, base);
	// 82674978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674988 size=96
    let mut pc: u32 = 0x82674988;
    'dispatch: loop {
        match pc {
            0x82674988 => {
    //   block [0x82674988..0x826749E8)
	// 82674988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674994: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267499C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826749A0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826749A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826749A8: 386A3750  addi r3, r10, 0x3750
	ctx.r[3].s64 = ctx.r[10].s64 + 14160;
	// 826749AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826749B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826749B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826749B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826749BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826749C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826749C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826749C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826749CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826749D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826749D4: 4BDF244D  bl 0x82466e20
	ctx.lr = 0x826749D8;
	sub_82466E20(ctx, base);
	// 826749D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826749DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826749E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826749E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826749E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826749E8 size=112
    let mut pc: u32 = 0x826749E8;
    'dispatch: loop {
        match pc {
            0x826749E8 => {
    //   block [0x826749E8..0x82674A58)
	// 826749E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826749EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826749F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826749F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826749F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826749FC: 38AA4E60  addi r5, r10, 0x4e60
	ctx.r[5].s64 = ctx.r[10].s64 + 20064;
	// 82674A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674A04: 390B3C88  addi r8, r11, 0x3c88
	ctx.r[8].s64 = ctx.r[11].s64 + 15496;
	// 82674A08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82674A0C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82674A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674A20: 386A3780  addi r3, r10, 0x3780
	ctx.r[3].s64 = ctx.r[10].s64 + 14208;
	// 82674A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674A44: 4BDF23DD  bl 0x82466e20
	ctx.lr = 0x82674A48;
	sub_82466E20(ctx, base);
	// 82674A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674A58 size=96
    let mut pc: u32 = 0x82674A58;
    'dispatch: loop {
        match pc {
            0x82674A58 => {
    //   block [0x82674A58..0x82674AB8)
	// 82674A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674A64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674A6C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82674A70: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674A78: 386A37B0  addi r3, r10, 0x37b0
	ctx.r[3].s64 = ctx.r[10].s64 + 14256;
	// 82674A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674A84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674A98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674AA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674AA4: 4BDF237D  bl 0x82466e20
	ctx.lr = 0x82674AA8;
	sub_82466E20(ctx, base);
	// 82674AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674AB8 size=100
    let mut pc: u32 = 0x82674AB8;
    'dispatch: loop {
        match pc {
            0x82674AB8 => {
    //   block [0x82674AB8..0x82674B1C)
	// 82674AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674AC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674ACC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82674AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674AD8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82674ADC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674AE4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674AEC: 386A37E0  addi r3, r10, 0x37e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14304;
	// 82674AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674AF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674B00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674B08: 4BDF2319  bl 0x82466e20
	ctx.lr = 0x82674B0C;
	sub_82466E20(ctx, base);
	// 82674B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674B20 size=96
    let mut pc: u32 = 0x82674B20;
    'dispatch: loop {
        match pc {
            0x82674B20 => {
    //   block [0x82674B20..0x82674B80)
	// 82674B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674B2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674B34: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82674B38: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674B40: 386A3810  addi r3, r10, 0x3810
	ctx.r[3].s64 = ctx.r[10].s64 + 14352;
	// 82674B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674B4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674B6C: 4BDF22B5  bl 0x82466e20
	ctx.lr = 0x82674B70;
	sub_82466E20(ctx, base);
	// 82674B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674B80 size=112
    let mut pc: u32 = 0x82674B80;
    'dispatch: loop {
        match pc {
            0x82674B80 => {
    //   block [0x82674B80..0x82674BF0)
	// 82674B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674B8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674B90: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674B94: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82674B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674B9C: 390B3CE8  addi r8, r11, 0x3ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 15592;
	// 82674BA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82674BA4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82674BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674BAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674BB8: 386A3840  addi r3, r10, 0x3840
	ctx.r[3].s64 = ctx.r[10].s64 + 14400;
	// 82674BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674BDC: 4BDF2245  bl 0x82466e20
	ctx.lr = 0x82674BE0;
	sub_82466E20(ctx, base);
	// 82674BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674BF0 size=112
    let mut pc: u32 = 0x82674BF0;
    'dispatch: loop {
        match pc {
            0x82674BF0 => {
    //   block [0x82674BF0..0x82674C60)
	// 82674BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674BFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674C04: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82674C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674C0C: 390B3D18  addi r8, r11, 0x3d18
	ctx.r[8].s64 = ctx.r[11].s64 + 15640;
	// 82674C10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674C14: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82674C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674C1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674C28: 386A3870  addi r3, r10, 0x3870
	ctx.r[3].s64 = ctx.r[10].s64 + 14448;
	// 82674C2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674C30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674C4C: 4BDF21D5  bl 0x82466e20
	ctx.lr = 0x82674C50;
	sub_82466E20(ctx, base);
	// 82674C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674C60 size=100
    let mut pc: u32 = 0x82674C60;
    'dispatch: loop {
        match pc {
            0x82674C60 => {
    //   block [0x82674C60..0x82674CC4)
	// 82674C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674C6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674C74: 38AA37E0  addi r5, r10, 0x37e0
	ctx.r[5].s64 = ctx.r[10].s64 + 14304;
	// 82674C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674C80: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82674C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674C94: 386A38A0  addi r3, r10, 0x38a0
	ctx.r[3].s64 = ctx.r[10].s64 + 14496;
	// 82674C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674CA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674CA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674CB0: 4BDF2171  bl 0x82466e20
	ctx.lr = 0x82674CB4;
	sub_82466E20(ctx, base);
	// 82674CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674CC8 size=96
    let mut pc: u32 = 0x82674CC8;
    'dispatch: loop {
        match pc {
            0x82674CC8 => {
    //   block [0x82674CC8..0x82674D28)
	// 82674CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674CD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674CDC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82674CE0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674CE8: 386A38D0  addi r3, r10, 0x38d0
	ctx.r[3].s64 = ctx.r[10].s64 + 14544;
	// 82674CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674CF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82674CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674D08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82674D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674D10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82674D14: 4BDF210D  bl 0x82466e20
	ctx.lr = 0x82674D18;
	sub_82466E20(ctx, base);
	// 82674D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674D28 size=112
    let mut pc: u32 = 0x82674D28;
    'dispatch: loop {
        match pc {
            0x82674D28 => {
    //   block [0x82674D28..0x82674D98)
	// 82674D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674D38: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674D3C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82674D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674D44: 390B3D30  addi r8, r11, 0x3d30
	ctx.r[8].s64 = ctx.r[11].s64 + 15664;
	// 82674D48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674D4C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82674D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674D60: 386A3900  addi r3, r10, 0x3900
	ctx.r[3].s64 = ctx.r[10].s64 + 14592;
	// 82674D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674D84: 4BDF209D  bl 0x82466e20
	ctx.lr = 0x82674D88;
	sub_82466E20(ctx, base);
	// 82674D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674D98 size=108
    let mut pc: u32 = 0x82674D98;
    'dispatch: loop {
        match pc {
            0x82674D98 => {
    //   block [0x82674D98..0x82674E04)
	// 82674D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674DA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674DAC: 38EB3D48  addi r7, r11, 0x3d48
	ctx.r[7].s64 = ctx.r[11].s64 + 15688;
	// 82674DB0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82674DB4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82674DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674DBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674DC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82674DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674DC8: 386A3930  addi r3, r10, 0x3930
	ctx.r[3].s64 = ctx.r[10].s64 + 14640;
	// 82674DCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82674DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82674DF0: 4BDF2031  bl 0x82466e20
	ctx.lr = 0x82674DF4;
	sub_82466E20(ctx, base);
	// 82674DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674E08 size=112
    let mut pc: u32 = 0x82674E08;
    'dispatch: loop {
        match pc {
            0x82674E08 => {
    //   block [0x82674E08..0x82674E78)
	// 82674E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674E18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674E1C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82674E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674E24: 390B3DA8  addi r8, r11, 0x3da8
	ctx.r[8].s64 = ctx.r[11].s64 + 15784;
	// 82674E28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674E2C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82674E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674E34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674E40: 386A3960  addi r3, r10, 0x3960
	ctx.r[3].s64 = ctx.r[10].s64 + 14688;
	// 82674E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674E64: 4BDF1FBD  bl 0x82466e20
	ctx.lr = 0x82674E68;
	sub_82466E20(ctx, base);
	// 82674E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674E78 size=112
    let mut pc: u32 = 0x82674E78;
    'dispatch: loop {
        match pc {
            0x82674E78 => {
    //   block [0x82674E78..0x82674EE8)
	// 82674E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674E88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674E8C: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82674E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674E94: 390B3DC0  addi r8, r11, 0x3dc0
	ctx.r[8].s64 = ctx.r[11].s64 + 15808;
	// 82674E98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82674E9C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82674EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674EB0: 386A3990  addi r3, r10, 0x3990
	ctx.r[3].s64 = ctx.r[10].s64 + 14736;
	// 82674EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674ED4: 4BDF1F4D  bl 0x82466e20
	ctx.lr = 0x82674ED8;
	sub_82466E20(ctx, base);
	// 82674ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674EE8 size=112
    let mut pc: u32 = 0x82674EE8;
    'dispatch: loop {
        match pc {
            0x82674EE8 => {
    //   block [0x82674EE8..0x82674F58)
	// 82674EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674EF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674EFC: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82674F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674F04: 390B3DF0  addi r8, r11, 0x3df0
	ctx.r[8].s64 = ctx.r[11].s64 + 15856;
	// 82674F08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674F0C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82674F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674F14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674F20: 386A39C0  addi r3, r10, 0x39c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14784;
	// 82674F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674F44: 4BDF1EDD  bl 0x82466e20
	ctx.lr = 0x82674F48;
	sub_82466E20(ctx, base);
	// 82674F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674F58 size=112
    let mut pc: u32 = 0x82674F58;
    'dispatch: loop {
        match pc {
            0x82674F58 => {
    //   block [0x82674F58..0x82674FC8)
	// 82674F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674F64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674F68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674F6C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82674F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674F74: 390B3E08  addi r8, r11, 0x3e08
	ctx.r[8].s64 = ctx.r[11].s64 + 15880;
	// 82674F78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82674F7C: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82674F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82674F90: 386A39F0  addi r3, r10, 0x39f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14832;
	// 82674F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82674F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82674F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82674FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82674FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82674FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82674FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82674FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82674FB4: 4BDF1E6D  bl 0x82466e20
	ctx.lr = 0x82674FB8;
	sub_82466E20(ctx, base);
	// 82674FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82674FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82674FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82674FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82674FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82674FC8 size=112
    let mut pc: u32 = 0x82674FC8;
    'dispatch: loop {
        match pc {
            0x82674FC8 => {
    //   block [0x82674FC8..0x82675038)
	// 82674FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82674FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82674FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82674FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674FD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82674FDC: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82674FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82674FE4: 390B3E38  addi r8, r11, 0x3e38
	ctx.r[8].s64 = ctx.r[11].s64 + 15928;
	// 82674FE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82674FEC: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82674FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82674FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82674FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82674FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675000: 386A3A20  addi r3, r10, 0x3a20
	ctx.r[3].s64 = ctx.r[10].s64 + 14880;
	// 82675004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267500C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267501C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675024: 4BDF1DFD  bl 0x82466e20
	ctx.lr = 0x82675028;
	sub_82466E20(ctx, base);
	// 82675028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267502C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675038 size=112
    let mut pc: u32 = 0x82675038;
    'dispatch: loop {
        match pc {
            0x82675038 => {
    //   block [0x82675038..0x826750A8)
	// 82675038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267503C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675048: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267504C: 38AA3F00  addi r5, r10, 0x3f00
	ctx.r[5].s64 = ctx.r[10].s64 + 16128;
	// 82675050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675054: 390B3E50  addi r8, r11, 0x3e50
	ctx.r[8].s64 = ctx.r[11].s64 + 15952;
	// 82675058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267505C: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82675060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267506C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675070: 386A3A50  addi r3, r10, 0x3a50
	ctx.r[3].s64 = ctx.r[10].s64 + 14928;
	// 82675074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267507C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267508C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675094: 4BDF1D8D  bl 0x82466e20
	ctx.lr = 0x82675098;
	sub_82466E20(ctx, base);
	// 82675098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267509C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826750A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826750A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826750A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826750A8 size=112
    let mut pc: u32 = 0x826750A8;
    'dispatch: loop {
        match pc {
            0x826750A8 => {
    //   block [0x826750A8..0x82675118)
	// 826750A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826750AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826750B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826750B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826750B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826750BC: 38AA3C60  addi r5, r10, 0x3c60
	ctx.r[5].s64 = ctx.r[10].s64 + 15456;
	// 826750C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826750C4: 390B3E68  addi r8, r11, 0x3e68
	ctx.r[8].s64 = ctx.r[11].s64 + 15976;
	// 826750C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826750CC: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826750D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826750D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826750D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826750DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826750E0: 386A3A80  addi r3, r10, 0x3a80
	ctx.r[3].s64 = ctx.r[10].s64 + 14976;
	// 826750E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826750E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826750EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826750F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826750F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826750F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826750FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675104: 4BDF1D1D  bl 0x82466e20
	ctx.lr = 0x82675108;
	sub_82466E20(ctx, base);
	// 82675108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267510C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675118 size=112
    let mut pc: u32 = 0x82675118;
    'dispatch: loop {
        match pc {
            0x82675118 => {
    //   block [0x82675118..0x82675188)
	// 82675118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267511C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675128: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267512C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675134: 390B3E80  addi r8, r11, 0x3e80
	ctx.r[8].s64 = ctx.r[11].s64 + 16000;
	// 82675138: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267513C: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82675140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675144: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267514C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675150: 386A3AB0  addi r3, r10, 0x3ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 15024;
	// 82675154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267515C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267516C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675174: 4BDF1CAD  bl 0x82466e20
	ctx.lr = 0x82675178;
	sub_82466E20(ctx, base);
	// 82675178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267517C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675188 size=112
    let mut pc: u32 = 0x82675188;
    'dispatch: loop {
        match pc {
            0x82675188 => {
    //   block [0x82675188..0x826751F8)
	// 82675188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267518C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675198: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267519C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 826751A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826751A4: 390B3EC8  addi r8, r11, 0x3ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 16072;
	// 826751A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826751AC: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826751B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826751B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826751B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826751BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826751C0: 386A3AE0  addi r3, r10, 0x3ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 15072;
	// 826751C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826751C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826751CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826751D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826751D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826751D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826751DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826751E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826751E4: 4BDF1C3D  bl 0x82466e20
	ctx.lr = 0x826751E8;
	sub_82466E20(ctx, base);
	// 826751E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826751EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826751F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826751F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826751F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826751F8 size=112
    let mut pc: u32 = 0x826751F8;
    'dispatch: loop {
        match pc {
            0x826751F8 => {
    //   block [0x826751F8..0x82675268)
	// 826751F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826751FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675208: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267520C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675214: 390B3EF8  addi r8, r11, 0x3ef8
	ctx.r[8].s64 = ctx.r[11].s64 + 16120;
	// 82675218: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267521C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82675220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675224: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267522C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675230: 386A3B10  addi r3, r10, 0x3b10
	ctx.r[3].s64 = ctx.r[10].s64 + 15120;
	// 82675234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267523C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267524C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675254: 4BDF1BCD  bl 0x82466e20
	ctx.lr = 0x82675258;
	sub_82466E20(ctx, base);
	// 82675258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267525C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675268 size=108
    let mut pc: u32 = 0x82675268;
    'dispatch: loop {
        match pc {
            0x82675268 => {
    //   block [0x82675268..0x826752D4)
	// 82675268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267526C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675274: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267527C: 38EB3F28  addi r7, r11, 0x3f28
	ctx.r[7].s64 = ctx.r[11].s64 + 16168;
	// 82675280: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82675284: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82675288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267528C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675298: 386A3B40  addi r3, r10, 0x3b40
	ctx.r[3].s64 = ctx.r[10].s64 + 15168;
	// 8267529C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826752A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826752A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826752A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826752AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826752B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826752B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826752B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826752BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826752C0: 4BDF1B61  bl 0x82466e20
	ctx.lr = 0x826752C4;
	sub_82466E20(ctx, base);
	// 826752C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826752C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826752CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826752D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826752D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826752D8 size=112
    let mut pc: u32 = 0x826752D8;
    'dispatch: loop {
        match pc {
            0x826752D8 => {
    //   block [0x826752D8..0x82675348)
	// 826752D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826752DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826752E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826752E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826752E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826752EC: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 826752F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826752F4: 390B3F70  addi r8, r11, 0x3f70
	ctx.r[8].s64 = ctx.r[11].s64 + 16240;
	// 826752F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826752FC: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82675300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267530C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675310: 386A3B70  addi r3, r10, 0x3b70
	ctx.r[3].s64 = ctx.r[10].s64 + 15216;
	// 82675314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267531C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267532C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675334: 4BDF1AED  bl 0x82466e20
	ctx.lr = 0x82675338;
	sub_82466E20(ctx, base);
	// 82675338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675348 size=112
    let mut pc: u32 = 0x82675348;
    'dispatch: loop {
        match pc {
            0x82675348 => {
    //   block [0x82675348..0x826753B8)
	// 82675348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675358: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267535C: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675364: 390B3FE8  addi r8, r11, 0x3fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 16360;
	// 82675368: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267536C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82675370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267537C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675380: 386A3BA0  addi r3, r10, 0x3ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 15264;
	// 82675384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267538C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267539C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826753A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826753A4: 4BDF1A7D  bl 0x82466e20
	ctx.lr = 0x826753A8;
	sub_82466E20(ctx, base);
	// 826753A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826753AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826753B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826753B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826753B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826753B8 size=100
    let mut pc: u32 = 0x826753B8;
    'dispatch: loop {
        match pc {
            0x826753B8 => {
    //   block [0x826753B8..0x8267541C)
	// 826753B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826753BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826753C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826753C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826753C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826753CC: 38AA3CF0  addi r5, r10, 0x3cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 15600;
	// 826753D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826753D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826753D8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826753DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826753E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826753E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826753E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826753EC: 386A3BD0  addi r3, r10, 0x3bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 15312;
	// 826753F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826753F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826753F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826753FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675400: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82675404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675408: 4BDF1A19  bl 0x82466e20
	ctx.lr = 0x8267540C;
	sub_82466E20(ctx, base);
	// 8267540C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675420 size=100
    let mut pc: u32 = 0x82675420;
    'dispatch: loop {
        match pc {
            0x82675420 => {
    //   block [0x82675420..0x82675484)
	// 82675420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267542C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675434: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82675438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267543C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675440: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 82675444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267544C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675454: 386A3C00  addi r3, r10, 0x3c00
	ctx.r[3].s64 = ctx.r[10].s64 + 15360;
	// 82675458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267545C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82675464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267546C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675470: 4BDF19B1  bl 0x82466e20
	ctx.lr = 0x82675474;
	sub_82466E20(ctx, base);
	// 82675474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267547C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675488 size=108
    let mut pc: u32 = 0x82675488;
    'dispatch: loop {
        match pc {
            0x82675488 => {
    //   block [0x82675488..0x826754F4)
	// 82675488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267548C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675494: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267549C: 38EB4060  addi r7, r11, 0x4060
	ctx.r[7].s64 = ctx.r[11].s64 + 16480;
	// 826754A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826754A4: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826754A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826754AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826754B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826754B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826754B8: 386A3C30  addi r3, r10, 0x3c30
	ctx.r[3].s64 = ctx.r[10].s64 + 15408;
	// 826754BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826754C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826754C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826754C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826754CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826754D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826754D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826754D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826754DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826754E0: 4BDF1941  bl 0x82466e20
	ctx.lr = 0x826754E4;
	sub_82466E20(ctx, base);
	// 826754E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826754E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826754EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826754F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826754F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826754F8 size=112
    let mut pc: u32 = 0x826754F8;
    'dispatch: loop {
        match pc {
            0x826754F8 => {
    //   block [0x826754F8..0x82675568)
	// 826754F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826754FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267550C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675514: 390B4090  addi r8, r11, 0x4090
	ctx.r[8].s64 = ctx.r[11].s64 + 16528;
	// 82675518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267551C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82675520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675524: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267552C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675530: 386A3C60  addi r3, r10, 0x3c60
	ctx.r[3].s64 = ctx.r[10].s64 + 15456;
	// 82675534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267553C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267554C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675554: 4BDF18CD  bl 0x82466e20
	ctx.lr = 0x82675558;
	sub_82466E20(ctx, base);
	// 82675558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675568 size=108
    let mut pc: u32 = 0x82675568;
    'dispatch: loop {
        match pc {
            0x82675568 => {
    //   block [0x82675568..0x826755D4)
	// 82675568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675574: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267557C: 38EB40A8  addi r7, r11, 0x40a8
	ctx.r[7].s64 = ctx.r[11].s64 + 16552;
	// 82675580: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82675584: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82675588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267558C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675598: 386A3C90  addi r3, r10, 0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + 15504;
	// 8267559C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826755A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826755A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826755A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826755AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826755B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826755B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826755B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826755BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826755C0: 4BDF1861  bl 0x82466e20
	ctx.lr = 0x826755C4;
	sub_82466E20(ctx, base);
	// 826755C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826755C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826755CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826755D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826755D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826755D8 size=24
    let mut pc: u32 = 0x826755D8;
    'dispatch: loop {
        match pc {
            0x826755D8 => {
    //   block [0x826755D8..0x826755F0)
	// 826755D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826755DC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826755E0: 394A6D50  addi r10, r10, 0x6d50
	ctx.r[10].s64 = ctx.r[10].s64 + 27984;
	// 826755E4: 816B40C0  lwz r11, 0x40c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16576 as u32) ) } as u64;
	// 826755E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826755EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826755F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826755F0 size=108
    let mut pc: u32 = 0x826755F0;
    'dispatch: loop {
        match pc {
            0x826755F0 => {
    //   block [0x826755F0..0x8267565C)
	// 826755F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826755F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826755F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826755FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675604: 38EB6D50  addi r7, r11, 0x6d50
	ctx.r[7].s64 = ctx.r[11].s64 + 27984;
	// 82675608: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267560C: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82675610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267561C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675620: 386A3CC0  addi r3, r10, 0x3cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 15552;
	// 82675624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267562C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267563C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675648: 4BDF17D9  bl 0x82466e20
	ctx.lr = 0x8267564C;
	sub_82466E20(ctx, base);
	// 8267564C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675660 size=116
    let mut pc: u32 = 0x82675660;
    'dispatch: loop {
        match pc {
            0x82675660 => {
    //   block [0x82675660..0x826756D4)
	// 82675660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267566C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675670: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675674: 390B40C8  addi r8, r11, 0x40c8
	ctx.r[8].s64 = ctx.r[11].s64 + 16584;
	// 82675678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267567C: 392A31F8  addi r9, r10, 0x31f8
	ctx.r[9].s64 = ctx.r[10].s64 + 12792;
	// 82675680: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675684: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82675688: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 8267568C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675694: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267569C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826756A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826756A4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826756A8: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826756AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826756B0: 386B3CF0  addi r3, r11, 0x3cf0
	ctx.r[3].s64 = ctx.r[11].s64 + 15600;
	// 826756B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826756B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826756BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826756C0: 4BDF1761  bl 0x82466e20
	ctx.lr = 0x826756C4;
	sub_82466E20(ctx, base);
	// 826756C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826756C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826756CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826756D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826756D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826756D8 size=112
    let mut pc: u32 = 0x826756D8;
    'dispatch: loop {
        match pc {
            0x826756D8 => {
    //   block [0x826756D8..0x82675748)
	// 826756D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826756DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826756E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826756E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826756E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826756EC: 38AA39C0  addi r5, r10, 0x39c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14784;
	// 826756F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826756F4: 390B4128  addi r8, r11, 0x4128
	ctx.r[8].s64 = ctx.r[11].s64 + 16680;
	// 826756F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826756FC: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82675700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675704: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267570C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675710: 386A3D20  addi r3, r10, 0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + 15648;
	// 82675714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267571C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267572C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675734: 4BDF16ED  bl 0x82466e20
	ctx.lr = 0x82675738;
	sub_82466E20(ctx, base);
	// 82675738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267573C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675748 size=108
    let mut pc: u32 = 0x82675748;
    'dispatch: loop {
        match pc {
            0x82675748 => {
    //   block [0x82675748..0x826757B4)
	// 82675748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267574C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675754: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267575C: 38EB4140  addi r7, r11, 0x4140
	ctx.r[7].s64 = ctx.r[11].s64 + 16704;
	// 82675760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82675764: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82675768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267576C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675778: 386A3D50  addi r3, r10, 0x3d50
	ctx.r[3].s64 = ctx.r[10].s64 + 15696;
	// 8267577C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267578C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267579C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826757A0: 4BDF1681  bl 0x82466e20
	ctx.lr = 0x826757A4;
	sub_82466E20(ctx, base);
	// 826757A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826757A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826757AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826757B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826757B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826757B8 size=112
    let mut pc: u32 = 0x826757B8;
    'dispatch: loop {
        match pc {
            0x826757B8 => {
    //   block [0x826757B8..0x82675828)
	// 826757B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826757BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826757C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826757C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826757C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826757CC: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 826757D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826757D4: 390B4170  addi r8, r11, 0x4170
	ctx.r[8].s64 = ctx.r[11].s64 + 16752;
	// 826757D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826757DC: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826757E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826757E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826757E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826757EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826757F0: 386A3D80  addi r3, r10, 0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + 15744;
	// 826757F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826757F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826757FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267580C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675814: 4BDF160D  bl 0x82466e20
	ctx.lr = 0x82675818;
	sub_82466E20(ctx, base);
	// 82675818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267581C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675828 size=116
    let mut pc: u32 = 0x82675828;
    'dispatch: loop {
        match pc {
            0x82675828 => {
    //   block [0x82675828..0x8267589C)
	// 82675828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267582C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675834: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675838: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267583C: 390B41A0  addi r8, r11, 0x41a0
	ctx.r[8].s64 = ctx.r[11].s64 + 16800;
	// 82675840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675844: 392A3214  addi r9, r10, 0x3214
	ctx.r[9].s64 = ctx.r[10].s64 + 12820;
	// 82675848: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267584C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82675850: 38AA3F00  addi r5, r10, 0x3f00
	ctx.r[5].s64 = ctx.r[10].s64 + 16128;
	// 82675854: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267585C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267586C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82675870: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82675874: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82675878: 386B3DB0  addi r3, r11, 0x3db0
	ctx.r[3].s64 = ctx.r[11].s64 + 15792;
	// 8267587C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82675880: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675888: 4BDF1599  bl 0x82466e20
	ctx.lr = 0x8267588C;
	sub_82466E20(ctx, base);
	// 8267588C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826758A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826758A0 size=100
    let mut pc: u32 = 0x826758A0;
    'dispatch: loop {
        match pc {
            0x826758A0 => {
    //   block [0x826758A0..0x82675904)
	// 826758A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826758A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826758A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826758AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826758B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826758B4: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 826758B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826758BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826758C0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826758C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826758C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826758CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826758D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826758D4: 386A3DE0  addi r3, r10, 0x3de0
	ctx.r[3].s64 = ctx.r[10].s64 + 15840;
	// 826758D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826758DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826758E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826758E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826758E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826758EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826758F0: 4BDF1531  bl 0x82466e20
	ctx.lr = 0x826758F4;
	sub_82466E20(ctx, base);
	// 826758F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826758F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826758FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675908 size=112
    let mut pc: u32 = 0x82675908;
    'dispatch: loop {
        match pc {
            0x82675908 => {
    //   block [0x82675908..0x82675978)
	// 82675908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267590C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675914: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675918: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267591C: 38AA3C00  addi r5, r10, 0x3c00
	ctx.r[5].s64 = ctx.r[10].s64 + 15360;
	// 82675920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675924: 390B41D0  addi r8, r11, 0x41d0
	ctx.r[8].s64 = ctx.r[11].s64 + 16848;
	// 82675928: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267592C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82675930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267593C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675940: 386A3E10  addi r3, r10, 0x3e10
	ctx.r[3].s64 = ctx.r[10].s64 + 15888;
	// 82675944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267594C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267595C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675964: 4BDF14BD  bl 0x82466e20
	ctx.lr = 0x82675968;
	sub_82466E20(ctx, base);
	// 82675968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267596C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675978 size=112
    let mut pc: u32 = 0x82675978;
    'dispatch: loop {
        match pc {
            0x82675978 => {
    //   block [0x82675978..0x826759E8)
	// 82675978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267597C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267598C: 38AA3C00  addi r5, r10, 0x3c00
	ctx.r[5].s64 = ctx.r[10].s64 + 15360;
	// 82675990: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675994: 390B4218  addi r8, r11, 0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + 16920;
	// 82675998: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8267599C: 388AAF68  addi r4, r10, -0x5098
	ctx.r[4].s64 = ctx.r[10].s64 + -20632;
	// 826759A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826759A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826759A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826759AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826759B0: 386A3E40  addi r3, r10, 0x3e40
	ctx.r[3].s64 = ctx.r[10].s64 + 15936;
	// 826759B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826759B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826759BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826759C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826759C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826759C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826759CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826759D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826759D4: 4BDF144D  bl 0x82466e20
	ctx.lr = 0x826759D8;
	sub_82466E20(ctx, base);
	// 826759D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826759DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826759E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826759E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826759E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826759E8 size=108
    let mut pc: u32 = 0x826759E8;
    'dispatch: loop {
        match pc {
            0x826759E8 => {
    //   block [0x826759E8..0x82675A54)
	// 826759E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826759EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826759F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826759F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826759F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826759FC: 38EB42D8  addi r7, r11, 0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + 17112;
	// 82675A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82675A04: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82675A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675A0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675A18: 386A3E70  addi r3, r10, 0x3e70
	ctx.r[3].s64 = ctx.r[10].s64 + 15984;
	// 82675A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675A40: 4BDF13E1  bl 0x82466e20
	ctx.lr = 0x82675A44;
	sub_82466E20(ctx, base);
	// 82675A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675A58 size=112
    let mut pc: u32 = 0x82675A58;
    'dispatch: loop {
        match pc {
            0x82675A58 => {
    //   block [0x82675A58..0x82675AC8)
	// 82675A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675A64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675A68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675A6C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675A74: 390B4320  addi r8, r11, 0x4320
	ctx.r[8].s64 = ctx.r[11].s64 + 17184;
	// 82675A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82675A7C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82675A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675A90: 386A3EA0  addi r3, r10, 0x3ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 16032;
	// 82675A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675AB4: 4BDF136D  bl 0x82466e20
	ctx.lr = 0x82675AB8;
	sub_82466E20(ctx, base);
	// 82675AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675AC8 size=100
    let mut pc: u32 = 0x82675AC8;
    'dispatch: loop {
        match pc {
            0x82675AC8 => {
    //   block [0x82675AC8..0x82675B2C)
	// 82675AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675ADC: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675AE8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82675AEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675AFC: 386A3ED0  addi r3, r10, 0x3ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 16080;
	// 82675B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675B04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675B08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82675B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675B10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82675B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675B18: 4BDF1309  bl 0x82466e20
	ctx.lr = 0x82675B1C;
	sub_82466E20(ctx, base);
	// 82675B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675B30 size=100
    let mut pc: u32 = 0x82675B30;
    'dispatch: loop {
        match pc {
            0x82675B30 => {
    //   block [0x82675B30..0x82675B94)
	// 82675B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675B3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675B44: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82675B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675B50: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 82675B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675B64: 386A3F00  addi r3, r10, 0x3f00
	ctx.r[3].s64 = ctx.r[10].s64 + 16128;
	// 82675B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82675B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82675B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675B80: 4BDF12A1  bl 0x82466e20
	ctx.lr = 0x82675B84;
	sub_82466E20(ctx, base);
	// 82675B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675B98 size=108
    let mut pc: u32 = 0x82675B98;
    'dispatch: loop {
        match pc {
            0x82675B98 => {
    //   block [0x82675B98..0x82675C04)
	// 82675B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675BA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675BAC: 38EB4380  addi r7, r11, 0x4380
	ctx.r[7].s64 = ctx.r[11].s64 + 17280;
	// 82675BB0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82675BB4: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82675BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675BBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675BC8: 386A3F30  addi r3, r10, 0x3f30
	ctx.r[3].s64 = ctx.r[10].s64 + 16176;
	// 82675BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675BF0: 4BDF1231  bl 0x82466e20
	ctx.lr = 0x82675BF4;
	sub_82466E20(ctx, base);
	// 82675BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675C08 size=112
    let mut pc: u32 = 0x82675C08;
    'dispatch: loop {
        match pc {
            0x82675C08 => {
    //   block [0x82675C08..0x82675C78)
	// 82675C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675C14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675C18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675C1C: 38AA3CF0  addi r5, r10, 0x3cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 15600;
	// 82675C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675C24: 390B43F8  addi r8, r11, 0x43f8
	ctx.r[8].s64 = ctx.r[11].s64 + 17400;
	// 82675C28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82675C2C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82675C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675C40: 386A3F60  addi r3, r10, 0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + 16224;
	// 82675C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675C64: 4BDF11BD  bl 0x82466e20
	ctx.lr = 0x82675C68;
	sub_82466E20(ctx, base);
	// 82675C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675C78 size=112
    let mut pc: u32 = 0x82675C78;
    'dispatch: loop {
        match pc {
            0x82675C78 => {
    //   block [0x82675C78..0x82675CE8)
	// 82675C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675C88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675C8C: 38AA3900  addi r5, r10, 0x3900
	ctx.r[5].s64 = ctx.r[10].s64 + 14592;
	// 82675C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675C94: 390B4410  addi r8, r11, 0x4410
	ctx.r[8].s64 = ctx.r[11].s64 + 17424;
	// 82675C98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82675C9C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82675CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675CB0: 386A3F90  addi r3, r10, 0x3f90
	ctx.r[3].s64 = ctx.r[10].s64 + 16272;
	// 82675CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675CD4: 4BDF114D  bl 0x82466e20
	ctx.lr = 0x82675CD8;
	sub_82466E20(ctx, base);
	// 82675CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675CE8 size=112
    let mut pc: u32 = 0x82675CE8;
    'dispatch: loop {
        match pc {
            0x82675CE8 => {
    //   block [0x82675CE8..0x82675D58)
	// 82675CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675CF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675CF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675CFC: 38AA3A50  addi r5, r10, 0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + 14928;
	// 82675D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675D04: 390B4458  addi r8, r11, 0x4458
	ctx.r[8].s64 = ctx.r[11].s64 + 17496;
	// 82675D08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82675D0C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82675D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675D20: 386A3FC0  addi r3, r10, 0x3fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 16320;
	// 82675D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675D44: 4BDF10DD  bl 0x82466e20
	ctx.lr = 0x82675D48;
	sub_82466E20(ctx, base);
	// 82675D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675D58 size=108
    let mut pc: u32 = 0x82675D58;
    'dispatch: loop {
        match pc {
            0x82675D58 => {
    //   block [0x82675D58..0x82675DC4)
	// 82675D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675D64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675D68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82675D6C: 38EB44A0  addi r7, r11, 0x44a0
	ctx.r[7].s64 = ctx.r[11].s64 + 17568;
	// 82675D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82675D74: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 82675D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675D7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675D88: 386A3FF0  addi r3, r10, 0x3ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 16368;
	// 82675D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675DB0: 4BDF1071  bl 0x82466e20
	ctx.lr = 0x82675DB4;
	sub_82466E20(ctx, base);
	// 82675DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675DC8 size=112
    let mut pc: u32 = 0x82675DC8;
    'dispatch: loop {
        match pc {
            0x82675DC8 => {
    //   block [0x82675DC8..0x82675E38)
	// 82675DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675DD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675DD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675DDC: 38AA39C0  addi r5, r10, 0x39c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14784;
	// 82675DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675DE4: 390B44E8  addi r8, r11, 0x44e8
	ctx.r[8].s64 = ctx.r[11].s64 + 17640;
	// 82675DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82675DEC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82675DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675DF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675E00: 386A4020  addi r3, r10, 0x4020
	ctx.r[3].s64 = ctx.r[10].s64 + 16416;
	// 82675E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675E24: 4BDF0FFD  bl 0x82466e20
	ctx.lr = 0x82675E28;
	sub_82466E20(ctx, base);
	// 82675E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675E38 size=112
    let mut pc: u32 = 0x82675E38;
    'dispatch: loop {
        match pc {
            0x82675E38 => {
    //   block [0x82675E38..0x82675EA8)
	// 82675E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675E44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675E48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675E4C: 38AA3A20  addi r5, r10, 0x3a20
	ctx.r[5].s64 = ctx.r[10].s64 + 14880;
	// 82675E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82675E54: 390B4500  addi r8, r11, 0x4500
	ctx.r[8].s64 = ctx.r[11].s64 + 17664;
	// 82675E58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82675E5C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 82675E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675E64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675E70: 386A4050  addi r3, r10, 0x4050
	ctx.r[3].s64 = ctx.r[10].s64 + 16464;
	// 82675E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675E94: 4BDF0F8D  bl 0x82466e20
	ctx.lr = 0x82675E98;
	sub_82466E20(ctx, base);
	// 82675E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675EA8 size=112
    let mut pc: u32 = 0x82675EA8;
    'dispatch: loop {
        match pc {
            0x82675EA8 => {
    //   block [0x82675EA8..0x82675F18)
	// 82675EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675EB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675EB8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675EBC: 38AA40B0  addi r5, r10, 0x40b0
	ctx.r[5].s64 = ctx.r[10].s64 + 16560;
	// 82675EC0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675EC4: 390B4530  addi r8, r11, 0x4530
	ctx.r[8].s64 = ctx.r[11].s64 + 17712;
	// 82675EC8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82675ECC: 388AAEE4  addi r4, r10, -0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + -20764;
	// 82675ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675ED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675EE0: 386A4080  addi r3, r10, 0x4080
	ctx.r[3].s64 = ctx.r[10].s64 + 16512;
	// 82675EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82675EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675F04: 4BDF0F1D  bl 0x82466e20
	ctx.lr = 0x82675F08;
	sub_82466E20(ctx, base);
	// 82675F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82675F18 size=24
    let mut pc: u32 = 0x82675F18;
    'dispatch: loop {
        match pc {
            0x82675F18 => {
    //   block [0x82675F18..0x82675F30)
	// 82675F18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675F1C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82675F20: 394A6E70  addi r10, r10, 0x6e70
	ctx.r[10].s64 = ctx.r[10].s64 + 28272;
	// 82675F24: 816B4620  lwz r11, 0x4620(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17952 as u32) ) } as u64;
	// 82675F28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82675F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675F30 size=112
    let mut pc: u32 = 0x82675F30;
    'dispatch: loop {
        match pc {
            0x82675F30 => {
    //   block [0x82675F30..0x82675FA0)
	// 82675F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675F3C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675F40: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675F44: 392A3240  addi r9, r10, 0x3240
	ctx.r[9].s64 = ctx.r[10].s64 + 12864;
	// 82675F48: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675F4C: 390B6E70  addi r8, r11, 0x6e70
	ctx.r[8].s64 = ctx.r[11].s64 + 28272;
	// 82675F50: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82675F54: 388AAEBC  addi r4, r10, -0x5144
	ctx.r[4].s64 = ctx.r[10].s64 + -20804;
	// 82675F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675F5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82675F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675F68: 386A40B0  addi r3, r10, 0x40b0
	ctx.r[3].s64 = ctx.r[10].s64 + 16560;
	// 82675F6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82675F70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82675F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675F8C: 4BDF0E95  bl 0x82466e20
	ctx.lr = 0x82675F90;
	sub_82466E20(ctx, base);
	// 82675F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82675F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82675F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82675F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82675FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82675FA0 size=108
    let mut pc: u32 = 0x82675FA0;
    'dispatch: loop {
        match pc {
            0x82675FA0 => {
    //   block [0x82675FA0..0x8267600C)
	// 82675FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82675FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82675FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82675FAC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82675FB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82675FB4: 38EB4624  addi r7, r11, 0x4624
	ctx.r[7].s64 = ctx.r[11].s64 + 17956;
	// 82675FB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82675FBC: 388AAED4  addi r4, r10, -0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + -20780;
	// 82675FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82675FC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82675FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82675FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82675FD0: 386A40E0  addi r3, r10, 0x40e0
	ctx.r[3].s64 = ctx.r[10].s64 + 16608;
	// 82675FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82675FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82675FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82675FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82675FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82675FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82675FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82675FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82675FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82675FF8: 4BDF0E29  bl 0x82466e20
	ctx.lr = 0x82675FFC;
	sub_82466E20(ctx, base);
	// 82675FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676010 size=108
    let mut pc: u32 = 0x82676010;
    'dispatch: loop {
        match pc {
            0x82676010 => {
    //   block [0x82676010..0x8267607C)
	// 82676010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267601C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676024: 38EB4658  addi r7, r11, 0x4658
	ctx.r[7].s64 = ctx.r[11].s64 + 18008;
	// 82676028: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8267602C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82676030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676034: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267603C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676040: 386A4110  addi r3, r10, 0x4110
	ctx.r[3].s64 = ctx.r[10].s64 + 16656;
	// 82676044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267604C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267605C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676068: 4BDF0DB9  bl 0x82466e20
	ctx.lr = 0x8267606C;
	sub_82466E20(ctx, base);
	// 8267606C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676080 size=108
    let mut pc: u32 = 0x82676080;
    'dispatch: loop {
        match pc {
            0x82676080 => {
    //   block [0x82676080..0x826760EC)
	// 82676080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267608C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676094: 38EB4748  addi r7, r11, 0x4748
	ctx.r[7].s64 = ctx.r[11].s64 + 18248;
	// 82676098: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267609C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826760A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826760A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826760A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826760AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826760B0: 386A4140  addi r3, r10, 0x4140
	ctx.r[3].s64 = ctx.r[10].s64 + 16704;
	// 826760B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826760B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826760BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826760C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826760C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826760C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826760CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826760D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826760D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826760D8: 4BDF0D49  bl 0x82466e20
	ctx.lr = 0x826760DC;
	sub_82466E20(ctx, base);
	// 826760DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826760E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826760E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826760E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826760F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826760F0 size=108
    let mut pc: u32 = 0x826760F0;
    'dispatch: loop {
        match pc {
            0x826760F0 => {
    //   block [0x826760F0..0x8267615C)
	// 826760F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826760F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826760F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826760FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676104: 38EB4790  addi r7, r11, 0x4790
	ctx.r[7].s64 = ctx.r[11].s64 + 18320;
	// 82676108: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267610C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82676110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267611C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676120: 386A4170  addi r3, r10, 0x4170
	ctx.r[3].s64 = ctx.r[10].s64 + 16752;
	// 82676124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267612C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267613C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676148: 4BDF0CD9  bl 0x82466e20
	ctx.lr = 0x8267614C;
	sub_82466E20(ctx, base);
	// 8267614C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676160 size=108
    let mut pc: u32 = 0x82676160;
    'dispatch: loop {
        match pc {
            0x82676160 => {
    //   block [0x82676160..0x826761CC)
	// 82676160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267616C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676174: 38EB4838  addi r7, r11, 0x4838
	ctx.r[7].s64 = ctx.r[11].s64 + 18488;
	// 82676178: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267617C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82676180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267618C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676190: 386A41A0  addi r3, r10, 0x41a0
	ctx.r[3].s64 = ctx.r[10].s64 + 16800;
	// 82676194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267619C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826761A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826761A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826761A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826761AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826761B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826761B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826761B8: 4BDF0C69  bl 0x82466e20
	ctx.lr = 0x826761BC;
	sub_82466E20(ctx, base);
	// 826761BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826761C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826761C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826761C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826761D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826761D0 size=112
    let mut pc: u32 = 0x826761D0;
    'dispatch: loop {
        match pc {
            0x826761D0 => {
    //   block [0x826761D0..0x82676240)
	// 826761D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826761D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826761D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826761DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826761E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826761E4: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 826761E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826761EC: 390B4850  addi r8, r11, 0x4850
	ctx.r[8].s64 = ctx.r[11].s64 + 18512;
	// 826761F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826761F4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826761F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826761FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676208: 386A41D0  addi r3, r10, 0x41d0
	ctx.r[3].s64 = ctx.r[10].s64 + 16848;
	// 8267620C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267621C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267622C: 4BDF0BF5  bl 0x82466e20
	ctx.lr = 0x82676230;
	sub_82466E20(ctx, base);
	// 82676230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676240 size=112
    let mut pc: u32 = 0x82676240;
    'dispatch: loop {
        match pc {
            0x82676240 => {
    //   block [0x82676240..0x826762B0)
	// 82676240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267624C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676250: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676254: 38AA41D0  addi r5, r10, 0x41d0
	ctx.r[5].s64 = ctx.r[10].s64 + 16848;
	// 82676258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267625C: 390B48B0  addi r8, r11, 0x48b0
	ctx.r[8].s64 = ctx.r[11].s64 + 18608;
	// 82676260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82676264: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82676268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267626C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676278: 386A4200  addi r3, r10, 0x4200
	ctx.r[3].s64 = ctx.r[10].s64 + 16896;
	// 8267627C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267628C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267629C: 4BDF0B85  bl 0x82466e20
	ctx.lr = 0x826762A0;
	sub_82466E20(ctx, base);
	// 826762A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826762A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826762A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826762AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826762B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826762B0 size=112
    let mut pc: u32 = 0x826762B0;
    'dispatch: loop {
        match pc {
            0x826762B0 => {
    //   block [0x826762B0..0x82676320)
	// 826762B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826762B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826762B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826762BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826762C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826762C4: 38AA41D0  addi r5, r10, 0x41d0
	ctx.r[5].s64 = ctx.r[10].s64 + 16848;
	// 826762C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826762CC: 390B48C8  addi r8, r11, 0x48c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18632;
	// 826762D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826762D4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826762D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826762DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826762E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826762E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826762E8: 386A4230  addi r3, r10, 0x4230
	ctx.r[3].s64 = ctx.r[10].s64 + 16944;
	// 826762EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826762F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826762F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826762F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826762FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267630C: 4BDF0B15  bl 0x82466e20
	ctx.lr = 0x82676310;
	sub_82466E20(ctx, base);
	// 82676310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676320 size=112
    let mut pc: u32 = 0x82676320;
    'dispatch: loop {
        match pc {
            0x82676320 => {
    //   block [0x82676320..0x82676390)
	// 82676320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267632C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676330: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676334: 38AA41D0  addi r5, r10, 0x41d0
	ctx.r[5].s64 = ctx.r[10].s64 + 16848;
	// 82676338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267633C: 390B48F8  addi r8, r11, 0x48f8
	ctx.r[8].s64 = ctx.r[11].s64 + 18680;
	// 82676340: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82676344: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82676348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267634C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676358: 386A4260  addi r3, r10, 0x4260
	ctx.r[3].s64 = ctx.r[10].s64 + 16992;
	// 8267635C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267636C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267637C: 4BDF0AA5  bl 0x82466e20
	ctx.lr = 0x82676380;
	sub_82466E20(ctx, base);
	// 82676380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267638C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82676390 size=24
    let mut pc: u32 = 0x82676390;
    'dispatch: loop {
        match pc {
            0x82676390 => {
    //   block [0x82676390..0x826763A8)
	// 82676390: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676394: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82676398: 394A6E88  addi r10, r10, 0x6e88
	ctx.r[10].s64 = ctx.r[10].s64 + 28296;
	// 8267639C: 816B4654  lwz r11, 0x4654(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18004 as u32) ) } as u64;
	// 826763A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826763A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826763A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826763A8 size=112
    let mut pc: u32 = 0x826763A8;
    'dispatch: loop {
        match pc {
            0x826763A8 => {
    //   block [0x826763A8..0x82676418)
	// 826763A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826763AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826763B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826763B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826763B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826763BC: 392A327C  addi r9, r10, 0x327c
	ctx.r[9].s64 = ctx.r[10].s64 + 12924;
	// 826763C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826763C4: 390B6E88  addi r8, r11, 0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + 28296;
	// 826763C8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826763CC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826763D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826763D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826763D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826763DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826763E0: 386A4290  addi r3, r10, 0x4290
	ctx.r[3].s64 = ctx.r[10].s64 + 17040;
	// 826763E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826763E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826763EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826763F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826763F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826763F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826763FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676404: 4BDF0A1D  bl 0x82466e20
	ctx.lr = 0x82676408;
	sub_82466E20(ctx, base);
	// 82676408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267640C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676418 size=108
    let mut pc: u32 = 0x82676418;
    'dispatch: loop {
        match pc {
            0x82676418 => {
    //   block [0x82676418..0x82676484)
	// 82676418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676424: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267642C: 38EB4910  addi r7, r11, 0x4910
	ctx.r[7].s64 = ctx.r[11].s64 + 18704;
	// 82676430: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82676434: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82676438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267643C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676448: 386A42C0  addi r3, r10, 0x42c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17088;
	// 8267644C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267645C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267646C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676470: 4BDF09B1  bl 0x82466e20
	ctx.lr = 0x82676474;
	sub_82466E20(ctx, base);
	// 82676474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267647C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676488 size=108
    let mut pc: u32 = 0x82676488;
    'dispatch: loop {
        match pc {
            0x82676488 => {
    //   block [0x82676488..0x826764F4)
	// 82676488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676494: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267649C: 38EB4928  addi r7, r11, 0x4928
	ctx.r[7].s64 = ctx.r[11].s64 + 18728;
	// 826764A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826764A4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826764A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826764AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826764B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826764B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826764B8: 386A42F0  addi r3, r10, 0x42f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17136;
	// 826764BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826764C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826764C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826764C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826764CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826764D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826764D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826764D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826764DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826764E0: 4BDF0941  bl 0x82466e20
	ctx.lr = 0x826764E4;
	sub_82466E20(ctx, base);
	// 826764E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826764E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826764EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826764F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826764F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826764F8 size=112
    let mut pc: u32 = 0x826764F8;
    'dispatch: loop {
        match pc {
            0x826764F8 => {
    //   block [0x826764F8..0x82676568)
	// 826764F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826764FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676504: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267650C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82676510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676514: 390B495C  addi r8, r11, 0x495c
	ctx.r[8].s64 = ctx.r[11].s64 + 18780;
	// 82676518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267651C: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82676520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676524: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267652C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676530: 386A4320  addi r3, r10, 0x4320
	ctx.r[3].s64 = ctx.r[10].s64 + 17184;
	// 82676534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267653C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267654C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676554: 4BDF08CD  bl 0x82466e20
	ctx.lr = 0x82676558;
	sub_82466E20(ctx, base);
	// 82676558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267655C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676568 size=108
    let mut pc: u32 = 0x82676568;
    'dispatch: loop {
        match pc {
            0x82676568 => {
    //   block [0x82676568..0x826765D4)
	// 82676568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267656C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676574: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676578: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267657C: 38EB4978  addi r7, r11, 0x4978
	ctx.r[7].s64 = ctx.r[11].s64 + 18808;
	// 82676580: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82676584: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82676588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267658C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676598: 386A4350  addi r3, r10, 0x4350
	ctx.r[3].s64 = ctx.r[10].s64 + 17232;
	// 8267659C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826765A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826765A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826765A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826765AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826765B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826765B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826765B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826765BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826765C0: 4BDF0861  bl 0x82466e20
	ctx.lr = 0x826765C4;
	sub_82466E20(ctx, base);
	// 826765C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826765C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826765CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826765D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826765D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826765D8 size=24
    let mut pc: u32 = 0x826765D8;
    'dispatch: loop {
        match pc {
            0x826765D8 => {
    //   block [0x826765D8..0x826765F0)
	// 826765D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826765DC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826765E0: 394A6ED0  addi r10, r10, 0x6ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 28368;
	// 826765E4: 816B4974  lwz r11, 0x4974(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18804 as u32) ) } as u64;
	// 826765E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826765EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826765F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826765F0 size=116
    let mut pc: u32 = 0x826765F0;
    'dispatch: loop {
        match pc {
            0x826765F0 => {
    //   block [0x826765F0..0x82676664)
	// 826765F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826765F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826765F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826765FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676600: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676604: 390B6ED0  addi r8, r11, 0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 28368;
	// 82676608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267660C: 392A333C  addi r9, r10, 0x333c
	ctx.r[9].s64 = ctx.r[10].s64 + 13116;
	// 82676610: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676614: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82676618: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 8267661C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676634: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82676638: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8267663C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82676640: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 82676644: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82676648: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267664C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676650: 4BDF07D1  bl 0x82466e20
	ctx.lr = 0x82676654;
	sub_82466E20(ctx, base);
	// 82676654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267665C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676668 size=112
    let mut pc: u32 = 0x82676668;
    'dispatch: loop {
        match pc {
            0x82676668 => {
    //   block [0x82676668..0x826766D8)
	// 82676668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267666C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676678: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267667C: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676684: 390B49D8  addi r8, r11, 0x49d8
	ctx.r[8].s64 = ctx.r[11].s64 + 18904;
	// 82676688: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267668C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82676690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676694: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267669C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826766A0: 386A43B0  addi r3, r10, 0x43b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17328;
	// 826766A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826766A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826766AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826766B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826766B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826766B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826766BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826766C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826766C4: 4BDF075D  bl 0x82466e20
	ctx.lr = 0x826766C8;
	sub_82466E20(ctx, base);
	// 826766C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826766CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826766D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826766D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826766D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826766D8 size=108
    let mut pc: u32 = 0x826766D8;
    'dispatch: loop {
        match pc {
            0x826766D8 => {
    //   block [0x826766D8..0x82676744)
	// 826766D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826766DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826766E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826766E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826766E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826766EC: 38EB4A08  addi r7, r11, 0x4a08
	ctx.r[7].s64 = ctx.r[11].s64 + 18952;
	// 826766F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826766F4: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826766F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826766FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676708: 386A43E0  addi r3, r10, 0x43e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17376;
	// 8267670C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267671C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267672C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676730: 4BDF06F1  bl 0x82466e20
	ctx.lr = 0x82676734;
	sub_82466E20(ctx, base);
	// 82676734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267673C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676748 size=108
    let mut pc: u32 = 0x82676748;
    'dispatch: loop {
        match pc {
            0x82676748 => {
    //   block [0x82676748..0x826767B4)
	// 82676748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267674C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676754: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676758: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267675C: 38EB4A50  addi r7, r11, 0x4a50
	ctx.r[7].s64 = ctx.r[11].s64 + 19024;
	// 82676760: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82676764: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82676768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267676C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676778: 386A4410  addi r3, r10, 0x4410
	ctx.r[3].s64 = ctx.r[10].s64 + 17424;
	// 8267677C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267678C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267679C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826767A0: 4BDF0681  bl 0x82466e20
	ctx.lr = 0x826767A4;
	sub_82466E20(ctx, base);
	// 826767A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826767A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826767AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826767B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826767B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826767B8 size=112
    let mut pc: u32 = 0x826767B8;
    'dispatch: loop {
        match pc {
            0x826767B8 => {
    //   block [0x826767B8..0x82676828)
	// 826767B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826767BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826767C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826767C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826767C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826767CC: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 826767D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826767D4: 390B4A80  addi r8, r11, 0x4a80
	ctx.r[8].s64 = ctx.r[11].s64 + 19072;
	// 826767D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826767DC: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826767E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826767E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826767E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826767EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826767F0: 386A4440  addi r3, r10, 0x4440
	ctx.r[3].s64 = ctx.r[10].s64 + 17472;
	// 826767F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826767F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826767FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267680C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676814: 4BDF060D  bl 0x82466e20
	ctx.lr = 0x82676818;
	sub_82466E20(ctx, base);
	// 82676818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267681C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676828 size=108
    let mut pc: u32 = 0x82676828;
    'dispatch: loop {
        match pc {
            0x82676828 => {
    //   block [0x82676828..0x82676894)
	// 82676828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267682C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676834: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676838: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267683C: 38EB4AB0  addi r7, r11, 0x4ab0
	ctx.r[7].s64 = ctx.r[11].s64 + 19120;
	// 82676840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82676844: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 82676848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267684C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676858: 386A4470  addi r3, r10, 0x4470
	ctx.r[3].s64 = ctx.r[10].s64 + 17520;
	// 8267685C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267686C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267687C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676880: 4BDF05A1  bl 0x82466e20
	ctx.lr = 0x82676884;
	sub_82466E20(ctx, base);
	// 82676884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676898 size=108
    let mut pc: u32 = 0x82676898;
    'dispatch: loop {
        match pc {
            0x82676898 => {
    //   block [0x82676898..0x82676904)
	// 82676898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267689C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826768A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826768A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826768A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826768AC: 38EB4B10  addi r7, r11, 0x4b10
	ctx.r[7].s64 = ctx.r[11].s64 + 19216;
	// 826768B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826768B4: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 826768B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826768BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826768C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826768C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826768C8: 386A44A0  addi r3, r10, 0x44a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17568;
	// 826768CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826768D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826768D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826768D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826768DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826768E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826768E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826768E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826768EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826768F0: 4BDF0531  bl 0x82466e20
	ctx.lr = 0x826768F4;
	sub_82466E20(ctx, base);
	// 826768F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826768F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826768FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676908 size=112
    let mut pc: u32 = 0x82676908;
    'dispatch: loop {
        match pc {
            0x82676908 => {
    //   block [0x82676908..0x82676978)
	// 82676908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267690C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676914: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676918: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267691C: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676924: 390B4B58  addi r8, r11, 0x4b58
	ctx.r[8].s64 = ctx.r[11].s64 + 19288;
	// 82676928: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267692C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82676930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267693C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676940: 386A44D0  addi r3, r10, 0x44d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17616;
	// 82676944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267694C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267695C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676964: 4BDF04BD  bl 0x82466e20
	ctx.lr = 0x82676968;
	sub_82466E20(ctx, base);
	// 82676968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267696C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676978 size=112
    let mut pc: u32 = 0x82676978;
    'dispatch: loop {
        match pc {
            0x82676978 => {
    //   block [0x82676978..0x826769E8)
	// 82676978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267697C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676984: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676988: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267698C: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676990: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676994: 390B4BD0  addi r8, r11, 0x4bd0
	ctx.r[8].s64 = ctx.r[11].s64 + 19408;
	// 82676998: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267699C: 388AAFD0  addi r4, r10, -0x5030
	ctx.r[4].s64 = ctx.r[10].s64 + -20528;
	// 826769A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826769A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826769A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826769AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826769B0: 386A4500  addi r3, r10, 0x4500
	ctx.r[3].s64 = ctx.r[10].s64 + 17664;
	// 826769B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826769B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826769BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826769C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826769C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826769C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826769CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826769D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826769D4: 4BDF044D  bl 0x82466e20
	ctx.lr = 0x826769D8;
	sub_82466E20(ctx, base);
	// 826769D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826769DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826769E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826769E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826769E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826769E8 size=112
    let mut pc: u32 = 0x826769E8;
    'dispatch: loop {
        match pc {
            0x826769E8 => {
    //   block [0x826769E8..0x82676A58)
	// 826769E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826769EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826769F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826769F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826769F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826769FC: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82676A00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676A04: 390B4C30  addi r8, r11, 0x4c30
	ctx.r[8].s64 = ctx.r[11].s64 + 19504;
	// 82676A08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82676A0C: 388AAF84  addi r4, r10, -0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + -20604;
	// 82676A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676A20: 386A4530  addi r3, r10, 0x4530
	ctx.r[3].s64 = ctx.r[10].s64 + 17712;
	// 82676A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676A44: 4BDF03DD  bl 0x82466e20
	ctx.lr = 0x82676A48;
	sub_82466E20(ctx, base);
	// 82676A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676A58 size=112
    let mut pc: u32 = 0x82676A58;
    'dispatch: loop {
        match pc {
            0x82676A58 => {
    //   block [0x82676A58..0x82676AC8)
	// 82676A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676A64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676A68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676A6C: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676A70: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676A74: 390B4C48  addi r8, r11, 0x4c48
	ctx.r[8].s64 = ctx.r[11].s64 + 19528;
	// 82676A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82676A7C: 388AAFBC  addi r4, r10, -0x5044
	ctx.r[4].s64 = ctx.r[10].s64 + -20548;
	// 82676A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676A90: 386A4560  addi r3, r10, 0x4560
	ctx.r[3].s64 = ctx.r[10].s64 + 17760;
	// 82676A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676AB4: 4BDF036D  bl 0x82466e20
	ctx.lr = 0x82676AB8;
	sub_82466E20(ctx, base);
	// 82676AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676AC8 size=112
    let mut pc: u32 = 0x82676AC8;
    'dispatch: loop {
        match pc {
            0x82676AC8 => {
    //   block [0x82676AC8..0x82676B38)
	// 82676AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676AD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676ADC: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676AE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676AE4: 390B4CA8  addi r8, r11, 0x4ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 19624;
	// 82676AE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82676AEC: 388AAFA8  addi r4, r10, -0x5058
	ctx.r[4].s64 = ctx.r[10].s64 + -20568;
	// 82676AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676AF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676B00: 386A4590  addi r3, r10, 0x4590
	ctx.r[3].s64 = ctx.r[10].s64 + 17808;
	// 82676B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676B24: 4BDF02FD  bl 0x82466e20
	ctx.lr = 0x82676B28;
	sub_82466E20(ctx, base);
	// 82676B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676B38 size=112
    let mut pc: u32 = 0x82676B38;
    'dispatch: loop {
        match pc {
            0x82676B38 => {
    //   block [0x82676B38..0x82676BA8)
	// 82676B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676B44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676B48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676B4C: 38AA4530  addi r5, r10, 0x4530
	ctx.r[5].s64 = ctx.r[10].s64 + 17712;
	// 82676B50: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676B54: 390B4CD8  addi r8, r11, 0x4cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 19672;
	// 82676B58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82676B5C: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 82676B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676B64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676B70: 386A45C0  addi r3, r10, 0x45c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17856;
	// 82676B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676B94: 4BDF028D  bl 0x82466e20
	ctx.lr = 0x82676B98;
	sub_82466E20(ctx, base);
	// 82676B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676BA8 size=112
    let mut pc: u32 = 0x82676BA8;
    'dispatch: loop {
        match pc {
            0x82676BA8 => {
    //   block [0x82676BA8..0x82676C18)
	// 82676BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676BB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676BB8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676BBC: 38AA4620  addi r5, r10, 0x4620
	ctx.r[5].s64 = ctx.r[10].s64 + 17952;
	// 82676BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676BC4: 390B4D08  addi r8, r11, 0x4d08
	ctx.r[8].s64 = ctx.r[11].s64 + 19720;
	// 82676BC8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82676BCC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82676BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676BD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676BE0: 386A45F0  addi r3, r10, 0x45f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17904;
	// 82676BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676C04: 4BDF021D  bl 0x82466e20
	ctx.lr = 0x82676C08;
	sub_82466E20(ctx, base);
	// 82676C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676C18 size=100
    let mut pc: u32 = 0x82676C18;
    'dispatch: loop {
        match pc {
            0x82676C18 => {
    //   block [0x82676C18..0x82676C7C)
	// 82676C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676C24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676C2C: 38AA3000  addi r5, r10, 0x3000
	ctx.r[5].s64 = ctx.r[10].s64 + 12288;
	// 82676C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676C38: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82676C3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676C4C: 386A4620  addi r3, r10, 0x4620
	ctx.r[3].s64 = ctx.r[10].s64 + 17952;
	// 82676C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676C58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82676C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676C60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82676C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676C68: 4BDF01B9  bl 0x82466e20
	ctx.lr = 0x82676C6C;
	sub_82466E20(ctx, base);
	// 82676C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82676C80 size=24
    let mut pc: u32 = 0x82676C80;
    'dispatch: loop {
        match pc {
            0x82676C80 => {
    //   block [0x82676C80..0x82676C98)
	// 82676C80: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676C84: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82676C88: 394A6F90  addi r10, r10, 0x6f90
	ctx.r[10].s64 = ctx.r[10].s64 + 28560;
	// 82676C8C: 816B4D80  lwz r11, 0x4d80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19840 as u32) ) } as u64;
	// 82676C90: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82676C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676C98 size=116
    let mut pc: u32 = 0x82676C98;
    'dispatch: loop {
        match pc {
            0x82676C98 => {
    //   block [0x82676C98..0x82676D0C)
	// 82676C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676CA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676CA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82676CAC: 390B6F90  addi r8, r11, 0x6f90
	ctx.r[8].s64 = ctx.r[11].s64 + 28560;
	// 82676CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676CB4: 392A3378  addi r9, r10, 0x3378
	ctx.r[9].s64 = ctx.r[10].s64 + 13176;
	// 82676CB8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676CBC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82676CC0: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676CC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676CCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676CDC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82676CE0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82676CE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82676CE8: 386B4650  addi r3, r11, 0x4650
	ctx.r[3].s64 = ctx.r[11].s64 + 18000;
	// 82676CEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82676CF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676CF8: 4BDF0129  bl 0x82466e20
	ctx.lr = 0x82676CFC;
	sub_82466E20(ctx, base);
	// 82676CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676D10 size=112
    let mut pc: u32 = 0x82676D10;
    'dispatch: loop {
        match pc {
            0x82676D10 => {
    //   block [0x82676D10..0x82676D80)
	// 82676D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676D1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676D20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676D24: 38AA4320  addi r5, r10, 0x4320
	ctx.r[5].s64 = ctx.r[10].s64 + 17184;
	// 82676D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82676D2C: 390B4D88  addi r8, r11, 0x4d88
	ctx.r[8].s64 = ctx.r[11].s64 + 19848;
	// 82676D30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82676D34: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82676D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676D48: 386A4680  addi r3, r10, 0x4680
	ctx.r[3].s64 = ctx.r[10].s64 + 18048;
	// 82676D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676D6C: 4BDF00B5  bl 0x82466e20
	ctx.lr = 0x82676D70;
	sub_82466E20(ctx, base);
	// 82676D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676D80 size=112
    let mut pc: u32 = 0x82676D80;
    'dispatch: loop {
        match pc {
            0x82676D80 => {
    //   block [0x82676D80..0x82676DF0)
	// 82676D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676D8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676D90: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676D94: 38AA44D0  addi r5, r10, 0x44d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17616;
	// 82676D98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676D9C: 390B4DD0  addi r8, r11, 0x4dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 19920;
	// 82676DA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82676DA4: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82676DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676DB8: 386A46B0  addi r3, r10, 0x46b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18096;
	// 82676DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676DDC: 4BDF0045  bl 0x82466e20
	ctx.lr = 0x82676DE0;
	sub_82466E20(ctx, base);
	// 82676DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676DF0 size=112
    let mut pc: u32 = 0x82676DF0;
    'dispatch: loop {
        match pc {
            0x82676DF0 => {
    //   block [0x82676DF0..0x82676E60)
	// 82676DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676DFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676E00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676E04: 38AA4800  addi r5, r10, 0x4800
	ctx.r[5].s64 = ctx.r[10].s64 + 18432;
	// 82676E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676E0C: 390B4E18  addi r8, r11, 0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + 19992;
	// 82676E10: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82676E14: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82676E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676E1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82676E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676E28: 386A46E0  addi r3, r10, 0x46e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18144;
	// 82676E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82676E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676E4C: 4BDEFFD5  bl 0x82466e20
	ctx.lr = 0x82676E50;
	sub_82466E20(ctx, base);
	// 82676E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676E60 size=108
    let mut pc: u32 = 0x82676E60;
    'dispatch: loop {
        match pc {
            0x82676E60 => {
    //   block [0x82676E60..0x82676ECC)
	// 82676E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676E6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676E70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676E74: 38EB4E90  addi r7, r11, 0x4e90
	ctx.r[7].s64 = ctx.r[11].s64 + 20112;
	// 82676E78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82676E7C: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82676E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676E90: 386A4710  addi r3, r10, 0x4710
	ctx.r[3].s64 = ctx.r[10].s64 + 18192;
	// 82676E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676EB8: 4BDEFF69  bl 0x82466e20
	ctx.lr = 0x82676EBC;
	sub_82466E20(ctx, base);
	// 82676EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82676ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82676ED0 size=108
    let mut pc: u32 = 0x82676ED0;
    'dispatch: loop {
        match pc {
            0x82676ED0 => {
    //   block [0x82676ED0..0x82676F3C)
	// 82676ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82676ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82676ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82676EDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82676EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82676EE4: 38EB4ED8  addi r7, r11, 0x4ed8
	ctx.r[7].s64 = ctx.r[11].s64 + 20184;
	// 82676EE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82676EEC: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82676EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82676EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82676EF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82676EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82676F00: 386A4740  addi r3, r10, 0x4740
	ctx.r[3].s64 = ctx.r[10].s64 + 18240;
	// 82676F04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82676F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82676F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82676F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82676F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82676F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82676F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82676F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82676F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82676F28: 4BDEFEF9  bl 0x82466e20
	ctx.lr = 0x82676F2C;
	sub_82466E20(ctx, base);
	// 82676F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82676F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82676F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82676F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


