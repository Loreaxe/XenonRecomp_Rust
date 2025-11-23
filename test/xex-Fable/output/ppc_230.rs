pub fn sub_832A5460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A5460 size=72
    let mut pc: u32 = 0x832A5460;
    'dispatch: loop {
        match pc {
            0x832A5460 => {
    //   block [0x832A5460..0x832A54A8)
	// 832A5460: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A5464: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A5468: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A546C: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A5470: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A5474: 3CC08336  lis r6, -0x7cca
	ctx.r[6].s64 = -2093613056;
	// 832A5478: 816BF66C  lwz r11, -0x994(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2452 as u32) ) } as u64;
	// 832A547C: 814AF670  lwz r10, -0x990(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2448 as u32) ) } as u64;
	// 832A5480: 8129F674  lwz r9, -0x98c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2444 as u32) ) } as u64;
	// 832A5484: 38A69700  addi r5, r6, -0x6900
	ctx.r[5].s64 = ctx.r[6].s64 + -26880;
	// 832A5488: 8108F678  lwz r8, -0x988(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2440 as u32) ) } as u64;
	// 832A548C: 80E7F67C  lwz r7, -0x984(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2436 as u32) ) } as u64;
	// 832A5490: 91669700  stw r11, -0x6900(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-26880 as u32), ctx.r[11].u32 ) };
	// 832A5494: 91450004  stw r10, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A5498: 91250008  stw r9, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832A549C: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 832A54A0: 90E50010  stw r7, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 832A54A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A54A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A54A8 size=320
    let mut pc: u32 = 0x832A54A8;
    'dispatch: loop {
        match pc {
            0x832A54A8 => {
    //   block [0x832A54A8..0x832A55E8)
	// 832A54A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A54AC: 4BA03F39  bl 0x82ca93e4
	ctx.lr = 0x832A54B0;
	sub_82CA93D0(ctx, base);
	// 832A54B0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A54B4: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A54B8: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A54BC: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A54C0: 3CC08330  lis r6, -0x7cd0
	ctx.r[6].s64 = -2094006272;
	// 832A54C4: 3CA08336  lis r5, -0x7cca
	ctx.r[5].s64 = -2093613056;
	// 832A54C8: 814BF69C  lwz r10, -0x964(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2404 as u32) ) } as u64;
	// 832A54CC: 8129F6A0  lwz r9, -0x960(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2400 as u32) ) } as u64;
	// 832A54D0: 3C808330  lis r4, -0x7cd0
	ctx.r[4].s64 = -2094006272;
	// 832A54D4: 39659718  addi r11, r5, -0x68e8
	ctx.r[11].s64 = ctx.r[5].s64 + -26856;
	// 832A54D8: 8108F6A4  lwz r8, -0x95c(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2396 as u32) ) } as u64;
	// 832A54DC: 80E7F6AC  lwz r7, -0x954(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2388 as u32) ) } as u64;
	// 832A54E0: 3C608330  lis r3, -0x7cd0
	ctx.r[3].s64 = -2094006272;
	// 832A54E4: 80C6F6B0  lwz r6, -0x950(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2384 as u32) ) } as u64;
	// 832A54E8: 3FE08330  lis r31, -0x7cd0
	ctx.r[31].s64 = -2094006272;
	// 832A54EC: 91459718  stw r10, -0x68e8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(-26856 as u32), ctx.r[10].u32 ) };
	// 832A54F0: 3FC08330  lis r30, -0x7cd0
	ctx.r[30].s64 = -2094006272;
	// 832A54F4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A54F8: 80A4F6B8  lwz r5, -0x948(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-2376 as u32) ) } as u64;
	// 832A54FC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A5500: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A5504: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 832A5508: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A550C: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 832A5510: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A5514: 90CB0010  stw r6, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 832A5518: 3CC08330  lis r6, -0x7cd0
	ctx.r[6].s64 = -2094006272;
	// 832A551C: 3FA08330  lis r29, -0x7cd0
	ctx.r[29].s64 = -2094006272;
	// 832A5520: 8083F6C0  lwz r4, -0x940(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-2368 as u32) ) } as u64;
	// 832A5524: 8389F6CC  lwz r28, -0x934(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2356 as u32) ) } as u64;
	// 832A5528: 3F008330  lis r24, -0x7cd0
	ctx.r[24].s64 = -2094006272;
	// 832A552C: 8368F6D0  lwz r27, -0x930(r8)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2352 as u32) ) } as u64;
	// 832A5530: 3EE08330  lis r23, -0x7cd0
	ctx.r[23].s64 = -2094006272;
	// 832A5534: 8347F6D4  lwz r26, -0x92c(r7)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2348 as u32) ) } as u64;
	// 832A5538: 3EC08330  lis r22, -0x7cd0
	ctx.r[22].s64 = -2094006272;
	// 832A553C: 8326F6D8  lwz r25, -0x928(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2344 as u32) ) } as u64;
	// 832A5540: 3EA08330  lis r21, -0x7cd0
	ctx.r[21].s64 = -2094006272;
	// 832A5544: 3E808330  lis r20, -0x7cd0
	ctx.r[20].s64 = -2094006272;
	// 832A5548: 807FF6C8  lwz r3, -0x938(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-2360 as u32) ) } as u64;
	// 832A554C: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A5550: 83FEF6B4  lwz r31, -0x94c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-2380 as u32) ) } as u64;
	// 832A5554: 3D008330  lis r8, -0x7cd0
	ctx.r[8].s64 = -2094006272;
	// 832A5558: 83CAF6BC  lwz r30, -0x944(r10)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2372 as u32) ) } as u64;
	// 832A555C: 3CE08330  lis r7, -0x7cd0
	ctx.r[7].s64 = -2094006272;
	// 832A5560: 83BDF6C4  lwz r29, -0x93c(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-2364 as u32) ) } as u64;
	// 832A5564: 3CC08330  lis r6, -0x7cd0
	ctx.r[6].s64 = -2094006272;
	// 832A5568: 8318F6E0  lwz r24, -0x920(r24)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(-2336 as u32) ) } as u64;
	// 832A556C: 394B0060  addi r10, r11, 0x60
	ctx.r[10].s64 = ctx.r[11].s64 + 96;
	// 832A5570: 82F7F6E4  lwz r23, -0x91c(r23)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-2332 as u32) ) } as u64;
	// 832A5574: 82D6F6EC  lwz r22, -0x914(r22)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(-2324 as u32) ) } as u64;
	// 832A5578: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 832A557C: 82B5F6F4  lwz r21, -0x90c(r21)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-2316 as u32) ) } as u64;
	// 832A5580: 8154F6FC  lwz r10, -0x904(r20)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-2308 as u32) ) } as u64;
	// 832A5584: 8129F6E8  lwz r9, -0x918(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 832A5588: 8108F6F0  lwz r8, -0x910(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-2320 as u32) ) } as u64;
	// 832A558C: 80E7F6F8  lwz r7, -0x908(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-2312 as u32) ) } as u64;
	// 832A5590: 80C6F700  lwz r6, -0x900(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-2304 as u32) ) } as u64;
	// 832A5594: 90AB0014  stw r5, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 832A5598: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 832A559C: 906B001C  stw r3, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 832A55A0: 93EB0020  stw r31, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 832A55A4: 93CB0024  stw r30, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 832A55A8: 93AB0028  stw r29, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 832A55AC: 938B002C  stw r28, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 832A55B0: 936B0030  stw r27, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[27].u32 ) };
	// 832A55B4: 934B0034  stw r26, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[26].u32 ) };
	// 832A55B8: 932B0038  stw r25, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 832A55BC: 930B003C  stw r24, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[24].u32 ) };
	// 832A55C0: 92EB0040  stw r23, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[23].u32 ) };
	// 832A55C4: 92CB0044  stw r22, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[22].u32 ) };
	// 832A55C8: 92AB0048  stw r21, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[21].u32 ) };
	// 832A55CC: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 832A55D0: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 832A55D4: 910B0054  stw r8, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A55D8: 90EB0058  stw r7, 0x58(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 832A55DC: 90CB005C  stw r6, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832A55E0: 926B0060  stw r19, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[19].u32 ) };
	// 832A55E4: 4BA03E50  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A55E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A55E8 size=56
    let mut pc: u32 = 0x832A55E8;
    'dispatch: loop {
        match pc {
            0x832A55E8 => {
    //   block [0x832A55E8..0x832A5620)
	// 832A55E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A55EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A55F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A55F4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A55F8: 396BF784  addi r11, r11, -0x87c
	ctx.r[11].s64 = ctx.r[11].s64 + -2172;
	// 832A55FC: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 832A5600: 48014685  bl 0x832b9c84
	ctx.lr = 0x832A5604;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832A5604: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5608: 386B9358  addi r3, r11, -0x6ca8
	ctx.r[3].s64 = ctx.r[11].s64 + -27816;
	// 832A560C: 4BA04915  bl 0x82ca9f20
	ctx.lr = 0x832A5610;
	sub_82CA9F20(ctx, base);
	// 832A5610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A561C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5620 size=96
    let mut pc: u32 = 0x832A5620;
    'dispatch: loop {
        match pc {
            0x832A5620 => {
    //   block [0x832A5620..0x832A5680)
	// 832A5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A562C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5630: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5634: 3BEB9780  addi r31, r11, -0x6880
	ctx.r[31].s64 = ctx.r[11].s64 + -26752;
	// 832A5638: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832A563C: 4BD5D3E5  bl 0x83002a20
	ctx.lr = 0x832A5640;
	sub_83002A20(ctx, base);
	// 832A5640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A5644: 48014641  bl 0x832b9c84
	ctx.lr = 0x832A5648;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 832A5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A564C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A5650: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A5654: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 832A5658: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 832A565C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5660: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 832A5664: 386B9360  addi r3, r11, -0x6ca0
	ctx.r[3].s64 = ctx.r[11].s64 + -27808;
	// 832A5668: 4BA048B9  bl 0x82ca9f20
	ctx.lr = 0x832A566C;
	sub_82CA9F20(ctx, base);
	// 832A566C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5680 size=52
    let mut pc: u32 = 0x832A5680;
    'dispatch: loop {
        match pc {
            0x832A5680 => {
    //   block [0x832A5680..0x832A56B4)
	// 832A5680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A568C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5690: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5694: 388BB320  addi r4, r11, -0x4ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -19680;
	// 832A5698: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A569C: 386B97C0  addi r3, r11, -0x6840
	ctx.r[3].s64 = ctx.r[11].s64 + -26688;
	// 832A56A0: 4BD5DEF9  bl 0x83003598
	ctx.lr = 0x832A56A4;
	sub_83003598(ctx, base);
	// 832A56A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A56A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A56AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A56B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A56B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A56B8 size=52
    let mut pc: u32 = 0x832A56B8;
    'dispatch: loop {
        match pc {
            0x832A56B8 => {
    //   block [0x832A56B8..0x832A56EC)
	// 832A56B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A56BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A56C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A56C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A56C8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A56CC: 388BB38C  addi r4, r11, -0x4c74
	ctx.r[4].s64 = ctx.r[11].s64 + -19572;
	// 832A56D0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A56D4: 386B97C8  addi r3, r11, -0x6838
	ctx.r[3].s64 = ctx.r[11].s64 + -26680;
	// 832A56D8: 4BD5DEC1  bl 0x83003598
	ctx.lr = 0x832A56DC;
	sub_83003598(ctx, base);
	// 832A56DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A56E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A56E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A56E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A56F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A56F0 size=52
    let mut pc: u32 = 0x832A56F0;
    'dispatch: loop {
        match pc {
            0x832A56F0 => {
    //   block [0x832A56F0..0x832A5724)
	// 832A56F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A56F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A56F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A56FC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5700: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5704: 388BB778  addi r4, r11, -0x4888
	ctx.r[4].s64 = ctx.r[11].s64 + -18568;
	// 832A5708: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A570C: 386B97D0  addi r3, r11, -0x6830
	ctx.r[3].s64 = ctx.r[11].s64 + -26672;
	// 832A5710: 4BD5DE89  bl 0x83003598
	ctx.lr = 0x832A5714;
	sub_83003598(ctx, base);
	// 832A5714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A571C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5728 size=52
    let mut pc: u32 = 0x832A5728;
    'dispatch: loop {
        match pc {
            0x832A5728 => {
    //   block [0x832A5728..0x832A575C)
	// 832A5728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5734: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 832A5738: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A573C: 388BB808  addi r4, r11, -0x47f8
	ctx.r[4].s64 = ctx.r[11].s64 + -18424;
	// 832A5740: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5744: 386B97D8  addi r3, r11, -0x6828
	ctx.r[3].s64 = ctx.r[11].s64 + -26664;
	// 832A5748: 4BD5DE51  bl 0x83003598
	ctx.lr = 0x832A574C;
	sub_83003598(ctx, base);
	// 832A574C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5760 size=52
    let mut pc: u32 = 0x832A5760;
    'dispatch: loop {
        match pc {
            0x832A5760 => {
    //   block [0x832A5760..0x832A5794)
	// 832A5760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A576C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5770: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5774: 388BB970  addi r4, r11, -0x4690
	ctx.r[4].s64 = ctx.r[11].s64 + -18064;
	// 832A5778: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A577C: 386B97E0  addi r3, r11, -0x6820
	ctx.r[3].s64 = ctx.r[11].s64 + -26656;
	// 832A5780: 4BD5DE19  bl 0x83003598
	ctx.lr = 0x832A5784;
	sub_83003598(ctx, base);
	// 832A5784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A578C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5798 size=52
    let mut pc: u32 = 0x832A5798;
    'dispatch: loop {
        match pc {
            0x832A5798 => {
    //   block [0x832A5798..0x832A57CC)
	// 832A5798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A57A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A57A4: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A57A8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A57AC: 388BB9B8  addi r4, r11, -0x4648
	ctx.r[4].s64 = ctx.r[11].s64 + -17992;
	// 832A57B0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A57B4: 386B97E8  addi r3, r11, -0x6818
	ctx.r[3].s64 = ctx.r[11].s64 + -26648;
	// 832A57B8: 4BD5DDE1  bl 0x83003598
	ctx.lr = 0x832A57BC;
	sub_83003598(ctx, base);
	// 832A57BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A57C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A57C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A57C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A57D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A57D0 size=52
    let mut pc: u32 = 0x832A57D0;
    'dispatch: loop {
        match pc {
            0x832A57D0 => {
    //   block [0x832A57D0..0x832A5804)
	// 832A57D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A57D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A57D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A57DC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A57E0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A57E4: 388BBA48  addi r4, r11, -0x45b8
	ctx.r[4].s64 = ctx.r[11].s64 + -17848;
	// 832A57E8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A57EC: 386B97F0  addi r3, r11, -0x6810
	ctx.r[3].s64 = ctx.r[11].s64 + -26640;
	// 832A57F0: 4BD5DDA9  bl 0x83003598
	ctx.lr = 0x832A57F4;
	sub_83003598(ctx, base);
	// 832A57F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A57F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A57FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5808 size=52
    let mut pc: u32 = 0x832A5808;
    'dispatch: loop {
        match pc {
            0x832A5808 => {
    //   block [0x832A5808..0x832A583C)
	// 832A5808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A580C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5814: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A5818: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A581C: 388BBAD8  addi r4, r11, -0x4528
	ctx.r[4].s64 = ctx.r[11].s64 + -17704;
	// 832A5820: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5824: 386B97F8  addi r3, r11, -0x6808
	ctx.r[3].s64 = ctx.r[11].s64 + -26632;
	// 832A5828: 4BD5DD71  bl 0x83003598
	ctx.lr = 0x832A582C;
	sub_83003598(ctx, base);
	// 832A582C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5840 size=52
    let mut pc: u32 = 0x832A5840;
    'dispatch: loop {
        match pc {
            0x832A5840 => {
    //   block [0x832A5840..0x832A5874)
	// 832A5840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A584C: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832A5850: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5854: 388BBB68  addi r4, r11, -0x4498
	ctx.r[4].s64 = ctx.r[11].s64 + -17560;
	// 832A5858: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A585C: 386B9800  addi r3, r11, -0x6800
	ctx.r[3].s64 = ctx.r[11].s64 + -26624;
	// 832A5860: 4BD5DD39  bl 0x83003598
	ctx.lr = 0x832A5864;
	sub_83003598(ctx, base);
	// 832A5864: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A586C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5878 size=52
    let mut pc: u32 = 0x832A5878;
    'dispatch: loop {
        match pc {
            0x832A5878 => {
    //   block [0x832A5878..0x832A58AC)
	// 832A5878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5884: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 832A5888: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A588C: 388BBD18  addi r4, r11, -0x42e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17128;
	// 832A5890: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5894: 386B9808  addi r3, r11, -0x67f8
	ctx.r[3].s64 = ctx.r[11].s64 + -26616;
	// 832A5898: 4BD5DD01  bl 0x83003598
	ctx.lr = 0x832A589C;
	sub_83003598(ctx, base);
	// 832A589C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A58A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A58A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A58A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A58B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A58B0 size=52
    let mut pc: u32 = 0x832A58B0;
    'dispatch: loop {
        match pc {
            0x832A58B0 => {
    //   block [0x832A58B0..0x832A58E4)
	// 832A58B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A58B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A58B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A58BC: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 832A58C0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A58C4: 388BBDF0  addi r4, r11, -0x4210
	ctx.r[4].s64 = ctx.r[11].s64 + -16912;
	// 832A58C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A58CC: 386B9810  addi r3, r11, -0x67f0
	ctx.r[3].s64 = ctx.r[11].s64 + -26608;
	// 832A58D0: 4BD5DCC9  bl 0x83003598
	ctx.lr = 0x832A58D4;
	sub_83003598(ctx, base);
	// 832A58D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A58D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A58DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A58E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A58E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A58E8 size=52
    let mut pc: u32 = 0x832A58E8;
    'dispatch: loop {
        match pc {
            0x832A58E8 => {
    //   block [0x832A58E8..0x832A591C)
	// 832A58E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A58EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A58F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A58F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A58F8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A58FC: 388BBF7C  addi r4, r11, -0x4084
	ctx.r[4].s64 = ctx.r[11].s64 + -16516;
	// 832A5900: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5904: 386B9818  addi r3, r11, -0x67e8
	ctx.r[3].s64 = ctx.r[11].s64 + -26600;
	// 832A5908: 4BD5DC91  bl 0x83003598
	ctx.lr = 0x832A590C;
	sub_83003598(ctx, base);
	// 832A590C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5920 size=52
    let mut pc: u32 = 0x832A5920;
    'dispatch: loop {
        match pc {
            0x832A5920 => {
    //   block [0x832A5920..0x832A5954)
	// 832A5920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A592C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A5930: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5934: 388BBFE8  addi r4, r11, -0x4018
	ctx.r[4].s64 = ctx.r[11].s64 + -16408;
	// 832A5938: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A593C: 386B9820  addi r3, r11, -0x67e0
	ctx.r[3].s64 = ctx.r[11].s64 + -26592;
	// 832A5940: 4BD5DC59  bl 0x83003598
	ctx.lr = 0x832A5944;
	sub_83003598(ctx, base);
	// 832A5944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A594C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5958 size=52
    let mut pc: u32 = 0x832A5958;
    'dispatch: loop {
        match pc {
            0x832A5958 => {
    //   block [0x832A5958..0x832A598C)
	// 832A5958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A595C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5964: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 832A5968: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A596C: 388BC030  addi r4, r11, -0x3fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -16336;
	// 832A5970: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5974: 386B9828  addi r3, r11, -0x67d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26584;
	// 832A5978: 4BD5DC21  bl 0x83003598
	ctx.lr = 0x832A597C;
	sub_83003598(ctx, base);
	// 832A597C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5990 size=44
    let mut pc: u32 = 0x832A5990;
    'dispatch: loop {
        match pc {
            0x832A5990 => {
    //   block [0x832A5990..0x832A59BC)
	// 832A5990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A599C: 4BD5DBA5  bl 0x83003540
	ctx.lr = 0x832A59A0;
	sub_83003540(ctx, base);
	// 832A59A0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A59A4: 396B9830  addi r11, r11, -0x67d0
	ctx.r[11].s64 = ctx.r[11].s64 + -26576;
	// 832A59A8: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 832A59AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A59B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A59B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A59B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A59C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A59C0 size=52
    let mut pc: u32 = 0x832A59C0;
    'dispatch: loop {
        match pc {
            0x832A59C0 => {
    //   block [0x832A59C0..0x832A59F4)
	// 832A59C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A59C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A59C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A59CC: 38A0000A  li r5, 0xa
	ctx.r[5].s64 = 10;
	// 832A59D0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A59D4: 388BC278  addi r4, r11, -0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + -15752;
	// 832A59D8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A59DC: 386B9834  addi r3, r11, -0x67cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26572;
	// 832A59E0: 4BD5DBB9  bl 0x83003598
	ctx.lr = 0x832A59E4;
	sub_83003598(ctx, base);
	// 832A59E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A59E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A59EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A59F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A59F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A59F8 size=52
    let mut pc: u32 = 0x832A59F8;
    'dispatch: loop {
        match pc {
            0x832A59F8 => {
    //   block [0x832A59F8..0x832A5A2C)
	// 832A59F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A59FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A04: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A08: 386B983C  addi r3, r11, -0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + -26564;
	// 832A5A0C: 4BD5F6E5  bl 0x830050f0
	ctx.lr = 0x832A5A10;
	sub_830050F0(ctx, base);
	// 832A5A10: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A14: 386B9370  addi r3, r11, -0x6c90
	ctx.r[3].s64 = ctx.r[11].s64 + -27792;
	// 832A5A18: 4BA04509  bl 0x82ca9f20
	ctx.lr = 0x832A5A1C;
	sub_82CA9F20(ctx, base);
	// 832A5A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5A30 size=52
    let mut pc: u32 = 0x832A5A30;
    'dispatch: loop {
        match pc {
            0x832A5A30 => {
    //   block [0x832A5A30..0x832A5A64)
	// 832A5A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A3C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A40: 386B9870  addi r3, r11, -0x6790
	ctx.r[3].s64 = ctx.r[11].s64 + -26512;
	// 832A5A44: 4BD5DBE5  bl 0x83003628
	ctx.lr = 0x832A5A48;
	sub_83003628(ctx, base);
	// 832A5A48: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A4C: 386B9398  addi r3, r11, -0x6c68
	ctx.r[3].s64 = ctx.r[11].s64 + -27752;
	// 832A5A50: 4BA044D1  bl 0x82ca9f20
	ctx.lr = 0x832A5A54;
	sub_82CA9F20(ctx, base);
	// 832A5A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5A68 size=52
    let mut pc: u32 = 0x832A5A68;
    'dispatch: loop {
        match pc {
            0x832A5A68 => {
    //   block [0x832A5A68..0x832A5A9C)
	// 832A5A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5A74: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5A78: 386B9864  addi r3, r11, -0x679c
	ctx.r[3].s64 = ctx.r[11].s64 + -26524;
	// 832A5A7C: 4BD5EDED  bl 0x83004868
	ctx.lr = 0x832A5A80;
	sub_83004868(ctx, base);
	// 832A5A80: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5A84: 386B93C0  addi r3, r11, -0x6c40
	ctx.r[3].s64 = ctx.r[11].s64 + -27712;
	// 832A5A88: 4BA04499  bl 0x82ca9f20
	ctx.lr = 0x832A5A8C;
	sub_82CA9F20(ctx, base);
	// 832A5A8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5AA0 size=52
    let mut pc: u32 = 0x832A5AA0;
    'dispatch: loop {
        match pc {
            0x832A5AA0 => {
    //   block [0x832A5AA0..0x832A5AD4)
	// 832A5AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5AAC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5AB0: 386B98A0  addi r3, r11, -0x6760
	ctx.r[3].s64 = ctx.r[11].s64 + -26464;
	// 832A5AB4: 4BD5DB75  bl 0x83003628
	ctx.lr = 0x832A5AB8;
	sub_83003628(ctx, base);
	// 832A5AB8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5ABC: 386B93E8  addi r3, r11, -0x6c18
	ctx.r[3].s64 = ctx.r[11].s64 + -27672;
	// 832A5AC0: 4BA04461  bl 0x82ca9f20
	ctx.lr = 0x832A5AC4;
	sub_82CA9F20(ctx, base);
	// 832A5AC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5AD8 size=52
    let mut pc: u32 = 0x832A5AD8;
    'dispatch: loop {
        match pc {
            0x832A5AD8 => {
    //   block [0x832A5AD8..0x832A5B0C)
	// 832A5AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5AE4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5AE8: 386B9894  addi r3, r11, -0x676c
	ctx.r[3].s64 = ctx.r[11].s64 + -26476;
	// 832A5AEC: 4BD5ED7D  bl 0x83004868
	ctx.lr = 0x832A5AF0;
	sub_83004868(ctx, base);
	// 832A5AF0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5AF4: 386B9410  addi r3, r11, -0x6bf0
	ctx.r[3].s64 = ctx.r[11].s64 + -27632;
	// 832A5AF8: 4BA04429  bl 0x82ca9f20
	ctx.lr = 0x832A5AFC;
	sub_82CA9F20(ctx, base);
	// 832A5AFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B10 size=52
    let mut pc: u32 = 0x832A5B10;
    'dispatch: loop {
        match pc {
            0x832A5B10 => {
    //   block [0x832A5B10..0x832A5B44)
	// 832A5B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B1C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B20: 386B98D0  addi r3, r11, -0x6730
	ctx.r[3].s64 = ctx.r[11].s64 + -26416;
	// 832A5B24: 4BD5DB05  bl 0x83003628
	ctx.lr = 0x832A5B28;
	sub_83003628(ctx, base);
	// 832A5B28: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5B2C: 386B9438  addi r3, r11, -0x6bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -27592;
	// 832A5B30: 4BA043F1  bl 0x82ca9f20
	ctx.lr = 0x832A5B34;
	sub_82CA9F20(ctx, base);
	// 832A5B34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B48 size=52
    let mut pc: u32 = 0x832A5B48;
    'dispatch: loop {
        match pc {
            0x832A5B48 => {
    //   block [0x832A5B48..0x832A5B7C)
	// 832A5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B54: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B58: 386B98C4  addi r3, r11, -0x673c
	ctx.r[3].s64 = ctx.r[11].s64 + -26428;
	// 832A5B5C: 4BD5ED0D  bl 0x83004868
	ctx.lr = 0x832A5B60;
	sub_83004868(ctx, base);
	// 832A5B60: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5B64: 386B9460  addi r3, r11, -0x6ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -27552;
	// 832A5B68: 4BA043B9  bl 0x82ca9f20
	ctx.lr = 0x832A5B6C;
	sub_82CA9F20(ctx, base);
	// 832A5B6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5B80 size=52
    let mut pc: u32 = 0x832A5B80;
    'dispatch: loop {
        match pc {
            0x832A5B80 => {
    //   block [0x832A5B80..0x832A5BB4)
	// 832A5B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5B8C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5B90: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5B94: 388BC490  addi r4, r11, -0x3b70
	ctx.r[4].s64 = ctx.r[11].s64 + -15216;
	// 832A5B98: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5B9C: 386B98F4  addi r3, r11, -0x670c
	ctx.r[3].s64 = ctx.r[11].s64 + -26380;
	// 832A5BA0: 4BD5D9F9  bl 0x83003598
	ctx.lr = 0x832A5BA4;
	sub_83003598(ctx, base);
	// 832A5BA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5BB8 size=52
    let mut pc: u32 = 0x832A5BB8;
    'dispatch: loop {
        match pc {
            0x832A5BB8 => {
    //   block [0x832A5BB8..0x832A5BEC)
	// 832A5BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5BC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5BC8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5BCC: 388BC4B4  addi r4, r11, -0x3b4c
	ctx.r[4].s64 = ctx.r[11].s64 + -15180;
	// 832A5BD0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5BD4: 386B98FC  addi r3, r11, -0x6704
	ctx.r[3].s64 = ctx.r[11].s64 + -26372;
	// 832A5BD8: 4BD5D9C1  bl 0x83003598
	ctx.lr = 0x832A5BDC;
	sub_83003598(ctx, base);
	// 832A5BDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5BF0 size=52
    let mut pc: u32 = 0x832A5BF0;
    'dispatch: loop {
        match pc {
            0x832A5BF0 => {
    //   block [0x832A5BF0..0x832A5C24)
	// 832A5BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5BFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C00: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C04: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 832A5C08: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C0C: 386B9904  addi r3, r11, -0x66fc
	ctx.r[3].s64 = ctx.r[11].s64 + -26364;
	// 832A5C10: 4BD5D989  bl 0x83003598
	ctx.lr = 0x832A5C14;
	sub_83003598(ctx, base);
	// 832A5C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C28 size=52
    let mut pc: u32 = 0x832A5C28;
    'dispatch: loop {
        match pc {
            0x832A5C28 => {
    //   block [0x832A5C28..0x832A5C5C)
	// 832A5C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5C34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C38: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C3C: 388BC4FC  addi r4, r11, -0x3b04
	ctx.r[4].s64 = ctx.r[11].s64 + -15108;
	// 832A5C40: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C44: 386B990C  addi r3, r11, -0x66f4
	ctx.r[3].s64 = ctx.r[11].s64 + -26356;
	// 832A5C48: 4BD5D951  bl 0x83003598
	ctx.lr = 0x832A5C4C;
	sub_83003598(ctx, base);
	// 832A5C4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C60 size=52
    let mut pc: u32 = 0x832A5C60;
    'dispatch: loop {
        match pc {
            0x832A5C60 => {
    //   block [0x832A5C60..0x832A5C94)
	// 832A5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5C6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5C70: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5C74: 388BC520  addi r4, r11, -0x3ae0
	ctx.r[4].s64 = ctx.r[11].s64 + -15072;
	// 832A5C78: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5C7C: 386B9914  addi r3, r11, -0x66ec
	ctx.r[3].s64 = ctx.r[11].s64 + -26348;
	// 832A5C80: 4BD5D919  bl 0x83003598
	ctx.lr = 0x832A5C84;
	sub_83003598(ctx, base);
	// 832A5C84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5C98 size=52
    let mut pc: u32 = 0x832A5C98;
    'dispatch: loop {
        match pc {
            0x832A5C98 => {
    //   block [0x832A5C98..0x832A5CCC)
	// 832A5C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5CA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5CA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5CA8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5CAC: 388BC544  addi r4, r11, -0x3abc
	ctx.r[4].s64 = ctx.r[11].s64 + -15036;
	// 832A5CB0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5CB4: 386B991C  addi r3, r11, -0x66e4
	ctx.r[3].s64 = ctx.r[11].s64 + -26340;
	// 832A5CB8: 4BD5D8E1  bl 0x83003598
	ctx.lr = 0x832A5CBC;
	sub_83003598(ctx, base);
	// 832A5CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5CD0 size=52
    let mut pc: u32 = 0x832A5CD0;
    'dispatch: loop {
        match pc {
            0x832A5CD0 => {
    //   block [0x832A5CD0..0x832A5D04)
	// 832A5CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5CD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5CDC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5CE0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5CE4: 388BC568  addi r4, r11, -0x3a98
	ctx.r[4].s64 = ctx.r[11].s64 + -15000;
	// 832A5CE8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5CEC: 386B9924  addi r3, r11, -0x66dc
	ctx.r[3].s64 = ctx.r[11].s64 + -26332;
	// 832A5CF0: 4BD5D8A9  bl 0x83003598
	ctx.lr = 0x832A5CF4;
	sub_83003598(ctx, base);
	// 832A5CF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D08 size=52
    let mut pc: u32 = 0x832A5D08;
    'dispatch: loop {
        match pc {
            0x832A5D08 => {
    //   block [0x832A5D08..0x832A5D3C)
	// 832A5D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D14: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 832A5D18: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A5D1C: 388BC58C  addi r4, r11, -0x3a74
	ctx.r[4].s64 = ctx.r[11].s64 + -14964;
	// 832A5D20: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D24: 386B992C  addi r3, r11, -0x66d4
	ctx.r[3].s64 = ctx.r[11].s64 + -26324;
	// 832A5D28: 4BD5D871  bl 0x83003598
	ctx.lr = 0x832A5D2C;
	sub_83003598(ctx, base);
	// 832A5D2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D40 size=52
    let mut pc: u32 = 0x832A5D40;
    'dispatch: loop {
        match pc {
            0x832A5D40 => {
    //   block [0x832A5D40..0x832A5D74)
	// 832A5D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D4C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D50: 386B9940  addi r3, r11, -0x66c0
	ctx.r[3].s64 = ctx.r[11].s64 + -26304;
	// 832A5D54: 4BD5D8D5  bl 0x83003628
	ctx.lr = 0x832A5D58;
	sub_83003628(ctx, base);
	// 832A5D58: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5D5C: 386B9488  addi r3, r11, -0x6b78
	ctx.r[3].s64 = ctx.r[11].s64 + -27512;
	// 832A5D60: 4BA041C1  bl 0x82ca9f20
	ctx.lr = 0x832A5D64;
	sub_82CA9F20(ctx, base);
	// 832A5D64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5D78 size=52
    let mut pc: u32 = 0x832A5D78;
    'dispatch: loop {
        match pc {
            0x832A5D78 => {
    //   block [0x832A5D78..0x832A5DAC)
	// 832A5D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5D84: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5D88: 386B9934  addi r3, r11, -0x66cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26316;
	// 832A5D8C: 4BD5EADD  bl 0x83004868
	ctx.lr = 0x832A5D90;
	sub_83004868(ctx, base);
	// 832A5D90: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5D94: 386B94B0  addi r3, r11, -0x6b50
	ctx.r[3].s64 = ctx.r[11].s64 + -27472;
	// 832A5D98: 4BA04189  bl 0x82ca9f20
	ctx.lr = 0x832A5D9C;
	sub_82CA9F20(ctx, base);
	// 832A5D9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5DB0 size=52
    let mut pc: u32 = 0x832A5DB0;
    'dispatch: loop {
        match pc {
            0x832A5DB0 => {
    //   block [0x832A5DB0..0x832A5DE4)
	// 832A5DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5DBC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5DC0: 386B9A00  addi r3, r11, -0x6600
	ctx.r[3].s64 = ctx.r[11].s64 + -26112;
	// 832A5DC4: 4BD5D865  bl 0x83003628
	ctx.lr = 0x832A5DC8;
	sub_83003628(ctx, base);
	// 832A5DC8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5DCC: 386B94D8  addi r3, r11, -0x6b28
	ctx.r[3].s64 = ctx.r[11].s64 + -27432;
	// 832A5DD0: 4BA04151  bl 0x82ca9f20
	ctx.lr = 0x832A5DD4;
	sub_82CA9F20(ctx, base);
	// 832A5DD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5DE8 size=52
    let mut pc: u32 = 0x832A5DE8;
    'dispatch: loop {
        match pc {
            0x832A5DE8 => {
    //   block [0x832A5DE8..0x832A5E1C)
	// 832A5DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5DF4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5DF8: 386B99F4  addi r3, r11, -0x660c
	ctx.r[3].s64 = ctx.r[11].s64 + -26124;
	// 832A5DFC: 4BD5EA6D  bl 0x83004868
	ctx.lr = 0x832A5E00;
	sub_83004868(ctx, base);
	// 832A5E00: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E04: 386B9500  addi r3, r11, -0x6b00
	ctx.r[3].s64 = ctx.r[11].s64 + -27392;
	// 832A5E08: 4BA04119  bl 0x82ca9f20
	ctx.lr = 0x832A5E0C;
	sub_82CA9F20(ctx, base);
	// 832A5E0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E20 size=52
    let mut pc: u32 = 0x832A5E20;
    'dispatch: loop {
        match pc {
            0x832A5E20 => {
    //   block [0x832A5E20..0x832A5E54)
	// 832A5E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E2C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5E30: 386B99D0  addi r3, r11, -0x6630
	ctx.r[3].s64 = ctx.r[11].s64 + -26160;
	// 832A5E34: 4BD5D7F5  bl 0x83003628
	ctx.lr = 0x832A5E38;
	sub_83003628(ctx, base);
	// 832A5E38: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E3C: 386B9528  addi r3, r11, -0x6ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -27352;
	// 832A5E40: 4BA040E1  bl 0x82ca9f20
	ctx.lr = 0x832A5E44;
	sub_82CA9F20(ctx, base);
	// 832A5E44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E58 size=52
    let mut pc: u32 = 0x832A5E58;
    'dispatch: loop {
        match pc {
            0x832A5E58 => {
    //   block [0x832A5E58..0x832A5E8C)
	// 832A5E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E64: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5E68: 386B99C4  addi r3, r11, -0x663c
	ctx.r[3].s64 = ctx.r[11].s64 + -26172;
	// 832A5E6C: 4BD5E9FD  bl 0x83004868
	ctx.lr = 0x832A5E70;
	sub_83004868(ctx, base);
	// 832A5E70: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5E74: 386B9550  addi r3, r11, -0x6ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -27312;
	// 832A5E78: 4BA040A9  bl 0x82ca9f20
	ctx.lr = 0x832A5E7C;
	sub_82CA9F20(ctx, base);
	// 832A5E7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5E90 size=52
    let mut pc: u32 = 0x832A5E90;
    'dispatch: loop {
        match pc {
            0x832A5E90 => {
    //   block [0x832A5E90..0x832A5EC4)
	// 832A5E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5E9C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5EA0: 386B99A0  addi r3, r11, -0x6660
	ctx.r[3].s64 = ctx.r[11].s64 + -26208;
	// 832A5EA4: 4BD5D785  bl 0x83003628
	ctx.lr = 0x832A5EA8;
	sub_83003628(ctx, base);
	// 832A5EA8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5EAC: 386B9578  addi r3, r11, -0x6a88
	ctx.r[3].s64 = ctx.r[11].s64 + -27272;
	// 832A5EB0: 4BA04071  bl 0x82ca9f20
	ctx.lr = 0x832A5EB4;
	sub_82CA9F20(ctx, base);
	// 832A5EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5EC8 size=52
    let mut pc: u32 = 0x832A5EC8;
    'dispatch: loop {
        match pc {
            0x832A5EC8 => {
    //   block [0x832A5EC8..0x832A5EFC)
	// 832A5EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5ED4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5ED8: 386B9994  addi r3, r11, -0x666c
	ctx.r[3].s64 = ctx.r[11].s64 + -26220;
	// 832A5EDC: 4BD5E98D  bl 0x83004868
	ctx.lr = 0x832A5EE0;
	sub_83004868(ctx, base);
	// 832A5EE0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5EE4: 386B95A0  addi r3, r11, -0x6a60
	ctx.r[3].s64 = ctx.r[11].s64 + -27232;
	// 832A5EE8: 4BA04039  bl 0x82ca9f20
	ctx.lr = 0x832A5EEC;
	sub_82CA9F20(ctx, base);
	// 832A5EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F00 size=52
    let mut pc: u32 = 0x832A5F00;
    'dispatch: loop {
        match pc {
            0x832A5F00 => {
    //   block [0x832A5F00..0x832A5F34)
	// 832A5F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F0C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F10: 386B9970  addi r3, r11, -0x6690
	ctx.r[3].s64 = ctx.r[11].s64 + -26256;
	// 832A5F14: 4BD5D715  bl 0x83003628
	ctx.lr = 0x832A5F18;
	sub_83003628(ctx, base);
	// 832A5F18: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F1C: 386B95C8  addi r3, r11, -0x6a38
	ctx.r[3].s64 = ctx.r[11].s64 + -27192;
	// 832A5F20: 4BA04001  bl 0x82ca9f20
	ctx.lr = 0x832A5F24;
	sub_82CA9F20(ctx, base);
	// 832A5F24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F38 size=52
    let mut pc: u32 = 0x832A5F38;
    'dispatch: loop {
        match pc {
            0x832A5F38 => {
    //   block [0x832A5F38..0x832A5F6C)
	// 832A5F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F44: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F48: 386B9964  addi r3, r11, -0x669c
	ctx.r[3].s64 = ctx.r[11].s64 + -26268;
	// 832A5F4C: 4BD5E91D  bl 0x83004868
	ctx.lr = 0x832A5F50;
	sub_83004868(ctx, base);
	// 832A5F50: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F54: 386B95F0  addi r3, r11, -0x6a10
	ctx.r[3].s64 = ctx.r[11].s64 + -27152;
	// 832A5F58: 4BA03FC9  bl 0x82ca9f20
	ctx.lr = 0x832A5F5C;
	sub_82CA9F20(ctx, base);
	// 832A5F5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5F70 size=52
    let mut pc: u32 = 0x832A5F70;
    'dispatch: loop {
        match pc {
            0x832A5F70 => {
    //   block [0x832A5F70..0x832A5FA4)
	// 832A5F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5F7C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5F80: 386B9A34  addi r3, r11, -0x65cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26060;
	// 832A5F84: 4BD5D6A5  bl 0x83003628
	ctx.lr = 0x832A5F88;
	sub_83003628(ctx, base);
	// 832A5F88: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5F8C: 386B9618  addi r3, r11, -0x69e8
	ctx.r[3].s64 = ctx.r[11].s64 + -27112;
	// 832A5F90: 4BA03F91  bl 0x82ca9f20
	ctx.lr = 0x832A5F94;
	sub_82CA9F20(ctx, base);
	// 832A5F94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5FA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5FA8 size=52
    let mut pc: u32 = 0x832A5FA8;
    'dispatch: loop {
        match pc {
            0x832A5FA8 => {
    //   block [0x832A5FA8..0x832A5FDC)
	// 832A5FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5FB4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5FB8: 386B9A28  addi r3, r11, -0x65d8
	ctx.r[3].s64 = ctx.r[11].s64 + -26072;
	// 832A5FBC: 4BD5E8AD  bl 0x83004868
	ctx.lr = 0x832A5FC0;
	sub_83004868(ctx, base);
	// 832A5FC0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5FC4: 386B9640  addi r3, r11, -0x69c0
	ctx.r[3].s64 = ctx.r[11].s64 + -27072;
	// 832A5FC8: 4BA03F59  bl 0x82ca9f20
	ctx.lr = 0x832A5FCC;
	sub_82CA9F20(ctx, base);
	// 832A5FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A5FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A5FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A5FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A5FE0 size=52
    let mut pc: u32 = 0x832A5FE0;
    'dispatch: loop {
        match pc {
            0x832A5FE0 => {
    //   block [0x832A5FE0..0x832A6014)
	// 832A5FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A5FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A5FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A5FEC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A5FF0: 386B9A64  addi r3, r11, -0x659c
	ctx.r[3].s64 = ctx.r[11].s64 + -26012;
	// 832A5FF4: 4BD5D635  bl 0x83003628
	ctx.lr = 0x832A5FF8;
	sub_83003628(ctx, base);
	// 832A5FF8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A5FFC: 386B9668  addi r3, r11, -0x6998
	ctx.r[3].s64 = ctx.r[11].s64 + -27032;
	// 832A6000: 4BA03F21  bl 0x82ca9f20
	ctx.lr = 0x832A6004;
	sub_82CA9F20(ctx, base);
	// 832A6004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6018 size=52
    let mut pc: u32 = 0x832A6018;
    'dispatch: loop {
        match pc {
            0x832A6018 => {
    //   block [0x832A6018..0x832A604C)
	// 832A6018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A601C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6024: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6028: 386B9A58  addi r3, r11, -0x65a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26024;
	// 832A602C: 4BD5E83D  bl 0x83004868
	ctx.lr = 0x832A6030;
	sub_83004868(ctx, base);
	// 832A6030: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A6034: 386B9690  addi r3, r11, -0x6970
	ctx.r[3].s64 = ctx.r[11].s64 + -26992;
	// 832A6038: 4BA03EE9  bl 0x82ca9f20
	ctx.lr = 0x832A603C;
	sub_82CA9F20(ctx, base);
	// 832A603C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6050 size=52
    let mut pc: u32 = 0x832A6050;
    'dispatch: loop {
        match pc {
            0x832A6050 => {
    //   block [0x832A6050..0x832A6084)
	// 832A6050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A605C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6060: 386B9AC4  addi r3, r11, -0x653c
	ctx.r[3].s64 = ctx.r[11].s64 + -25916;
	// 832A6064: 4BD5D5C5  bl 0x83003628
	ctx.lr = 0x832A6068;
	sub_83003628(ctx, base);
	// 832A6068: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A606C: 386B96B8  addi r3, r11, -0x6948
	ctx.r[3].s64 = ctx.r[11].s64 + -26952;
	// 832A6070: 4BA03EB1  bl 0x82ca9f20
	ctx.lr = 0x832A6074;
	sub_82CA9F20(ctx, base);
	// 832A6074: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A607C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6088 size=52
    let mut pc: u32 = 0x832A6088;
    'dispatch: loop {
        match pc {
            0x832A6088 => {
    //   block [0x832A6088..0x832A60BC)
	// 832A6088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A608C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6090: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6094: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6098: 386B9AB8  addi r3, r11, -0x6548
	ctx.r[3].s64 = ctx.r[11].s64 + -25928;
	// 832A609C: 4BD5E7CD  bl 0x83004868
	ctx.lr = 0x832A60A0;
	sub_83004868(ctx, base);
	// 832A60A0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A60A4: 386B96E0  addi r3, r11, -0x6920
	ctx.r[3].s64 = ctx.r[11].s64 + -26912;
	// 832A60A8: 4BA03E79  bl 0x82ca9f20
	ctx.lr = 0x832A60AC;
	sub_82CA9F20(ctx, base);
	// 832A60AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A60B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A60B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A60B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A60C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A60C0 size=52
    let mut pc: u32 = 0x832A60C0;
    'dispatch: loop {
        match pc {
            0x832A60C0 => {
    //   block [0x832A60C0..0x832A60F4)
	// 832A60C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A60C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A60C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A60CC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A60D0: 386B9A94  addi r3, r11, -0x656c
	ctx.r[3].s64 = ctx.r[11].s64 + -25964;
	// 832A60D4: 4BD5D555  bl 0x83003628
	ctx.lr = 0x832A60D8;
	sub_83003628(ctx, base);
	// 832A60D8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A60DC: 386B9708  addi r3, r11, -0x68f8
	ctx.r[3].s64 = ctx.r[11].s64 + -26872;
	// 832A60E0: 4BA03E41  bl 0x82ca9f20
	ctx.lr = 0x832A60E4;
	sub_82CA9F20(ctx, base);
	// 832A60E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A60E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A60EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A60F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A60F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A60F8 size=52
    let mut pc: u32 = 0x832A60F8;
    'dispatch: loop {
        match pc {
            0x832A60F8 => {
    //   block [0x832A60F8..0x832A612C)
	// 832A60F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A60FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6104: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6108: 386B9A88  addi r3, r11, -0x6578
	ctx.r[3].s64 = ctx.r[11].s64 + -25976;
	// 832A610C: 4BD5E75D  bl 0x83004868
	ctx.lr = 0x832A6110;
	sub_83004868(ctx, base);
	// 832A6110: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A6114: 386B9730  addi r3, r11, -0x68d0
	ctx.r[3].s64 = ctx.r[11].s64 + -26832;
	// 832A6118: 4BA03E09  bl 0x82ca9f20
	ctx.lr = 0x832A611C;
	sub_82CA9F20(ctx, base);
	// 832A611C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6130 size=44
    let mut pc: u32 = 0x832A6130;
    'dispatch: loop {
        match pc {
            0x832A6130 => {
    //   block [0x832A6130..0x832A615C)
	// 832A6130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A613C: 4BD5EE5D  bl 0x83004f98
	ctx.lr = 0x832A6140;
	sub_83004F98(ctx, base);
	// 832A6140: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6144: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 832A6148: F86B0000  std r3, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 832A614C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6160 size=44
    let mut pc: u32 = 0x832A6160;
    'dispatch: loop {
        match pc {
            0x832A6160 => {
    //   block [0x832A6160..0x832A618C)
	// 832A6160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A616C: 4BD5EE5D  bl 0x83004fc8
	ctx.lr = 0x832A6170;
	sub_83004FC8(ctx, base);
	// 832A6170: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6174: 396B9AF0  addi r11, r11, -0x6510
	ctx.r[11].s64 = ctx.r[11].s64 + -25872;
	// 832A6178: D82B0000  stfd f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.f[1].u64 ) };
	// 832A617C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6190 size=52
    let mut pc: u32 = 0x832A6190;
    'dispatch: loop {
        match pc {
            0x832A6190 => {
    //   block [0x832A6190..0x832A61C4)
	// 832A6190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A619C: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 832A61A0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A61A4: 388BD340  addi r4, r11, -0x2cc0
	ctx.r[4].s64 = ctx.r[11].s64 + -11456;
	// 832A61A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A61AC: 386B9AFC  addi r3, r11, -0x6504
	ctx.r[3].s64 = ctx.r[11].s64 + -25860;
	// 832A61B0: 4BD5D3E9  bl 0x83003598
	ctx.lr = 0x832A61B4;
	sub_83003598(ctx, base);
	// 832A61B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A61B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A61BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A61C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A61C8 size=52
    let mut pc: u32 = 0x832A61C8;
    'dispatch: loop {
        match pc {
            0x832A61C8 => {
    //   block [0x832A61C8..0x832A61FC)
	// 832A61C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A61CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A61D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A61D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A61D8: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A61DC: 388BD3D0  addi r4, r11, -0x2c30
	ctx.r[4].s64 = ctx.r[11].s64 + -11312;
	// 832A61E0: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A61E4: 386B9B04  addi r3, r11, -0x64fc
	ctx.r[3].s64 = ctx.r[11].s64 + -25852;
	// 832A61E8: 4BD5D3B1  bl 0x83003598
	ctx.lr = 0x832A61EC;
	sub_83003598(ctx, base);
	// 832A61EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A61F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A61F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A61F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6200 size=52
    let mut pc: u32 = 0x832A6200;
    'dispatch: loop {
        match pc {
            0x832A6200 => {
    //   block [0x832A6200..0x832A6234)
	// 832A6200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A620C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A6210: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A6214: 388BD4C8  addi r4, r11, -0x2b38
	ctx.r[4].s64 = ctx.r[11].s64 + -11064;
	// 832A6218: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A621C: 386B9B0C  addi r3, r11, -0x64f4
	ctx.r[3].s64 = ctx.r[11].s64 + -25844;
	// 832A6220: 4BD5D379  bl 0x83003598
	ctx.lr = 0x832A6224;
	sub_83003598(ctx, base);
	// 832A6224: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A622C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6238 size=52
    let mut pc: u32 = 0x832A6238;
    'dispatch: loop {
        match pc {
            0x832A6238 => {
    //   block [0x832A6238..0x832A626C)
	// 832A6238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6244: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 832A6248: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 832A624C: 388BD558  addi r4, r11, -0x2aa8
	ctx.r[4].s64 = ctx.r[11].s64 + -10920;
	// 832A6250: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6254: 386B9B14  addi r3, r11, -0x64ec
	ctx.r[3].s64 = ctx.r[11].s64 + -25836;
	// 832A6258: 4BD5D341  bl 0x83003598
	ctx.lr = 0x832A625C;
	sub_83003598(ctx, base);
	// 832A625C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6270 size=52
    let mut pc: u32 = 0x832A6270;
    'dispatch: loop {
        match pc {
            0x832A6270 => {
    //   block [0x832A6270..0x832A62A4)
	// 832A6270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A627C: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6280: 386B9B1C  addi r3, r11, -0x64e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25828;
	// 832A6284: 4BD5EE6D  bl 0x830050f0
	ctx.lr = 0x832A6288;
	sub_830050F0(ctx, base);
	// 832A6288: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A628C: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 832A6290: 4BA03C91  bl 0x82ca9f20
	ctx.lr = 0x832A6294;
	sub_82CA9F20(ctx, base);
	// 832A6294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A629C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A62A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A62A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A62A8 size=52
    let mut pc: u32 = 0x832A62A8;
    'dispatch: loop {
        match pc {
            0x832A62A8 => {
    //   block [0x832A62A8..0x832A62DC)
	// 832A62A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A62AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A62B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A62B4: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A62B8: 386B9B44  addi r3, r11, -0x64bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25788;
	// 832A62BC: 4BD5D36D  bl 0x83003628
	ctx.lr = 0x832A62C0;
	sub_83003628(ctx, base);
	// 832A62C0: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A62C4: 386B9780  addi r3, r11, -0x6880
	ctx.r[3].s64 = ctx.r[11].s64 + -26752;
	// 832A62C8: 4BA03C59  bl 0x82ca9f20
	ctx.lr = 0x832A62CC;
	sub_82CA9F20(ctx, base);
	// 832A62CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A62D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A62D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A62D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A62E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A62E0 size=52
    let mut pc: u32 = 0x832A62E0;
    'dispatch: loop {
        match pc {
            0x832A62E0 => {
    //   block [0x832A62E0..0x832A6314)
	// 832A62E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A62E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A62E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A62EC: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A62F0: 386B9B68  addi r3, r11, -0x6498
	ctx.r[3].s64 = ctx.r[11].s64 + -25752;
	// 832A62F4: 4BD5FC4D  bl 0x83005f40
	ctx.lr = 0x832A62F8;
	sub_83005F40(ctx, base);
	// 832A62F8: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A62FC: 386B97A8  addi r3, r11, -0x6858
	ctx.r[3].s64 = ctx.r[11].s64 + -26712;
	// 832A6300: 4BA03C21  bl 0x82ca9f20
	ctx.lr = 0x832A6304;
	sub_82CA9F20(ctx, base);
	// 832A6304: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A630C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6318 size=112
    let mut pc: u32 = 0x832A6318;
    'dispatch: loop {
        match pc {
            0x832A6318 => {
    //   block [0x832A6318..0x832A6388)
	// 832A6318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A631C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6324: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6328: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A632C: 38AAB2C4  addi r5, r10, -0x4d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -19772;
	// 832A6330: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6334: 390B86C4  addi r8, r11, -0x793c
	ctx.r[8].s64 = ctx.r[11].s64 + -31036;
	// 832A6338: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 832A633C: 388A86F4  addi r4, r10, -0x790c
	ctx.r[4].s64 = ctx.r[10].s64 + -30988;
	// 832A6340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6344: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A634C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6350: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 832A6354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A6358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A635C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A636C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6374: 4BAAF8CD  bl 0x82d55c40
	ctx.lr = 0x832A6378;
	sub_82D55C40(ctx, base);
	// 832A6378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A637C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6388 size=108
    let mut pc: u32 = 0x832A6388;
    'dispatch: loop {
        match pc {
            0x832A6388 => {
    //   block [0x832A6388..0x832A63F4)
	// 832A6388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A638C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6394: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A639C: 38EB87A0  addi r7, r11, -0x7860
	ctx.r[7].s64 = ctx.r[11].s64 + -30816;
	// 832A63A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 832A63A4: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A63A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A63AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A63B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A63B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A63B8: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 832A63BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A63C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A63C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A63C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A63CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A63D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A63D4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A63D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A63DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A63E0: 4BAAF861  bl 0x82d55c40
	ctx.lr = 0x832A63E4;
	sub_82D55C40(ctx, base);
	// 832A63E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A63E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A63EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A63F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A63F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A63F8 size=108
    let mut pc: u32 = 0x832A63F8;
    'dispatch: loop {
        match pc {
            0x832A63F8 => {
    //   block [0x832A63F8..0x832A6464)
	// 832A63F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A63FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6404: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A640C: 38EB8848  addi r7, r11, -0x77b8
	ctx.r[7].s64 = ctx.r[11].s64 + -30648;
	// 832A6410: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 832A6414: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A6418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A641C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6428: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 832A642C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A643C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6444: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A6448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A644C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6450: 4BAAF7F1  bl 0x82d55c40
	ctx.lr = 0x832A6454;
	sub_82D55C40(ctx, base);
	// 832A6454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A645C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6468 size=244
    let mut pc: u32 = 0x832A6468;
    'dispatch: loop {
        match pc {
            0x832A6468 => {
    //   block [0x832A6468..0x832A655C)
	// 832A6468: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A646C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6470: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 832A6474: 396B1640  addi r11, r11, 0x1640
	ctx.r[11].s64 = ctx.r[11].s64 + 5696;
	// 832A6478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A647C: 81291630  lwz r9, 0x1630(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5680 as u32) ) } as u64;
	// 832A6480: 390841D8  addi r8, r8, 0x41d8
	ctx.r[8].s64 = ctx.r[8].s64 + 16856;
	// 832A6484: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 832A6488: 3CC08202  lis r6, -0x7dfe
	ctx.r[6].s64 = -2113798144;
	// 832A648C: 38E741CC  addi r7, r7, 0x41cc
	ctx.r[7].s64 = ctx.r[7].s64 + 16844;
	// 832A6490: 38C63BE0  addi r6, r6, 0x3be0
	ctx.r[6].s64 = ctx.r[6].s64 + 15328;
	// 832A6494: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 832A6498: 3CA08202  lis r5, -0x7dfe
	ctx.r[5].s64 = -2113798144;
	// 832A649C: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832A64A0: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 832A64A4: 38A541C4  addi r5, r5, 0x41c4
	ctx.r[5].s64 = ctx.r[5].s64 + 16836;
	// 832A64A8: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 832A64AC: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 832A64B0: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 832A64B4: 992B006C  stb r9, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u8 ) };
	// 832A64B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A64BC: 992B006D  stb r9, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[9].u8 ) };
	// 832A64C0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 832A64C4: B14B006E  sth r10, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[10].u16 ) };
	// 832A64C8: B14B0070  sth r10, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 832A64CC: B12B0072  sth r9, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[9].u16 ) };
	// 832A64D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A64D4: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 832A64D8: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 832A64DC: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 832A64E0: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 832A64E4: 992B0084  stb r9, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 832A64E8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 832A64EC: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 832A64F0: B14B0086  sth r10, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 832A64F4: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 832A64F8: B12B008A  sth r9, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[9].u16 ) };
	// 832A64FC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A6500: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 832A6504: 90CB0090  stw r6, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
	// 832A6508: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 832A650C: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832A6510: 992B009C  stb r9, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u8 ) };
	// 832A6514: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 832A6518: 994B009D  stb r10, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[10].u8 ) };
	// 832A651C: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 832A6520: B14B00A0  sth r10, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 832A6524: B12B00A2  sth r9, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[9].u16 ) };
	// 832A6528: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A652C: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 832A6530: 90AB00A8  stw r5, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[5].u32 ) };
	// 832A6534: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 832A6538: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 832A653C: 992B00B4  stb r9, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u8 ) };
	// 832A6540: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 832A6544: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 832A6548: B14B00B6  sth r10, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[10].u16 ) };
	// 832A654C: B14B00B8  sth r10, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u16 ) };
	// 832A6550: B12B00BA  sth r9, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[9].u16 ) };
	// 832A6554: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 832A6558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6560 size=112
    let mut pc: u32 = 0x832A6560;
    'dispatch: loop {
        match pc {
            0x832A6560 => {
    //   block [0x832A6560..0x832A65D0)
	// 832A6560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A656C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6570: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6574: 392A8A60  addi r9, r10, -0x75a0
	ctx.r[9].s64 = ctx.r[10].s64 + -30112;
	// 832A6578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A657C: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 832A6580: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 832A6584: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 832A6588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A658C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6594: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A6598: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 832A659C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A65A0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A65A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A65A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A65AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A65B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A65B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A65B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A65BC: 4BAAF685  bl 0x82d55c40
	ctx.lr = 0x832A65C0;
	sub_82D55C40(ctx, base);
	// 832A65C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A65C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A65C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A65CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A65D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A65D0 size=108
    let mut pc: u32 = 0x832A65D0;
    'dispatch: loop {
        match pc {
            0x832A65D0 => {
    //   block [0x832A65D0..0x832A663C)
	// 832A65D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A65D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A65D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A65DC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A65E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A65E4: 38EB8A9C  addi r7, r11, -0x7564
	ctx.r[7].s64 = ctx.r[11].s64 + -30052;
	// 832A65E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A65EC: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832A65F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A65F4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A65F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A65FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6600: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 832A6604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A660C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A661C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6628: 4BAAF619  bl 0x82d55c40
	ctx.lr = 0x832A662C;
	sub_82D55C40(ctx, base);
	// 832A662C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6640 size=108
    let mut pc: u32 = 0x832A6640;
    'dispatch: loop {
        match pc {
            0x832A6640 => {
    //   block [0x832A6640..0x832A66AC)
	// 832A6640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A664C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6654: 38EB8ACC  addi r7, r11, -0x7534
	ctx.r[7].s64 = ctx.r[11].s64 + -30004;
	// 832A6658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A665C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 832A6660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6664: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A666C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6670: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 832A6674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A667C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A668C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A6690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6698: 4BAAF5A9  bl 0x82d55c40
	ctx.lr = 0x832A669C;
	sub_82D55C40(ctx, base);
	// 832A669C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A66A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A66A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A66A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A66B0 size=108
    let mut pc: u32 = 0x832A66B0;
    'dispatch: loop {
        match pc {
            0x832A66B0 => {
    //   block [0x832A66B0..0x832A671C)
	// 832A66B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A66B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A66B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A66BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A66C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A66C4: 38EB8B10  addi r7, r11, -0x74f0
	ctx.r[7].s64 = ctx.r[11].s64 + -29936;
	// 832A66C8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 832A66CC: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A66D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A66D4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A66D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A66DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A66E0: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 832A66E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A66E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A66EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A66F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A66F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A66F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A66FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A6700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6708: 4BAAF539  bl 0x82d55c40
	ctx.lr = 0x832A670C;
	sub_82D55C40(ctx, base);
	// 832A670C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6720 size=108
    let mut pc: u32 = 0x832A6720;
    'dispatch: loop {
        match pc {
            0x832A6720 => {
    //   block [0x832A6720..0x832A678C)
	// 832A6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A672C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6734: 38EB8C00  addi r7, r11, -0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + -29696;
	// 832A6738: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 832A673C: 388A3CD8  addi r4, r10, 0x3cd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15576;
	// 832A6740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6744: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6750: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 832A6754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A675C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A676C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A6770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6778: 4BAAF4C9  bl 0x82d55c40
	ctx.lr = 0x832A677C;
	sub_82D55C40(ctx, base);
	// 832A677C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6790 size=292
    let mut pc: u32 = 0x832A6790;
    'dispatch: loop {
        match pc {
            0x832A6790 => {
    //   block [0x832A6790..0x832A68B4)
	// 832A6790: 3D208330  lis r9, -0x7cd0
	ctx.r[9].s64 = -2094006272;
	// 832A6794: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6798: 3D008202  lis r8, -0x7dfe
	ctx.r[8].s64 = -2113798144;
	// 832A679C: 396B1710  addi r11, r11, 0x1710
	ctx.r[11].s64 = ctx.r[11].s64 + 5904;
	// 832A67A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A67A4: 81291700  lwz r9, 0x1700(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5888 as u32) ) } as u64;
	// 832A67A8: 390841D8  addi r8, r8, 0x41d8
	ctx.r[8].s64 = ctx.r[8].s64 + 16856;
	// 832A67AC: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 832A67B0: 3CC08202  lis r6, -0x7dfe
	ctx.r[6].s64 = -2113798144;
	// 832A67B4: 38E741CC  addi r7, r7, 0x41cc
	ctx.r[7].s64 = ctx.r[7].s64 + 16844;
	// 832A67B8: 38C63BE0  addi r6, r6, 0x3be0
	ctx.r[6].s64 = ctx.r[6].s64 + 15328;
	// 832A67BC: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 832A67C0: 3CA08202  lis r5, -0x7dfe
	ctx.r[5].s64 = -2113798144;
	// 832A67C4: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 832A67C8: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 832A67CC: 38A541C4  addi r5, r5, 0x41c4
	ctx.r[5].s64 = ctx.r[5].s64 + 16836;
	// 832A67D0: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 832A67D4: 3C808208  lis r4, -0x7df8
	ctx.r[4].s64 = -2113404928;
	// 832A67D8: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 832A67DC: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 832A67E0: 38848AFC  addi r4, r4, -0x7504
	ctx.r[4].s64 = ctx.r[4].s64 + -29956;
	// 832A67E4: 992B006C  stb r9, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u8 ) };
	// 832A67E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A67EC: 992B006D  stb r9, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[9].u8 ) };
	// 832A67F0: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 832A67F4: B14B006E  sth r10, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[10].u16 ) };
	// 832A67F8: B14B0070  sth r10, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 832A67FC: B12B0072  sth r9, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[9].u16 ) };
	// 832A6800: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A6804: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 832A6808: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 832A680C: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 832A6810: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 832A6814: 992B0084  stb r9, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 832A6818: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 832A681C: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 832A6820: B14B0086  sth r10, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 832A6824: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 832A6828: B12B008A  sth r9, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[9].u16 ) };
	// 832A682C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A6830: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 832A6834: 90CB0090  stw r6, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
	// 832A6838: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 832A683C: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 832A6840: 992B009C  stb r9, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u8 ) };
	// 832A6844: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 832A6848: 994B009D  stb r10, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[10].u8 ) };
	// 832A684C: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 832A6850: B14B00A0  sth r10, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 832A6854: B12B00A2  sth r9, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[9].u16 ) };
	// 832A6858: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 832A685C: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 832A6860: 90AB00A8  stw r5, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[5].u32 ) };
	// 832A6864: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 832A6868: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 832A686C: 992B00B4  stb r9, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u8 ) };
	// 832A6870: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 832A6874: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 832A6878: B14B00B6  sth r10, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[10].u16 ) };
	// 832A687C: B14B00B8  sth r10, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u16 ) };
	// 832A6880: B12B00BA  sth r9, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[9].u16 ) };
	// 832A6884: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 832A6888: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 832A688C: 908B00C0  stw r4, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[4].u32 ) };
	// 832A6890: 914B00C4  stw r10, 0xc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 832A6894: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 832A6898: 992B00CC  stb r9, 0xcc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), ctx.r[9].u8 ) };
	// 832A689C: 994B00CD  stb r10, 0xcd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(205 as u32), ctx.r[10].u8 ) };
	// 832A68A0: B14B00CE  sth r10, 0xce(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(206 as u32), ctx.r[10].u16 ) };
	// 832A68A4: B14B00D0  sth r10, 0xd0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[10].u16 ) };
	// 832A68A8: B12B00D2  sth r9, 0xd2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(210 as u32), ctx.r[9].u16 ) };
	// 832A68AC: 914B00D4  stw r10, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 832A68B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A68B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A68B8 size=112
    let mut pc: u32 = 0x832A68B8;
    'dispatch: loop {
        match pc {
            0x832A68B8 => {
    //   block [0x832A68B8..0x832A6928)
	// 832A68B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A68BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A68C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A68C4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A68C8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A68CC: 392A8E60  addi r9, r10, -0x71a0
	ctx.r[9].s64 = ctx.r[10].s64 + -29088;
	// 832A68D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A68D4: 390B1710  addi r8, r11, 0x1710
	ctx.r[8].s64 = ctx.r[11].s64 + 5904;
	// 832A68D8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A68DC: 388A41B4  addi r4, r10, 0x41b4
	ctx.r[4].s64 = ctx.r[10].s64 + 16820;
	// 832A68E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A68E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A68E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A68EC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 832A68F0: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 832A68F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A68F8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A68FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A690C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6914: 4BAAF32D  bl 0x82d55c40
	ctx.lr = 0x832A6918;
	sub_82D55C40(ctx, base);
	// 832A6918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A691C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6928 size=108
    let mut pc: u32 = 0x832A6928;
    'dispatch: loop {
        match pc {
            0x832A6928 => {
    //   block [0x832A6928..0x832A6994)
	// 832A6928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A692C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6934: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A693C: 38EB8E9C  addi r7, r11, -0x7164
	ctx.r[7].s64 = ctx.r[11].s64 + -29028;
	// 832A6940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6944: 388A3D48  addi r4, r10, 0x3d48
	ctx.r[4].s64 = ctx.r[10].s64 + 15688;
	// 832A6948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A694C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6958: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 832A695C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6974: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A697C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6980: 4BAAF2C1  bl 0x82d55c40
	ctx.lr = 0x832A6984;
	sub_82D55C40(ctx, base);
	// 832A6984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A698C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6998 size=108
    let mut pc: u32 = 0x832A6998;
    'dispatch: loop {
        match pc {
            0x832A6998 => {
    //   block [0x832A6998..0x832A6A04)
	// 832A6998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A699C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A69A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A69A4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A69A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A69AC: 38EB8ED0  addi r7, r11, -0x7130
	ctx.r[7].s64 = ctx.r[11].s64 + -28976;
	// 832A69B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A69B4: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 832A69B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A69BC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A69C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A69C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A69C8: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 832A69CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A69D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A69D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A69D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A69DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A69E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A69E4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 832A69E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A69EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A69F0: 4BAAF251  bl 0x82d55c40
	ctx.lr = 0x832A69F4;
	sub_82D55C40(ctx, base);
	// 832A69F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A69F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A69FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6A08 size=28
    let mut pc: u32 = 0x832A6A08;
    'dispatch: loop {
        match pc {
            0x832A6A08 => {
    //   block [0x832A6A08..0x832A6A24)
	// 832A6A08: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6A0C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6A10: 396B17E8  addi r11, r11, 0x17e8
	ctx.r[11].s64 = ctx.r[11].s64 + 6120;
	// 832A6A14: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6A18: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6A1C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6A28 size=76
    let mut pc: u32 = 0x832A6A28;
    'dispatch: loop {
        match pc {
            0x832A6A28 => {
    //   block [0x832A6A28..0x832A6A74)
	// 832A6A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A6A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6A38: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A6A3C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A6A40: 3BEBB0A0  addi r31, r11, -0x4f60
	ctx.r[31].s64 = ctx.r[11].s64 + -20320;
	// 832A6A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A6A48: 4801324D  bl 0x832b9c94
	ctx.lr = 0x832A6A4C;
	// extern call 0x832B9C94 → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 832A6A4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 832A6A50: 3D40832C  lis r10, -0x7cd4
	ctx.r[10].s64 = -2094268416;
	// 832A6A54: 386A97D0  addi r3, r10, -0x6830
	ctx.r[3].s64 = ctx.r[10].s64 + -26672;
	// 832A6A58: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 832A6A5C: 4BA034C5  bl 0x82ca9f20
	ctx.lr = 0x832A6A60;
	sub_82CA9F20(ctx, base);
	// 832A6A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A6A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6A6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A6A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6A78 size=108
    let mut pc: u32 = 0x832A6A78;
    'dispatch: loop {
        match pc {
            0x832A6A78 => {
    //   block [0x832A6A78..0x832A6AE4)
	// 832A6A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6A84: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6A88: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6A8C: 38EB8FB8  addi r7, r11, -0x7048
	ctx.r[7].s64 = ctx.r[11].s64 + -28744;
	// 832A6A90: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 832A6A94: 388A9090  addi r4, r10, -0x6f70
	ctx.r[4].s64 = ctx.r[10].s64 + -28528;
	// 832A6A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6A9C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6AA8: 386AB0DC  addi r3, r10, -0x4f24
	ctx.r[3].s64 = ctx.r[10].s64 + -20260;
	// 832A6AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6AC4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 832A6AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6AD0: 4BAAF171  bl 0x82d55c40
	ctx.lr = 0x832A6AD4;
	sub_82D55C40(ctx, base);
	// 832A6AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6AE8 size=108
    let mut pc: u32 = 0x832A6AE8;
    'dispatch: loop {
        match pc {
            0x832A6AE8 => {
    //   block [0x832A6AE8..0x832A6B54)
	// 832A6AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6AF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6AF8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6AFC: 38EB90B0  addi r7, r11, -0x6f50
	ctx.r[7].s64 = ctx.r[11].s64 + -28496;
	// 832A6B00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6B04: 388A9170  addi r4, r10, -0x6e90
	ctx.r[4].s64 = ctx.r[10].s64 + -28304;
	// 832A6B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6B0C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6B18: 386AB10C  addi r3, r10, -0x4ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -20212;
	// 832A6B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6B34: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6B40: 4BAAF101  bl 0x82d55c40
	ctx.lr = 0x832A6B44;
	sub_82D55C40(ctx, base);
	// 832A6B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6B58 size=108
    let mut pc: u32 = 0x832A6B58;
    'dispatch: loop {
        match pc {
            0x832A6B58 => {
    //   block [0x832A6B58..0x832A6BC4)
	// 832A6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6B64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6B68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6B6C: 38EB90E0  addi r7, r11, -0x6f20
	ctx.r[7].s64 = ctx.r[11].s64 + -28448;
	// 832A6B70: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 832A6B74: 388A9178  addi r4, r10, -0x6e88
	ctx.r[4].s64 = ctx.r[10].s64 + -28296;
	// 832A6B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6B7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6B88: 386AB13C  addi r3, r10, -0x4ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -20164;
	// 832A6B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6BA4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6BB0: 4BAAF091  bl 0x82d55c40
	ctx.lr = 0x832A6BB4;
	sub_82D55C40(ctx, base);
	// 832A6BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6BC8 size=112
    let mut pc: u32 = 0x832A6BC8;
    'dispatch: loop {
        match pc {
            0x832A6BC8 => {
    //   block [0x832A6BC8..0x832A6C38)
	// 832A6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6BD4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6BD8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6BDC: 392A9248  addi r9, r10, -0x6db8
	ctx.r[9].s64 = ctx.r[10].s64 + -28088;
	// 832A6BE0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6BE4: 390B9270  addi r8, r11, -0x6d90
	ctx.r[8].s64 = ctx.r[11].s64 + -28048;
	// 832A6BE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 832A6BEC: 388A92B8  addi r4, r10, -0x6d48
	ctx.r[4].s64 = ctx.r[10].s64 + -27976;
	// 832A6BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6BF4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6BFC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 832A6C00: 386AB16C  addi r3, r10, -0x4e94
	ctx.r[3].s64 = ctx.r[10].s64 + -20116;
	// 832A6C04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A6C08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832A6C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6C24: 4BAAF01D  bl 0x82d55c40
	ctx.lr = 0x832A6C28;
	sub_82D55C40(ctx, base);
	// 832A6C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6C38 size=28
    let mut pc: u32 = 0x832A6C38;
    'dispatch: loop {
        match pc {
            0x832A6C38 => {
    //   block [0x832A6C38..0x832A6C54)
	// 832A6C38: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6C3C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6C40: 396B18BC  addi r11, r11, 0x18bc
	ctx.r[11].s64 = ctx.r[11].s64 + 6332;
	// 832A6C44: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6C48: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6C4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6C58 size=108
    let mut pc: u32 = 0x832A6C58;
    'dispatch: loop {
        match pc {
            0x832A6C58 => {
    //   block [0x832A6C58..0x832A6CC4)
	// 832A6C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6C64: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6C68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6C6C: 38EB93D4  addi r7, r11, -0x6c2c
	ctx.r[7].s64 = ctx.r[11].s64 + -27692;
	// 832A6C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6C74: 388A9448  addi r4, r10, -0x6bb8
	ctx.r[4].s64 = ctx.r[10].s64 + -27576;
	// 832A6C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6C7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6C88: 386AB1A4  addi r3, r10, -0x4e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20060;
	// 832A6C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6CA4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6CB0: 4BAAEF91  bl 0x82d55c40
	ctx.lr = 0x832A6CB4;
	sub_82D55C40(ctx, base);
	// 832A6CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6CC8 size=108
    let mut pc: u32 = 0x832A6CC8;
    'dispatch: loop {
        match pc {
            0x832A6CC8 => {
    //   block [0x832A6CC8..0x832A6D34)
	// 832A6CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6CD4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6CD8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6CDC: 38EB9404  addi r7, r11, -0x6bfc
	ctx.r[7].s64 = ctx.r[11].s64 + -27644;
	// 832A6CE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A6CE4: 388A946C  addi r4, r10, -0x6b94
	ctx.r[4].s64 = ctx.r[10].s64 + -27540;
	// 832A6CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6CEC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6CF8: 386AB1D4  addi r3, r10, -0x4e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -20012;
	// 832A6CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6D14: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 832A6D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6D20: 4BAAEF21  bl 0x82d55c40
	ctx.lr = 0x832A6D24;
	sub_82D55C40(ctx, base);
	// 832A6D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6D38 size=24
    let mut pc: u32 = 0x832A6D38;
    'dispatch: loop {
        match pc {
            0x832A6D38 => {
    //   block [0x832A6D38..0x832A6D50)
	// 832A6D38: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6D3C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A6D40: 394A1900  addi r10, r10, 0x1900
	ctx.r[10].s64 = ctx.r[10].s64 + 6400;
	// 832A6D44: 816B18C8  lwz r11, 0x18c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6344 as u32) ) } as u64;
	// 832A6D48: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 832A6D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6D50 size=112
    let mut pc: u32 = 0x832A6D50;
    'dispatch: loop {
        match pc {
            0x832A6D50 => {
    //   block [0x832A6D50..0x832A6DC0)
	// 832A6D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6D5C: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6D60: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6D64: 392A9434  addi r9, r10, -0x6bcc
	ctx.r[9].s64 = ctx.r[10].s64 + -27596;
	// 832A6D68: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6D6C: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 832A6D70: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 832A6D74: 388A9488  addi r4, r10, -0x6b78
	ctx.r[4].s64 = ctx.r[10].s64 + -27512;
	// 832A6D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6D7C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A6D84: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A6D88: 386AB204  addi r3, r10, -0x4dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19964;
	// 832A6D8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832A6D90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832A6D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6DAC: 4BAAEE95  bl 0x82d55c40
	ctx.lr = 0x832A6DB0;
	sub_82D55C40(ctx, base);
	// 832A6DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6DC0 size=108
    let mut pc: u32 = 0x832A6DC0;
    'dispatch: loop {
        match pc {
            0x832A6DC0 => {
    //   block [0x832A6DC0..0x832A6E2C)
	// 832A6DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6DCC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6DD0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6DD4: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 832A6DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832A6DDC: 388A97A8  addi r4, r10, -0x6858
	ctx.r[4].s64 = ctx.r[10].s64 + -26712;
	// 832A6DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6DE4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6DF0: 386AB234  addi r3, r10, -0x4dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -19916;
	// 832A6DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6E0C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 832A6E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6E18: 4BAAEE29  bl 0x82d55c40
	ctx.lr = 0x832A6E1C;
	sub_82D55C40(ctx, base);
	// 832A6E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6E30 size=108
    let mut pc: u32 = 0x832A6E30;
    'dispatch: loop {
        match pc {
            0x832A6E30 => {
    //   block [0x832A6E30..0x832A6E9C)
	// 832A6E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6E3C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6E40: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6E44: 38EB9828  addi r7, r11, -0x67d8
	ctx.r[7].s64 = ctx.r[11].s64 + -26584;
	// 832A6E48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A6E4C: 388A98E8  addi r4, r10, -0x6718
	ctx.r[4].s64 = ctx.r[10].s64 + -26392;
	// 832A6E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6E54: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6E60: 386AB264  addi r3, r10, -0x4d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19868;
	// 832A6E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6E7C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6E88: 4BAAEDB9  bl 0x82d55c40
	ctx.lr = 0x832A6E8C;
	sub_82D55C40(ctx, base);
	// 832A6E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6EA0 size=108
    let mut pc: u32 = 0x832A6EA0;
    'dispatch: loop {
        match pc {
            0x832A6EA0 => {
    //   block [0x832A6EA0..0x832A6F0C)
	// 832A6EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6EAC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6EB0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6EB4: 38EB9888  addi r7, r11, -0x6778
	ctx.r[7].s64 = ctx.r[11].s64 + -26488;
	// 832A6EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A6EBC: 388A9900  addi r4, r10, -0x6700
	ctx.r[4].s64 = ctx.r[10].s64 + -26368;
	// 832A6EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6EC4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6ED0: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 832A6ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6EEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A6EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6EF8: 4BAAED49  bl 0x82d55c40
	ctx.lr = 0x832A6EFC;
	sub_82D55C40(ctx, base);
	// 832A6EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6F10 size=96
    let mut pc: u32 = 0x832A6F10;
    'dispatch: loop {
        match pc {
            0x832A6F10 => {
    //   block [0x832A6F10..0x832A6F70)
	// 832A6F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6F1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 832A6F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6F24: 388A39E4  addi r4, r10, 0x39e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14820;
	// 832A6F28: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6F30: 386AB2C4  addi r3, r10, -0x4d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19772;
	// 832A6F34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6F3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A6F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6F4C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 832A6F50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 832A6F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6F58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A6F5C: 4BAAECE5  bl 0x82d55c40
	ctx.lr = 0x832A6F60;
	sub_82D55C40(ctx, base);
	// 832A6F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A6F70 size=108
    let mut pc: u32 = 0x832A6F70;
    'dispatch: loop {
        match pc {
            0x832A6F70 => {
    //   block [0x832A6F70..0x832A6FDC)
	// 832A6F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A6F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A6F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A6F7C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A6F80: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A6F84: 38EB9968  addi r7, r11, -0x6698
	ctx.r[7].s64 = ctx.r[11].s64 + -26264;
	// 832A6F88: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 832A6F8C: 388A99E0  addi r4, r10, -0x6620
	ctx.r[4].s64 = ctx.r[10].s64 + -26144;
	// 832A6F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A6F94: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A6F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A6F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A6FA0: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 832A6FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A6FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A6FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A6FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A6FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A6FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A6FBC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A6FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A6FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A6FC8: 4BAAEC79  bl 0x82d55c40
	ctx.lr = 0x832A6FCC;
	sub_82D55C40(ctx, base);
	// 832A6FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A6FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A6FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A6FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A6FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A6FE0 size=28
    let mut pc: u32 = 0x832A6FE0;
    'dispatch: loop {
        match pc {
            0x832A6FE0 => {
    //   block [0x832A6FE0..0x832A6FFC)
	// 832A6FE0: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 832A6FE4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A6FE8: 396B1C24  addi r11, r11, 0x1c24
	ctx.r[11].s64 = ctx.r[11].s64 + 7204;
	// 832A6FEC: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 832A6FF0: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 832A6FF4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 832A6FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7000 size=108
    let mut pc: u32 = 0x832A7000;
    'dispatch: loop {
        match pc {
            0x832A7000 => {
    //   block [0x832A7000..0x832A706C)
	// 832A7000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A700C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A7010: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7014: 38EBA0D0  addi r7, r11, -0x5f30
	ctx.r[7].s64 = ctx.r[11].s64 + -24368;
	// 832A7018: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 832A701C: 388AA1D8  addi r4, r10, -0x5e28
	ctx.r[4].s64 = ctx.r[10].s64 + -24104;
	// 832A7020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7024: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A702C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7030: 386AB324  addi r3, r10, -0x4cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19676;
	// 832A7034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A7038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A703C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A7040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A7044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A7048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A704C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A7050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A7054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A7058: 4BAAEBE9  bl 0x82d55c40
	ctx.lr = 0x832A705C;
	sub_82D55C40(ctx, base);
	// 832A705C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7070 size=108
    let mut pc: u32 = 0x832A7070;
    'dispatch: loop {
        match pc {
            0x832A7070 => {
    //   block [0x832A7070..0x832A70DC)
	// 832A7070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A707C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A7080: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7084: 38EBA280  addi r7, r11, -0x5d80
	ctx.r[7].s64 = ctx.r[11].s64 + -23936;
	// 832A7088: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 832A708C: 388AA358  addi r4, r10, -0x5ca8
	ctx.r[4].s64 = ctx.r[10].s64 + -23720;
	// 832A7090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7094: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A709C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A70A0: 386AB354  addi r3, r10, -0x4cac
	ctx.r[3].s64 = ctx.r[10].s64 + -19628;
	// 832A70A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A70A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A70AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A70B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A70B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A70B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A70BC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 832A70C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A70C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A70C8: 4BAAEB79  bl 0x82d55c40
	ctx.lr = 0x832A70CC;
	sub_82D55C40(ctx, base);
	// 832A70CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A70D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A70D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A70D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A70E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A70E0 size=68
    let mut pc: u32 = 0x832A70E0;
    'dispatch: loop {
        match pc {
            0x832A70E0 => {
    //   block [0x832A70E0..0x832A7124)
	// 832A70E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A70E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A70E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A70EC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832A70F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A70F4: 388BA674  addi r4, r11, -0x598c
	ctx.r[4].s64 = ctx.r[11].s64 + -22924;
	// 832A70F8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A70FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A7100: 386BB38C  addi r3, r11, -0x4c74
	ctx.r[3].s64 = ctx.r[11].s64 + -19572;
	// 832A7104: 4BBE29D5  bl 0x82e89ad8
	ctx.lr = 0x832A7108;
	sub_82E89AD8(ctx, base);
	// 832A7108: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A710C: 386B9830  addi r3, r11, -0x67d0
	ctx.r[3].s64 = ctx.r[11].s64 + -26576;
	// 832A7110: 4BA02E11  bl 0x82ca9f20
	ctx.lr = 0x832A7114;
	sub_82CA9F20(ctx, base);
	// 832A7114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A711C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7128 size=12
    let mut pc: u32 = 0x832A7128;
    'dispatch: loop {
        match pc {
            0x832A7128 => {
    //   block [0x832A7128..0x832A7134)
	// 832A7128: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A712C: 386B97E0  addi r3, r11, -0x6820
	ctx.r[3].s64 = ctx.r[11].s64 + -26656;
	// 832A7130: 4BA02DF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7138 size=80
    let mut pc: u32 = 0x832A7138;
    'dispatch: loop {
        match pc {
            0x832A7138 => {
    //   block [0x832A7138..0x832A7188)
	// 832A7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7144: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832A7148: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 832A714C: 388AA6AC  addi r4, r10, -0x5954
	ctx.r[4].s64 = ctx.r[10].s64 + -22868;
	// 832A7150: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A7154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A7158: 386ABA88  addi r3, r10, -0x4578
	ctx.r[3].s64 = ctx.r[10].s64 + -17784;
	// 832A715C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A7160: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832A7164: 38ABF7D8  addi r5, r11, -0x828
	ctx.r[5].s64 = ctx.r[11].s64 + -2088;
	// 832A7168: 4BBDE659  bl 0x82e857c0
	ctx.lr = 0x832A716C;
	sub_82E857C0(ctx, base);
	// 832A716C: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A7170: 386B98B0  addi r3, r11, -0x6750
	ctx.r[3].s64 = ctx.r[11].s64 + -26448;
	// 832A7174: 4BA02DAD  bl 0x82ca9f20
	ctx.lr = 0x832A7178;
	sub_82CA9F20(ctx, base);
	// 832A7178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A717C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7188 size=12
    let mut pc: u32 = 0x832A7188;
    'dispatch: loop {
        match pc {
            0x832A7188 => {
    //   block [0x832A7188..0x832A7194)
	// 832A7188: 3D60832C  lis r11, -0x7cd4
	ctx.r[11].s64 = -2094268416;
	// 832A718C: 386B98C8  addi r3, r11, -0x6738
	ctx.r[3].s64 = ctx.r[11].s64 + -26424;
	// 832A7190: 4BA02D90  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7198 size=16
    let mut pc: u32 = 0x832A7198;
    'dispatch: loop {
        match pc {
            0x832A7198 => {
    //   block [0x832A7198..0x832A71A8)
	// 832A7198: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A719C: 38800072  li r4, 0x72
	ctx.r[4].s64 = 114;
	// 832A71A0: 386BC344  addi r3, r11, -0x3cbc
	ctx.r[3].s64 = ctx.r[11].s64 + -15548;
	// 832A71A4: 4BF29FDC  b 0x831d1180
	sub_831D1180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A71A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A71A8 size=20
    let mut pc: u32 = 0x832A71A8;
    'dispatch: loop {
        match pc {
            0x832A71A8 => {
    //   block [0x832A71A8..0x832A71BC)
	// 832A71A8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 832A71AC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 832A71B0: 388BC344  addi r4, r11, -0x3cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -15548;
	// 832A71B4: 386AC370  addi r3, r10, -0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + -15504;
	// 832A71B8: 4BF2AC00  b 0x831d1db8
	crate::recompiler::externs::call(ctx, base, 0x831D1DB8);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A71C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A71C0 size=12
    let mut pc: u32 = 0x832A71C0;
    'dispatch: loop {
        match pc {
            0x832A71C0 => {
    //   block [0x832A71C0..0x832A71CC)
	// 832A71C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832A71C4: 386B1CB8  addi r3, r11, 0x1cb8
	ctx.r[3].s64 = ctx.r[11].s64 + 7352;
	// 832A71C8: 4B048EB8  b 0x822f0080
	sub_822F0080(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A71D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A71D0 size=12
    let mut pc: u32 = 0x832A71D0;
    'dispatch: loop {
        match pc {
            0x832A71D0 => {
    //   block [0x832A71D0..0x832A71DC)
	// 832A71D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A71D4: 386B72AC  addi r3, r11, 0x72ac
	ctx.r[3].s64 = ctx.r[11].s64 + 29356;
	// 832A71D8: 4AF6DC00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A71E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A71E0 size=12
    let mut pc: u32 = 0x832A71E0;
    'dispatch: loop {
        match pc {
            0x832A71E0 => {
    //   block [0x832A71E0..0x832A71EC)
	// 832A71E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A71E4: 386B72B0  addi r3, r11, 0x72b0
	ctx.r[3].s64 = ctx.r[11].s64 + 29360;
	// 832A71E8: 4B049450  b 0x822f0638
	sub_822F0638(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A71F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A71F0 size=12
    let mut pc: u32 = 0x832A71F0;
    'dispatch: loop {
        match pc {
            0x832A71F0 => {
    //   block [0x832A71F0..0x832A71FC)
	// 832A71F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A71F4: 386B72BC  addi r3, r11, 0x72bc
	ctx.r[3].s64 = ctx.r[11].s64 + 29372;
	// 832A71F8: 4AF6DBE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7200 size=12
    let mut pc: u32 = 0x832A7200;
    'dispatch: loop {
        match pc {
            0x832A7200 => {
    //   block [0x832A7200..0x832A720C)
	// 832A7200: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7204: 386B72C0  addi r3, r11, 0x72c0
	ctx.r[3].s64 = ctx.r[11].s64 + 29376;
	// 832A7208: 4AF6DBD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7210 size=12
    let mut pc: u32 = 0x832A7210;
    'dispatch: loop {
        match pc {
            0x832A7210 => {
    //   block [0x832A7210..0x832A721C)
	// 832A7210: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7214: 386B72C4  addi r3, r11, 0x72c4
	ctx.r[3].s64 = ctx.r[11].s64 + 29380;
	// 832A7218: 4AF6DBC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7220 size=12
    let mut pc: u32 = 0x832A7220;
    'dispatch: loop {
        match pc {
            0x832A7220 => {
    //   block [0x832A7220..0x832A722C)
	// 832A7220: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7224: 386B72C8  addi r3, r11, 0x72c8
	ctx.r[3].s64 = ctx.r[11].s64 + 29384;
	// 832A7228: 4AF6DBB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7230 size=12
    let mut pc: u32 = 0x832A7230;
    'dispatch: loop {
        match pc {
            0x832A7230 => {
    //   block [0x832A7230..0x832A723C)
	// 832A7230: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7234: 386B72CC  addi r3, r11, 0x72cc
	ctx.r[3].s64 = ctx.r[11].s64 + 29388;
	// 832A7238: 4AF6DBA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7240 size=12
    let mut pc: u32 = 0x832A7240;
    'dispatch: loop {
        match pc {
            0x832A7240 => {
    //   block [0x832A7240..0x832A724C)
	// 832A7240: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7244: 386B72D0  addi r3, r11, 0x72d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29392;
	// 832A7248: 4AF6DB90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7250 size=12
    let mut pc: u32 = 0x832A7250;
    'dispatch: loop {
        match pc {
            0x832A7250 => {
    //   block [0x832A7250..0x832A725C)
	// 832A7250: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7254: 386B72D4  addi r3, r11, 0x72d4
	ctx.r[3].s64 = ctx.r[11].s64 + 29396;
	// 832A7258: 4AF6DB80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7260 size=12
    let mut pc: u32 = 0x832A7260;
    'dispatch: loop {
        match pc {
            0x832A7260 => {
    //   block [0x832A7260..0x832A726C)
	// 832A7260: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7264: 386B72D8  addi r3, r11, 0x72d8
	ctx.r[3].s64 = ctx.r[11].s64 + 29400;
	// 832A7268: 4AF6DB70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7270 size=68
    let mut pc: u32 = 0x832A7270;
    'dispatch: loop {
        match pc {
            0x832A7270 => {
    //   block [0x832A7270..0x832A72B4)
	// 832A7270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A727C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7280: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7284: 3BEB72DC  addi r31, r11, 0x72dc
	ctx.r[31].s64 = ctx.r[11].s64 + 29404;
	// 832A7288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A728C: 4B76DB05  bl 0x82a14d90
	ctx.lr = 0x832A7290;
	sub_82A14D90(ctx, base);
	// 832A7290: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7294: 4AF74AA5  bl 0x8221bd38
	ctx.lr = 0x832A7298;
	sub_8221BD38(ctx, base);
	// 832A7298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A729C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A72A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A72A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A72A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A72AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A72B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A72B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A72B8 size=68
    let mut pc: u32 = 0x832A72B8;
    'dispatch: loop {
        match pc {
            0x832A72B8 => {
    //   block [0x832A72B8..0x832A72FC)
	// 832A72B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A72BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A72C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A72C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A72C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A72CC: 3BEB72E8  addi r31, r11, 0x72e8
	ctx.r[31].s64 = ctx.r[11].s64 + 29416;
	// 832A72D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A72D4: 4B058B1D  bl 0x822ffdf0
	ctx.lr = 0x832A72D8;
	sub_822FFDF0(ctx, base);
	// 832A72D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A72DC: 4AF74A5D  bl 0x8221bd38
	ctx.lr = 0x832A72E0;
	sub_8221BD38(ctx, base);
	// 832A72E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A72E4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A72E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A72EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A72F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A72F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A72F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7300 size=12
    let mut pc: u32 = 0x832A7300;
    'dispatch: loop {
        match pc {
            0x832A7300 => {
    //   block [0x832A7300..0x832A730C)
	// 832A7300: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7304: 386B72F4  addi r3, r11, 0x72f4
	ctx.r[3].s64 = ctx.r[11].s64 + 29428;
	// 832A7308: 4AF6DAD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7310 size=12
    let mut pc: u32 = 0x832A7310;
    'dispatch: loop {
        match pc {
            0x832A7310 => {
    //   block [0x832A7310..0x832A731C)
	// 832A7310: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7314: 386B72F8  addi r3, r11, 0x72f8
	ctx.r[3].s64 = ctx.r[11].s64 + 29432;
	// 832A7318: 4AF6DAC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7320 size=12
    let mut pc: u32 = 0x832A7320;
    'dispatch: loop {
        match pc {
            0x832A7320 => {
    //   block [0x832A7320..0x832A732C)
	// 832A7320: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7324: 386B72FC  addi r3, r11, 0x72fc
	ctx.r[3].s64 = ctx.r[11].s64 + 29436;
	// 832A7328: 4AF6DAB0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7330 size=12
    let mut pc: u32 = 0x832A7330;
    'dispatch: loop {
        match pc {
            0x832A7330 => {
    //   block [0x832A7330..0x832A733C)
	// 832A7330: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7334: 386B7300  addi r3, r11, 0x7300
	ctx.r[3].s64 = ctx.r[11].s64 + 29440;
	// 832A7338: 4AF6DAA0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7340 size=12
    let mut pc: u32 = 0x832A7340;
    'dispatch: loop {
        match pc {
            0x832A7340 => {
    //   block [0x832A7340..0x832A734C)
	// 832A7340: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7344: 386B7304  addi r3, r11, 0x7304
	ctx.r[3].s64 = ctx.r[11].s64 + 29444;
	// 832A7348: 4AF6DA90  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7350 size=12
    let mut pc: u32 = 0x832A7350;
    'dispatch: loop {
        match pc {
            0x832A7350 => {
    //   block [0x832A7350..0x832A735C)
	// 832A7350: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7354: 386B7308  addi r3, r11, 0x7308
	ctx.r[3].s64 = ctx.r[11].s64 + 29448;
	// 832A7358: 4AF6DA80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7360 size=12
    let mut pc: u32 = 0x832A7360;
    'dispatch: loop {
        match pc {
            0x832A7360 => {
    //   block [0x832A7360..0x832A736C)
	// 832A7360: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7364: 386B730C  addi r3, r11, 0x730c
	ctx.r[3].s64 = ctx.r[11].s64 + 29452;
	// 832A7368: 4AF6DA70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7370 size=12
    let mut pc: u32 = 0x832A7370;
    'dispatch: loop {
        match pc {
            0x832A7370 => {
    //   block [0x832A7370..0x832A737C)
	// 832A7370: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7374: 386B7310  addi r3, r11, 0x7310
	ctx.r[3].s64 = ctx.r[11].s64 + 29456;
	// 832A7378: 4AF6DA60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7380 size=12
    let mut pc: u32 = 0x832A7380;
    'dispatch: loop {
        match pc {
            0x832A7380 => {
    //   block [0x832A7380..0x832A738C)
	// 832A7380: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7384: 386B7350  addi r3, r11, 0x7350
	ctx.r[3].s64 = ctx.r[11].s64 + 29520;
	// 832A7388: 4AF6DA50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7390 size=12
    let mut pc: u32 = 0x832A7390;
    'dispatch: loop {
        match pc {
            0x832A7390 => {
    //   block [0x832A7390..0x832A739C)
	// 832A7390: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7394: 386B7354  addi r3, r11, 0x7354
	ctx.r[3].s64 = ctx.r[11].s64 + 29524;
	// 832A7398: 4AF6DA40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73A0 size=12
    let mut pc: u32 = 0x832A73A0;
    'dispatch: loop {
        match pc {
            0x832A73A0 => {
    //   block [0x832A73A0..0x832A73AC)
	// 832A73A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A73A4: 386B7358  addi r3, r11, 0x7358
	ctx.r[3].s64 = ctx.r[11].s64 + 29528;
	// 832A73A8: 4AF6DA30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73B0 size=12
    let mut pc: u32 = 0x832A73B0;
    'dispatch: loop {
        match pc {
            0x832A73B0 => {
    //   block [0x832A73B0..0x832A73BC)
	// 832A73B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A73B4: 386B735C  addi r3, r11, 0x735c
	ctx.r[3].s64 = ctx.r[11].s64 + 29532;
	// 832A73B8: 4AF6DA20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73C0 size=12
    let mut pc: u32 = 0x832A73C0;
    'dispatch: loop {
        match pc {
            0x832A73C0 => {
    //   block [0x832A73C0..0x832A73CC)
	// 832A73C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A73C4: 386B73B4  addi r3, r11, 0x73b4
	ctx.r[3].s64 = ctx.r[11].s64 + 29620;
	// 832A73C8: 4AF6DA10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73D0 size=4
    let mut pc: u32 = 0x832A73D0;
    'dispatch: loop {
        match pc {
            0x832A73D0 => {
    //   block [0x832A73D0..0x832A73D4)
	// 832A73D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73D8 size=12
    let mut pc: u32 = 0x832A73D8;
    'dispatch: loop {
        match pc {
            0x832A73D8 => {
    //   block [0x832A73D8..0x832A73E4)
	// 832A73D8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A73DC: 386B7428  addi r3, r11, 0x7428
	ctx.r[3].s64 = ctx.r[11].s64 + 29736;
	// 832A73E0: 4AF6D9F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73E8 size=12
    let mut pc: u32 = 0x832A73E8;
    'dispatch: loop {
        match pc {
            0x832A73E8 => {
    //   block [0x832A73E8..0x832A73F4)
	// 832A73E8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A73EC: 386B742C  addi r3, r11, 0x742c
	ctx.r[3].s64 = ctx.r[11].s64 + 29740;
	// 832A73F0: 4AF6D9E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A73F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A73F8 size=12
    let mut pc: u32 = 0x832A73F8;
    'dispatch: loop {
        match pc {
            0x832A73F8 => {
    //   block [0x832A73F8..0x832A7404)
	// 832A73F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A73FC: 386B7430  addi r3, r11, 0x7430
	ctx.r[3].s64 = ctx.r[11].s64 + 29744;
	// 832A7400: 4AF6D9D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7408 size=12
    let mut pc: u32 = 0x832A7408;
    'dispatch: loop {
        match pc {
            0x832A7408 => {
    //   block [0x832A7408..0x832A7414)
	// 832A7408: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A740C: 386B7434  addi r3, r11, 0x7434
	ctx.r[3].s64 = ctx.r[11].s64 + 29748;
	// 832A7410: 4AF6D9C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7418 size=12
    let mut pc: u32 = 0x832A7418;
    'dispatch: loop {
        match pc {
            0x832A7418 => {
    //   block [0x832A7418..0x832A7424)
	// 832A7418: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A741C: 386B7438  addi r3, r11, 0x7438
	ctx.r[3].s64 = ctx.r[11].s64 + 29752;
	// 832A7420: 4AF6D9B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7428 size=12
    let mut pc: u32 = 0x832A7428;
    'dispatch: loop {
        match pc {
            0x832A7428 => {
    //   block [0x832A7428..0x832A7434)
	// 832A7428: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A742C: 386B743C  addi r3, r11, 0x743c
	ctx.r[3].s64 = ctx.r[11].s64 + 29756;
	// 832A7430: 4AF6D9A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7438 size=12
    let mut pc: u32 = 0x832A7438;
    'dispatch: loop {
        match pc {
            0x832A7438 => {
    //   block [0x832A7438..0x832A7444)
	// 832A7438: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A743C: 386B7440  addi r3, r11, 0x7440
	ctx.r[3].s64 = ctx.r[11].s64 + 29760;
	// 832A7440: 4AF6D998  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7448 size=12
    let mut pc: u32 = 0x832A7448;
    'dispatch: loop {
        match pc {
            0x832A7448 => {
    //   block [0x832A7448..0x832A7454)
	// 832A7448: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A744C: 386B7444  addi r3, r11, 0x7444
	ctx.r[3].s64 = ctx.r[11].s64 + 29764;
	// 832A7450: 4AF6D988  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7458 size=12
    let mut pc: u32 = 0x832A7458;
    'dispatch: loop {
        match pc {
            0x832A7458 => {
    //   block [0x832A7458..0x832A7464)
	// 832A7458: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A745C: 386B7448  addi r3, r11, 0x7448
	ctx.r[3].s64 = ctx.r[11].s64 + 29768;
	// 832A7460: 4AF6D978  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7468 size=12
    let mut pc: u32 = 0x832A7468;
    'dispatch: loop {
        match pc {
            0x832A7468 => {
    //   block [0x832A7468..0x832A7474)
	// 832A7468: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A746C: 386B744C  addi r3, r11, 0x744c
	ctx.r[3].s64 = ctx.r[11].s64 + 29772;
	// 832A7470: 4AF6D968  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7478 size=12
    let mut pc: u32 = 0x832A7478;
    'dispatch: loop {
        match pc {
            0x832A7478 => {
    //   block [0x832A7478..0x832A7484)
	// 832A7478: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A747C: 386B7450  addi r3, r11, 0x7450
	ctx.r[3].s64 = ctx.r[11].s64 + 29776;
	// 832A7480: 4AF6D958  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7488 size=12
    let mut pc: u32 = 0x832A7488;
    'dispatch: loop {
        match pc {
            0x832A7488 => {
    //   block [0x832A7488..0x832A7494)
	// 832A7488: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A748C: 386B7454  addi r3, r11, 0x7454
	ctx.r[3].s64 = ctx.r[11].s64 + 29780;
	// 832A7490: 4AF6D948  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7498 size=12
    let mut pc: u32 = 0x832A7498;
    'dispatch: loop {
        match pc {
            0x832A7498 => {
    //   block [0x832A7498..0x832A74A4)
	// 832A7498: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A749C: 386B7490  addi r3, r11, 0x7490
	ctx.r[3].s64 = ctx.r[11].s64 + 29840;
	// 832A74A0: 4AF10678  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A74A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A74A8 size=100
    let mut pc: u32 = 0x832A74A8;
    'dispatch: loop {
        match pc {
            0x832A74A8 => {
    //   block [0x832A74A8..0x832A74E0)
	// 832A74A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A74AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A74B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A74B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A74B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A74BC: 3BEB7498  addi r31, r11, 0x7498
	ctx.r[31].s64 = ctx.r[11].s64 + 29848;
	// 832A74C0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A74C4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A74C8: 419A0018  beq cr6, 0x832a74e0
	if ctx.cr[6].eq {
	pc = 0x832A74E0; continue 'dispatch;
	}
	// 832A74CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A74D0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A74D4: 4B09B335  bl 0x82342808
	ctx.lr = 0x832A74D8;
	sub_82342808(ctx, base);
	// 832A74D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A74DC: 4AF7485D  bl 0x8221bd38
	ctx.lr = 0x832A74E0;
	sub_8221BD38(ctx, base);
	pc = 0x832A74E0; continue 'dispatch;
            }
            0x832A74E0 => {
    //   block [0x832A74E0..0x832A750C)
	// 832A74E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A74E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A74E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A74EC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A74F0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A74F4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A74F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A74FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7510 size=100
    let mut pc: u32 = 0x832A7510;
    'dispatch: loop {
        match pc {
            0x832A7510 => {
    //   block [0x832A7510..0x832A7548)
	// 832A7510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7518: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A751C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7520: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7524: 3BEB74A8  addi r31, r11, 0x74a8
	ctx.r[31].s64 = ctx.r[11].s64 + 29864;
	// 832A7528: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A752C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7530: 419A0018  beq cr6, 0x832a7548
	if ctx.cr[6].eq {
	pc = 0x832A7548; continue 'dispatch;
	}
	// 832A7534: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7538: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A753C: 4B09B9CD  bl 0x82342f08
	ctx.lr = 0x832A7540;
	sub_82342F08(ctx, base);
	// 832A7540: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7544: 4AF747F5  bl 0x8221bd38
	ctx.lr = 0x832A7548;
	sub_8221BD38(ctx, base);
	pc = 0x832A7548; continue 'dispatch;
            }
            0x832A7548 => {
    //   block [0x832A7548..0x832A7574)
	// 832A7548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A754C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7550: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7554: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A7558: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A755C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A7560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A756C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7578 size=100
    let mut pc: u32 = 0x832A7578;
    'dispatch: loop {
        match pc {
            0x832A7578 => {
    //   block [0x832A7578..0x832A75B0)
	// 832A7578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A757C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7584: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7588: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A758C: 3BEB74B8  addi r31, r11, 0x74b8
	ctx.r[31].s64 = ctx.r[11].s64 + 29880;
	// 832A7590: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7594: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7598: 419A0018  beq cr6, 0x832a75b0
	if ctx.cr[6].eq {
	pc = 0x832A75B0; continue 'dispatch;
	}
	// 832A759C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A75A0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A75A4: 4B09B965  bl 0x82342f08
	ctx.lr = 0x832A75A8;
	sub_82342F08(ctx, base);
	// 832A75A8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A75AC: 4AF7478D  bl 0x8221bd38
	ctx.lr = 0x832A75B0;
	sub_8221BD38(ctx, base);
	pc = 0x832A75B0; continue 'dispatch;
            }
            0x832A75B0 => {
    //   block [0x832A75B0..0x832A75DC)
	// 832A75B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A75B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A75B8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A75BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A75C0: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A75C4: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A75C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A75CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A75D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A75D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A75D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A75E0 size=16
    let mut pc: u32 = 0x832A75E0;
    'dispatch: loop {
        match pc {
            0x832A75E0 => {
    //   block [0x832A75E0..0x832A75F0)
	// 832A75E0: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A75E4: 396B1630  addi r11, r11, 0x1630
	ctx.r[11].s64 = ctx.r[11].s64 + 5680;
	// 832A75E8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 832A75EC: 4B8EF88C  b 0x82b96e78
	sub_82B96E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A75F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A75F0 size=16
    let mut pc: u32 = 0x832A75F0;
    'dispatch: loop {
        match pc {
            0x832A75F0 => {
    //   block [0x832A75F0..0x832A7600)
	// 832A75F0: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A75F4: 396BD620  addi r11, r11, -0x29e0
	ctx.r[11].s64 = ctx.r[11].s64 + -10720;
	// 832A75F8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 832A75FC: 4B8EF87C  b 0x82b96e78
	sub_82B96E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7600 size=12
    let mut pc: u32 = 0x832A7600;
    'dispatch: loop {
        match pc {
            0x832A7600 => {
    //   block [0x832A7600..0x832A760C)
	// 832A7600: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7604: 386B74C8  addi r3, r11, 0x74c8
	ctx.r[3].s64 = ctx.r[11].s64 + 29896;
	// 832A7608: 4AF6D7D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7610 size=12
    let mut pc: u32 = 0x832A7610;
    'dispatch: loop {
        match pc {
            0x832A7610 => {
    //   block [0x832A7610..0x832A761C)
	// 832A7610: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7614: 386B74CC  addi r3, r11, 0x74cc
	ctx.r[3].s64 = ctx.r[11].s64 + 29900;
	// 832A7618: 4AF6D7C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7620 size=12
    let mut pc: u32 = 0x832A7620;
    'dispatch: loop {
        match pc {
            0x832A7620 => {
    //   block [0x832A7620..0x832A762C)
	// 832A7620: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7624: 386B74D0  addi r3, r11, 0x74d0
	ctx.r[3].s64 = ctx.r[11].s64 + 29904;
	// 832A7628: 4AF6D7B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7630 size=12
    let mut pc: u32 = 0x832A7630;
    'dispatch: loop {
        match pc {
            0x832A7630 => {
    //   block [0x832A7630..0x832A763C)
	// 832A7630: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A7634: 386B76B0  addi r3, r11, 0x76b0
	ctx.r[3].s64 = ctx.r[11].s64 + 30384;
	// 832A7638: 4AF104E0  b 0x821b7b18
	sub_821B7B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7640 size=12
    let mut pc: u32 = 0x832A7640;
    'dispatch: loop {
        match pc {
            0x832A7640 => {
    //   block [0x832A7640..0x832A764C)
	// 832A7640: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7644: 386B74D4  addi r3, r11, 0x74d4
	ctx.r[3].s64 = ctx.r[11].s64 + 29908;
	// 832A7648: 4AF6D790  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7650 size=12
    let mut pc: u32 = 0x832A7650;
    'dispatch: loop {
        match pc {
            0x832A7650 => {
    //   block [0x832A7650..0x832A765C)
	// 832A7650: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7654: 386B74D8  addi r3, r11, 0x74d8
	ctx.r[3].s64 = ctx.r[11].s64 + 29912;
	// 832A7658: 4AF6D780  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7660 size=12
    let mut pc: u32 = 0x832A7660;
    'dispatch: loop {
        match pc {
            0x832A7660 => {
    //   block [0x832A7660..0x832A766C)
	// 832A7660: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7664: 386B74DC  addi r3, r11, 0x74dc
	ctx.r[3].s64 = ctx.r[11].s64 + 29916;
	// 832A7668: 4AF6D770  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7670 size=104
    let mut pc: u32 = 0x832A7670;
    'dispatch: loop {
        match pc {
            0x832A7670 => {
    //   block [0x832A7670..0x832A7698)
	// 832A7670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7674: 4BA01D95  bl 0x82ca9408
	ctx.lr = 0x832A7678;
	sub_82CA93D0(ctx, base);
	// 832A7678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A767C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7680: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A7684: 396B74E0  addi r11, r11, 0x74e0
	ctx.r[11].s64 = ctx.r[11].s64 + 29920;
	// 832A7688: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A768C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A7690: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7694: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A7698; continue 'dispatch;
            }
            0x832A7698 => {
    //   block [0x832A7698..0x832A76A8)
	// 832A7698: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A769C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A76A0: 4AF1F0C9  bl 0x821c6768
	ctx.lr = 0x832A76A4;
	sub_821C6768(ctx, base);
	// 832A76A4: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832A76A8; continue 'dispatch;
            }
            0x832A76A8 => {
    //   block [0x832A76A8..0x832A76D8)
	// 832A76A8: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A76AC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A76B0: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A76B4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A76B8: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A76BC: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A76C0: 4082FFE8  bne 0x832a76a8
	if !ctx.cr[0].eq {
	pc = 0x832A76A8; continue 'dispatch;
	}
	// 832A76C4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A76C8: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A76CC: 4080FFCC  bge 0x832a7698
	if !ctx.cr[0].lt {
	pc = 0x832A7698; continue 'dispatch;
	}
	// 832A76D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A76D4: 4BA01D84  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A76D8 size=104
    let mut pc: u32 = 0x832A76D8;
    'dispatch: loop {
        match pc {
            0x832A76D8 => {
    //   block [0x832A76D8..0x832A7700)
	// 832A76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A76DC: 4BA01D2D  bl 0x82ca9408
	ctx.lr = 0x832A76E0;
	sub_82CA93D0(ctx, base);
	// 832A76E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A76E4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A76E8: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A76EC: 396B7510  addi r11, r11, 0x7510
	ctx.r[11].s64 = ctx.r[11].s64 + 29968;
	// 832A76F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A76F4: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A76F8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A76FC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A7700; continue 'dispatch;
            }
            0x832A7700 => {
    //   block [0x832A7700..0x832A7710)
	// 832A7700: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7708: 4AF1F061  bl 0x821c6768
	ctx.lr = 0x832A770C;
	sub_821C6768(ctx, base);
	// 832A770C: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832A7710; continue 'dispatch;
            }
            0x832A7710 => {
    //   block [0x832A7710..0x832A7740)
	// 832A7710: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7714: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7718: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A771C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7720: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7724: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7728: 4082FFE8  bne 0x832a7710
	if !ctx.cr[0].eq {
	pc = 0x832A7710; continue 'dispatch;
	}
	// 832A772C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7730: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7734: 4080FFCC  bge 0x832a7700
	if !ctx.cr[0].lt {
	pc = 0x832A7700; continue 'dispatch;
	}
	// 832A7738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A773C: 4BA01D1C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7740 size=104
    let mut pc: u32 = 0x832A7740;
    'dispatch: loop {
        match pc {
            0x832A7740 => {
    //   block [0x832A7740..0x832A7768)
	// 832A7740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7744: 4BA01CC5  bl 0x82ca9408
	ctx.lr = 0x832A7748;
	sub_82CA93D0(ctx, base);
	// 832A7748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A774C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7750: 3BC0000B  li r30, 0xb
	ctx.r[30].s64 = 11;
	// 832A7754: 396B7540  addi r11, r11, 0x7540
	ctx.r[11].s64 = ctx.r[11].s64 + 30016;
	// 832A7758: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A775C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 832A7760: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7764: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A7768; continue 'dispatch;
            }
            0x832A7768 => {
    //   block [0x832A7768..0x832A7778)
	// 832A7768: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A776C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7770: 4AF1EFF9  bl 0x821c6768
	ctx.lr = 0x832A7774;
	sub_821C6768(ctx, base);
	// 832A7774: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832A7778; continue 'dispatch;
            }
            0x832A7778 => {
    //   block [0x832A7778..0x832A77A8)
	// 832A7778: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A777C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7780: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7784: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7788: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A778C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7790: 4082FFE8  bne 0x832a7778
	if !ctx.cr[0].eq {
	pc = 0x832A7778; continue 'dispatch;
	}
	// 832A7794: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7798: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A779C: 4080FFCC  bge 0x832a7768
	if !ctx.cr[0].lt {
	pc = 0x832A7768; continue 'dispatch;
	}
	// 832A77A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A77A4: 4BA01CB4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A77A8 size=104
    let mut pc: u32 = 0x832A77A8;
    'dispatch: loop {
        match pc {
            0x832A77A8 => {
    //   block [0x832A77A8..0x832A77D0)
	// 832A77A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A77AC: 4BA01C5D  bl 0x82ca9408
	ctx.lr = 0x832A77B0;
	sub_82CA93D0(ctx, base);
	// 832A77B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A77B4: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A77B8: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 832A77BC: 396B3830  addi r11, r11, 0x3830
	ctx.r[11].s64 = ctx.r[11].s64 + 14384;
	// 832A77C0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A77C4: 3BEB004C  addi r31, r11, 0x4c
	ctx.r[31].s64 = ctx.r[11].s64 + 76;
	// 832A77C8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A77CC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A77D0; continue 'dispatch;
            }
            0x832A77D0 => {
    //   block [0x832A77D0..0x832A77E0)
	// 832A77D0: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 832A77D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A77D8: 4AF1EF91  bl 0x821c6768
	ctx.lr = 0x832A77DC;
	sub_821C6768(ctx, base);
	// 832A77DC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832A77E0; continue 'dispatch;
            }
            0x832A77E0 => {
    //   block [0x832A77E0..0x832A7810)
	// 832A77E0: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A77E4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A77E8: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A77EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A77F0: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A77F4: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A77F8: 4082FFE8  bne 0x832a77e0
	if !ctx.cr[0].eq {
	pc = 0x832A77E0; continue 'dispatch;
	}
	// 832A77FC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7800: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7804: 4080FFCC  bge 0x832a77d0
	if !ctx.cr[0].lt {
	pc = 0x832A77D0; continue 'dispatch;
	}
	// 832A7808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A780C: 4BA01C4C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7810 size=12
    let mut pc: u32 = 0x832A7810;
    'dispatch: loop {
        match pc {
            0x832A7810 => {
    //   block [0x832A7810..0x832A781C)
	// 832A7810: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7814: 386B7570  addi r3, r11, 0x7570
	ctx.r[3].s64 = ctx.r[11].s64 + 30064;
	// 832A7818: 4AF6D5C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7820 size=12
    let mut pc: u32 = 0x832A7820;
    'dispatch: loop {
        match pc {
            0x832A7820 => {
    //   block [0x832A7820..0x832A782C)
	// 832A7820: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7824: 386B7574  addi r3, r11, 0x7574
	ctx.r[3].s64 = ctx.r[11].s64 + 30068;
	// 832A7828: 4AF6D5B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7830 size=12
    let mut pc: u32 = 0x832A7830;
    'dispatch: loop {
        match pc {
            0x832A7830 => {
    //   block [0x832A7830..0x832A783C)
	// 832A7830: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7834: 386B7578  addi r3, r11, 0x7578
	ctx.r[3].s64 = ctx.r[11].s64 + 30072;
	// 832A7838: 4AF6D5A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7840 size=12
    let mut pc: u32 = 0x832A7840;
    'dispatch: loop {
        match pc {
            0x832A7840 => {
    //   block [0x832A7840..0x832A784C)
	// 832A7840: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7844: 386B757C  addi r3, r11, 0x757c
	ctx.r[3].s64 = ctx.r[11].s64 + 30076;
	// 832A7848: 4AF6D590  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7850 size=12
    let mut pc: u32 = 0x832A7850;
    'dispatch: loop {
        match pc {
            0x832A7850 => {
    //   block [0x832A7850..0x832A785C)
	// 832A7850: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7854: 386B7580  addi r3, r11, 0x7580
	ctx.r[3].s64 = ctx.r[11].s64 + 30080;
	// 832A7858: 4AF6D580  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7860 size=12
    let mut pc: u32 = 0x832A7860;
    'dispatch: loop {
        match pc {
            0x832A7860 => {
    //   block [0x832A7860..0x832A786C)
	// 832A7860: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7864: 386B7584  addi r3, r11, 0x7584
	ctx.r[3].s64 = ctx.r[11].s64 + 30084;
	// 832A7868: 4AF6D570  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7870 size=12
    let mut pc: u32 = 0x832A7870;
    'dispatch: loop {
        match pc {
            0x832A7870 => {
    //   block [0x832A7870..0x832A787C)
	// 832A7870: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7874: 386B7588  addi r3, r11, 0x7588
	ctx.r[3].s64 = ctx.r[11].s64 + 30088;
	// 832A7878: 4AF6D560  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7880 size=12
    let mut pc: u32 = 0x832A7880;
    'dispatch: loop {
        match pc {
            0x832A7880 => {
    //   block [0x832A7880..0x832A788C)
	// 832A7880: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7884: 386B758C  addi r3, r11, 0x758c
	ctx.r[3].s64 = ctx.r[11].s64 + 30092;
	// 832A7888: 4AF6D550  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7890 size=12
    let mut pc: u32 = 0x832A7890;
    'dispatch: loop {
        match pc {
            0x832A7890 => {
    //   block [0x832A7890..0x832A789C)
	// 832A7890: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7894: 386B7590  addi r3, r11, 0x7590
	ctx.r[3].s64 = ctx.r[11].s64 + 30096;
	// 832A7898: 4AF6D540  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A78A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A78A0 size=12
    let mut pc: u32 = 0x832A78A0;
    'dispatch: loop {
        match pc {
            0x832A78A0 => {
    //   block [0x832A78A0..0x832A78AC)
	// 832A78A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A78A4: 386B7594  addi r3, r11, 0x7594
	ctx.r[3].s64 = ctx.r[11].s64 + 30100;
	// 832A78A8: 4AF6D530  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A78B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A78B0 size=12
    let mut pc: u32 = 0x832A78B0;
    'dispatch: loop {
        match pc {
            0x832A78B0 => {
    //   block [0x832A78B0..0x832A78BC)
	// 832A78B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A78B4: 386B7598  addi r3, r11, 0x7598
	ctx.r[3].s64 = ctx.r[11].s64 + 30104;
	// 832A78B8: 4AF6D520  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A78C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A78C0 size=12
    let mut pc: u32 = 0x832A78C0;
    'dispatch: loop {
        match pc {
            0x832A78C0 => {
    //   block [0x832A78C0..0x832A78CC)
	// 832A78C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A78C4: 386B759C  addi r3, r11, 0x759c
	ctx.r[3].s64 = ctx.r[11].s64 + 30108;
	// 832A78C8: 4AF6D510  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A78D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A78D0 size=12
    let mut pc: u32 = 0x832A78D0;
    'dispatch: loop {
        match pc {
            0x832A78D0 => {
    //   block [0x832A78D0..0x832A78DC)
	// 832A78D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A78D4: 386B75A0  addi r3, r11, 0x75a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30112;
	// 832A78D8: 4AF6D500  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A78E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A78E0 size=12
    let mut pc: u32 = 0x832A78E0;
    'dispatch: loop {
        match pc {
            0x832A78E0 => {
    //   block [0x832A78E0..0x832A78EC)
	// 832A78E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A78E4: 386B75A4  addi r3, r11, 0x75a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30116;
	// 832A78E8: 4AF6D4F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A78F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A78F0 size=12
    let mut pc: u32 = 0x832A78F0;
    'dispatch: loop {
        match pc {
            0x832A78F0 => {
    //   block [0x832A78F0..0x832A78FC)
	// 832A78F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A78F4: 386B75A8  addi r3, r11, 0x75a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30120;
	// 832A78F8: 4AF6D4E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7900 size=12
    let mut pc: u32 = 0x832A7900;
    'dispatch: loop {
        match pc {
            0x832A7900 => {
    //   block [0x832A7900..0x832A790C)
	// 832A7900: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7904: 386B75AC  addi r3, r11, 0x75ac
	ctx.r[3].s64 = ctx.r[11].s64 + 30124;
	// 832A7908: 4AF6D4D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7910 size=12
    let mut pc: u32 = 0x832A7910;
    'dispatch: loop {
        match pc {
            0x832A7910 => {
    //   block [0x832A7910..0x832A791C)
	// 832A7910: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7914: 386B75B0  addi r3, r11, 0x75b0
	ctx.r[3].s64 = ctx.r[11].s64 + 30128;
	// 832A7918: 4AF6D4C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7920 size=12
    let mut pc: u32 = 0x832A7920;
    'dispatch: loop {
        match pc {
            0x832A7920 => {
    //   block [0x832A7920..0x832A792C)
	// 832A7920: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7924: 386B75B4  addi r3, r11, 0x75b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30132;
	// 832A7928: 4AF6D4B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7930 size=12
    let mut pc: u32 = 0x832A7930;
    'dispatch: loop {
        match pc {
            0x832A7930 => {
    //   block [0x832A7930..0x832A793C)
	// 832A7930: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7934: 386B75B8  addi r3, r11, 0x75b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30136;
	// 832A7938: 4AF6D4A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7940 size=12
    let mut pc: u32 = 0x832A7940;
    'dispatch: loop {
        match pc {
            0x832A7940 => {
    //   block [0x832A7940..0x832A794C)
	// 832A7940: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7944: 386B75BC  addi r3, r11, 0x75bc
	ctx.r[3].s64 = ctx.r[11].s64 + 30140;
	// 832A7948: 4AF6D490  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7950 size=12
    let mut pc: u32 = 0x832A7950;
    'dispatch: loop {
        match pc {
            0x832A7950 => {
    //   block [0x832A7950..0x832A795C)
	// 832A7950: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7954: 386B75C0  addi r3, r11, 0x75c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30144;
	// 832A7958: 4AF6D480  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7960 size=12
    let mut pc: u32 = 0x832A7960;
    'dispatch: loop {
        match pc {
            0x832A7960 => {
    //   block [0x832A7960..0x832A796C)
	// 832A7960: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7964: 386B75C4  addi r3, r11, 0x75c4
	ctx.r[3].s64 = ctx.r[11].s64 + 30148;
	// 832A7968: 4AF6D470  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7970 size=12
    let mut pc: u32 = 0x832A7970;
    'dispatch: loop {
        match pc {
            0x832A7970 => {
    //   block [0x832A7970..0x832A797C)
	// 832A7970: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7974: 386B75C8  addi r3, r11, 0x75c8
	ctx.r[3].s64 = ctx.r[11].s64 + 30152;
	// 832A7978: 4B0B3E28  b 0x8235b7a0
	sub_8235B7A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7980 size=12
    let mut pc: u32 = 0x832A7980;
    'dispatch: loop {
        match pc {
            0x832A7980 => {
    //   block [0x832A7980..0x832A798C)
	// 832A7980: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7984: 386B75D8  addi r3, r11, 0x75d8
	ctx.r[3].s64 = ctx.r[11].s64 + 30168;
	// 832A7988: 4B0B3E80  b 0x8235b808
	sub_8235B808(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7990 size=68
    let mut pc: u32 = 0x832A7990;
    'dispatch: loop {
        match pc {
            0x832A7990 => {
    //   block [0x832A7990..0x832A79D4)
	// 832A7990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7998: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A799C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A79A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A79A4: 3BEB75E8  addi r31, r11, 0x75e8
	ctx.r[31].s64 = ctx.r[11].s64 + 30184;
	// 832A79A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A79AC: 4B07AE3D  bl 0x823227e8
	ctx.lr = 0x832A79B0;
	sub_823227E8(ctx, base);
	// 832A79B0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A79B4: 4AF74385  bl 0x8221bd38
	ctx.lr = 0x832A79B8;
	sub_8221BD38(ctx, base);
	// 832A79B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A79BC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A79C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A79C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A79C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A79CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A79D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A79D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A79D8 size=4
    let mut pc: u32 = 0x832A79D8;
    'dispatch: loop {
        match pc {
            0x832A79D8 => {
    //   block [0x832A79D8..0x832A79DC)
	// 832A79D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A79E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A79E0 size=12
    let mut pc: u32 = 0x832A79E0;
    'dispatch: loop {
        match pc {
            0x832A79E0 => {
    //   block [0x832A79E0..0x832A79EC)
	// 832A79E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A79E4: 386B75F4  addi r3, r11, 0x75f4
	ctx.r[3].s64 = ctx.r[11].s64 + 30196;
	// 832A79E8: 4AF6D3F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A79F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A79F0 size=12
    let mut pc: u32 = 0x832A79F0;
    'dispatch: loop {
        match pc {
            0x832A79F0 => {
    //   block [0x832A79F0..0x832A79FC)
	// 832A79F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A79F4: 386B7630  addi r3, r11, 0x7630
	ctx.r[3].s64 = ctx.r[11].s64 + 30256;
	// 832A79F8: 4AF6D3E0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A00 size=12
    let mut pc: u32 = 0x832A7A00;
    'dispatch: loop {
        match pc {
            0x832A7A00 => {
    //   block [0x832A7A00..0x832A7A0C)
	// 832A7A00: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A04: 386B7634  addi r3, r11, 0x7634
	ctx.r[3].s64 = ctx.r[11].s64 + 30260;
	// 832A7A08: 4AF6D3D0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A10 size=12
    let mut pc: u32 = 0x832A7A10;
    'dispatch: loop {
        match pc {
            0x832A7A10 => {
    //   block [0x832A7A10..0x832A7A1C)
	// 832A7A10: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A14: 386B7638  addi r3, r11, 0x7638
	ctx.r[3].s64 = ctx.r[11].s64 + 30264;
	// 832A7A18: 4AF6D3C0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A20 size=12
    let mut pc: u32 = 0x832A7A20;
    'dispatch: loop {
        match pc {
            0x832A7A20 => {
    //   block [0x832A7A20..0x832A7A2C)
	// 832A7A20: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A24: 386B763C  addi r3, r11, 0x763c
	ctx.r[3].s64 = ctx.r[11].s64 + 30268;
	// 832A7A28: 4AF6D3B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A30 size=12
    let mut pc: u32 = 0x832A7A30;
    'dispatch: loop {
        match pc {
            0x832A7A30 => {
    //   block [0x832A7A30..0x832A7A3C)
	// 832A7A30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A34: 386B7640  addi r3, r11, 0x7640
	ctx.r[3].s64 = ctx.r[11].s64 + 30272;
	// 832A7A38: 4AF6D3A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A40 size=12
    let mut pc: u32 = 0x832A7A40;
    'dispatch: loop {
        match pc {
            0x832A7A40 => {
    //   block [0x832A7A40..0x832A7A4C)
	// 832A7A40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A44: 386B7644  addi r3, r11, 0x7644
	ctx.r[3].s64 = ctx.r[11].s64 + 30276;
	// 832A7A48: 4AF6D390  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A50 size=12
    let mut pc: u32 = 0x832A7A50;
    'dispatch: loop {
        match pc {
            0x832A7A50 => {
    //   block [0x832A7A50..0x832A7A5C)
	// 832A7A50: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A54: 386B7648  addi r3, r11, 0x7648
	ctx.r[3].s64 = ctx.r[11].s64 + 30280;
	// 832A7A58: 4AF6D380  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A60 size=12
    let mut pc: u32 = 0x832A7A60;
    'dispatch: loop {
        match pc {
            0x832A7A60 => {
    //   block [0x832A7A60..0x832A7A6C)
	// 832A7A60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A64: 386B764C  addi r3, r11, 0x764c
	ctx.r[3].s64 = ctx.r[11].s64 + 30284;
	// 832A7A68: 4AF6D370  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A70 size=12
    let mut pc: u32 = 0x832A7A70;
    'dispatch: loop {
        match pc {
            0x832A7A70 => {
    //   block [0x832A7A70..0x832A7A7C)
	// 832A7A70: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A74: 386B7650  addi r3, r11, 0x7650
	ctx.r[3].s64 = ctx.r[11].s64 + 30288;
	// 832A7A78: 4AF6D360  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A80 size=12
    let mut pc: u32 = 0x832A7A80;
    'dispatch: loop {
        match pc {
            0x832A7A80 => {
    //   block [0x832A7A80..0x832A7A8C)
	// 832A7A80: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A84: 386B7654  addi r3, r11, 0x7654
	ctx.r[3].s64 = ctx.r[11].s64 + 30292;
	// 832A7A88: 4AF6D350  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7A90 size=12
    let mut pc: u32 = 0x832A7A90;
    'dispatch: loop {
        match pc {
            0x832A7A90 => {
    //   block [0x832A7A90..0x832A7A9C)
	// 832A7A90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7A94: 386B7658  addi r3, r11, 0x7658
	ctx.r[3].s64 = ctx.r[11].s64 + 30296;
	// 832A7A98: 4AF6D340  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AA0 size=12
    let mut pc: u32 = 0x832A7AA0;
    'dispatch: loop {
        match pc {
            0x832A7AA0 => {
    //   block [0x832A7AA0..0x832A7AAC)
	// 832A7AA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7AA4: 386B765C  addi r3, r11, 0x765c
	ctx.r[3].s64 = ctx.r[11].s64 + 30300;
	// 832A7AA8: 4AF6D330  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AB0 size=12
    let mut pc: u32 = 0x832A7AB0;
    'dispatch: loop {
        match pc {
            0x832A7AB0 => {
    //   block [0x832A7AB0..0x832A7ABC)
	// 832A7AB0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7AB4: 386B7660  addi r3, r11, 0x7660
	ctx.r[3].s64 = ctx.r[11].s64 + 30304;
	// 832A7AB8: 4AF6D320  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AC0 size=12
    let mut pc: u32 = 0x832A7AC0;
    'dispatch: loop {
        match pc {
            0x832A7AC0 => {
    //   block [0x832A7AC0..0x832A7ACC)
	// 832A7AC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7AC4: 386B7664  addi r3, r11, 0x7664
	ctx.r[3].s64 = ctx.r[11].s64 + 30308;
	// 832A7AC8: 4AF6D310  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AD0 size=12
    let mut pc: u32 = 0x832A7AD0;
    'dispatch: loop {
        match pc {
            0x832A7AD0 => {
    //   block [0x832A7AD0..0x832A7ADC)
	// 832A7AD0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7AD4: 386B7668  addi r3, r11, 0x7668
	ctx.r[3].s64 = ctx.r[11].s64 + 30312;
	// 832A7AD8: 4AF6D300  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AE0 size=12
    let mut pc: u32 = 0x832A7AE0;
    'dispatch: loop {
        match pc {
            0x832A7AE0 => {
    //   block [0x832A7AE0..0x832A7AEC)
	// 832A7AE0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7AE4: 386B766C  addi r3, r11, 0x766c
	ctx.r[3].s64 = ctx.r[11].s64 + 30316;
	// 832A7AE8: 4AF6D2F0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AF0 size=4
    let mut pc: u32 = 0x832A7AF0;
    'dispatch: loop {
        match pc {
            0x832A7AF0 => {
    //   block [0x832A7AF0..0x832A7AF4)
	// 832A7AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7AF8 size=12
    let mut pc: u32 = 0x832A7AF8;
    'dispatch: loop {
        match pc {
            0x832A7AF8 => {
    //   block [0x832A7AF8..0x832A7B04)
	// 832A7AF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7AFC: 386B7694  addi r3, r11, 0x7694
	ctx.r[3].s64 = ctx.r[11].s64 + 30356;
	// 832A7B00: 4AF6D2D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B08 size=12
    let mut pc: u32 = 0x832A7B08;
    'dispatch: loop {
        match pc {
            0x832A7B08 => {
    //   block [0x832A7B08..0x832A7B14)
	// 832A7B08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B0C: 386B7698  addi r3, r11, 0x7698
	ctx.r[3].s64 = ctx.r[11].s64 + 30360;
	// 832A7B10: 4AF6D2C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B18 size=4
    let mut pc: u32 = 0x832A7B18;
    'dispatch: loop {
        match pc {
            0x832A7B18 => {
    //   block [0x832A7B18..0x832A7B1C)
	// 832A7B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B20 size=12
    let mut pc: u32 = 0x832A7B20;
    'dispatch: loop {
        match pc {
            0x832A7B20 => {
    //   block [0x832A7B20..0x832A7B2C)
	// 832A7B20: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B24: 386B76A0  addi r3, r11, 0x76a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30368;
	// 832A7B28: 4AF6D2B0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B30 size=12
    let mut pc: u32 = 0x832A7B30;
    'dispatch: loop {
        match pc {
            0x832A7B30 => {
    //   block [0x832A7B30..0x832A7B3C)
	// 832A7B30: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B34: 386B76A4  addi r3, r11, 0x76a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30372;
	// 832A7B38: 4AF6D2A0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7B40 size=12
    let mut pc: u32 = 0x832A7B40;
    'dispatch: loop {
        match pc {
            0x832A7B40 => {
    //   block [0x832A7B40..0x832A7B4C)
	// 832A7B40: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B44: 386B76A8  addi r3, r11, 0x76a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30376;
	// 832A7B48: 4AF6D290  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7B50 size=100
    let mut pc: u32 = 0x832A7B50;
    'dispatch: loop {
        match pc {
            0x832A7B50 => {
    //   block [0x832A7B50..0x832A7B88)
	// 832A7B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7B58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7B5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7B60: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7B64: 3BEB76AC  addi r31, r11, 0x76ac
	ctx.r[31].s64 = ctx.r[11].s64 + 30380;
	// 832A7B68: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7B6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7B70: 419A0018  beq cr6, 0x832a7b88
	if ctx.cr[6].eq {
	pc = 0x832A7B88; continue 'dispatch;
	}
	// 832A7B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7B78: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7B7C: 4B7EA385  bl 0x82a91f00
	ctx.lr = 0x832A7B80;
	sub_82A91F00(ctx, base);
	// 832A7B80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832A7B84: 4AF741B5  bl 0x8221bd38
	ctx.lr = 0x832A7B88;
	sub_8221BD38(ctx, base);
	pc = 0x832A7B88; continue 'dispatch;
            }
            0x832A7B88 => {
    //   block [0x832A7B88..0x832A7BB4)
	// 832A7B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7B90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7B94: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A7B98: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A7B9C: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 832A7BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7BAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7BC8 size=104
    let mut pc: u32 = 0x832A7BC8;
    'dispatch: loop {
        match pc {
            0x832A7BC8 => {
    //   block [0x832A7BC8..0x832A7BF0)
	// 832A7BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7BCC: 4BA0183D  bl 0x82ca9408
	ctx.lr = 0x832A7BD0;
	sub_82CA93D0(ctx, base);
	// 832A7BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7BD4: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BD8: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A7BDC: 396B76C0  addi r11, r11, 0x76c0
	ctx.r[11].s64 = ctx.r[11].s64 + 30400;
	// 832A7BE0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7BE4: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A7BE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7BEC: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A7BF0; continue 'dispatch;
            }
            0x832A7BF0 => {
    //   block [0x832A7BF0..0x832A7C00)
	// 832A7BF0: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7BF8: 4AF1EB71  bl 0x821c6768
	ctx.lr = 0x832A7BFC;
	sub_821C6768(ctx, base);
	// 832A7BFC: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832A7C00; continue 'dispatch;
            }
            0x832A7C00 => {
    //   block [0x832A7C00..0x832A7C30)
	// 832A7C00: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7C04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C08: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7C0C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7C10: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7C14: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C18: 4082FFE8  bne 0x832a7c00
	if !ctx.cr[0].eq {
	pc = 0x832A7C00; continue 'dispatch;
	}
	// 832A7C1C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7C20: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7C24: 4080FFCC  bge 0x832a7bf0
	if !ctx.cr[0].lt {
	pc = 0x832A7BF0; continue 'dispatch;
	}
	// 832A7C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7C2C: 4BA0182C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7C30 size=104
    let mut pc: u32 = 0x832A7C30;
    'dispatch: loop {
        match pc {
            0x832A7C30 => {
    //   block [0x832A7C30..0x832A7C58)
	// 832A7C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7C34: 4BA017D5  bl 0x82ca9408
	ctx.lr = 0x832A7C38;
	sub_82CA93D0(ctx, base);
	// 832A7C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7C3C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C40: 3BC0000F  li r30, 0xf
	ctx.r[30].s64 = 15;
	// 832A7C44: 396B7700  addi r11, r11, 0x7700
	ctx.r[11].s64 = ctx.r[11].s64 + 30464;
	// 832A7C48: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7C4C: 3BEB0040  addi r31, r11, 0x40
	ctx.r[31].s64 = ctx.r[11].s64 + 64;
	// 832A7C50: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C54: 3BAB7088  addi r29, r11, 0x7088
	ctx.r[29].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A7C58; continue 'dispatch;
            }
            0x832A7C58 => {
    //   block [0x832A7C58..0x832A7C68)
	// 832A7C58: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 832A7C5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7C60: 4AF1EB09  bl 0x821c6768
	ctx.lr = 0x832A7C64;
	sub_821C6768(ctx, base);
	// 832A7C64: 7FA9EB78  mr r9, r29
	ctx.r[9].u64 = ctx.r[29].u64;
	pc = 0x832A7C68; continue 'dispatch;
            }
            0x832A7C68 => {
    //   block [0x832A7C68..0x832A7C98)
	// 832A7C68: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7C6C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C70: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7C74: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7C78: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7C7C: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7C80: 4082FFE8  bne 0x832a7c68
	if !ctx.cr[0].eq {
	pc = 0x832A7C68; continue 'dispatch;
	}
	// 832A7C84: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832A7C88: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832A7C8C: 4080FFCC  bge 0x832a7c58
	if !ctx.cr[0].lt {
	pc = 0x832A7C58; continue 'dispatch;
	}
	// 832A7C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7C94: 4BA017C4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7C98 size=12
    let mut pc: u32 = 0x832A7C98;
    'dispatch: loop {
        match pc {
            0x832A7C98 => {
    //   block [0x832A7C98..0x832A7CA4)
	// 832A7C98: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7C9C: 386B7794  addi r3, r11, 0x7794
	ctx.r[3].s64 = ctx.r[11].s64 + 30612;
	// 832A7CA0: 4AF6D138  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CA8 size=12
    let mut pc: u32 = 0x832A7CA8;
    'dispatch: loop {
        match pc {
            0x832A7CA8 => {
    //   block [0x832A7CA8..0x832A7CB4)
	// 832A7CA8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CAC: 386B7798  addi r3, r11, 0x7798
	ctx.r[3].s64 = ctx.r[11].s64 + 30616;
	// 832A7CB0: 4AF6D128  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CB8 size=12
    let mut pc: u32 = 0x832A7CB8;
    'dispatch: loop {
        match pc {
            0x832A7CB8 => {
    //   block [0x832A7CB8..0x832A7CC4)
	// 832A7CB8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CBC: 386B779C  addi r3, r11, 0x779c
	ctx.r[3].s64 = ctx.r[11].s64 + 30620;
	// 832A7CC0: 4AF6D118  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CC8 size=12
    let mut pc: u32 = 0x832A7CC8;
    'dispatch: loop {
        match pc {
            0x832A7CC8 => {
    //   block [0x832A7CC8..0x832A7CD4)
	// 832A7CC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CCC: 386B77A0  addi r3, r11, 0x77a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30624;
	// 832A7CD0: 4AF6D108  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CD8 size=12
    let mut pc: u32 = 0x832A7CD8;
    'dispatch: loop {
        match pc {
            0x832A7CD8 => {
    //   block [0x832A7CD8..0x832A7CE4)
	// 832A7CD8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CDC: 386B77A4  addi r3, r11, 0x77a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30628;
	// 832A7CE0: 4AF6D0F8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CE8 size=12
    let mut pc: u32 = 0x832A7CE8;
    'dispatch: loop {
        match pc {
            0x832A7CE8 => {
    //   block [0x832A7CE8..0x832A7CF4)
	// 832A7CE8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CEC: 386B77A8  addi r3, r11, 0x77a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30632;
	// 832A7CF0: 4AF6D0E8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7CF8 size=12
    let mut pc: u32 = 0x832A7CF8;
    'dispatch: loop {
        match pc {
            0x832A7CF8 => {
    //   block [0x832A7CF8..0x832A7D04)
	// 832A7CF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7CFC: 386B77AC  addi r3, r11, 0x77ac
	ctx.r[3].s64 = ctx.r[11].s64 + 30636;
	// 832A7D00: 4AF6D0D8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D08 size=12
    let mut pc: u32 = 0x832A7D08;
    'dispatch: loop {
        match pc {
            0x832A7D08 => {
    //   block [0x832A7D08..0x832A7D14)
	// 832A7D08: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D0C: 386B77B0  addi r3, r11, 0x77b0
	ctx.r[3].s64 = ctx.r[11].s64 + 30640;
	// 832A7D10: 4AF6D0C8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D18 size=12
    let mut pc: u32 = 0x832A7D18;
    'dispatch: loop {
        match pc {
            0x832A7D18 => {
    //   block [0x832A7D18..0x832A7D24)
	// 832A7D18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D1C: 386B77B4  addi r3, r11, 0x77b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30644;
	// 832A7D20: 4AF6D0B8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D28 size=12
    let mut pc: u32 = 0x832A7D28;
    'dispatch: loop {
        match pc {
            0x832A7D28 => {
    //   block [0x832A7D28..0x832A7D34)
	// 832A7D28: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D2C: 386B77B8  addi r3, r11, 0x77b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30648;
	// 832A7D30: 4AF6D0A8  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7D38 size=88
    let mut pc: u32 = 0x832A7D38;
    'dispatch: loop {
        match pc {
            0x832A7D38 => {
    //   block [0x832A7D38..0x832A7D6C)
	// 832A7D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7D48: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A7D4C: 3BEB2144  addi r31, r11, 0x2144
	ctx.r[31].s64 = ctx.r[11].s64 + 8516;
	// 832A7D50: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832A7D54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7D58: 419A0014  beq cr6, 0x832a7d6c
	if ctx.cr[6].eq {
	pc = 0x832A7D6C; continue 'dispatch;
	}
	// 832A7D5C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7D60: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7D64: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7D68: 4E800421  bctrl
	ctx.lr = 0x832A7D6C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832A7D6C => {
    //   block [0x832A7D6C..0x832A7D90)
	// 832A7D6C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832A7D70: 4B0CE171  bl 0x82375ee0
	ctx.lr = 0x832A7D74;
	sub_82375EE0(ctx, base);
	// 832A7D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7D78: 4B0CD309  bl 0x82375080
	ctx.lr = 0x832A7D7C;
	sub_82375080(ctx, base);
	// 832A7D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7D90 size=12
    let mut pc: u32 = 0x832A7D90;
    'dispatch: loop {
        match pc {
            0x832A7D90 => {
    //   block [0x832A7D90..0x832A7D9C)
	// 832A7D90: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7D94: 386B77C0  addi r3, r11, 0x77c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30656;
	// 832A7D98: 4AF6D040  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DA0 size=12
    let mut pc: u32 = 0x832A7DA0;
    'dispatch: loop {
        match pc {
            0x832A7DA0 => {
    //   block [0x832A7DA0..0x832A7DAC)
	// 832A7DA0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DA4: 386B77C4  addi r3, r11, 0x77c4
	ctx.r[3].s64 = ctx.r[11].s64 + 30660;
	// 832A7DA8: 4AF6D030  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DB0 size=12
    let mut pc: u32 = 0x832A7DB0;
    'dispatch: loop {
        match pc {
            0x832A7DB0 => {
    //   block [0x832A7DB0..0x832A7DBC)
	// 832A7DB0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DB4: 386B77C8  addi r3, r11, 0x77c8
	ctx.r[3].s64 = ctx.r[11].s64 + 30664;
	// 832A7DB8: 4AF6D020  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DC0 size=12
    let mut pc: u32 = 0x832A7DC0;
    'dispatch: loop {
        match pc {
            0x832A7DC0 => {
    //   block [0x832A7DC0..0x832A7DCC)
	// 832A7DC0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DC4: 386B77CC  addi r3, r11, 0x77cc
	ctx.r[3].s64 = ctx.r[11].s64 + 30668;
	// 832A7DC8: 4AF6D010  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DD0 size=12
    let mut pc: u32 = 0x832A7DD0;
    'dispatch: loop {
        match pc {
            0x832A7DD0 => {
    //   block [0x832A7DD0..0x832A7DDC)
	// 832A7DD0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DD4: 386B77D0  addi r3, r11, 0x77d0
	ctx.r[3].s64 = ctx.r[11].s64 + 30672;
	// 832A7DD8: 4AF6D000  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DE0 size=12
    let mut pc: u32 = 0x832A7DE0;
    'dispatch: loop {
        match pc {
            0x832A7DE0 => {
    //   block [0x832A7DE0..0x832A7DEC)
	// 832A7DE0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DE4: 386B77D4  addi r3, r11, 0x77d4
	ctx.r[3].s64 = ctx.r[11].s64 + 30676;
	// 832A7DE8: 4AF6CFF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7DF0 size=12
    let mut pc: u32 = 0x832A7DF0;
    'dispatch: loop {
        match pc {
            0x832A7DF0 => {
    //   block [0x832A7DF0..0x832A7DFC)
	// 832A7DF0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7DF4: 386B77D8  addi r3, r11, 0x77d8
	ctx.r[3].s64 = ctx.r[11].s64 + 30680;
	// 832A7DF8: 4AF6CFE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E00 size=12
    let mut pc: u32 = 0x832A7E00;
    'dispatch: loop {
        match pc {
            0x832A7E00 => {
    //   block [0x832A7E00..0x832A7E0C)
	// 832A7E00: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7E04: 386B77DC  addi r3, r11, 0x77dc
	ctx.r[3].s64 = ctx.r[11].s64 + 30684;
	// 832A7E08: 4AF6CFD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E10 size=4
    let mut pc: u32 = 0x832A7E10;
    'dispatch: loop {
        match pc {
            0x832A7E10 => {
    //   block [0x832A7E10..0x832A7E14)
	// 832A7E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E18 size=4
    let mut pc: u32 = 0x832A7E18;
    'dispatch: loop {
        match pc {
            0x832A7E18 => {
    //   block [0x832A7E18..0x832A7E1C)
	// 832A7E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E20 size=4
    let mut pc: u32 = 0x832A7E20;
    'dispatch: loop {
        match pc {
            0x832A7E20 => {
    //   block [0x832A7E20..0x832A7E24)
	// 832A7E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E28 size=4
    let mut pc: u32 = 0x832A7E28;
    'dispatch: loop {
        match pc {
            0x832A7E28 => {
    //   block [0x832A7E28..0x832A7E2C)
	// 832A7E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E30 size=12
    let mut pc: u32 = 0x832A7E30;
    'dispatch: loop {
        match pc {
            0x832A7E30 => {
    //   block [0x832A7E30..0x832A7E3C)
	// 832A7E30: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 832A7E34: 806B76BC  lwz r3, 0x76bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30396 as u32) ) } as u64;
	// 832A7E38: 4AF73F00  b 0x8221bd38
	sub_8221BD38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A7E40 size=4
    let mut pc: u32 = 0x832A7E40;
    'dispatch: loop {
        match pc {
            0x832A7E40 => {
    //   block [0x832A7E40..0x832A7E44)
	// 832A7E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7E48 size=172
    let mut pc: u32 = 0x832A7E48;
    'dispatch: loop {
        match pc {
            0x832A7E48 => {
    //   block [0x832A7E48..0x832A7E7C)
	// 832A7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7E58: 3D60834C  lis r11, -0x7cb4
	ctx.r[11].s64 = -2092171264;
	// 832A7E5C: 3BEB3978  addi r31, r11, 0x3978
	ctx.r[31].s64 = ctx.r[11].s64 + 14712;
	// 832A7E60: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 832A7E64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7E68: 419A0014  beq cr6, 0x832a7e7c
	if ctx.cr[6].eq {
	pc = 0x832A7E7C; continue 'dispatch;
	}
	// 832A7E6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7E70: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7E74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7E78: 4E800421  bctrl
	ctx.lr = 0x832A7E7C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832A7E7C => {
    //   block [0x832A7E7C..0x832A7E98)
	// 832A7E7C: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 832A7E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7E84: 419A0014  beq cr6, 0x832a7e98
	if ctx.cr[6].eq {
	pc = 0x832A7E98; continue 'dispatch;
	}
	// 832A7E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7E8C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7E90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7E94: 4E800421  bctrl
	ctx.lr = 0x832A7E98;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832A7E98 => {
    //   block [0x832A7E98..0x832A7EC0)
	// 832A7E98: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832A7E9C: 4B0D8285  bl 0x82380120
	ctx.lr = 0x832A7EA0;
	sub_82380120(ctx, base);
	// 832A7EA0: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 832A7EA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 832A7EA8: 419A0018  beq cr6, 0x832a7ec0
	if ctx.cr[6].eq {
	pc = 0x832A7EC0; continue 'dispatch;
	}
	// 832A7EAC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832A7EB0: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 832A7EB4: 4B0D889D  bl 0x82380750
	ctx.lr = 0x832A7EB8;
	sub_82380750(ctx, base);
	// 832A7EB8: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 832A7EBC: 4AF73E7D  bl 0x8221bd38
	ctx.lr = 0x832A7EC0;
	sub_8221BD38(ctx, base);
	pc = 0x832A7EC0; continue 'dispatch;
            }
            0x832A7EC0 => {
    //   block [0x832A7EC0..0x832A7EF4)
	// 832A7EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A7EC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A7EC8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A7ECC: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 832A7ED0: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 832A7ED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832A7ED8: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 832A7EDC: 4B9CADE5  bl 0x82c72cc0
	ctx.lr = 0x832A7EE0;
	sub_82C72CC0(ctx, base);
	// 832A7EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A7EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A7EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A7EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A7EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7EF8 size=220
    let mut pc: u32 = 0x832A7EF8;
    'dispatch: loop {
        match pc {
            0x832A7EF8 => {
    //   block [0x832A7EF8..0x832A7F20)
	// 832A7EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7EFC: 4BA0150D  bl 0x82ca9408
	ctx.lr = 0x832A7F00;
	sub_82CA93D0(ctx, base);
	// 832A7F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7F04: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7F08: 3BA00005  li r29, 5
	ctx.r[29].s64 = 5;
	// 832A7F0C: 396B7810  addi r11, r11, 0x7810
	ctx.r[11].s64 = ctx.r[11].s64 + 30736;
	// 832A7F10: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832A7F14: 3BEB0088  addi r31, r11, 0x88
	ctx.r[31].s64 = ctx.r[11].s64 + 136;
	// 832A7F18: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A7F1C: 3BCB7088  addi r30, r11, 0x7088
	ctx.r[30].s64 = ctx.r[11].s64 + 28808;
	pc = 0x832A7F20; continue 'dispatch;
            }
            0x832A7F20 => {
    //   block [0x832A7F20..0x832A7F40)
	// 832A7F20: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 832A7F24: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7F28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7F2C: 419A0014  beq cr6, 0x832a7f40
	if ctx.cr[6].eq {
	pc = 0x832A7F40; continue 'dispatch;
	}
	// 832A7F30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832A7F34: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832A7F38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 832A7F3C: 4E800421  bctrl
	ctx.lr = 0x832A7F40;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x832A7F40 => {
    //   block [0x832A7F40..0x832A7F4C)
	// 832A7F40: 387FFFFC  addi r3, r31, -4
	ctx.r[3].s64 = ctx.r[31].s64 + -4;
	// 832A7F44: 4AF1E825  bl 0x821c6768
	ctx.lr = 0x832A7F48;
	sub_821C6768(ctx, base);
	// 832A7F48: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	pc = 0x832A7F4C; continue 'dispatch;
            }
            0x832A7F4C => {
    //   block [0x832A7F4C..0x832A7F78)
	// 832A7F4C: 7D4000A6  mfmsr r10
	ctx.r[10].u64 = ctx.msr;
	// 832A7F50: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F54: 7D604828  lwarx r11, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[11].u64 = ctx.reserved.u32 as u64;
	// 832A7F58: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 832A7F5C: 7D60492D  stwcx. r11, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[11].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7F60: 7D410164  mtmsrd r10, 1
	ctx.msr = (ctx.r[10].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F64: 4082FFE8  bne 0x832a7f4c
	if !ctx.cr[0].eq {
	pc = 0x832A7F4C; continue 'dispatch;
	}
	// 832A7F68: 939FFFFC  stw r28, -4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[28].u32 ) };
	// 832A7F6C: 387FFFF4  addi r3, r31, -0xc
	ctx.r[3].s64 = ctx.r[31].s64 + -12;
	// 832A7F70: 4AF1E7F9  bl 0x821c6768
	ctx.lr = 0x832A7F74;
	sub_821C6768(ctx, base);
	// 832A7F74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	pc = 0x832A7F78; continue 'dispatch;
            }
            0x832A7F78 => {
    //   block [0x832A7F78..0x832A7FA4)
	// 832A7F78: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832A7F7C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F80: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832A7F84: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832A7F88: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7F8C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7F90: 4082FFE8  bne 0x832a7f78
	if !ctx.cr[0].eq {
	pc = 0x832A7F78; continue 'dispatch;
	}
	// 832A7F94: 939FFFF4  stw r28, -0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-12 as u32), ctx.r[28].u32 ) };
	// 832A7F98: 387FFFF0  addi r3, r31, -0x10
	ctx.r[3].s64 = ctx.r[31].s64 + -16;
	// 832A7F9C: 4AF1E7CD  bl 0x821c6768
	ctx.lr = 0x832A7FA0;
	sub_821C6768(ctx, base);
	// 832A7FA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x832A7FA4; continue 'dispatch;
            }
            0x832A7FA4 => {
    //   block [0x832A7FA4..0x832A7FD4)
	// 832A7FA4: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 832A7FA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7FAC: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 832A7FB0: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 832A7FB4: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832A7FB8: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832A7FBC: 4082FFE8  bne 0x832a7fa4
	if !ctx.cr[0].eq {
	pc = 0x832A7FA4; continue 'dispatch;
	}
	// 832A7FC0: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 832A7FC4: 939FFFF0  stw r28, -0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), ctx.r[28].u32 ) };
	// 832A7FC8: 4080FF58  bge 0x832a7f20
	if !ctx.cr[0].lt {
	pc = 0x832A7F20; continue 'dispatch;
	}
	// 832A7FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A7FD0: 4BA01488  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A7FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A7FD8 size=120
    let mut pc: u32 = 0x832A7FD8;
    'dispatch: loop {
        match pc {
            0x832A7FD8 => {
    //   block [0x832A7FD8..0x832A802C)
	// 832A7FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A7FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A7FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832A7FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A7FE8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 832A7FEC: 3BEB9930  addi r31, r11, -0x66d0
	ctx.r[31].s64 = ctx.r[11].s64 + -26320;
	// 832A7FF0: 806B9930  lwz r3, -0x66d0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26320 as u32) ) } as u64;
	// 832A7FF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832A7FF8: 419A0034  beq cr6, 0x832a802c
	if ctx.cr[6].eq {
	pc = 0x832A802C; continue 'dispatch;
	}
	// 832A7FFC: 4AF73D3D  bl 0x8221bd38
	ctx.lr = 0x832A8000;
	sub_8221BD38(ctx, base);
	// 832A8000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A8008: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 832A800C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832A8010: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A8014: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 832A8018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A8028: 4E800020  blr
	return;
            }
            0x832A802C => {
    //   block [0x832A802C..0x832A8050)
	// 832A802C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A8030: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 832A8034: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832A8038: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A803C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832A8040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A8044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A8048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832A804C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8050 size=12
    let mut pc: u32 = 0x832A8050;
    'dispatch: loop {
        match pc {
            0x832A8050 => {
    //   block [0x832A8050..0x832A805C)
	// 832A8050: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8054: 386B7888  addi r3, r11, 0x7888
	ctx.r[3].s64 = ctx.r[11].s64 + 30856;
	// 832A8058: 4AF6CD80  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8060 size=12
    let mut pc: u32 = 0x832A8060;
    'dispatch: loop {
        match pc {
            0x832A8060 => {
    //   block [0x832A8060..0x832A806C)
	// 832A8060: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8064: 386B788C  addi r3, r11, 0x788c
	ctx.r[3].s64 = ctx.r[11].s64 + 30860;
	// 832A8068: 4AF6CD70  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8070 size=12
    let mut pc: u32 = 0x832A8070;
    'dispatch: loop {
        match pc {
            0x832A8070 => {
    //   block [0x832A8070..0x832A807C)
	// 832A8070: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8074: 386B7890  addi r3, r11, 0x7890
	ctx.r[3].s64 = ctx.r[11].s64 + 30864;
	// 832A8078: 4AF6CD60  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8080 size=12
    let mut pc: u32 = 0x832A8080;
    'dispatch: loop {
        match pc {
            0x832A8080 => {
    //   block [0x832A8080..0x832A808C)
	// 832A8080: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8084: 386B7894  addi r3, r11, 0x7894
	ctx.r[3].s64 = ctx.r[11].s64 + 30868;
	// 832A8088: 4AF6CD50  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8090 size=12
    let mut pc: u32 = 0x832A8090;
    'dispatch: loop {
        match pc {
            0x832A8090 => {
    //   block [0x832A8090..0x832A809C)
	// 832A8090: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8094: 386B7898  addi r3, r11, 0x7898
	ctx.r[3].s64 = ctx.r[11].s64 + 30872;
	// 832A8098: 4AF6CD40  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80A0 size=12
    let mut pc: u32 = 0x832A80A0;
    'dispatch: loop {
        match pc {
            0x832A80A0 => {
    //   block [0x832A80A0..0x832A80AC)
	// 832A80A0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80A4: 386B789C  addi r3, r11, 0x789c
	ctx.r[3].s64 = ctx.r[11].s64 + 30876;
	// 832A80A8: 4AF6CD30  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80B0 size=12
    let mut pc: u32 = 0x832A80B0;
    'dispatch: loop {
        match pc {
            0x832A80B0 => {
    //   block [0x832A80B0..0x832A80BC)
	// 832A80B0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80B4: 386B78A0  addi r3, r11, 0x78a0
	ctx.r[3].s64 = ctx.r[11].s64 + 30880;
	// 832A80B8: 4AF6CD20  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80C0 size=12
    let mut pc: u32 = 0x832A80C0;
    'dispatch: loop {
        match pc {
            0x832A80C0 => {
    //   block [0x832A80C0..0x832A80CC)
	// 832A80C0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80C4: 386B78A4  addi r3, r11, 0x78a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30884;
	// 832A80C8: 4AF6CD10  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80D0 size=12
    let mut pc: u32 = 0x832A80D0;
    'dispatch: loop {
        match pc {
            0x832A80D0 => {
    //   block [0x832A80D0..0x832A80DC)
	// 832A80D0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80D4: 386B78A8  addi r3, r11, 0x78a8
	ctx.r[3].s64 = ctx.r[11].s64 + 30888;
	// 832A80D8: 4AF6CD00  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80E0 size=12
    let mut pc: u32 = 0x832A80E0;
    'dispatch: loop {
        match pc {
            0x832A80E0 => {
    //   block [0x832A80E0..0x832A80EC)
	// 832A80E0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80E4: 386B78AC  addi r3, r11, 0x78ac
	ctx.r[3].s64 = ctx.r[11].s64 + 30892;
	// 832A80E8: 4AF6CCF0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A80F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A80F0 size=12
    let mut pc: u32 = 0x832A80F0;
    'dispatch: loop {
        match pc {
            0x832A80F0 => {
    //   block [0x832A80F0..0x832A80FC)
	// 832A80F0: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A80F4: 386B78B0  addi r3, r11, 0x78b0
	ctx.r[3].s64 = ctx.r[11].s64 + 30896;
	// 832A80F8: 4AF6CCE0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8100 size=12
    let mut pc: u32 = 0x832A8100;
    'dispatch: loop {
        match pc {
            0x832A8100 => {
    //   block [0x832A8100..0x832A810C)
	// 832A8100: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8104: 386B78B4  addi r3, r11, 0x78b4
	ctx.r[3].s64 = ctx.r[11].s64 + 30900;
	// 832A8108: 4AF6CCD0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A8110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A8110 size=12
    let mut pc: u32 = 0x832A8110;
    'dispatch: loop {
        match pc {
            0x832A8110 => {
    //   block [0x832A8110..0x832A811C)
	// 832A8110: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832A8114: 386B78B8  addi r3, r11, 0x78b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30904;
	// 832A8118: 4AF6CCC0  b 0x82214dd8
	sub_82214DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


