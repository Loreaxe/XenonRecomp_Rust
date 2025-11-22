pub fn sub_82F1D138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D138 size=8
    let mut pc: u32 = 0x82F1D138;
    'dispatch: loop {
        match pc {
            0x82F1D138 => {
    //   block [0x82F1D138..0x82F1D140)
	// 82F1D138: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D13C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D140 size=44
    let mut pc: u32 = 0x82F1D140;
    'dispatch: loop {
        match pc {
            0x82F1D140 => {
    //   block [0x82F1D140..0x82F1D16C)
	// 82F1D140: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D144: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D148: 392BE374  addi r9, r11, -0x1c8c
	ctx.r[9].s64 = ctx.r[11].s64 + -7308;
	// 82F1D14C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1D150: 38EABD10  addi r7, r10, -0x42f0
	ctx.r[7].s64 = ctx.r[10].s64 + -17136;
	// 82F1D154: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1D158: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F1D15C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1D160: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F1D164: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F1D168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D170 size=12
    let mut pc: u32 = 0x82F1D170;
    'dispatch: loop {
        match pc {
            0x82F1D170 => {
    //   block [0x82F1D170..0x82F1D17C)
	// 82F1D170: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D174: 386BE374  addi r3, r11, -0x1c8c
	ctx.r[3].s64 = ctx.r[11].s64 + -7308;
	// 82F1D178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D180 size=140
    let mut pc: u32 = 0x82F1D180;
    'dispatch: loop {
        match pc {
            0x82F1D180 => {
    //   block [0x82F1D180..0x82F1D20C)
	// 82F1D180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D188: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D18C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D190: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D198: 3BC5FFA0  addi r30, r5, -0x60
	ctx.r[30].s64 = ctx.r[5].s64 + -96;
	// 82F1D19C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F1D1A0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1D1A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D1A8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1D1AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D1B0: 4E800421  bctrl
	ctx.lr = 0x82F1D1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1D1B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F1D1B8: 4098000C  bge cr6, 0x82f1d1c4
	if !ctx.cr[6].lt {
	pc = 0x82F1D1C4; continue 'dispatch;
	}
	// 82F1D1BC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F1D1C0: 48000034  b 0x82f1d1f4
	pc = 0x82F1D1F4; continue 'dispatch;
	// 82F1D1C4: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F1D1C8: 4199FFF4  bgt cr6, 0x82f1d1bc
	if ctx.cr[6].gt {
	pc = 0x82F1D1BC; continue 'dispatch;
	}
	// 82F1D1CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1D1D0: 395F0060  addi r10, r31, 0x60
	ctx.r[10].s64 = ctx.r[31].s64 + 96;
	// 82F1D1D4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F1D1D8: 409A0014  bne cr6, 0x82f1d1ec
	if !ctx.cr[6].eq {
	pc = 0x82F1D1EC; continue 'dispatch;
	}
	// 82F1D1DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1D1E0: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82F1D1E4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82F1D1E8: 4800000C  b 0x82f1d1f4
	pc = 0x82F1D1F4; continue 'dispatch;
	// 82F1D1EC: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82F1D1F0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 82F1D1F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D210 size=116
    let mut pc: u32 = 0x82F1D210;
    'dispatch: loop {
        match pc {
            0x82F1D210 => {
    //   block [0x82F1D210..0x82F1D284)
	// 82F1D210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D21C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D228: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D22C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82F1D230: 4BFFA439  bl 0x82f17668
	ctx.lr = 0x82F1D234;
	sub_82F17668(ctx, base);
	// 82F1D234: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1D238: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D23C: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F1D240: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F1D244: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1D248: 419A0020  beq cr6, 0x82f1d268
	if ctx.cr[6].eq {
	pc = 0x82F1D268; continue 'dispatch;
	}
	// 82F1D24C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D250: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D254: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D258: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D25C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D260: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D264: 4BF8354D  bl 0x82ea07b0
	ctx.lr = 0x82F1D268;
	sub_82EA07B0(ctx, base);
	// 82F1D268: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D26C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D278: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D27C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D288 size=20
    let mut pc: u32 = 0x82F1D288;
    'dispatch: loop {
        match pc {
            0x82F1D288 => {
    //   block [0x82F1D288..0x82F1D29C)
	// 82F1D288: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D28C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D290: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D294: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D298: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D2A0 size=8
    let mut pc: u32 = 0x82F1D2A0;
    'dispatch: loop {
        match pc {
            0x82F1D2A0 => {
    //   block [0x82F1D2A0..0x82F1D2A8)
	// 82F1D2A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D2A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D2A8 size=56
    let mut pc: u32 = 0x82F1D2A8;
    'dispatch: loop {
        match pc {
            0x82F1D2A8 => {
    //   block [0x82F1D2A8..0x82F1D2E0)
	// 82F1D2A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D2AC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D2B0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1D2B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1D2B8: 38EBBCD8  addi r7, r11, -0x4328
	ctx.r[7].s64 = ctx.r[11].s64 + -17192;
	// 82F1D2BC: 38CAE460  addi r6, r10, -0x1ba0
	ctx.r[6].s64 = ctx.r[10].s64 + -7072;
	// 82F1D2C0: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1D2C4: 38A9E43C  addi r5, r9, -0x1bc4
	ctx.r[5].s64 = ctx.r[9].s64 + -7108;
	// 82F1D2C8: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82F1D2CC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82F1D2D0: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1D2D4: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F1D2D8: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1D2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D2E0 size=12
    let mut pc: u32 = 0x82F1D2E0;
    'dispatch: loop {
        match pc {
            0x82F1D2E0 => {
    //   block [0x82F1D2E0..0x82F1D2EC)
	// 82F1D2E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D2E4: 386BE460  addi r3, r11, -0x1ba0
	ctx.r[3].s64 = ctx.r[11].s64 + -7072;
	// 82F1D2E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D2F0 size=100
    let mut pc: u32 = 0x82F1D2F0;
    'dispatch: loop {
        match pc {
            0x82F1D2F0 => {
    //   block [0x82F1D2F0..0x82F1D354)
	// 82F1D2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D2F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D2FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D300: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D304: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D308: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D30C: 48015175  bl 0x82f32480
	ctx.lr = 0x82F1D310;
	sub_82F32480(ctx, base);
	// 82F1D310: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1D318: 419A0020  beq cr6, 0x82f1d338
	if ctx.cr[6].eq {
	pc = 0x82F1D338; continue 'dispatch;
	}
	// 82F1D31C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D320: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D324: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D328: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D32C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D330: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D334: 4BF8347D  bl 0x82ea07b0
	ctx.lr = 0x82F1D338;
	sub_82EA07B0(ctx, base);
	// 82F1D338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D33C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D34C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D358 size=8
    let mut pc: u32 = 0x82F1D358;
    'dispatch: loop {
        match pc {
            0x82F1D358 => {
    //   block [0x82F1D358..0x82F1D360)
	// 82F1D358: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1D35C: 4BFFFF94  b 0x82f1d2f0
	sub_82F1D2F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D360 size=20
    let mut pc: u32 = 0x82F1D360;
    'dispatch: loop {
        match pc {
            0x82F1D360 => {
    //   block [0x82F1D360..0x82F1D374)
	// 82F1D360: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D364: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D368: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D36C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D370: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D378 size=8
    let mut pc: u32 = 0x82F1D378;
    'dispatch: loop {
        match pc {
            0x82F1D378 => {
    //   block [0x82F1D378..0x82F1D380)
	// 82F1D378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D37C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D380 size=56
    let mut pc: u32 = 0x82F1D380;
    'dispatch: loop {
        match pc {
            0x82F1D380 => {
    //   block [0x82F1D380..0x82F1D3B8)
	// 82F1D380: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D384: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D388: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1D38C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1D390: 38EBBCD8  addi r7, r11, -0x4328
	ctx.r[7].s64 = ctx.r[11].s64 + -17192;
	// 82F1D394: 38CAE584  addi r6, r10, -0x1a7c
	ctx.r[6].s64 = ctx.r[10].s64 + -6780;
	// 82F1D398: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1D39C: 38A9E55C  addi r5, r9, -0x1aa4
	ctx.r[5].s64 = ctx.r[9].s64 + -6820;
	// 82F1D3A0: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F1D3A4: 38800015  li r4, 0x15
	ctx.r[4].s64 = 21;
	// 82F1D3A8: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1D3AC: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82F1D3B0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1D3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D3B8 size=12
    let mut pc: u32 = 0x82F1D3B8;
    'dispatch: loop {
        match pc {
            0x82F1D3B8 => {
    //   block [0x82F1D3B8..0x82F1D3C4)
	// 82F1D3B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D3BC: 386BE584  addi r3, r11, -0x1a7c
	ctx.r[3].s64 = ctx.r[11].s64 + -6780;
	// 82F1D3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D3C8 size=8
    let mut pc: u32 = 0x82F1D3C8;
    'dispatch: loop {
        match pc {
            0x82F1D3C8 => {
    //   block [0x82F1D3C8..0x82F1D3D0)
	// 82F1D3C8: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F1D3CC: 48000004  b 0x82f1d3d0
	sub_82F1D3D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D3D0 size=100
    let mut pc: u32 = 0x82F1D3D0;
    'dispatch: loop {
        match pc {
            0x82F1D3D0 => {
    //   block [0x82F1D3D0..0x82F1D434)
	// 82F1D3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D3D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D3DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D3E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D3E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D3E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D3EC: 480055AD  bl 0x82f22998
	ctx.lr = 0x82F1D3F0;
	sub_82F22998(ctx, base);
	// 82F1D3F0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D3F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1D3F8: 419A0020  beq cr6, 0x82f1d418
	if ctx.cr[6].eq {
	pc = 0x82F1D418; continue 'dispatch;
	}
	// 82F1D3FC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D400: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D404: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D408: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D40C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D410: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D414: 4BF8339D  bl 0x82ea07b0
	ctx.lr = 0x82F1D418;
	sub_82EA07B0(ctx, base);
	// 82F1D418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D41C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D428: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D42C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D438 size=20
    let mut pc: u32 = 0x82F1D438;
    'dispatch: loop {
        match pc {
            0x82F1D438 => {
    //   block [0x82F1D438..0x82F1D44C)
	// 82F1D438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D43C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D440: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D444: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D448: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D450 size=12
    let mut pc: u32 = 0x82F1D450;
    'dispatch: loop {
        match pc {
            0x82F1D450 => {
    //   block [0x82F1D450..0x82F1D45C)
	// 82F1D450: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1D454: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D458: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D45C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D45C size=8
    let mut pc: u32 = 0x82F1D45C;
    'dispatch: loop {
        match pc {
            0x82F1D45C => {
    //   block [0x82F1D45C..0x82F1D464)
	// 82F1D45C: 4800003C  b 0x82f1d498
	sub_82F1D498(ctx, base);
	return;
	// 82F1D460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D468 size=44
    let mut pc: u32 = 0x82F1D468;
    'dispatch: loop {
        match pc {
            0x82F1D468 => {
    //   block [0x82F1D468..0x82F1D494)
	// 82F1D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D470: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D474: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F1D47C: 4800001D  bl 0x82f1d498
	ctx.lr = 0x82F1D480;
	sub_82F1D498(ctx, base);
	// 82F1D480: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1D484: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F1D488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D498 size=116
    let mut pc: u32 = 0x82F1D498;
    'dispatch: loop {
        match pc {
            0x82F1D498 => {
    //   block [0x82F1D498..0x82F1D50C)
	// 82F1D498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D4A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D4A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D4A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D4AC: 48000515  bl 0x82f1d9c0
	ctx.lr = 0x82F1D4B0;
	sub_82F1D9C0(ctx, base);
	// 82F1D4B0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D4B4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D4B8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F1D4BC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1D4C0: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F1D4C4: 38CBE640  addi r6, r11, -0x19c0
	ctx.r[6].s64 = ctx.r[11].s64 + -6592;
	// 82F1D4C8: 38AAE634  addi r5, r10, -0x19cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6604;
	// 82F1D4CC: 3868E628  addi r3, r8, -0x19d8
	ctx.r[3].s64 = ctx.r[8].s64 + -6616;
	// 82F1D4D0: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1D4D4: 3889E614  addi r4, r9, -0x19ec
	ctx.r[4].s64 = ctx.r[9].s64 + -6636;
	// 82F1D4D8: 90BF0008  stw r5, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F1D4DC: 3967E608  addi r11, r7, -0x19f8
	ctx.r[11].s64 = ctx.r[7].s64 + -6648;
	// 82F1D4E0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F1D4E4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82F1D4E8: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1D4EC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F1D4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D4F4: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F1D4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1D4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D504: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D510 size=100
    let mut pc: u32 = 0x82F1D510;
    'dispatch: loop {
        match pc {
            0x82F1D510 => {
    //   block [0x82F1D510..0x82F1D574)
	// 82F1D510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D518: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D51C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D520: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D524: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D528: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D52C: 48015BF5  bl 0x82f33120
	ctx.lr = 0x82F1D530;
	sub_82F33120(ctx, base);
	// 82F1D530: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D534: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1D538: 419A0020  beq cr6, 0x82f1d558
	if ctx.cr[6].eq {
	pc = 0x82F1D558; continue 'dispatch;
	}
	// 82F1D53C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D540: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D544: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D548: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D54C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D550: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D554: 4BF8325D  bl 0x82ea07b0
	ctx.lr = 0x82F1D558;
	sub_82EA07B0(ctx, base);
	// 82F1D558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D55C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D568: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D56C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D578 size=8
    let mut pc: u32 = 0x82F1D578;
    'dispatch: loop {
        match pc {
            0x82F1D578 => {
    //   block [0x82F1D578..0x82F1D580)
	// 82F1D578: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F1D57C: 4BFFFF94  b 0x82f1d510
	sub_82F1D510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D580 size=8
    let mut pc: u32 = 0x82F1D580;
    'dispatch: loop {
        match pc {
            0x82F1D580 => {
    //   block [0x82F1D580..0x82F1D588)
	// 82F1D580: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 82F1D584: 4BFFFF8C  b 0x82f1d510
	sub_82F1D510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D588 size=8
    let mut pc: u32 = 0x82F1D588;
    'dispatch: loop {
        match pc {
            0x82F1D588 => {
    //   block [0x82F1D588..0x82F1D590)
	// 82F1D588: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1D58C: 4BFFFF84  b 0x82f1d510
	sub_82F1D510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D590 size=8
    let mut pc: u32 = 0x82F1D590;
    'dispatch: loop {
        match pc {
            0x82F1D590 => {
    //   block [0x82F1D590..0x82F1D598)
	// 82F1D590: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F1D594: 4BFFFF7C  b 0x82f1d510
	sub_82F1D510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D598 size=20
    let mut pc: u32 = 0x82F1D598;
    'dispatch: loop {
        match pc {
            0x82F1D598 => {
    //   block [0x82F1D598..0x82F1D5AC)
	// 82F1D598: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D59C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D5A0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D5A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D5A8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D5B0 size=8
    let mut pc: u32 = 0x82F1D5B0;
    'dispatch: loop {
        match pc {
            0x82F1D5B0 => {
    //   block [0x82F1D5B0..0x82F1D5B8)
	// 82F1D5B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D5B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D5B8 size=32
    let mut pc: u32 = 0x82F1D5B8;
    'dispatch: loop {
        match pc {
            0x82F1D5B8 => {
    //   block [0x82F1D5B8..0x82F1D5D8)
	// 82F1D5B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D5BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1D5C0: 392BE6F4  addi r9, r11, -0x190c
	ctx.r[9].s64 = ctx.r[11].s64 + -6412;
	// 82F1D5C4: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82F1D5C8: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1D5CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1D5D0: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1D5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D5D8 size=12
    let mut pc: u32 = 0x82F1D5D8;
    'dispatch: loop {
        match pc {
            0x82F1D5D8 => {
    //   block [0x82F1D5D8..0x82F1D5E4)
	// 82F1D5D8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D5DC: 386BE6F4  addi r3, r11, -0x190c
	ctx.r[3].s64 = ctx.r[11].s64 + -6412;
	// 82F1D5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D5E8 size=20
    let mut pc: u32 = 0x82F1D5E8;
    'dispatch: loop {
        match pc {
            0x82F1D5E8 => {
    //   block [0x82F1D5E8..0x82F1D5FC)
	// 82F1D5E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D5EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D5F0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D5F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D5F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D600 size=8
    let mut pc: u32 = 0x82F1D600;
    'dispatch: loop {
        match pc {
            0x82F1D600 => {
    //   block [0x82F1D600..0x82F1D608)
	// 82F1D600: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D604: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D608 size=44
    let mut pc: u32 = 0x82F1D608;
    'dispatch: loop {
        match pc {
            0x82F1D608 => {
    //   block [0x82F1D608..0x82F1D634)
	// 82F1D608: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D60C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D610: 392BE78C  addi r9, r11, -0x1874
	ctx.r[9].s64 = ctx.r[11].s64 + -6260;
	// 82F1D614: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1D618: 38EABD10  addi r7, r10, -0x42f0
	ctx.r[7].s64 = ctx.r[10].s64 + -17136;
	// 82F1D61C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1D620: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82F1D624: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F1D628: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F1D62C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F1D630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D638 size=12
    let mut pc: u32 = 0x82F1D638;
    'dispatch: loop {
        match pc {
            0x82F1D638 => {
    //   block [0x82F1D638..0x82F1D644)
	// 82F1D638: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D63C: 386BE78C  addi r3, r11, -0x1874
	ctx.r[3].s64 = ctx.r[11].s64 + -6260;
	// 82F1D640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D648 size=100
    let mut pc: u32 = 0x82F1D648;
    'dispatch: loop {
        match pc {
            0x82F1D648 => {
    //   block [0x82F1D648..0x82F1D6AC)
	// 82F1D648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D650: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D654: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D658: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D65C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D660: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D664: 48015D85  bl 0x82f333e8
	ctx.lr = 0x82F1D668;
	sub_82F333E8(ctx, base);
	// 82F1D668: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D66C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1D670: 419A0020  beq cr6, 0x82f1d690
	if ctx.cr[6].eq {
	pc = 0x82F1D690; continue 'dispatch;
	}
	// 82F1D674: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D678: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D67C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D680: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D684: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D688: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D68C: 4BF83125  bl 0x82ea07b0
	ctx.lr = 0x82F1D690;
	sub_82EA07B0(ctx, base);
	// 82F1D690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D6A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D6A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D6B0 size=20
    let mut pc: u32 = 0x82F1D6B0;
    'dispatch: loop {
        match pc {
            0x82F1D6B0 => {
    //   block [0x82F1D6B0..0x82F1D6C4)
	// 82F1D6B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D6B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1D6B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D6BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F1D6C0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D6C8 size=8
    let mut pc: u32 = 0x82F1D6C8;
    'dispatch: loop {
        match pc {
            0x82F1D6C8 => {
    //   block [0x82F1D6C8..0x82F1D6D0)
	// 82F1D6C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F1D6CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D6D0 size=32
    let mut pc: u32 = 0x82F1D6D0;
    'dispatch: loop {
        match pc {
            0x82F1D6D0 => {
    //   block [0x82F1D6D0..0x82F1D6F0)
	// 82F1D6D0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D6D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1D6D8: 392BE7E4  addi r9, r11, -0x181c
	ctx.r[9].s64 = ctx.r[11].s64 + -6172;
	// 82F1D6DC: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82F1D6E0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1D6E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1D6E8: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F1D6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D6F0 size=12
    let mut pc: u32 = 0x82F1D6F0;
    'dispatch: loop {
        match pc {
            0x82F1D6F0 => {
    //   block [0x82F1D6F0..0x82F1D6FC)
	// 82F1D6F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D6F4: 386BE7E4  addi r3, r11, -0x181c
	ctx.r[3].s64 = ctx.r[11].s64 + -6172;
	// 82F1D6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D700 size=8
    let mut pc: u32 = 0x82F1D700;
    'dispatch: loop {
        match pc {
            0x82F1D700 => {
    //   block [0x82F1D700..0x82F1D708)
	// 82F1D700: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82F1D704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D708 size=100
    let mut pc: u32 = 0x82F1D708;
    'dispatch: loop {
        match pc {
            0x82F1D708 => {
    //   block [0x82F1D708..0x82F1D76C)
	// 82F1D708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1D714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D71C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1D724: 480026ED  bl 0x82f1fe10
	ctx.lr = 0x82F1D728;
	sub_82F1FE10(ctx, base);
	// 82F1D728: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F1D72C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1D730: 419A0020  beq cr6, 0x82f1d750
	if ctx.cr[6].eq {
	pc = 0x82F1D750; continue 'dispatch;
	}
	// 82F1D734: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D738: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F1D73C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F1D740: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F1D748: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1D74C: 4BF83065  bl 0x82ea07b0
	ctx.lr = 0x82F1D750;
	sub_82EA07B0(ctx, base);
	// 82F1D750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1D758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F1D764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D770 size=4
    let mut pc: u32 = 0x82F1D770;
    'dispatch: loop {
        match pc {
            0x82F1D770 => {
    //   block [0x82F1D770..0x82F1D774)
	// 82F1D770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D778 size=8
    let mut pc: u32 = 0x82F1D778;
    'dispatch: loop {
        match pc {
            0x82F1D778 => {
    //   block [0x82F1D778..0x82F1D780)
	// 82F1D778: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F1D77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D780 size=260
    let mut pc: u32 = 0x82F1D780;
    'dispatch: loop {
        match pc {
            0x82F1D780 => {
    //   block [0x82F1D780..0x82F1D884)
	// 82F1D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D788: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D78C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D790: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 82F1D794: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D798: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82F1D79C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F1D7A0: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82F1D7A4: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D888 size=4
    let mut pc: u32 = 0x82F1D888;
    'dispatch: loop {
        match pc {
            0x82F1D888 => {
    //   block [0x82F1D888..0x82F1D88C)
	// 82F1D888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1D890 size=96
    let mut pc: u32 = 0x82F1D890;
    'dispatch: loop {
        match pc {
            0x82F1D890 => {
    //   block [0x82F1D890..0x82F1D8F0)
	// 82F1D890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1D898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1D89C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D8A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1D8A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D8A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F1D8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D8B0: 388AE830  addi r4, r10, -0x17d0
	ctx.r[4].s64 = ctx.r[10].s64 + -6096;
	// 82F1D8B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D8B8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1D8BC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1D8C0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F1D8C4: 4E800421  bctrl
	ctx.lr = 0x82F1D8C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1D8C8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1D8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1D8D0: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1D8D4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F1D8D8: 4E800421  bctrl
	ctx.lr = 0x82F1D8DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1D8DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1D8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1D8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1D8E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1D8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D8F0 size=32
    let mut pc: u32 = 0x82F1D8F0;
    'dispatch: loop {
        match pc {
            0x82F1D8F0 => {
    //   block [0x82F1D8F0..0x82F1D910)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D910 size=12
    let mut pc: u32 = 0x82F1D910;
    'dispatch: loop {
        match pc {
            0x82F1D910 => {
    //   block [0x82F1D910..0x82F1D91C)
	// 82F1D910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F1D914: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F1D918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1D920 size=152
    let mut pc: u32 = 0x82F1D920;
    'dispatch: loop {
        match pc {
            0x82F1D920 => {
    //   block [0x82F1D920..0x82F1D9B8)
	// 82F1D920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1D924: 4828A849  bl 0x831a816c
	ctx.lr = 0x82F1D928;
	sub_831A8130(ctx, base);
	// 82F1D928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1D92C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1D930: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F1D934: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F1D938: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1D93C: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1D940: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82F1D944: 4098006C  bge cr6, 0x82f1d9b0
	if !ctx.cr[6].lt {
	pc = 0x82F1D9B0; continue 'dispatch;
	}
	// 82F1D948: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D9B8 size=8
    let mut pc: u32 = 0x82F1D9B8;
    'dispatch: loop {
        match pc {
            0x82F1D9B8 => {
    //   block [0x82F1D9B8..0x82F1D9C0)
	// 82F1D9B8: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82F1D9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1D9C0 size=128
    let mut pc: u32 = 0x82F1D9C0;
    'dispatch: loop {
        match pc {
            0x82F1D9C0 => {
    //   block [0x82F1D9C0..0x82F1DA40)
	// 82F1D9C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1D9C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F1D9C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82F1D9CC: 38EBBD68  addi r7, r11, -0x4298
	ctx.r[7].s64 = ctx.r[11].s64 + -17048;
	// 82F1D9D0: 38AABD80  addi r5, r10, -0x4280
	ctx.r[5].s64 = ctx.r[10].s64 + -17024;
	// 82F1D9D4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F1D9D8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F1D9DC: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F1D9E0: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82F1D9E4: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82F1D9E8: 38A8BD94  addi r5, r8, -0x426c
	ctx.r[5].s64 = ctx.r[8].s64 + -17004;
	// 82F1D9EC: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F1D9F0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1D9F4: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F1D9F8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1D9FC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1DA00: 3906BD74  addi r8, r6, -0x428c
	ctx.r[8].s64 = ctx.r[6].s64 + -17036;
	// 82F1DA04: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F1DA08: 38C4E878  addi r6, r4, -0x1788
	ctx.r[6].s64 = ctx.r[4].s64 + -6024;
	// 82F1DA0C: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82F1DA10: 38ABE86C  addi r5, r11, -0x1794
	ctx.r[5].s64 = ctx.r[11].s64 + -6036;
	// 82F1DA14: 388AE858  addi r4, r10, -0x17a8
	ctx.r[4].s64 = ctx.r[10].s64 + -6056;
	// 82F1DA18: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F1DA1C: 3969E84C  addi r11, r9, -0x17b4
	ctx.r[11].s64 = ctx.r[9].s64 + -6068;
	// 82F1DA20: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F1DA24: 3947E840  addi r10, r7, -0x17c0
	ctx.r[10].s64 = ctx.r[7].s64 + -6080;
	// 82F1DA28: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F1DA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F1DA30: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F1DA34: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82F1DA38: 91230020  stw r9, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 82F1DA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DA40 size=8
    let mut pc: u32 = 0x82F1DA40;
    'dispatch: loop {
        match pc {
            0x82F1DA40 => {
    //   block [0x82F1DA40..0x82F1DA48)
	// 82F1DA40: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F1DA44: 4802F4A4  b 0x82f4cee8
	sub_82F4CEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DA48 size=8
    let mut pc: u32 = 0x82F1DA48;
    'dispatch: loop {
        match pc {
            0x82F1DA48 => {
    //   block [0x82F1DA48..0x82F1DA50)
	// 82F1DA48: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82F1DA4C: 4802F49C  b 0x82f4cee8
	sub_82F4CEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1DA50 size=292
    let mut pc: u32 = 0x82F1DA50;
    'dispatch: loop {
        match pc {
            0x82F1DA50 => {
    //   block [0x82F1DA50..0x82F1DB74)
	// 82F1DA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1DA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1DA58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1DA5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1DA60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DA64: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82F1DA68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1DA6C: 48007D8D  bl 0x82f257f8
	ctx.lr = 0x82F1DA70;
	sub_82F257F8(ctx, base);
	// 82F1DA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DA74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DA78: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82F1DA7C: 48007D7D  bl 0x82f257f8
	ctx.lr = 0x82F1DA80;
	sub_82F257F8(ctx, base);
	// 82F1DA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DA84: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DA88: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82F1DA8C: 48007D6D  bl 0x82f257f8
	ctx.lr = 0x82F1DA90;
	sub_82F257F8(ctx, base);
	// 82F1DA90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DA94: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DA98: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82F1DA9C: 48007D5D  bl 0x82f257f8
	ctx.lr = 0x82F1DAA0;
	sub_82F257F8(ctx, base);
	// 82F1DAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DAA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DAA8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82F1DAAC: 48007D4D  bl 0x82f257f8
	ctx.lr = 0x82F1DAB0;
	sub_82F257F8(ctx, base);
	// 82F1DAB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DAB4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DAB8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82F1DABC: 48007D3D  bl 0x82f257f8
	ctx.lr = 0x82F1DAC0;
	sub_82F257F8(ctx, base);
	// 82F1DAC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DAC4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DAC8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82F1DACC: 48007D2D  bl 0x82f257f8
	ctx.lr = 0x82F1DAD0;
	sub_82F257F8(ctx, base);
	// 82F1DAD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DAD4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DAD8: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82F1DADC: 48007D1D  bl 0x82f257f8
	ctx.lr = 0x82F1DAE0;
	sub_82F257F8(ctx, base);
	// 82F1DAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DAE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DAE8: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 82F1DAEC: 48007D0D  bl 0x82f257f8
	ctx.lr = 0x82F1DAF0;
	sub_82F257F8(ctx, base);
	// 82F1DAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DAF4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F1DAF8: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 82F1DAFC: 48007CFD  bl 0x82f257f8
	ctx.lr = 0x82F1DB00;
	sub_82F257F8(ctx, base);
	// 82F1DB00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DB04: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F1DB08: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82F1DB0C: 48007CED  bl 0x82f257f8
	ctx.lr = 0x82F1DB10;
	sub_82F257F8(ctx, base);
	// 82F1DB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DB14: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F1DB18: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82F1DB1C: 48007CDD  bl 0x82f257f8
	ctx.lr = 0x82F1DB20;
	sub_82F257F8(ctx, base);
	// 82F1DB20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DB24: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82F1DB28: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82F1DB2C: 48007CCD  bl 0x82f257f8
	ctx.lr = 0x82F1DB30;
	sub_82F257F8(ctx, base);
	// 82F1DB30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DB34: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82F1DB38: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1DB3C: 48007CBD  bl 0x82f257f8
	ctx.lr = 0x82F1DB40;
	sub_82F257F8(ctx, base);
	// 82F1DB40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DB44: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82F1DB48: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 82F1DB4C: 48007CAD  bl 0x82f257f8
	ctx.lr = 0x82F1DB50;
	sub_82F257F8(ctx, base);
	// 82F1DB50: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82F1DB54: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82F1DB58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DB5C: 48007C9D  bl 0x82f257f8
	ctx.lr = 0x82F1DB60;
	sub_82F257F8(ctx, base);
	// 82F1DB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1DB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1DB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1DB6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1DB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DB78 size=36
    let mut pc: u32 = 0x82F1DB78;
    'dispatch: loop {
        match pc {
            0x82F1DB78 => {
    //   block [0x82F1DB78..0x82F1DB9C)
	// 82F1DB78: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82F1DB7C: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 82F1DB80: 41990230  bgt cr6, 0x82f1ddb0
	if ctx.cr[6].gt {
		sub_82F1DDB0(ctx, base);
		return;
	}
	// 82F1DB84: 3D8082F2  lis r12, -0x7d0e
	ctx.r[12].s64 = -2098069504;
	// 82F1DB88: 398CDB9C  addi r12, r12, -0x2464
	ctx.r[12].s64 = ctx.r[12].s64 + -9316;
	// 82F1DB8C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82F1DB90: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82F1DB94: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82F1DB98: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x82F1DC30
			return;
		},
		1 => {
			// ERROR: 0x82F1DC24
			return;
		},
		2 => {
			// ERROR: 0x82F1DC3C
			return;
		},
		3 => {
			// ERROR: 0x82F1DC60
			return;
		},
		4 => {
			// ERROR: 0x82F1DC90
			return;
		},
		5 => {
			// ERROR: 0x82F1DC6C
			return;
		},
		6 => {
			// ERROR: 0x82F1DC78
			return;
		},
		7 => {
			// ERROR: 0x82F1DC84
			return;
		},
		8 => {
			// ERROR: 0x82F1DC9C
			return;
		},
		9 => {
			// ERROR: 0x82F1DC48
			return;
		},
		10 => {
			// ERROR: 0x82F1DC54
			return;
		},
		11 => {
			// ERROR: 0x82F1DCC0
			return;
		},
		12 => {
			// ERROR: 0x82F1DD2C
			return;
		},
		13 => {
			// ERROR: 0x82F1DD44
			return;
		},
		14 => {
			// ERROR: 0x82F1DD50
			return;
		},
		15 => {
			// ERROR: 0x82F1DCFC
			return;
		},
		16 => {
			// ERROR: 0x82F1DD5C
			return;
		},
		17 => {
			// ERROR: 0x82F1DDB0
			return;
		},
		18 => {
			// ERROR: 0x82F1DD68
			return;
		},
		19 => {
			// ERROR: 0x82F1DD74
			return;
		},
		20 => {
			// ERROR: 0x82F1DCA8
			return;
		},
		21 => {
			// ERROR: 0x82F1DCB4
			return;
		},
		22 => {
			// ERROR: 0x82F1DCCC
			return;
		},
		23 => {
			// ERROR: 0x82F1DCD8
			return;
		},
		24 => {
			// ERROR: 0x82F1DCE4
			return;
		},
		25 => {
			// ERROR: 0x82F1DCF0
			return;
		},
		26 => {
			// ERROR: 0x82F1DD08
			return;
		},
		27 => {
			// ERROR: 0x82F1DD14
			return;
		},
		28 => {
			// ERROR: 0x82F1DD20
			return;
		},
		29 => {
			// ERROR: 0x82F1DD38
			return;
		},
		30 => {
			// ERROR: 0x82F1DD80
			return;
		},
		31 => {
			// ERROR: 0x82F1DD8C
			return;
		},
		32 => {
			// ERROR: 0x82F1DD98
			return;
		},
		33 => {
			// ERROR: 0x82F1DDA4
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DB9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DB9C size=148
    let mut pc: u32 = 0x82F1DB9C;
    'dispatch: loop {
        match pc {
            0x82F1DB9C => {
    //   block [0x82F1DB9C..0x82F1DC30)
	// 82F1DB9C: 82F1DC30  lwz r23, -0x23d0(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9168 as u32) ) } as u64;
	// 82F1DBA0: 82F1DC24  lwz r23, -0x23dc(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9180 as u32) ) } as u64;
	// 82F1DBA4: 82F1DC3C  lwz r23, -0x23c4(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9156 as u32) ) } as u64;
	// 82F1DBA8: 82F1DC60  lwz r23, -0x23a0(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9120 as u32) ) } as u64;
	// 82F1DBAC: 82F1DC90  lwz r23, -0x2370(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9072 as u32) ) } as u64;
	// 82F1DBB0: 82F1DC6C  lwz r23, -0x2394(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9108 as u32) ) } as u64;
	// 82F1DBB4: 82F1DC78  lwz r23, -0x2388(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9096 as u32) ) } as u64;
	// 82F1DBB8: 82F1DC84  lwz r23, -0x237c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9084 as u32) ) } as u64;
	// 82F1DBBC: 82F1DC9C  lwz r23, -0x2364(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9060 as u32) ) } as u64;
	// 82F1DBC0: 82F1DC48  lwz r23, -0x23b8(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9144 as u32) ) } as u64;
	// 82F1DBC4: 82F1DC54  lwz r23, -0x23ac(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9132 as u32) ) } as u64;
	// 82F1DBC8: 82F1DCC0  lwz r23, -0x2340(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9024 as u32) ) } as u64;
	// 82F1DBCC: 82F1DD2C  lwz r23, -0x22d4(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8916 as u32) ) } as u64;
	// 82F1DBD0: 82F1DD44  lwz r23, -0x22bc(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8892 as u32) ) } as u64;
	// 82F1DBD4: 82F1DD50  lwz r23, -0x22b0(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8880 as u32) ) } as u64;
	// 82F1DBD8: 82F1DCFC  lwz r23, -0x2304(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8964 as u32) ) } as u64;
	// 82F1DBDC: 82F1DD5C  lwz r23, -0x22a4(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8868 as u32) ) } as u64;
	// 82F1DBE0: 82F1DDB0  lwz r23, -0x2250(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8784 as u32) ) } as u64;
	// 82F1DBE4: 82F1DD68  lwz r23, -0x2298(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8856 as u32) ) } as u64;
	// 82F1DBE8: 82F1DD74  lwz r23, -0x228c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8844 as u32) ) } as u64;
	// 82F1DBEC: 82F1DCA8  lwz r23, -0x2358(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9048 as u32) ) } as u64;
	// 82F1DBF0: 82F1DCB4  lwz r23, -0x234c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9036 as u32) ) } as u64;
	// 82F1DBF4: 82F1DCCC  lwz r23, -0x2334(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9012 as u32) ) } as u64;
	// 82F1DBF8: 82F1DCD8  lwz r23, -0x2328(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-9000 as u32) ) } as u64;
	// 82F1DBFC: 82F1DCE4  lwz r23, -0x231c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8988 as u32) ) } as u64;
	// 82F1DC00: 82F1DCF0  lwz r23, -0x2310(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8976 as u32) ) } as u64;
	// 82F1DC04: 82F1DD08  lwz r23, -0x22f8(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8952 as u32) ) } as u64;
	// 82F1DC08: 82F1DD14  lwz r23, -0x22ec(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8940 as u32) ) } as u64;
	// 82F1DC0C: 82F1DD20  lwz r23, -0x22e0(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8928 as u32) ) } as u64;
	// 82F1DC10: 82F1DD38  lwz r23, -0x22c8(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8904 as u32) ) } as u64;
	// 82F1DC14: 82F1DD80  lwz r23, -0x2280(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8832 as u32) ) } as u64;
	// 82F1DC18: 82F1DD8C  lwz r23, -0x2274(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8820 as u32) ) } as u64;
	// 82F1DC1C: 82F1DD98  lwz r23, -0x2268(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8808 as u32) ) } as u64;
	// 82F1DC20: 82F1DDA4  lwz r23, -0x225c(r17)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(-8796 as u32) ) } as u64;
	// 82F1DC24: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC28: 386BEB30  addi r3, r11, -0x14d0
	ctx.r[3].s64 = ctx.r[11].s64 + -5328;
	// 82F1DC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC30 size=12
    let mut pc: u32 = 0x82F1DC30;
    'dispatch: loop {
        match pc {
            0x82F1DC30 => {
    //   block [0x82F1DC30..0x82F1DC3C)
	// 82F1DC30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC34: 386BEB20  addi r3, r11, -0x14e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5344;
	// 82F1DC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC3C size=12
    let mut pc: u32 = 0x82F1DC3C;
    'dispatch: loop {
        match pc {
            0x82F1DC3C => {
    //   block [0x82F1DC3C..0x82F1DC48)
	// 82F1DC3C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC40: 386BEB10  addi r3, r11, -0x14f0
	ctx.r[3].s64 = ctx.r[11].s64 + -5360;
	// 82F1DC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC48 size=12
    let mut pc: u32 = 0x82F1DC48;
    'dispatch: loop {
        match pc {
            0x82F1DC48 => {
    //   block [0x82F1DC48..0x82F1DC54)
	// 82F1DC48: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC4C: 386BEAFC  addi r3, r11, -0x1504
	ctx.r[3].s64 = ctx.r[11].s64 + -5380;
	// 82F1DC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC54 size=12
    let mut pc: u32 = 0x82F1DC54;
    'dispatch: loop {
        match pc {
            0x82F1DC54 => {
    //   block [0x82F1DC54..0x82F1DC60)
	// 82F1DC54: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC58: 386BEAE8  addi r3, r11, -0x1518
	ctx.r[3].s64 = ctx.r[11].s64 + -5400;
	// 82F1DC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC60 size=12
    let mut pc: u32 = 0x82F1DC60;
    'dispatch: loop {
        match pc {
            0x82F1DC60 => {
    //   block [0x82F1DC60..0x82F1DC6C)
	// 82F1DC60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC64: 386BEAD8  addi r3, r11, -0x1528
	ctx.r[3].s64 = ctx.r[11].s64 + -5416;
	// 82F1DC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC6C size=12
    let mut pc: u32 = 0x82F1DC6C;
    'dispatch: loop {
        match pc {
            0x82F1DC6C => {
    //   block [0x82F1DC6C..0x82F1DC78)
	// 82F1DC6C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC70: 386BEAC4  addi r3, r11, -0x153c
	ctx.r[3].s64 = ctx.r[11].s64 + -5436;
	// 82F1DC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC78 size=12
    let mut pc: u32 = 0x82F1DC78;
    'dispatch: loop {
        match pc {
            0x82F1DC78 => {
    //   block [0x82F1DC78..0x82F1DC84)
	// 82F1DC78: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC7C: 386BEAB4  addi r3, r11, -0x154c
	ctx.r[3].s64 = ctx.r[11].s64 + -5452;
	// 82F1DC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC84 size=12
    let mut pc: u32 = 0x82F1DC84;
    'dispatch: loop {
        match pc {
            0x82F1DC84 => {
    //   block [0x82F1DC84..0x82F1DC90)
	// 82F1DC84: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC88: 386BEAA0  addi r3, r11, -0x1560
	ctx.r[3].s64 = ctx.r[11].s64 + -5472;
	// 82F1DC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC90 size=12
    let mut pc: u32 = 0x82F1DC90;
    'dispatch: loop {
        match pc {
            0x82F1DC90 => {
    //   block [0x82F1DC90..0x82F1DC9C)
	// 82F1DC90: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DC94: 386BEA8C  addi r3, r11, -0x1574
	ctx.r[3].s64 = ctx.r[11].s64 + -5492;
	// 82F1DC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DC9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DC9C size=12
    let mut pc: u32 = 0x82F1DC9C;
    'dispatch: loop {
        match pc {
            0x82F1DC9C => {
    //   block [0x82F1DC9C..0x82F1DCA8)
	// 82F1DC9C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCA0: 386BEA70  addi r3, r11, -0x1590
	ctx.r[3].s64 = ctx.r[11].s64 + -5520;
	// 82F1DCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCA8 size=12
    let mut pc: u32 = 0x82F1DCA8;
    'dispatch: loop {
        match pc {
            0x82F1DCA8 => {
    //   block [0x82F1DCA8..0x82F1DCB4)
	// 82F1DCA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCAC: 386BEA58  addi r3, r11, -0x15a8
	ctx.r[3].s64 = ctx.r[11].s64 + -5544;
	// 82F1DCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCB4 size=12
    let mut pc: u32 = 0x82F1DCB4;
    'dispatch: loop {
        match pc {
            0x82F1DCB4 => {
    //   block [0x82F1DCB4..0x82F1DCC0)
	// 82F1DCB4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCB8: 386BEA40  addi r3, r11, -0x15c0
	ctx.r[3].s64 = ctx.r[11].s64 + -5568;
	// 82F1DCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCC0 size=12
    let mut pc: u32 = 0x82F1DCC0;
    'dispatch: loop {
        match pc {
            0x82F1DCC0 => {
    //   block [0x82F1DCC0..0x82F1DCCC)
	// 82F1DCC0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCC4: 386BEA30  addi r3, r11, -0x15d0
	ctx.r[3].s64 = ctx.r[11].s64 + -5584;
	// 82F1DCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCCC size=12
    let mut pc: u32 = 0x82F1DCCC;
    'dispatch: loop {
        match pc {
            0x82F1DCCC => {
    //   block [0x82F1DCCC..0x82F1DCD8)
	// 82F1DCCC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCD0: 386BEA18  addi r3, r11, -0x15e8
	ctx.r[3].s64 = ctx.r[11].s64 + -5608;
	// 82F1DCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCD8 size=12
    let mut pc: u32 = 0x82F1DCD8;
    'dispatch: loop {
        match pc {
            0x82F1DCD8 => {
    //   block [0x82F1DCD8..0x82F1DCE4)
	// 82F1DCD8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCDC: 386BE9F8  addi r3, r11, -0x1608
	ctx.r[3].s64 = ctx.r[11].s64 + -5640;
	// 82F1DCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCE4 size=12
    let mut pc: u32 = 0x82F1DCE4;
    'dispatch: loop {
        match pc {
            0x82F1DCE4 => {
    //   block [0x82F1DCE4..0x82F1DCF0)
	// 82F1DCE4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCE8: 386BE9E4  addi r3, r11, -0x161c
	ctx.r[3].s64 = ctx.r[11].s64 + -5660;
	// 82F1DCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCF0 size=12
    let mut pc: u32 = 0x82F1DCF0;
    'dispatch: loop {
        match pc {
            0x82F1DCF0 => {
    //   block [0x82F1DCF0..0x82F1DCFC)
	// 82F1DCF0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DCF4: 386BE9CC  addi r3, r11, -0x1634
	ctx.r[3].s64 = ctx.r[11].s64 + -5684;
	// 82F1DCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DCFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DCFC size=12
    let mut pc: u32 = 0x82F1DCFC;
    'dispatch: loop {
        match pc {
            0x82F1DCFC => {
    //   block [0x82F1DCFC..0x82F1DD08)
	// 82F1DCFC: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD00: 386BE9AC  addi r3, r11, -0x1654
	ctx.r[3].s64 = ctx.r[11].s64 + -5716;
	// 82F1DD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD08 size=12
    let mut pc: u32 = 0x82F1DD08;
    'dispatch: loop {
        match pc {
            0x82F1DD08 => {
    //   block [0x82F1DD08..0x82F1DD14)
	// 82F1DD08: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD0C: 386BE998  addi r3, r11, -0x1668
	ctx.r[3].s64 = ctx.r[11].s64 + -5736;
	// 82F1DD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD14 size=12
    let mut pc: u32 = 0x82F1DD14;
    'dispatch: loop {
        match pc {
            0x82F1DD14 => {
    //   block [0x82F1DD14..0x82F1DD20)
	// 82F1DD14: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD18: 386BE98C  addi r3, r11, -0x1674
	ctx.r[3].s64 = ctx.r[11].s64 + -5748;
	// 82F1DD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD20 size=12
    let mut pc: u32 = 0x82F1DD20;
    'dispatch: loop {
        match pc {
            0x82F1DD20 => {
    //   block [0x82F1DD20..0x82F1DD2C)
	// 82F1DD20: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD24: 386BE97C  addi r3, r11, -0x1684
	ctx.r[3].s64 = ctx.r[11].s64 + -5764;
	// 82F1DD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD2C size=12
    let mut pc: u32 = 0x82F1DD2C;
    'dispatch: loop {
        match pc {
            0x82F1DD2C => {
    //   block [0x82F1DD2C..0x82F1DD38)
	// 82F1DD2C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD30: 386BE96C  addi r3, r11, -0x1694
	ctx.r[3].s64 = ctx.r[11].s64 + -5780;
	// 82F1DD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD38 size=12
    let mut pc: u32 = 0x82F1DD38;
    'dispatch: loop {
        match pc {
            0x82F1DD38 => {
    //   block [0x82F1DD38..0x82F1DD44)
	// 82F1DD38: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD3C: 386BE958  addi r3, r11, -0x16a8
	ctx.r[3].s64 = ctx.r[11].s64 + -5800;
	// 82F1DD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD44 size=12
    let mut pc: u32 = 0x82F1DD44;
    'dispatch: loop {
        match pc {
            0x82F1DD44 => {
    //   block [0x82F1DD44..0x82F1DD50)
	// 82F1DD44: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD48: 386BE93C  addi r3, r11, -0x16c4
	ctx.r[3].s64 = ctx.r[11].s64 + -5828;
	// 82F1DD4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD50 size=12
    let mut pc: u32 = 0x82F1DD50;
    'dispatch: loop {
        match pc {
            0x82F1DD50 => {
    //   block [0x82F1DD50..0x82F1DD5C)
	// 82F1DD50: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD54: 386BE920  addi r3, r11, -0x16e0
	ctx.r[3].s64 = ctx.r[11].s64 + -5856;
	// 82F1DD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD5C size=12
    let mut pc: u32 = 0x82F1DD5C;
    'dispatch: loop {
        match pc {
            0x82F1DD5C => {
    //   block [0x82F1DD5C..0x82F1DD68)
	// 82F1DD5C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD60: 386BE908  addi r3, r11, -0x16f8
	ctx.r[3].s64 = ctx.r[11].s64 + -5880;
	// 82F1DD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD68 size=12
    let mut pc: u32 = 0x82F1DD68;
    'dispatch: loop {
        match pc {
            0x82F1DD68 => {
    //   block [0x82F1DD68..0x82F1DD74)
	// 82F1DD68: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD6C: 386BE8E8  addi r3, r11, -0x1718
	ctx.r[3].s64 = ctx.r[11].s64 + -5912;
	// 82F1DD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD74 size=12
    let mut pc: u32 = 0x82F1DD74;
    'dispatch: loop {
        match pc {
            0x82F1DD74 => {
    //   block [0x82F1DD74..0x82F1DD80)
	// 82F1DD74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD78: 386BE8D0  addi r3, r11, -0x1730
	ctx.r[3].s64 = ctx.r[11].s64 + -5936;
	// 82F1DD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD80 size=12
    let mut pc: u32 = 0x82F1DD80;
    'dispatch: loop {
        match pc {
            0x82F1DD80 => {
    //   block [0x82F1DD80..0x82F1DD8C)
	// 82F1DD80: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD84: 386BE8B4  addi r3, r11, -0x174c
	ctx.r[3].s64 = ctx.r[11].s64 + -5964;
	// 82F1DD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD8C size=12
    let mut pc: u32 = 0x82F1DD8C;
    'dispatch: loop {
        match pc {
            0x82F1DD8C => {
    //   block [0x82F1DD8C..0x82F1DD98)
	// 82F1DD8C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD90: 386BE8A4  addi r3, r11, -0x175c
	ctx.r[3].s64 = ctx.r[11].s64 + -5980;
	// 82F1DD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DD98 size=12
    let mut pc: u32 = 0x82F1DD98;
    'dispatch: loop {
        match pc {
            0x82F1DD98 => {
    //   block [0x82F1DD98..0x82F1DDA4)
	// 82F1DD98: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DD9C: 386BE894  addi r3, r11, -0x176c
	ctx.r[3].s64 = ctx.r[11].s64 + -5996;
	// 82F1DDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DDA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DDA4 size=12
    let mut pc: u32 = 0x82F1DDA4;
    'dispatch: loop {
        match pc {
            0x82F1DDA4 => {
    //   block [0x82F1DDA4..0x82F1DDB0)
	// 82F1DDA4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1DDA8: 386BE884  addi r3, r11, -0x177c
	ctx.r[3].s64 = ctx.r[11].s64 + -6012;
	// 82F1DDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1DDB0 size=8
    let mut pc: u32 = 0x82F1DDB0;
    'dispatch: loop {
        match pc {
            0x82F1DDB0 => {
    //   block [0x82F1DDB0..0x82F1DDB8)
	// 82F1DDB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F1DDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1DDB8 size=124
    let mut pc: u32 = 0x82F1DDB8;
    'dispatch: loop {
        match pc {
            0x82F1DDB8 => {
    //   block [0x82F1DDB8..0x82F1DE34)
	// 82F1DDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1DDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1DDC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1DDC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1DDC8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F1DDCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F1DDD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1DDD4: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82F1DDD8: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1DDDC: 48006A35  bl 0x82f24810
	ctx.lr = 0x82F1DDE0;
	sub_82F24810(ctx, base);
	// 82F1DDE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DDE4: 48019E25  bl 0x82f37c08
	ctx.lr = 0x82F1DDE8;
	sub_82F37C08(ctx, base);
	// 82F1DDE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DDEC: 4801746D  bl 0x82f35258
	ctx.lr = 0x82F1DDF0;
	sub_82F35258(ctx, base);
	// 82F1DDF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DDF4: 480167F5  bl 0x82f345e8
	ctx.lr = 0x82F1DDF8;
	sub_82F345E8(ctx, base);
	// 82F1DDF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DDFC: 48015C75  bl 0x82f33a70
	ctx.lr = 0x82F1DE00;
	sub_82F33A70(ctx, base);
	// 82F1DE00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DE04: 48055C85  bl 0x82f73a88
	ctx.lr = 0x82F1DE08;
	sub_82F73A88(ctx, base);
	// 82F1DE08: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F1DE0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F1DE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DE14: 99010050  stb r8, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u8 ) };
	// 82F1DE18: 88890000  lbz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1DE1C: 480069F5  bl 0x82f24810
	ctx.lr = 0x82F1DE20;
	sub_82F24810(ctx, base);
	// 82F1DE20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1DE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1DE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1DE2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1DE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1DE38 size=76
    let mut pc: u32 = 0x82F1DE38;
    'dispatch: loop {
        match pc {
            0x82F1DE38 => {
    //   block [0x82F1DE38..0x82F1DE84)
	// 82F1DE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1DE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1DE40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1DE44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1DE48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1DE4C: 4801DD05  bl 0x82f3bb50
	ctx.lr = 0x82F1DE50;
	sub_82F3BB50(ctx, base);
	// 82F1DE50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DE54: 4801C9F5  bl 0x82f3a848
	ctx.lr = 0x82F1DE58;
	sub_82F3A848(ctx, base);
	// 82F1DE58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DE5C: 4805680D  bl 0x82f74668
	ctx.lr = 0x82F1DE60;
	sub_82F74668(ctx, base);
	// 82F1DE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DE64: 4801AD5D  bl 0x82f38bc0
	ctx.lr = 0x82F1DE68;
	sub_82F38BC0(ctx, base);
	// 82F1DE68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DE6C: 48056025  bl 0x82f73e90
	ctx.lr = 0x82F1DE70;
	sub_82F73E90(ctx, base);
	// 82F1DE70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1DE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1DE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1DE7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1DE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1DE88 size=164
    let mut pc: u32 = 0x82F1DE88;
    'dispatch: loop {
        match pc {
            0x82F1DE88 => {
    //   block [0x82F1DE88..0x82F1DF2C)
	// 82F1DE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1DE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1DE90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1DE94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1DE98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F1DE9C: 48026B1D  bl 0x82f449b8
	ctx.lr = 0x82F1DEA0;
	sub_82F449B8(ctx, base);
	// 82F1DEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEA4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1DEA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F1DEAC: 48058D25  bl 0x82f76bd0
	ctx.lr = 0x82F1DEB0;
	sub_82F76BD0(ctx, base);
	// 82F1DEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEB4: 4805841D  bl 0x82f762d0
	ctx.lr = 0x82F1DEB8;
	sub_82F762D0(ctx, base);
	// 82F1DEB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEBC: 4802695D  bl 0x82f44818
	ctx.lr = 0x82F1DEC0;
	sub_82F44818(ctx, base);
	// 82F1DEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEC4: 480574DD  bl 0x82f753a0
	ctx.lr = 0x82F1DEC8;
	sub_82F753A0(ctx, base);
	// 82F1DEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DECC: 48026095  bl 0x82f43f60
	ctx.lr = 0x82F1DED0;
	sub_82F43F60(ctx, base);
	// 82F1DED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DED4: 480252ED  bl 0x82f431c0
	ctx.lr = 0x82F1DED8;
	sub_82F431C0(ctx, base);
	// 82F1DED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEDC: 4802418D  bl 0x82f42068
	ctx.lr = 0x82F1DEE0;
	sub_82F42068(ctx, base);
	// 82F1DEE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEE4: 48023465  bl 0x82f41348
	ctx.lr = 0x82F1DEE8;
	sub_82F41348(ctx, base);
	// 82F1DEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEEC: 48022485  bl 0x82f40370
	ctx.lr = 0x82F1DEF0;
	sub_82F40370(ctx, base);
	// 82F1DEF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEF4: 4802128D  bl 0x82f3f180
	ctx.lr = 0x82F1DEF8;
	sub_82F3F180(ctx, base);
	// 82F1DEF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DEFC: 48056FB5  bl 0x82f74eb0
	ctx.lr = 0x82F1DF00;
	sub_82F74EB0(ctx, base);
	// 82F1DF00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DF04: 4801FC45  bl 0x82f3db48
	ctx.lr = 0x82F1DF08;
	sub_82F3DB48(ctx, base);
	// 82F1DF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DF0C: 4801E99D  bl 0x82f3c8a8
	ctx.lr = 0x82F1DF10;
	sub_82F3C8A8(ctx, base);
	// 82F1DF10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1DF14: 480167A5  bl 0x82f346b8
	ctx.lr = 0x82F1DF18;
	sub_82F346B8(ctx, base);
	// 82F1DF18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1DF1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1DF20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1DF24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1DF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1DF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1DF30 size=220
    let mut pc: u32 = 0x82F1DF30;
    'dispatch: loop {
        match pc {
            0x82F1DF30 => {
    //   block [0x82F1DF30..0x82F1E00C)
	// 82F1DF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1DF34: 4828A239  bl 0x831a816c
	ctx.lr = 0x82F1DF38;
	sub_831A8130(ctx, base);
	// 82F1DF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1DF3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F1DF40: 4BFFFB11  bl 0x82f1da50
	ctx.lr = 0x82F1DF44;
	sub_82F1DA50(ctx, base);
	// 82F1DF44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF48: 4802B6D1  bl 0x82f49618
	ctx.lr = 0x82F1DF4C;
	sub_82F49618(ctx, base);
	// 82F1DF4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF50: 4802A0D9  bl 0x82f48028
	ctx.lr = 0x82F1DF54;
	sub_82F48028(ctx, base);
	// 82F1DF54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF58: 4BFFFE61  bl 0x82f1ddb8
	ctx.lr = 0x82F1DF5C;
	sub_82F1DDB8(ctx, base);
	// 82F1DF5C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F1DF60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F1DF64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF68: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 82F1DF6C: 888B0000  lbz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1DF70: 480068A1  bl 0x82f24810
	ctx.lr = 0x82F1DF74;
	sub_82F24810(ctx, base);
	// 82F1DF74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF78: 480294D9  bl 0x82f47450
	ctx.lr = 0x82F1DF7C;
	sub_82F47450(ctx, base);
	// 82F1DF7C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F1DF80: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F1DF84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF88: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82F1DF8C: 888A0000  lbz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1DF90: 48006881  bl 0x82f24810
	ctx.lr = 0x82F1DF94;
	sub_82F24810(ctx, base);
	// 82F1DF94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DF98: 4801DBB9  bl 0x82f3bb50
	ctx.lr = 0x82F1DF9C;
	sub_82F3BB50(ctx, base);
	// 82F1DF9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFA0: 4801C8A9  bl 0x82f3a848
	ctx.lr = 0x82F1DFA4;
	sub_82F3A848(ctx, base);
	// 82F1DFA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFA8: 480566C1  bl 0x82f74668
	ctx.lr = 0x82F1DFAC;
	sub_82F74668(ctx, base);
	// 82F1DFAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFB0: 4801AC11  bl 0x82f38bc0
	ctx.lr = 0x82F1DFB4;
	sub_82F38BC0(ctx, base);
	// 82F1DFB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFB8: 48055ED9  bl 0x82f73e90
	ctx.lr = 0x82F1DFBC;
	sub_82F73E90(ctx, base);
	// 82F1DFBC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F1DFC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFC4: 9BC10050  stb r30, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u8 ) };
	// 82F1DFC8: 88890000  lbz r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1DFCC: 48006845  bl 0x82f24810
	ctx.lr = 0x82F1DFD0;
	sub_82F24810(ctx, base);
	// 82F1DFD0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFD4: 4805A3D5  bl 0x82f783a8
	ctx.lr = 0x82F1DFD8;
	sub_82F783A8(ctx, base);
	// 82F1DFD8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F1DFDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFE0: 9BE10050  stb r31, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u8 ) };
	// 82F1DFE4: 88880000  lbz r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1DFE8: 48006829  bl 0x82f24810
	ctx.lr = 0x82F1DFEC;
	sub_82F24810(ctx, base);
	// 82F1DFEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFF0: 48027DB1  bl 0x82f45da0
	ctx.lr = 0x82F1DFF4;
	sub_82F45DA0(ctx, base);
	// 82F1DFF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1DFF8: 48027389  bl 0x82f45380
	ctx.lr = 0x82F1DFFC;
	sub_82F45380(ctx, base);
	// 82F1DFFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F1E000: 4BFFFE89  bl 0x82f1de88
	ctx.lr = 0x82F1E004;
	sub_82F1DE88(ctx, base);
	// 82F1E004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F1E008: 4828A1B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E010 size=28
    let mut pc: u32 = 0x82F1E010;
    'dispatch: loop {
        match pc {
            0x82F1E010 => {
    //   block [0x82F1E010..0x82F1E02C)
	// 82F1E010: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F1E014: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1E018: 419A0014  beq cr6, 0x82f1e02c
	if ctx.cr[6].eq {
		sub_82F1E02C(ctx, base);
		return;
	}
	// 82F1E01C: C004001C  lfs f0, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1E020: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1E024: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F1E028: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E02C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1E02C size=132
    let mut pc: u32 = 0x82F1E02C;
    'dispatch: loop {
        match pc {
            0x82F1E02C => {
    //   block [0x82F1E02C..0x82F1E0B0)
	// 82F1E02C: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1E0B0 size=176
    let mut pc: u32 = 0x82F1E0B0;
    'dispatch: loop {
        match pc {
            0x82F1E0B0 => {
    //   block [0x82F1E0B0..0x82F1E160)
	// 82F1E0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1E0B4: 4828A0B9  bl 0x831a816c
	ctx.lr = 0x82F1E0B8;
	sub_831A8130(ctx, base);
	// 82F1E0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1E0BC: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1E0C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1E0C4: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1E0C8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F1E0CC: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1E0D0: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 82F1E0D4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F1E0D8: 409A0010  bne cr6, 0x82f1e0e8
	if !ctx.cr[6].eq {
	pc = 0x82F1E0E8; continue 'dispatch;
	}
	// 82F1E0DC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F1E0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1E0E4: 4BF8879D  bl 0x82ea6880
	ctx.lr = 0x82F1E0E8;
	sub_82EA6880(ctx, base);
	// 82F1E0E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1E0EC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82F1E0F0: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1E0F4: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82F1E0F8: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1E0FC: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F1E100: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1E104: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82F1E108: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1E10C: 419A0014  beq cr6, 0x82f1e120
	if ctx.cr[6].eq {
	pc = 0x82F1E120; continue 'dispatch;
	}
	// 82F1E110: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F1E114: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1E118: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1E11C: 409AFFF4  bne cr6, 0x82f1e110
	if !ctx.cr[6].eq {
	pc = 0x82F1E110; continue 'dispatch;
	}
	// 82F1E120: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1E124: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 82F1E128: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1E12C: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F1E130: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1E134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1E138: 419A0014  beq cr6, 0x82f1e14c
	if ctx.cr[6].eq {
	pc = 0x82F1E14C; continue 'dispatch;
	}
	// 82F1E13C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F1E140: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1E144: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F1E148: 409AFFF4  bne cr6, 0x82f1e13c
	if !ctx.cr[6].eq {
	pc = 0x82F1E13C; continue 'dispatch;
	}
	// 82F1E14C: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82F1E150: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1E154: 9169000C  stw r11, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F1E158: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F1E15C: 4828A060  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1E160 size=232
    let mut pc: u32 = 0x82F1E160;
    'dispatch: loop {
        match pc {
            0x82F1E160 => {
    //   block [0x82F1E160..0x82F1E248)
	// 82F1E160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1E164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1E168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F1E16C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1E170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1E174: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1E178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F1E17C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1E180: 3BE30010  addi r31, r3, 0x10
	ctx.r[31].s64 = ctx.r[3].s64 + 16;
	// 82F1E184: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F1E188: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F1E18C: 409A0010  bne cr6, 0x82f1e19c
	if !ctx.cr[6].eq {
	pc = 0x82F1E19C; continue 'dispatch;
	}
	// 82F1E190: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82F1E194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1E198: 4BF886E9  bl 0x82ea6880
	ctx.lr = 0x82F1E19C;
	sub_82EA6880(ctx, base);
	// 82F1E19C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1E1A0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F1E1A4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1E1A8: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F1E1AC: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82F1E1B0: 7CCB4A14  add r6, r11, r9
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F1E1B4: 90FF0004  stw r7, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1E248 size=72
    let mut pc: u32 = 0x82F1E248;
    'dispatch: loop {
        match pc {
            0x82F1E248 => {
    //   block [0x82F1E248..0x82F1E290)
	// 82F1E248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1E24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1E250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1E254: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F1E258: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F1E25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F1E260: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F1E264: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82F1E268: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F1E26C: 40990014  ble cr6, 0x82f1e280
	if !ctx.cr[6].gt {
	pc = 0x82F1E280; continue 'dispatch;
	}
	// 82F1E270: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82F1E274: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F1E278: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F1E27C: 48000015  bl 0x82f1e290
	ctx.lr = 0x82F1E280;
	sub_82F1E290(ctx, base);
	// 82F1E280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1E284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1E288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1E28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1E290 size=412
    let mut pc: u32 = 0x82F1E290;
    'dispatch: loop {
        match pc {
            0x82F1E290 => {
    //   block [0x82F1E290..0x82F1E42C)
	// 82F1E290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1E294: 48289ECD  bl 0x831a8160
	ctx.lr = 0x82F1E298;
	sub_831A8130(ctx, base);
	// 82F1E298: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1E29C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F1E2A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F1E2A4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F1E2A8: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82F1E2AC: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82F1E2B0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82F1E2B4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82F1E2B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F1E2BC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F1E2C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1E2C4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F1E2C8: 550B2036  slwi r11, r8, 4
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F1E2CC: 7CEBF214  add r7, r11, r30
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1E430 size=4
    let mut pc: u32 = 0x82F1E430;
    'dispatch: loop {
        match pc {
            0x82F1E430 => {
    //   block [0x82F1E430..0x82F1E434)
	// 82F1E430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E438 size=84
    let mut pc: u32 = 0x82F1E438;
    'dispatch: loop {
        match pc {
            0x82F1E438 => {
    //   block [0x82F1E438..0x82F1E48C)
	// 82F1E438: 7C6B1E70  srawi r11, r3, 3
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[3].s32 >> 3) as i64;
	// 82F1E43C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1E440: 7D4B0194  addze r10, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82F1E444: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F1E448: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 82F1E44C: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82F1E450: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 82F1E454: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1E458: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82F1E45C: C009964C  lfs f0, -0x69b4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1E460: FD606818  frsp f11, f13
	ctx.f[11].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82F1E464: C1A7EB44  lfs f13, -0x14bc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-5308 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1E468: 3CA08332  lis r5, -0x7cce
	ctx.r[5].s64 = -2093875200;
	// 82F1E46C: C18608A8  lfs f12, 0x8a8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2216 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1E470: 3C808332  lis r4, -0x7cce
	ctx.r[4].s64 = -2093875200;
	// 82F1E474: ED4B0028  fsubs f10, f11, f0
	ctx.f[10].f64 = (((ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F1E478: EC0A0372  fmuls f0, f10, f13
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F1E47C: D00502F0  stfs f0, 0x2f0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(752 as u32), tmp.u32 ) };
	// 82F1E480: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 82F1E484: D00402F4  stfs f0, 0x2f4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(756 as u32), tmp.u32 ) };
	// 82F1E488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E490 size=8
    let mut pc: u32 = 0x82F1E490;
    'dispatch: loop {
        match pc {
            0x82F1E490 => {
    //   block [0x82F1E490..0x82F1E498)
	// 82F1E490: C0230014  lfs f1, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F1E494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1E498 size=8
    let mut pc: u32 = 0x82F1E498;
    'dispatch: loop {
        match pc {
            0x82F1E498 => {
    //   block [0x82F1E498..0x82F1E4A0)
	// 82F1E498: 38600012  li r3, 0x12
	ctx.r[3].s64 = 18;
	// 82F1E49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1E4A0 size=96
    let mut pc: u32 = 0x82F1E4A0;
    'dispatch: loop {
        match pc {
            0x82F1E4A0 => {
    //   block [0x82F1E4A0..0x82F1E500)
	// 82F1E4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1E4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1E4A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1E4AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1E4B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1E4B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82F1E4B8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F1E4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1E4C0: 388AF628  addi r4, r10, -0x9d8
	ctx.r[4].s64 = ctx.r[10].s64 + -2520;
	// 82F1E4C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1E4C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1E4CC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1E4D0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F1E4D4: 4E800421  bctrl
	ctx.lr = 0x82F1E4D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1E4D8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1E4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1E4E0: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1E4E4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F1E4E8: 4E800421  bctrl
	ctx.lr = 0x82F1E4EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1E4EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1E4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1E4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1E4F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1E4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E500 size=44
    let mut pc: u32 = 0x82F1E500;
    'dispatch: loop {
        match pc {
            0x82F1E500 => {
    //   block [0x82F1E500..0x82F1E52C)
	// 82F1E500: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1E504: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F1E508: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F1E50C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1E510: C00A9528  lfs f0, -0x6ad8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1E514: C1AB0A94  lfs f13, 0xa94(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2708 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1E518: FD80081E  fctiwz f12, f1
	ctx.f[12].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 82F1E51C: D981FFF0  stfd f12, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[12].u64 ) };
	// 82F1E520: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 82F1E524: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1E528: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E52C size=24
    let mut pc: u32 = 0x82F1E52C;
    'dispatch: loop {
        match pc {
            0x82F1E52C => {
    //   block [0x82F1E52C..0x82F1E544)
	// 82F1E52C: EC21002A  fadds f1, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82F1E530: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 82F1E534: 4198FFE4  blt cr6, 0x82f1e518
	if ctx.cr[6].lt {
		sub_82F1E500(ctx, base);
		return;
	}
	// 82F1E538: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1E53C: C02B08A8  lfs f1, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F1E540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1E548 size=296
    let mut pc: u32 = 0x82F1E548;
    'dispatch: loop {
        match pc {
            0x82F1E548 => {
    //   block [0x82F1E548..0x82F1E670)
	// 82F1E548: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82F1E54C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82F1E550: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 82F1E554: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 82F1E558: 3881FFF0  addi r4, r1, -0x10
	ctx.r[4].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E670 size=32
    let mut pc: u32 = 0x82F1E670;
    'dispatch: loop {
        match pc {
            0x82F1E670 => {
    //   block [0x82F1E670..0x82F1E690)
	// 82F1E670: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1E674: D0230014  stfs f1, 0x14(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82F1E678: EDA0082A  fadds f13, f0, f1
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82F1E67C: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F1E680: C1830010  lfs f12, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F1E684: ED6C082A  fadds f11, f12, f1
	ctx.f[11].f64 = ((ctx.f[12].f64 + ctx.f[1].f64) as f32) as f64;
	// 82F1E688: D163003C  stfs f11, 0x3c(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 82F1E68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1E690 size=568
    let mut pc: u32 = 0x82F1E690;
    'dispatch: loop {
        match pc {
            0x82F1E690 => {
    //   block [0x82F1E690..0x82F1E8C8)
	// 82F1E690: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82F1E694: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1E8C8 size=272
    let mut pc: u32 = 0x82F1E8C8;
    'dispatch: loop {
        match pc {
            0x82F1E8C8 => {
    //   block [0x82F1E8C8..0x82F1E9D8)
	// 82F1E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1E8CC: 4828989D  bl 0x831a8168
	ctx.lr = 0x82F1E8D0;
	sub_831A8130(ctx, base);
	// 82F1E8D0: 3565FFFF  addic. r11, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1E8D4: 41800100  blt 0x82f1e9d4
	if ctx.cr[0].lt {
	pc = 0x82F1E9D4; continue 'dispatch;
	}
	// 82F1E8D8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F1E8DC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1E8E0: 38A30014  addi r5, r3, 0x14
	ctx.r[5].s64 = ctx.r[3].s64 + 20;
	// 82F1E8E4: 3BE30050  addi r31, r3, 0x50
	ctx.r[31].s64 = ctx.r[3].s64 + 80;
	// 82F1E8E8: 3BC30040  addi r30, r3, 0x40
	ctx.r[30].s64 = ctx.r[3].s64 + 64;
	// 82F1E8EC: C14A08A8  lfs f10, 0x8a8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F1E8F0: 3FA08332  lis r29, -0x7cce
	ctx.r[29].s64 = -2093875200;
	// 82F1E8F4: C1699450  lfs f11, -0x6bb0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F1E8F8: A0E40000  lhz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1E8FC: C01D02F4  lfs f0, 0x2f4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(756 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1E900: 54E9073E  clrlwi r9, r7, 0x1c
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x0000000Fu64;
	// 82F1E904: 54FC06F6  rlwinm r28, r7, 0, 0x1b, 0x1b
	ctx.r[28].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 82F1E908: F921FFC8  std r9, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.r[9].u64 ) };
	// 82F1E90C: C9A1FFC8  lfd f13, -0x38(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82F1E910: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 82F1E914: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82F1E918: FD206018  frsp f9, f12
	ctx.f[9].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82F1E91C: 54E9CFFE  rlwinm r9, r7, 0x19, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[7].u32 as u64 & 0x0000007Fu64;
	// 82F1E920: 54E8D7FE  rlwinm r8, r7, 0x1a, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0x0000003Fu64;
	// 82F1E924: 54E7DFFE  rlwinm r7, r7, 0x1b, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82F1E928: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F1E92C: ED09582A  fadds f8, f9, f11
	ctx.f[8].f64 = ((ctx.f[9].f64 + ctx.f[11].f64) as f32) as f64;
	// 82F1E930: EC080032  fmuls f0, f8, f0
	ctx.f[0].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F1E934: ECE0503C  fnmsubs f7, f0, f0, f10
	ctx.f[7].f64 = -(((ctx.f[0].f64 * ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82F1E938: EDA0382C  fsqrts f13, f7
	ctx.f[13].f64 = ((ctx.f[7].f64).sqrt() as f32) as f64;
	// 82F1E93C: 419A000C  beq cr6, 0x82f1e948
	if ctx.cr[6].eq {
	pc = 0x82F1E948; continue 'dispatch;
	}
	// 82F1E940: FD806890  fmr f12, f13
	ctx.f[12].f64 = ctx.f[13].f64;
	// 82F1E944: 4800000C  b 0x82f1e950
	pc = 0x82F1E950; continue 'dispatch;
	// 82F1E948: FD800090  fmr f12, f0
	ctx.f[12].f64 = ctx.f[0].f64;
	// 82F1E94C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82F1E950: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82F1E954: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F1E958: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82F1E95C: 409A000C  bne cr6, 0x82f1e968
	if !ctx.cr[6].eq {
	pc = 0x82F1E968; continue 'dispatch;
	}
	// 82F1E960: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F1E964: D001FFC0  stfs f0, -0x40(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82F1E968: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82F1E96C: 409A000C  bne cr6, 0x82f1e978
	if !ctx.cr[6].eq {
	pc = 0x82F1E978; continue 'dispatch;
	}
	// 82F1E970: FC006050  fneg f0, f12
	ctx.f[0].u64 = ctx.f[12].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F1E974: D001FFC4  stfs f0, -0x3c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82F1E978: 3901FFC0  addi r8, r1, -0x40
	ctx.r[8].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1E9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1E9D8 size=56
    let mut pc: u32 = 0x82F1E9D8;
    'dispatch: loop {
        match pc {
            0x82F1E9D8 => {
    //   block [0x82F1E9D8..0x82F1EA10)
	// 82F1E9D8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82F1E9DC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F1E9E0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1E9E4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1EA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1EA10 size=16
    let mut pc: u32 = 0x82F1EA10;
    'dispatch: loop {
        match pc {
            0x82F1EA10 => {
    //   block [0x82F1EA10..0x82F1EA20)
	// 82F1EA10: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1EA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1EA20 size=584
    let mut pc: u32 = 0x82F1EA20;
    'dispatch: loop {
        match pc {
            0x82F1EA20 => {
    //   block [0x82F1EA20..0x82F1EC68)
	// 82F1EA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1EA24: 48289731  bl 0x831a8154
	ctx.lr = 0x82F1EA28;
	sub_831A8130(ctx, base);
	// 82F1EA28: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82F1EA2C: C0030014  lfs f0, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1EA30: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82F1EA34: D001FF90  stfs f0, -0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), tmp.u32 ) };
	// 82F1EA38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1EA3C: ED600032  fmuls f11, f0, f0
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82F1EA40: 3901FF94  addi r8, r1, -0x6c
	ctx.r[8].s64 = ctx.r[1].s64 + -108;
	// 82F1EA44: 3CE08208  lis r7, -0x7df8
	ctx.r[7].s64 = -2113404928;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1EC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1EC68 size=184
    let mut pc: u32 = 0x82F1EC68;
    'dispatch: loop {
        match pc {
            0x82F1EC68 => {
    //   block [0x82F1EC68..0x82F1ED20)
	// 82F1EC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1EC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1EC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1EC74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1EC78: D0430010  stfs f2, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F1EC7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F1EC80: FD600890  fmr f11, f1
	ctx.f[11].f64 = ctx.f[1].f64;
	// 82F1EC84: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82F1EC88: 390BEB54  addi r8, r11, -0x14ac
	ctx.r[8].s64 = ctx.r[11].s64 + -5292;
	// 82F1EC8C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F1EC90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F1EC94: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1EC98: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82F1EC9C: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1ECA0: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82F1ECA4: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82F1ECA8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F1ECAC: C00902F8  lfs f0, 0x2f8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1ECB0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F1ECB4: 40980018  bge cr6, 0x82f1eccc
	if !ctx.cr[6].lt {
	pc = 0x82F1ECCC; continue 'dispatch;
	}
	// 82F1ECB8: 4BFFF849  bl 0x82f1e500
	ctx.lr = 0x82F1ECBC;
	sub_82F1E500(ctx, base);
	// 82F1ECBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1ECC0: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1ECC4: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82F1ECC8: D00902F8  stfs f0, 0x2f8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(760 as u32), tmp.u32 ) };
	// 82F1ECCC: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1ED20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1ED20 size=112
    let mut pc: u32 = 0x82F1ED20;
    'dispatch: loop {
        match pc {
            0x82F1ED20 => {
    //   block [0x82F1ED20..0x82F1ED90)
	// 82F1ED20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1ED24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1ED28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1ED2C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1ED30: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F1ED34: 392AEB54  addi r9, r10, -0x14ac
	ctx.r[9].s64 = ctx.r[10].s64 + -5292;
	// 82F1ED38: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82F1ED3C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82F1ED40: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1ED44: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F1ED48: 419A0030  beq cr6, 0x82f1ed78
	if ctx.cr[6].eq {
	pc = 0x82F1ED78; continue 'dispatch;
	}
	// 82F1ED4C: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82F1ED50: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1ED54: C00902F8  lfs f0, 0x2f8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1ED58: C1AB08A4  lfs f13, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1ED5C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F1ED60: 40980018  bge cr6, 0x82f1ed78
	if !ctx.cr[6].lt {
	pc = 0x82F1ED78; continue 'dispatch;
	}
	// 82F1ED64: 4BFFF79D  bl 0x82f1e500
	ctx.lr = 0x82F1ED68;
	sub_82F1E500(ctx, base);
	// 82F1ED68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F1ED6C: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1ED70: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 82F1ED74: D00902F8  stfs f0, 0x2f8(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(760 as u32), tmp.u32 ) };
	// 82F1ED78: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82F1ED7C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F1ED80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1ED84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1ED88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1ED8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1ED90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1ED90 size=420
    let mut pc: u32 = 0x82F1ED90;
    'dispatch: loop {
        match pc {
            0x82F1ED90 => {
    //   block [0x82F1ED90..0x82F1EF34)
	// 82F1ED90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1ED94: 482893C5  bl 0x831a8158
	ctx.lr = 0x82F1ED98;
	sub_831A8130(ctx, base);
	// 82F1ED98: E9040000  ld r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82F1ED9C: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82F1EDA0: E8A40008  ld r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82F1EDA4: 38E1FF60  addi r7, r1, -0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + -160;
	// 82F1EDA8: EBC40010  ld r30, 0x10(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	// 82F1EDAC: 3B61FFA0  addi r27, r1, -0x60
	ctx.r[27].s64 = ctx.r[1].s64 + -96;
	// 82F1EDB0: EB840018  ld r28, 0x18(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	// 82F1EDB4: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82F1EDB8: EB440020  ld r26, 0x20(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	// 82F1EDBC: 3BA1FF80  addi r29, r1, -0x80
	ctx.r[29].s64 = ctx.r[1].s64 + -128;
	// 82F1EDC0: EB240028  ld r25, 0x28(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	// 82F1EDC4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F1EDC8: EB040030  ld r24, 0x30(r4)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	// 82F1EDCC: 3921FF90  addi r9, r1, -0x70
	ctx.r[9].s64 = ctx.r[1].s64 + -112;
	// 82F1EDD0: E8840038  ld r4, 0x38(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	// 82F1EDD4: 39430030  addi r10, r3, 0x30
	ctx.r[10].s64 = ctx.r[3].s64 + 48;
	// 82F1EDD8: F9070000  std r8, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82F1EDDC: 3901FF60  addi r8, r1, -0xa0
	ctx.r[8].s64 = ctx.r[1].s64 + -160;
	// 82F1EDE0: F8A70008  std r5, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82F1EDE4: 38E1FF70  addi r7, r1, -0x90
	ctx.r[7].s64 = ctx.r[1].s64 + -144;
	// 82F1EDE8: 38A1FF80  addi r5, r1, -0x80
	ctx.r[5].s64 = ctx.r[1].s64 + -128;
	// 82F1EDEC: FBDF0000  std r30, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 82F1EDF0: FB9F0008  std r28, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u64 ) };
	// 82F1EDF4: F89B0008  std r4, 8(r27)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 82F1EDF8: 3881FFA0  addi r4, r1, -0x60
	ctx.r[4].s64 = ctx.r[1].s64 + -96;
	// 82F1EDFC: FB5D0000  std r26, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1EF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F1EF38 size=1416
    let mut pc: u32 = 0x82F1EF38;
    'dispatch: loop {
        match pc {
            0x82F1EF38 => {
    //   block [0x82F1EF38..0x82F1F4C0)
	// 82F1EF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1EF3C: 48289221  bl 0x831a815c
	ctx.lr = 0x82F1EF40;
	sub_831A8130(ctx, base);
	// 82F1EF40: DBC1FFB0  stfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 82F1EF44: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82F1EF48: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1EF4C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1EF50: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F1EF54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F1EF58: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F1EF5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F1EF60: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F1EF64: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1EF68: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1EF6C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1EF70: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F1EF74: 40980020  bge cr6, 0x82f1ef94
	if !ctx.cr[6].lt {
	pc = 0x82F1EF94; continue 'dispatch;
	}
	// 82F1EF78: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1EF7C: 3909EB94  addi r8, r9, -0x146c
	ctx.r[8].s64 = ctx.r[9].s64 + -5228;
	// 82F1EF80: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1EF84: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F1EF88: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F1EF8C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F1EF90: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F1EF94: 39640040  addi r11, r4, 0x40
	ctx.r[11].s64 = ctx.r[4].s64 + 64;
	// 82F1EF98: C0040014  lfs f0, 0x14(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1EF9C: 39440050  addi r10, r4, 0x50
	ctx.r[10].s64 = ctx.r[4].s64 + 80;
	// 82F1EFA0: C1A40010  lfs f13, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1EFA4: 39240010  addi r9, r4, 0x10
	ctx.r[9].s64 = ctx.r[4].s64 + 16;
	// 82F1EFA8: EFE0682A  fadds f31, f0, f13
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82F1EFAC: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82F1EFB0: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F4C0 size=8
    let mut pc: u32 = 0x82F1F4C0;
    'dispatch: loop {
        match pc {
            0x82F1F4C0 => {
    //   block [0x82F1F4C0..0x82F1F4C8)
	// 82F1F4C0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82F1F4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1F4C8 size=96
    let mut pc: u32 = 0x82F1F4C8;
    'dispatch: loop {
        match pc {
            0x82F1F4C8 => {
    //   block [0x82F1F4C8..0x82F1F528)
	// 82F1F4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1F4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1F4D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1F4D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1F4D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1F4DC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1F4E0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F1F4E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1F4E8: 388AEBA4  addi r4, r10, -0x145c
	ctx.r[4].s64 = ctx.r[10].s64 + -5212;
	// 82F1F4EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1F4F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1F4F4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1F4F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F1F4FC: 4E800421  bctrl
	ctx.lr = 0x82F1F500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1F500: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1F504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1F508: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1F50C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F1F510: 4E800421  bctrl
	ctx.lr = 0x82F1F514;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1F514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1F518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1F51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1F520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1F524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F528 size=76
    let mut pc: u32 = 0x82F1F528;
    'dispatch: loop {
        match pc {
            0x82F1F528 => {
    //   block [0x82F1F528..0x82F1F574)
	// 82F1F528: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F574(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F574 size=20
    let mut pc: u32 = 0x82F1F574;
    'dispatch: loop {
        match pc {
            0x82F1F574 => {
    //   block [0x82F1F574..0x82F1F588)
	// 82F1F574: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F588 size=8
    let mut pc: u32 = 0x82F1F588;
    'dispatch: loop {
        match pc {
            0x82F1F588 => {
    //   block [0x82F1F588..0x82F1F590)
	// 82F1F588: 3565FFFF  addic. r11, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1F58C: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F590 size=44
    let mut pc: u32 = 0x82F1F590;
    'dispatch: loop {
        match pc {
            0x82F1F590 => {
    //   block [0x82F1F590..0x82F1F5BC)
	// 82F1F590: 39430020  addi r10, r3, 0x20
	ctx.r[10].s64 = ctx.r[3].s64 + 32;
	// 82F1F594: A1240000  lhz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1F598: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1F59C: 38840002  addi r4, r4, 2
	ctx.r[4].s64 = ctx.r[4].s64 + 2;
	// 82F1F5A0: 65283F00  oris r8, r9, 0x3f00
	ctx.r[8].u64 = ctx.r[9].u64 | 1056964608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F5C0 size=56
    let mut pc: u32 = 0x82F1F5C0;
    'dispatch: loop {
        match pc {
            0x82F1F5C0 => {
    //   block [0x82F1F5C0..0x82F1F5F8)
	// 82F1F5C0: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82F1F5C4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F1F5C8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F1F5CC: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F5F8 size=16
    let mut pc: u32 = 0x82F1F5F8;
    'dispatch: loop {
        match pc {
            0x82F1F5F8 => {
    //   block [0x82F1F5F8..0x82F1F608)
	// 82F1F5F8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F608 size=36
    let mut pc: u32 = 0x82F1F608;
    'dispatch: loop {
        match pc {
            0x82F1F608 => {
    //   block [0x82F1F608..0x82F1F62C)
	// 82F1F608: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82F1F60C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82F1F610: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F630 size=280
    let mut pc: u32 = 0x82F1F630;
    'dispatch: loop {
        match pc {
            0x82F1F630 => {
    //   block [0x82F1F630..0x82F1F748)
	// 82F1F630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1F634: 48288B31  bl 0x831a8164
	ctx.lr = 0x82F1F638;
	sub_831A8130(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1F748 size=76
    let mut pc: u32 = 0x82F1F748;
    'dispatch: loop {
        match pc {
            0x82F1F748 => {
    //   block [0x82F1F748..0x82F1F794)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F794(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1F794 size=20
    let mut pc: u32 = 0x82F1F794;
    'dispatch: loop {
        match pc {
            0x82F1F794 => {
    //   block [0x82F1F794..0x82F1F7A8)
	// 82F1F794: C1A1FFF4  lfs f13, -0xc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F1F798: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F1F79C: 4198000C  blt cr6, 0x82f1f7a8
	if ctx.cr[6].lt {
		sub_82F1F7A8(ctx, base);
		return;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1F7A8 size=32
    let mut pc: u32 = 0x82F1F7A8;
    'dispatch: loop {
        match pc {
            0x82F1F7A8 => {
    //   block [0x82F1F7A8..0x82F1F7C8)
	// 82F1F7A8: 3961FFF4  addi r11, r1, -0xc
	ctx.r[11].s64 = ctx.r[1].s64 + -12;
	// 82F1F7AC: EC006824  fdivs f0, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82F1F7B0: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1F7C8 size=76
    let mut pc: u32 = 0x82F1F7C8;
    'dispatch: loop {
        match pc {
            0x82F1F7C8 => {
    //   block [0x82F1F7C8..0x82F1F814)
	// 82F1F7C8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F1F7CC: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F1F7D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F1F7D4: 392BE6F4  addi r9, r11, -0x190c
	ctx.r[9].s64 = ctx.r[11].s64 + -6412;
	// 82F1F7D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F1F7DC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F1F7E0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82F1F7E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F1F7E8: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82F1F7EC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82F1F7F0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1F818 size=268
    let mut pc: u32 = 0x82F1F818;
    'dispatch: loop {
        match pc {
            0x82F1F818 => {
    //   block [0x82F1F818..0x82F1F924)
	// 82F1F818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1F81C: 4828893D  bl 0x831a8158
	ctx.lr = 0x82F1F820;
	sub_831A8130(ctx, base);
	// 82F1F820: 3961FF60  addi r11, r1, -0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + -160;
	// 82F1F824: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F1F828: EDA0082A  fadds f13, f0, f1
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82F1F82C: D1A1FF60  stfs f13, -0xa0(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), tmp.u32 ) };
	// 82F1F830: E9440000  ld r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82F1F834: 38E1FF60  addi r7, r1, -0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + -160;
	// 82F1F838: E9040008  ld r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82F1F83C: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 82F1F840: E8A40010  ld r5, 0x10(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	// 82F1F844: 3BA1FF80  addi r29, r1, -0x80
	ctx.r[29].s64 = ctx.r[1].s64 + -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1F928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1F928 size=1148
    let mut pc: u32 = 0x82F1F928;
    'dispatch: loop {
        match pc {
            0x82F1F928 => {
    //   block [0x82F1F928..0x82F1FDA4)
	// 82F1F928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1F92C: 48288825  bl 0x831a8150
	ctx.lr = 0x82F1F930;
	sub_831A8130(ctx, base);
	// 82F1F930: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1F934: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1F938: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F1F93C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82F1F940: 7F8A5A14  add r28, r10, r11
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F1F944: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82F1F948: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F1F94C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F1F950: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F1F954: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F1F958: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1F95C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F1F960: 40980020  bge cr6, 0x82f1f980
	if !ctx.cr[6].lt {
	pc = 0x82F1F980; continue 'dispatch;
	}
	// 82F1F964: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F1F968: 3909EBB4  addi r8, r9, -0x144c
	ctx.r[8].s64 = ctx.r[9].s64 + -5196;
	// 82F1F96C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F1F970: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F1F974: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F1F978: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F1F97C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F1F980: 3BB90030  addi r29, r25, 0x30
	ctx.r[29].s64 = ctx.r[25].s64 + 48;
	// 82F1F984: 3BD90020  addi r30, r25, 0x20
	ctx.r[30].s64 = ctx.r[25].s64 + 32;
	// 82F1F988: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F1F98C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F1F990: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F1F994: 4BFFFDB5  bl 0x82f1f748
	ctx.lr = 0x82F1F998;
	sub_82F1F748(ctx, base);
	// 82F1F998: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F1FDA8 size=96
    let mut pc: u32 = 0x82F1FDA8;
    'dispatch: loop {
        match pc {
            0x82F1FDA8 => {
    //   block [0x82F1FDA8..0x82F1FE08)
	// 82F1FDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F1FDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F1FDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F1FDB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F1FDB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F1FDBC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1FDC0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F1FDC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1FDC8: 388AEC40  addi r4, r10, -0x13c0
	ctx.r[4].s64 = ctx.r[10].s64 + -5056;
	// 82F1FDCC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1FDD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F1FDD4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F1FDD8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F1FDDC: 4E800421  bctrl
	ctx.lr = 0x82F1FDE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1FDE0: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F1FDE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F1FDE8: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F1FDEC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F1FDF0: 4E800421  bctrl
	ctx.lr = 0x82F1FDF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F1FDF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F1FDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F1FDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F1FE00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F1FE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FE08 size=8
    let mut pc: u32 = 0x82F1FE08;
    'dispatch: loop {
        match pc {
            0x82F1FE08 => {
    //   block [0x82F1FE08..0x82F1FE10)
	// 82F1FE08: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82F1FE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FE10 size=16
    let mut pc: u32 = 0x82F1FE10;
    'dispatch: loop {
        match pc {
            0x82F1FE10 => {
    //   block [0x82F1FE10..0x82F1FE20)
	// 82F1FE10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F1FE14: 394B9EAC  addi r10, r11, -0x6154
	ctx.r[10].s64 = ctx.r[11].s64 + -24916;
	// 82F1FE18: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F1FE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FE20 size=56
    let mut pc: u32 = 0x82F1FE20;
    'dispatch: loop {
        match pc {
            0x82F1FE20 => {
    //   block [0x82F1FE20..0x82F1FE58)
	// 82F1FE20: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F1FE58 size=8
    let mut pc: u32 = 0x82F1FE58;
    'dispatch: loop {
        match pc {
            0x82F1FE58 => {
    //   block [0x82F1FE58..0x82F1FE60)
	// 82F1FE58: D003002C  stfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F1FE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FE60 size=16
    let mut pc: u32 = 0x82F1FE60;
    'dispatch: loop {
        match pc {
            0x82F1FE60 => {
    //   block [0x82F1FE60..0x82F1FE70)
	// 82F1FE60: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FE70 size=156
    let mut pc: u32 = 0x82F1FE70;
    'dispatch: loop {
        match pc {
            0x82F1FE70 => {
    //   block [0x82F1FE70..0x82F1FF0C)
	// 82F1FE70: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FF10 size=148
    let mut pc: u32 = 0x82F1FF10;
    'dispatch: loop {
        match pc {
            0x82F1FF10 => {
    //   block [0x82F1FF10..0x82F1FFA4)
	// 82F1FF10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F1FF14: E9230020  ld r9, 0x20(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82F1FF18: E9030028  ld r8, 0x28(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FFA8 size=16
    let mut pc: u32 = 0x82F1FFA8;
    'dispatch: loop {
        match pc {
            0x82F1FFA8 => {
    //   block [0x82F1FFA8..0x82F1FFB8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FFB8 size=8
    let mut pc: u32 = 0x82F1FFB8;
    'dispatch: loop {
        match pc {
            0x82F1FFB8 => {
    //   block [0x82F1FFB8..0x82F1FFC0)
	// 82F1FFB8: 3565FFFF  addic. r11, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F1FFBC: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F1FFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F1FFC0 size=64
    let mut pc: u32 = 0x82F1FFC0;
    'dispatch: loop {
        match pc {
            0x82F1FFC0 => {
    //   block [0x82F1FFC0..0x82F20000)
	// 82F1FFC0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F1FFC4: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82F1FFC8: 394AEBC0  addi r10, r10, -0x1440
	ctx.r[10].s64 = ctx.r[10].s64 + -5184;
	// 82F1FFCC: A1040000  lhz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F20000 size=216
    let mut pc: u32 = 0x82F20000;
    'dispatch: loop {
        match pc {
            0x82F20000 => {
    //   block [0x82F20000..0x82F200D8)
	// 82F20000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F20004: 4828815D  bl 0x831a8160
	ctx.lr = 0x82F20008;
	sub_831A8130(ctx, base);
	// 82F20008: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F2000C: E9430020  ld r10, 0x20(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	// 82F20010: E9230028  ld r9, 0x28(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	// 82F20014: 3901FFB0  addi r8, r1, -0x50
	ctx.r[8].s64 = ctx.r[1].s64 + -80;
	// 82F20018: 38EBEBC0  addi r7, r11, -0x1440
	ctx.r[7].s64 = ctx.r[11].s64 + -5184;
	// 82F2001C: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F20020: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82F20024: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
	// 82F20028: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82F2002C: F9480000  std r10, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82F20030: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F20034: F9280008  std r9, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F200D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F200D8 size=120
    let mut pc: u32 = 0x82F200D8;
    'dispatch: loop {
        match pc {
            0x82F200D8 => {
    //   block [0x82F200D8..0x82F20150)
	// 82F200D8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F200DC: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F200E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F200E4: 392BE7E4  addi r9, r11, -0x181c
	ctx.r[9].s64 = ctx.r[11].s64 + -6172;
	// 82F200E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F200EC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F200F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82F200F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F200F8: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82F200FC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82F20100: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F20104: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20108: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82F2010C: 80A40004  lwz r5, 4(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F20110: 90A30024  stw r5, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[5].u32 ) };
	// 82F20114: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F20118: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82F2011C: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F20120: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82F20124: C0030020  lfs f0, 0x20(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F20128: C1A30024  lfs f13, 0x24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F2012C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F20130: 41980008  blt cr6, 0x82f20138
	if ctx.cr[6].lt {
	pc = 0x82F20138; continue 'dispatch;
	}
	// 82F20134: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82F20138: C1A30028  lfs f13, 0x28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F2013C: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F20140: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82F20144: 4098000C  bge cr6, 0x82f20150
	if !ctx.cr[6].lt {
		sub_82F20150(ctx, base);
		return;
	}
	// 82F20148: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F2014C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F20150 size=8
    let mut pc: u32 = 0x82F20150;
    'dispatch: loop {
        match pc {
            0x82F20150 => {
    //   block [0x82F20150..0x82F20158)
	// 82F20150: D003002C  stfs f0, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82F20154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F20158 size=736
    let mut pc: u32 = 0x82F20158;
    'dispatch: loop {
        match pc {
            0x82F20158 => {
    //   block [0x82F20158..0x82F20438)
	// 82F20158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2015C: 48288005  bl 0x831a8160
	ctx.lr = 0x82F20160;
	sub_831A8130(ctx, base);
	// 82F20160: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20164: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F20168: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F2016C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F20170: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F20174: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F20178: 40980020  bge cr6, 0x82f20198
	if !ctx.cr[6].lt {
	pc = 0x82F20198; continue 'dispatch;
	}
	// 82F2017C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F20180: 3909EC4C  addi r8, r9, -0x13b4
	ctx.r[8].s64 = ctx.r[9].s64 + -5044;
	// 82F20184: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F20188: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F2018C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F20190: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F20194: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F20198: 3961FFA0  addi r11, r1, -0x60
	ctx.r[11].s64 = ctx.r[1].s64 + -96;
	// 82F2019C: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F201A0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F201A4: D001FFA0  stfs f0, -0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F20438 size=320
    let mut pc: u32 = 0x82F20438;
    'dispatch: loop {
        match pc {
            0x82F20438 => {
    //   block [0x82F20438..0x82F20578)
	// 82F20438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2043C: 48287D21  bl 0x831a815c
	ctx.lr = 0x82F20440;
	sub_831A8130(ctx, base);
	// 82F20440: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F20444: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82F20448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F2044C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82F20450: 7FCB2214  add r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F20454: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F20458: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F2045C: 7F8B202E  lwzx r28, r11, r4
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82F20460: 7D2929D6  mullw r9, r9, r5
	ctx.r[9].s64 = (ctx.r[9].s32 as i64) * (ctx.r[5].s32 as i64);
	// 82F20464: 7F695214  add r27, r9, r10
	ctx.r[27].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F20468: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82F2046C: 419A0100  beq cr6, 0x82f2056c
	if ctx.cr[6].eq {
	pc = 0x82F2056C; continue 'dispatch;
	}
	// 82F20470: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F20474: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F20478: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F2047C: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82F20480: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F20484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F20488: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F2048C: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20490: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F20494: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F20498: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F2049C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F204A0: 4E800421  bctrl
	ctx.lr = 0x82F204A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F204A4: 88E30000  lbz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F204A8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F204AC: 419A00C0  beq cr6, 0x82f2056c
	if ctx.cr[6].eq {
	pc = 0x82F2056C; continue 'dispatch;
	}
	// 82F204B0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F204B4: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82F204B8: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F204BC: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82F204C0: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 82F204C4: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82F204C8: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F20578 size=300
    let mut pc: u32 = 0x82F20578;
    'dispatch: loop {
        match pc {
            0x82F20578 => {
    //   block [0x82F20578..0x82F206A4)
	// 82F20578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F20580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F20584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F20588: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2058C: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20590: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F20594: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F20598: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F2059C: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F205A0: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F205A4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F205A8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F205AC: 40980020  bge cr6, 0x82f205cc
	if !ctx.cr[6].lt {
	pc = 0x82F205CC; continue 'dispatch;
	}
	// 82F205B0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F205B4: 3888EC54  addi r4, r8, -0x13ac
	ctx.r[4].s64 = ctx.r[8].s64 + -5036;
	// 82F205B8: 90890000  stw r4, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F205BC: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F205C0: 3909000C  addi r8, r9, 0xc
	ctx.r[8].s64 = ctx.r[9].s64 + 12;
	// 82F205C4: 90890004  stw r4, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F205C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F205CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F205D0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F205D4: 90EA000C  stw r7, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F205D8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F205DC: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F205E0: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82F205E4: 409A0008  bne cr6, 0x82f205ec
	if !ctx.cr[6].eq {
	pc = 0x82F205EC; continue 'dispatch;
	}
	// 82F205E8: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82F205EC: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F205F0: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F205F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F205F8: 419A001C  beq cr6, 0x82f20614
	if ctx.cr[6].eq {
	pc = 0x82F20614; continue 'dispatch;
	}
	// 82F205FC: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F20600: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82F20604: 409A0008  bne cr6, 0x82f2060c
	if !ctx.cr[6].eq {
	pc = 0x82F2060C; continue 'dispatch;
	}
	// 82F20608: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82F2060C: 916A0044  stw r11, 0x44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F20610: 48000008  b 0x82f20618
	pc = 0x82F20618; continue 'dispatch;
	// 82F20614: 912A0044  stw r9, 0x44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82F20618: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F2061C: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F206A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F206A8 size=276
    let mut pc: u32 = 0x82F206A8;
    'dispatch: loop {
        match pc {
            0x82F206A8 => {
    //   block [0x82F206A8..0x82F207BC)
	// 82F206A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F206AC: 48287AC1  bl 0x831a816c
	ctx.lr = 0x82F206B0;
	sub_831A8130(ctx, base);
	// 82F206B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F206B4: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F206B8: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F206BC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F206C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F206C4: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F206C8: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F206CC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F206D0: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82F206D4: 40980020  bge cr6, 0x82f206f4
	if !ctx.cr[6].lt {
	pc = 0x82F206F4; continue 'dispatch;
	}
	// 82F206D8: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F206DC: 3884EC60  addi r4, r4, -0x13a0
	ctx.r[4].s64 = ctx.r[4].s64 + -5024;
	// 82F206E0: 90890000  stw r4, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F206E4: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F206E8: 3BA9000C  addi r29, r9, 0xc
	ctx.r[29].s64 = ctx.r[9].s64 + 12;
	// 82F206EC: 90890004  stw r4, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F206F0: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82F206F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F206F8: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F206FC: 910A000C  stw r8, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F20700: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F20704: 912A0010  stw r9, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F20708: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82F2070C: 409A0008  bne cr6, 0x82f20714
	if !ctx.cr[6].eq {
	pc = 0x82F20714; continue 'dispatch;
	}
	// 82F20710: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82F20714: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F20718: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F2071C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F20720: 419A001C  beq cr6, 0x82f2073c
	if ctx.cr[6].eq {
	pc = 0x82F2073C; continue 'dispatch;
	}
	// 82F20724: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F20728: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82F2072C: 409A0008  bne cr6, 0x82f20734
	if !ctx.cr[6].eq {
	pc = 0x82F20734; continue 'dispatch;
	}
	// 82F20730: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82F20734: 916A0044  stw r11, 0x44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F20738: 48000008  b 0x82f20740
	pc = 0x82F20740; continue 'dispatch;
	// 82F2073C: 912A0044  stw r9, 0x44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82F20740: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F20744: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F207C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F207C0 size=456
    let mut pc: u32 = 0x82F207C0;
    'dispatch: loop {
        match pc {
            0x82F207C0 => {
    //   block [0x82F207C0..0x82F20988)
	// 82F207C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F207C4: 48287989  bl 0x831a814c
	ctx.lr = 0x82F207C8;
	sub_831A8130(ctx, base);
	// 82F207C8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F207CC: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F207D0: 3AA00018  li r21, 0x18
	ctx.r[21].s64 = 24;
	// 82F207D4: 7D374B78  mr r23, r9
	ctx.r[23].u64 = ctx.r[9].u64;
	// 82F207D8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F207DC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F207E0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F207E4: 7D7AA82E  lwzx r11, r26, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 82F207E8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F207EC: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F207F0: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82F207F4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F207F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F207FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F20800: 40980020  bge cr6, 0x82f20820
	if !ctx.cr[6].lt {
	pc = 0x82F20820; continue 'dispatch;
	}
	// 82F20804: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F20808: 3909EC70  addi r8, r9, -0x1390
	ctx.r[8].s64 = ctx.r[9].s64 + -5008;
	// 82F2080C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F20810: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F20814: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F20818: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F2081C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F20820: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F20988 size=264
    let mut pc: u32 = 0x82F20988;
    'dispatch: loop {
        match pc {
            0x82F20988 => {
    //   block [0x82F20988..0x82F20A90)
	// 82F20988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2098C: 482877D9  bl 0x831a8164
	ctx.lr = 0x82F20990;
	sub_831A8130(ctx, base);
	// 82F20990: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F20994: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20998: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F2099C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F209A0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F209A4: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F209A8: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F209AC: 80CB0004  lwz r6, 4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F209B0: 7F062040  cmplw cr6, r6, r4
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82F209B4: 40980020  bge cr6, 0x82f209d4
	if !ctx.cr[6].lt {
	pc = 0x82F209D4; continue 'dispatch;
	}
	// 82F209B8: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F209BC: 3884EC80  addi r4, r4, -0x1380
	ctx.r[4].s64 = ctx.r[4].s64 + -4992;
	// 82F209C0: 90860000  stw r4, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F209C4: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F209C8: 3B66000C  addi r27, r6, 0xc
	ctx.r[27].s64 = ctx.r[6].s64 + 12;
	// 82F209CC: 90860004  stw r4, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F209D0: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82F209D4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F209D8: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F209DC: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 82F209E0: 409A0008  bne cr6, 0x82f209e8
	if !ctx.cr[6].eq {
	pc = 0x82F209E8; continue 'dispatch;
	}
	// 82F209E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F209E8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F209EC: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82F209F0: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82F209F4: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F209F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F209FC: 419A0010  beq cr6, 0x82f20a0c
	if ctx.cr[6].eq {
	pc = 0x82F20A0C; continue 'dispatch;
	}
	// 82F20A00: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F20A04: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 82F20A08: 409A0008  bne cr6, 0x82f20a10
	if !ctx.cr[6].eq {
	pc = 0x82F20A10; continue 'dispatch;
	}
	// 82F20A0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F20A10: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F20A14: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F20A18: 39250010  addi r9, r5, 0x10
	ctx.r[9].s64 = ctx.r[5].s64 + 16;
	// 82F20A1C: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82F20A20: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F20A24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F20A28: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F20A90 size=96
    let mut pc: u32 = 0x82F20A90;
    'dispatch: loop {
        match pc {
            0x82F20A90 => {
    //   block [0x82F20A90..0x82F20AF0)
	// 82F20A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F20A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F20A98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F20A9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F20AA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F20AA4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F20AA8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F20AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F20AB0: 388AEC90  addi r4, r10, -0x1370
	ctx.r[4].s64 = ctx.r[10].s64 + -4976;
	// 82F20AB4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20AB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F20ABC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F20AC0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F20AC4: 4E800421  bctrl
	ctx.lr = 0x82F20AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F20AC8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20ACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F20AD0: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F20AD4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F20AD8: 4E800421  bctrl
	ctx.lr = 0x82F20ADC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F20ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F20AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F20AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F20AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F20AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20AF0 size=28
    let mut pc: u32 = 0x82F20AF0;
    'dispatch: loop {
        match pc {
            0x82F20AF0 => {
    //   block [0x82F20AF0..0x82F20B0C)
	// 82F20AF0: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F20AF4: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 82F20AF8: 7D245A14  add r9, r4, r11
	ctx.r[9].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82F20AFC: 5528043E  clrlwi r8, r9, 0x10
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000FFFFu64;
	// 82F20B00: 7D474430  srw r7, r10, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[7].u64 = 0;
	} else {
		ctx.r[7].u64 = ((ctx.r[10].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 82F20B04: 54E306FE  clrlwi r3, r7, 0x1b
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 82F20B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20B10 size=20
    let mut pc: u32 = 0x82F20B10;
    'dispatch: loop {
        match pc {
            0x82F20B10 => {
    //   block [0x82F20B10..0x82F20B24)
	// 82F20B10: 89630017  lbz r11, 0x17(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(23 as u32) ) } as u64;
	// 82F20B14: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F20B18: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F20B1C: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F20B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20B28 size=16
    let mut pc: u32 = 0x82F20B28;
    'dispatch: loop {
        match pc {
            0x82F20B28 => {
    //   block [0x82F20B28..0x82F20B38)
	// 82F20B28: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20B38 size=72
    let mut pc: u32 = 0x82F20B38;
    'dispatch: loop {
        match pc {
            0x82F20B38 => {
    //   block [0x82F20B38..0x82F20B80)
	// 82F20B38: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 82F20B3C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F20B40: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82F20B44: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20B80 size=108
    let mut pc: u32 = 0x82F20B80;
    'dispatch: loop {
        match pc {
            0x82F20B80 => {
    //   block [0x82F20B80..0x82F20BEC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20BF0 size=228
    let mut pc: u32 = 0x82F20BF0;
    'dispatch: loop {
        match pc {
            0x82F20BF0 => {
    //   block [0x82F20BF0..0x82F20CD4)
	// 82F20BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F20BF4: 48287579  bl 0x831a816c
	ctx.lr = 0x82F20BF8;
	sub_831A8130(ctx, base);
	// 82F20BF8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20CD8 size=92
    let mut pc: u32 = 0x82F20CD8;
    'dispatch: loop {
        match pc {
            0x82F20CD8 => {
    //   block [0x82F20CD8..0x82F20D34)
	// 82F20CD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F20CDC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F20CE0: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82F20CE4: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 82F20CE8: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82F20CEC: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20D34 size=88
    let mut pc: u32 = 0x82F20D34;
    'dispatch: loop {
        match pc {
            0x82F20D34 => {
    //   block [0x82F20D34..0x82F20D8C)
	// 82F20D34: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20D90 size=96
    let mut pc: u32 = 0x82F20D90;
    'dispatch: loop {
        match pc {
            0x82F20D90 => {
    //   block [0x82F20D90..0x82F20DF0)
	// 82F20D90: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F20DF0 size=112
    let mut pc: u32 = 0x82F20DF0;
    'dispatch: loop {
        match pc {
            0x82F20DF0 => {
    //   block [0x82F20DF0..0x82F20E60)
	// 82F20DF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F20DF4: FC006890  fmr f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82F20DF8: C1A1FFF0  lfs f13, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F20DFC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82F20E00: 40990008  ble cr6, 0x82f20e08
	if !ctx.cr[6].gt {
	pc = 0x82F20E08; continue 'dispatch;
	}
	// 82F20E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F20E08: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82F20E0C: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82F20E10: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F20E14: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 82F20E18: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20E60 size=8
    let mut pc: u32 = 0x82F20E60;
    'dispatch: loop {
        match pc {
            0x82F20E60 => {
    //   block [0x82F20E60..0x82F20E68)
	// 82F20E60: 3565FFFF  addic. r11, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F20E64: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20E68 size=108
    let mut pc: u32 = 0x82F20E68;
    'dispatch: loop {
        match pc {
            0x82F20E68 => {
    //   block [0x82F20E68..0x82F20ED4)
	// 82F20E68: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 82F20E6C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 82F20E70: 38E30050  addi r7, r3, 0x50
	ctx.r[7].s64 = ctx.r[3].s64 + 80;
	// 82F20E74: 39290340  addi r9, r9, 0x340
	ctx.r[9].s64 = ctx.r[9].s64 + 832;
	// 82F20E78: 390A036C  addi r8, r10, 0x36c
	ctx.r[8].s64 = ctx.r[10].s64 + 876;
	// 82F20E7C: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F20E80: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F20E84: 5545103E  rotlwi r5, r10, 2
	ctx.r[5].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F20E88: 7D45402E  lwzx r10, r5, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F20E8C: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82F20E90: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F20ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F20ED8 size=1312
    let mut pc: u32 = 0x82F20ED8;
    'dispatch: loop {
        match pc {
            0x82F20ED8 => {
    //   block [0x82F20ED8..0x82F213F8)
	// 82F20ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F20EDC: 48287279  bl 0x831a8154
	ctx.lr = 0x82F20EE0;
	sub_831A8130(ctx, base);
	// 82F20EE0: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82F20EE4: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F213F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F213F8 size=632
    let mut pc: u32 = 0x82F213F8;
    'dispatch: loop {
        match pc {
            0x82F213F8 => {
    //   block [0x82F213F8..0x82F21670)
	// 82F213F8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F213FC: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F21400: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F21404: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F21408: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F2140C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F21410: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F21414: 40980020  bge cr6, 0x82f21434
	if !ctx.cr[6].lt {
	pc = 0x82F21434; continue 'dispatch;
	}
	// 82F21418: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F2141C: 38E8EC9C  addi r7, r8, -0x1364
	ctx.r[7].s64 = ctx.r[8].s64 + -4964;
	// 82F21420: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F21424: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F21428: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F2142C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F21430: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F21434: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 82F21438: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 82F2143C: 39040040  addi r8, r4, 0x40
	ctx.r[8].s64 = ctx.r[4].s64 + 64;
	// 82F21440: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 82F21444: 3881FFE4  addi r4, r1, -0x1c
	ctx.r[4].s64 = ctx.r[1].s64 + -28;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21670 size=60
    let mut pc: u32 = 0x82F21670;
    'dispatch: loop {
        match pc {
            0x82F21670 => {
    //   block [0x82F21670..0x82F216AC)
	// 82F21670: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21674: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F21678: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F2167C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F21680: 40980020  bge cr6, 0x82f216a0
	if !ctx.cr[6].lt {
	pc = 0x82F216A0; continue 'dispatch;
	}
	// 82F21684: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F21688: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F2168C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F21690: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F21694: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F21698: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F2169C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F216A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F216A4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F216A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F216B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F216B0 size=96
    let mut pc: u32 = 0x82F216B0;
    'dispatch: loop {
        match pc {
            0x82F216B0 => {
    //   block [0x82F216B0..0x82F21710)
	// 82F216B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F216B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F216B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F216BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F216C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F216C4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F216C8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82F216CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F216D0: 388AECAC  addi r4, r10, -0x1354
	ctx.r[4].s64 = ctx.r[10].s64 + -4948;
	// 82F216D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F216D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F216DC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F216E0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F216E4: 4E800421  bctrl
	ctx.lr = 0x82F216E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F216E8: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F216EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F216F0: 80E80018  lwz r7, 0x18(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F216F4: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F216F8: 4E800421  bctrl
	ctx.lr = 0x82F216FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F216FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F21700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F21704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F21708: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F2170C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F21710 size=172
    let mut pc: u32 = 0x82F21710;
    'dispatch: loop {
        match pc {
            0x82F21710 => {
    //   block [0x82F21710..0x82F217BC)
	// 82F21710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F21714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F21718: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F2171C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F21720: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F21724: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F21728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2172C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F21730: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F21734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F21738: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82F2173C: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82F21740: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F21744: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F21748: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F2174C: 80EB0020  lwz r7, 0x20(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F21750: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82F21754: 912100A0  stw r9, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[9].u32 ) };
	// 82F21758: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F2175C: 91010080  stw r8, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 82F21760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F21764: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F21768: 4E800421  bctrl
	ctx.lr = 0x82F2176C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F2176C: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F21770: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F21774: 419A0030  beq cr6, 0x82f217a4
	if ctx.cr[6].eq {
	pc = 0x82F217A4; continue 'dispatch;
	}
	// 82F21778: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F2177C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F21780: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F21784: 4BF857B5  bl 0x82ea6f38
	ctx.lr = 0x82F21788;
	sub_82EA6F38(ctx, base);
	// 82F21788: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2178C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F21790: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F21794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F21798: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2179C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F217A0: 4E800421  bctrl
	ctx.lr = 0x82F217A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F217A4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F217A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F217AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F217B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F217B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F217B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F217C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F217C0 size=348
    let mut pc: u32 = 0x82F217C0;
    'dispatch: loop {
        match pc {
            0x82F217C0 => {
    //   block [0x82F217C0..0x82F2191C)
	// 82F217C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F217C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F217C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F217CC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82F217D0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F217D4: 39490010  addi r10, r9, 0x10
	ctx.r[10].s64 = ctx.r[9].s64 + 16;
	// 82F217D8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F217DC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82F217E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21920 size=20
    let mut pc: u32 = 0x82F21920;
    'dispatch: loop {
        match pc {
            0x82F21920 => {
    //   block [0x82F21920..0x82F21934)
	// 82F21920: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F21924: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82F21928: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2192C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F21930: 4D800020  bltlr
	if ctx.cr[0].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21934 size=76
    let mut pc: u32 = 0x82F21934;
    'dispatch: loop {
        match pc {
            0x82F21934 => {
    //   block [0x82F21934..0x82F21980)
	// 82F21934: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21980 size=172
    let mut pc: u32 = 0x82F21980;
    'dispatch: loop {
        match pc {
            0x82F21980 => {
    //   block [0x82F21980..0x82F21A2C)
	// 82F21980: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F21984: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F21988: 392BDD8C  addi r9, r11, -0x2274
	ctx.r[9].s64 = ctx.r[11].s64 + -8820;
	// 82F2198C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F21990: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F21994: 38E0001B  li r7, 0x1b
	ctx.r[7].s64 = 27;
	// 82F21998: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F2199C: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82F219A0: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82F219A4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21A30 size=108
    let mut pc: u32 = 0x82F21A30;
    'dispatch: loop {
        match pc {
            0x82F21A30 => {
    //   block [0x82F21A30..0x82F21A9C)
	// 82F21A30: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F21A34: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F21A38: 392BDD8C  addi r9, r11, -0x2274
	ctx.r[9].s64 = ctx.r[11].s64 + -8820;
	// 82F21A3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F21A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F21A44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F21A48: 3960001B  li r11, 0x1b
	ctx.r[11].s64 = 27;
	// 82F21A4C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F21A50: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F21A54: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 82F21A58: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21AA0 size=24
    let mut pc: u32 = 0x82F21AA0;
    'dispatch: loop {
        match pc {
            0x82F21AA0 => {
    //   block [0x82F21AA0..0x82F21AB8)
	// 82F21AA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F21AA4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F21AA8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82F21AAC: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82F21AB0: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82F21AB4: 4BFF8324  b 0x82f19dd8
	sub_82F19DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21AB8 size=444
    let mut pc: u32 = 0x82F21AB8;
    'dispatch: loop {
        match pc {
            0x82F21AB8 => {
    //   block [0x82F21AB8..0x82F21C74)
	// 82F21AB8: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82F21ABC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21AC0: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F21AC4: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F21AC8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F21ACC: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F21AD0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F21AD4: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F21AD8: 40980020  bge cr6, 0x82f21af8
	if !ctx.cr[6].lt {
	pc = 0x82F21AF8; continue 'dispatch;
	}
	// 82F21ADC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F21AE0: 38E8ECB8  addi r7, r8, -0x1348
	ctx.r[7].s64 = ctx.r[8].s64 + -4936;
	// 82F21AE4: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F21AE8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F21AEC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F21AF0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F21AF4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F21AF8: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21C78 size=20
    let mut pc: u32 = 0x82F21C78;
    'dispatch: loop {
        match pc {
            0x82F21C78 => {
    //   block [0x82F21C78..0x82F21C8C)
	// 82F21C78: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F21C7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21C80: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F21C84: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F21C88: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F21C90 size=144
    let mut pc: u32 = 0x82F21C90;
    'dispatch: loop {
        match pc {
            0x82F21C90 => {
    //   block [0x82F21C90..0x82F21D20)
	// 82F21C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F21C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F21C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F21C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F21CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F21CA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F21CA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F21CAC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F21CB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F21CB4: 388AECCC  addi r4, r10, -0x1334
	ctx.r[4].s64 = ctx.r[10].s64 + -4916;
	// 82F21CB8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21CBC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F21CC0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F21CC4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F21CC8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F21CCC: 4E800421  bctrl
	ctx.lr = 0x82F21CD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F21CD0: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21CD4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F21CD8: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F21CDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F21CE0: 3888ECC4  addi r4, r8, -0x133c
	ctx.r[4].s64 = ctx.r[8].s64 + -4924;
	// 82F21CE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F21CE8: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F21CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F21CF0: 4E800421  bctrl
	ctx.lr = 0x82F21CF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F21CF4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F21CFC: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F21D00: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F21D04: 4E800421  bctrl
	ctx.lr = 0x82F21D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F21D08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F21D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F21D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F21D14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F21D18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F21D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21D20 size=8
    let mut pc: u32 = 0x82F21D20;
    'dispatch: loop {
        match pc {
            0x82F21D20 => {
    //   block [0x82F21D20..0x82F21D28)
	// 82F21D20: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82F21D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F21D28 size=52
    let mut pc: u32 = 0x82F21D28;
    'dispatch: loop {
        match pc {
            0x82F21D28 => {
    //   block [0x82F21D28..0x82F21D5C)
	// 82F21D28: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F21D60 size=136
    let mut pc: u32 = 0x82F21D60;
    'dispatch: loop {
        match pc {
            0x82F21D60 => {
    //   block [0x82F21D60..0x82F21DE8)
	// 82F21D60: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F21D64: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F21D68: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F21D6C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F21D70: 392BE374  addi r9, r11, -0x1c8c
	ctx.r[9].s64 = ctx.r[11].s64 + -7308;
	// 82F21D74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F21D78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F21D7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F21D80: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 82F21D84: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F21D88: 396ABD10  addi r11, r10, -0x42f0
	ctx.r[11].s64 = ctx.r[10].s64 + -17136;
	// 82F21D8C: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F21D90: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F21D94: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F21D98: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82F21D9C: A1440004  lhz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F21DA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F21DA4: 419A0010  beq cr6, 0x82f21db4
	if ctx.cr[6].eq {
	pc = 0x82F21DB4; continue 'dispatch;
	}
	// 82F21DA8: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F21DAC: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 82F21DB0: B1440006  sth r10, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F21DB4: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F21DE8 size=108
    let mut pc: u32 = 0x82F21DE8;
    'dispatch: loop {
        match pc {
            0x82F21DE8 => {
    //   block [0x82F21DE8..0x82F21E54)
	// 82F21DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F21DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F21DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F21DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F21DF8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F21DFC: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F21E00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F21E04: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F21E08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F21E0C: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 82F21E10: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F21E14: 4BF7F195  bl 0x82ea0fa8
	ctx.lr = 0x82F21E18;
	sub_82EA0FA8(ctx, base);
	// 82F21E18: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F21E1C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F21E20: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F21E24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F21E28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21E2C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F21E30: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F21E34: 4E800421  bctrl
	ctx.lr = 0x82F21E38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F21E38: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F21E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F21E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F21E44: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82F21E48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F21E4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F21E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F21E58 size=216
    let mut pc: u32 = 0x82F21E58;
    'dispatch: loop {
        match pc {
            0x82F21E58 => {
    //   block [0x82F21E58..0x82F21F30)
	// 82F21E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F21E5C: 4828630D  bl 0x831a8168
	ctx.lr = 0x82F21E60;
	sub_831A8130(ctx, base);
	// 82F21E60: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F21E64: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 82F21E68: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F21E6C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F21E70: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 82F21E74: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 82F21E78: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F21F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F21F30 size=436
    let mut pc: u32 = 0x82F21F30;
    'dispatch: loop {
        match pc {
            0x82F21F30 => {
    //   block [0x82F21F30..0x82F220E4)
	// 82F21F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F21F34: 48286231  bl 0x831a8164
	ctx.lr = 0x82F21F38;
	sub_831A8130(ctx, base);
	// 82F21F38: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F21F3C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F21F40: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82F21F44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F21F48: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F21F4C: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F21F50: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F21F54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F21F58: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F21F5C: 40980020  bge cr6, 0x82f21f7c
	if !ctx.cr[6].lt {
	pc = 0x82F21F7C; continue 'dispatch;
	}
	// 82F21F60: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F21F64: 3909ECDC  addi r8, r9, -0x1324
	ctx.r[8].s64 = ctx.r[9].s64 + -4900;
	// 82F21F68: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F21F6C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F21F70: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 82F21F74: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F21F78: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F21F7C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82F21F80: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F21F84: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82F21F88: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F21F8C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F21F90: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F21F94: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F21F98: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F21F9C: 4200FFF0  bdnz 0x82f21f8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F21F8C; continue 'dispatch;
	}
	// 82F21FA0: 3BE40020  addi r31, r4, 0x20
	ctx.r[31].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F220E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F220E8 size=384
    let mut pc: u32 = 0x82F220E8;
    'dispatch: loop {
        match pc {
            0x82F220E8 => {
    //   block [0x82F220E8..0x82F22268)
	// 82F220E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F220EC: 48286079  bl 0x831a8164
	ctx.lr = 0x82F220F0;
	sub_831A8130(ctx, base);
	// 82F220F0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F220F4: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F220F8: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F220FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22100: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82F22104: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F22108: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F2210C: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F22110: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F22114: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F22118: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F2211C: 40980020  bge cr6, 0x82f2213c
	if !ctx.cr[6].lt {
	pc = 0x82F2213C; continue 'dispatch;
	}
	// 82F22120: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F22124: 38E8ECEC  addi r7, r8, -0x1314
	ctx.r[7].s64 = ctx.r[8].s64 + -4884;
	// 82F22128: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F2212C: 7CCC42E6  mftb r6, 0x10c
	ctx.r[6].u64 = crate::rt::rdtsc_u64();
	// 82F22130: 3889000C  addi r4, r9, 0xc
	ctx.r[4].s64 = ctx.r[9].s64 + 12;
	// 82F22134: 90C90004  stw r6, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F22138: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F2213C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82F22140: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82F22144: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82F22148: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F2214C: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F22150: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F22154: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82F22158: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82F2215C: 4200FFF0  bdnz 0x82f2214c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F2214C; continue 'dispatch;
	}
	// 82F22160: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22268 size=188
    let mut pc: u32 = 0x82F22268;
    'dispatch: loop {
        match pc {
            0x82F22268 => {
    //   block [0x82F22268..0x82F22324)
	// 82F22268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2226C: 48285EFD  bl 0x831a8168
	ctx.lr = 0x82F22270;
	sub_831A8130(ctx, base);
	// 82F22270: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22274: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22328 size=240
    let mut pc: u32 = 0x82F22328;
    'dispatch: loop {
        match pc {
            0x82F22328 => {
    //   block [0x82F22328..0x82F22418)
	// 82F22328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2232C: 48285E39  bl 0x831a8164
	ctx.lr = 0x82F22330;
	sub_831A8130(ctx, base);
	// 82F22330: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22338: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F2233C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F22340: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F22344: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22348: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F2234C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22350: 4E800421  bctrl
	ctx.lr = 0x82F22354;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22354: 395EFFFF  addi r10, r30, -1
	ctx.r[10].s64 = ctx.r[30].s64 + -1;
	// 82F22358: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82F2235C: E87F0020  ld r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 82F22360: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F22364: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82F22368: 7D69EA14  add r11, r9, r29
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[29].u64;
	// 82F2236C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F22370: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F22374: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F22378: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F2237C: F8680000  std r3, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F22380: E93F0028  ld r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82F22384: E89F0030  ld r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	// 82F22388: EBDF0038  ld r30, 0x38(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 82F2238C: EBBF0040  ld r29, 0x40(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	// 82F22390: EB9F0048  ld r28, 0x48(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 82F22394: EB7F0050  ld r27, 0x50(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	// 82F22398: EBFF0058  ld r31, 0x58(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 82F2239C: F9280008  std r9, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 82F223A0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F223A4: F8870000  std r4, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
	// 82F223A8: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82F223AC: FBC70008  std r30, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[30].u64 ) };
	// 82F223B0: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F223B4: FBA60000  std r29, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 82F223B8: FB860008  std r28, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[28].u64 ) };
	// 82F223BC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F223C0: FB650000  std r27, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22418 size=140
    let mut pc: u32 = 0x82F22418;
    'dispatch: loop {
        match pc {
            0x82F22418 => {
    //   block [0x82F22418..0x82F224A4)
	// 82F22418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F22420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F22424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F22428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2242C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22430: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F22434: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F22438: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2243C: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F22440: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22444: 4E800421  bctrl
	ctx.lr = 0x82F22448;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22448: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F224A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F224A8 size=148
    let mut pc: u32 = 0x82F224A8;
    'dispatch: loop {
        match pc {
            0x82F224A8 => {
    //   block [0x82F224A8..0x82F2253C)
	// 82F224A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F224AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F224B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F224B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F224B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F224BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F224C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F224C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F224C8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F224CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F224D0: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F224D4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F224D8: 4E800421  bctrl
	ctx.lr = 0x82F224DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F224DC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82F224E0: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82F224E4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F224E8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82F224EC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22540 size=268
    let mut pc: u32 = 0x82F22540;
    'dispatch: loop {
        match pc {
            0x82F22540 => {
    //   block [0x82F22540..0x82F2264C)
	// 82F22540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22544: 48285C15  bl 0x831a8158
	ctx.lr = 0x82F22548;
	sub_831A8130(ctx, base);
	// 82F22548: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2254C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22550: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F22554: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F22558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2255C: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F22560: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22564: 4E800421  bctrl
	ctx.lr = 0x82F22568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22568: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F2256C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F22570: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82F22574: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22578: 80E80024  lwz r7, 0x24(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F2257C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F22580: 4E800421  bctrl
	ctx.lr = 0x82F22584;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22584: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82F22588: E8DF0020  ld r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 82F2258C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F22590: E8FF0038  ld r7, 0x38(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 82F22594: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82F22598: E89F0028  ld r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82F2259C: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82F225A0: E87F0030  ld r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	// 82F225A4: 3B810070  addi r28, r1, 0x70
	ctx.r[28].s64 = ctx.r[1].s64 + 112;
	// 82F225A8: EB7F0040  ld r27, 0x40(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	// 82F225AC: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82F225B0: EB3F0048  ld r25, 0x48(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	// 82F225B4: 55692036  slwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F225B8: EB1F0050  ld r24, 0x50(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	// 82F225BC: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82F225C0: EBFF0058  ld r31, 0x58(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	// 82F225C4: 7D49F214  add r10, r9, r30
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[30].u64;
	// 82F225C8: F8C50000  std r6, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 82F225CC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F225D0: F8E80008  std r7, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[7].u64 ) };
	// 82F225D4: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F225D8: F8850008  std r4, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 82F225DC: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F225E0: F8680000  std r3, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F225E4: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F225E8: FB7C0000  std r27, 0(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82F225EC: 7D3EE850  subf r9, r30, r29
	ctx.r[9].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 82F225F0: FB3C0008  std r25, 8(r28)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[25].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22650 size=8
    let mut pc: u32 = 0x82F22650;
    'dispatch: loop {
        match pc {
            0x82F22650 => {
    //   block [0x82F22650..0x82F22658)
	// 82F22650: 88630040  lbz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F22654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22658 size=8
    let mut pc: u32 = 0x82F22658;
    'dispatch: loop {
        match pc {
            0x82F22658 => {
    //   block [0x82F22658..0x82F22660)
	// 82F22658: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F2265C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22660 size=8
    let mut pc: u32 = 0x82F22660;
    'dispatch: loop {
        match pc {
            0x82F22660 => {
    //   block [0x82F22660..0x82F22668)
	// 82F22660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F22664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22668 size=12
    let mut pc: u32 = 0x82F22668;
    'dispatch: loop {
        match pc {
            0x82F22668 => {
    //   block [0x82F22668..0x82F22674)
	// 82F22668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F2266C: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82F22670: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22674 size=8
    let mut pc: u32 = 0x82F22674;
    'dispatch: loop {
        match pc {
            0x82F22674 => {
    //   block [0x82F22674..0x82F2267C)
	// 82F22674: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F22678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22680 size=140
    let mut pc: u32 = 0x82F22680;
    'dispatch: loop {
        match pc {
            0x82F22680 => {
    //   block [0x82F22680..0x82F2270C)
	// 82F22680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22684: 48285AE1  bl 0x831a8164
	ctx.lr = 0x82F22688;
	sub_831A8130(ctx, base);
	// 82F22688: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2268C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F22690: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F22694: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F22698: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82F2269C: 40990068  ble cr6, 0x82f22704
	if !ctx.cr[6].gt {
	pc = 0x82F22704; continue 'dispatch;
	}
	// 82F226A0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F226A4: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F226A8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F226AC: 815C0044  lwz r10, 0x44(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F226B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F226B4: 557BC23E  srwi r27, r11, 8
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82F226B8: 5569063E  clrlwi r9, r11, 0x18
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82F226BC: 5768103A  slwi r8, r27, 2
	ctx.r[8].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F226C0: B1210050  sth r9, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u16 ) };
	// 82F226C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F226C8: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F226CC: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F226D0: 81670034  lwz r11, 0x34(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F226D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F226D8: 4E800421  bctrl
	ctx.lr = 0x82F226DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F226DC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F226E0: 576B402E  slwi r11, r27, 8
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F226E4: 554A0202  rlwinm r10, r10, 0, 8, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82F226E8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F226EC: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F226F0: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 82F226F4: 65283F00  oris r8, r9, 0x3f00
	ctx.r[8].u64 = ctx.r[9].u64 | 1056964608;
	// 82F226F8: 911F000C  stw r8, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F226FC: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82F22700: 4082FFA4  bne 0x82f226a4
	if !ctx.cr[0].eq {
	pc = 0x82F226A4; continue 'dispatch;
	}
	// 82F22704: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F22708: 48285AAC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22710 size=24
    let mut pc: u32 = 0x82F22710;
    'dispatch: loop {
        match pc {
            0x82F22710 => {
    //   block [0x82F22710..0x82F22728)
	// 82F22710: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22714: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22718: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2271C: 812A002C  lwz r9, 0x2c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F22720: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22724: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22728 size=100
    let mut pc: u32 = 0x82F22728;
    'dispatch: loop {
        match pc {
            0x82F22728 => {
    //   block [0x82F22728..0x82F2278C)
	// 82F22728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2272C: 48285A3D  bl 0x831a8168
	ctx.lr = 0x82F22730;
	sub_831A8130(ctx, base);
	// 82F22730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22734: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F22738: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F2273C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F22740: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22744: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F22748: 40990038  ble cr6, 0x82f22780
	if !ctx.cr[6].gt {
	pc = 0x82F22780; continue 'dispatch;
	}
	// 82F2274C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F22750: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22754: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F22758: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2275C: 812A0024  lwz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F22760: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22764: 4E800421  bctrl
	ctx.lr = 0x82F22768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22768: 811D0048  lwz r8, 0x48(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F2276C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F22770: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 82F22774: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F22778: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F2277C: 4198FFD4  blt cr6, 0x82f22750
	if ctx.cr[6].lt {
	pc = 0x82F22750; continue 'dispatch;
	}
	// 82F22780: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F22784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F22788: 48285A30  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22790 size=136
    let mut pc: u32 = 0x82F22790;
    'dispatch: loop {
        match pc {
            0x82F22790 => {
    //   block [0x82F22790..0x82F22818)
	// 82F22790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22794: 482859CD  bl 0x831a8160
	ctx.lr = 0x82F22798;
	sub_831A8130(ctx, base);
	// 82F22798: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2279C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F227A0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F227A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F227A8: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82F227AC: 817C0048  lwz r11, 0x48(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F227B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F227B4: 40990058  ble cr6, 0x82f2280c
	if !ctx.cr[6].gt {
	pc = 0x82F2280C; continue 'dispatch;
	}
	// 82F227B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F227BC: 817C0044  lwz r11, 0x44(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F227C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F227C4: 7F5D582E  lwzx r26, r29, r11
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F227C8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F227CC: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F227D0: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F227D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F227D8: 4E800421  bctrl
	ctx.lr = 0x82F227DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F227DC: 811A0000  lwz r8, 0(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F227E0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F227E4: 80E80024  lwz r7, 0x24(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F227E8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F227EC: 4E800421  bctrl
	ctx.lr = 0x82F227F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F227F0: 80DC0048  lwz r6, 0x48(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F227F4: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F227F8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F227FC: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F22800: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82F22804: 7F1F3000  cmpw cr6, r31, r6
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F22808: 4198FFB4  blt cr6, 0x82f227bc
	if ctx.cr[6].lt {
	pc = 0x82F227BC; continue 'dispatch;
	}
	// 82F2280C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F22810: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F22814: 4828599C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22818 size=332
    let mut pc: u32 = 0x82F22818;
    'dispatch: loop {
        match pc {
            0x82F22818 => {
    //   block [0x82F22818..0x82F22964)
	// 82F22818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2281C: 4828594D  bl 0x831a8168
	ctx.lr = 0x82F22820;
	sub_831A8130(ctx, base);
	// 82F22820: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82F22824: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22968 size=8
    let mut pc: u32 = 0x82F22968;
    'dispatch: loop {
        match pc {
            0x82F22968 => {
    //   block [0x82F22968..0x82F22970)
	// 82F22968: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F2296C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22970 size=16
    let mut pc: u32 = 0x82F22970;
    'dispatch: loop {
        match pc {
            0x82F22970 => {
    //   block [0x82F22970..0x82F22980)
	// 82F22970: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F22974: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 82F22978: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F2297C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22980 size=8
    let mut pc: u32 = 0x82F22980;
    'dispatch: loop {
        match pc {
            0x82F22980 => {
    //   block [0x82F22980..0x82F22988)
	// 82F22980: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F22984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F22988 size=16
    let mut pc: u32 = 0x82F22988;
    'dispatch: loop {
        match pc {
            0x82F22988 => {
    //   block [0x82F22988..0x82F22998)
	// 82F22988: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F2298C: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F22990: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F22994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22998 size=220
    let mut pc: u32 = 0x82F22998;
    'dispatch: loop {
        match pc {
            0x82F22998 => {
    //   block [0x82F22998..0x82F22A74)
	// 82F22998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2299C: 482857D1  bl 0x831a816c
	ctx.lr = 0x82F229A0;
	sub_831A8130(ctx, base);
	// 82F229A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F229A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F229A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F229AC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F229B0: 390BE584  addi r8, r11, -0x1a7c
	ctx.r[8].s64 = ctx.r[11].s64 + -6780;
	// 82F229B4: 38E9E55C  addi r7, r9, -0x1aa4
	ctx.r[7].s64 = ctx.r[9].s64 + -6820;
	// 82F229B8: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F229BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F229C0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F229C4: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F229C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F229CC: 4099005C  ble cr6, 0x82f22a28
	if !ctx.cr[6].gt {
	pc = 0x82F22A28; continue 'dispatch;
	}
	// 82F229D0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F229D4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F229D8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F229DC: A1430004  lhz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F229E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F229E4: 419A0030  beq cr6, 0x82f22a14
	if ctx.cr[6].eq {
	pc = 0x82F22A14; continue 'dispatch;
	}
	// 82F229E8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F229EC: 394BFFFF  addi r10, r11, -1
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	// 82F229F0: 7D490734  extsh r9, r10
	ctx.r[9].s64 = ctx.r[10].s16 as i64;
	// 82F229F4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F229F8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F229FC: 409A0018  bne cr6, 0x82f22a14
	if !ctx.cr[6].eq {
	pc = 0x82F22A14; continue 'dispatch;
	}
	// 82F22A00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22A04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F22A08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22A0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22A10: 4E800421  bctrl
	ctx.lr = 0x82F22A14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22A14: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22A18: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F22A1C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F22A20: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F22A24: 4198FFB0  blt cr6, 0x82f229d4
	if ctx.cr[6].lt {
	pc = 0x82F229D4; continue 'dispatch;
	}
	// 82F22A28: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F22A2C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F22A30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F22A34: 409A0020  bne cr6, 0x82f22a54
	if !ctx.cr[6].eq {
	pc = 0x82F22A54; continue 'dispatch;
	}
	// 82F22A38: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22A3C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F22A40: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F22A44: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22A48: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F22A4C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F22A50: 4BF7DD61  bl 0x82ea07b0
	ctx.lr = 0x82F22A54;
	sub_82EA07B0(ctx, base);
	// 82F22A54: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F22A58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F22A5C: 392BBCD8  addi r9, r11, -0x4328
	ctx.r[9].s64 = ctx.r[11].s64 + -17192;
	// 82F22A60: 390A9EAC  addi r8, r10, -0x6154
	ctx.r[8].s64 = ctx.r[10].s64 + -24916;
	// 82F22A64: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82F22A68: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F22A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F22A70: 4828574C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F22A78 size=200
    let mut pc: u32 = 0x82F22A78;
    'dispatch: loop {
        match pc {
            0x82F22A78 => {
    //   block [0x82F22A78..0x82F22B40)
	// 82F22A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22A7C: 482856E1  bl 0x831a815c
	ctx.lr = 0x82F22A80;
	sub_831A8130(ctx, base);
	// 82F22A80: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 82F22A84: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22A88: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F22A8C: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22A90: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F22A94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F22A98: 3B830048  addi r28, r3, 0x48
	ctx.r[28].s64 = ctx.r[3].s64 + 72;
	// 82F22A9C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82F22AA0: C3EB3EF8  lfs f31, 0x3ef8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16120 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F22AA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F22AA8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F22AAC: 40990070  ble cr6, 0x82f22b1c
	if !ctx.cr[6].gt {
	pc = 0x82F22B1C; continue 'dispatch;
	}
	// 82F22AB0: 3B230044  addi r25, r3, 0x44
	ctx.r[25].s64 = ctx.r[3].s64 + 68;
	// 82F22AB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F22AB8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22ABC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F22AC0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F22AC4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F22AC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22ACC: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F22AD0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22AD4: 4E800421  bctrl
	ctx.lr = 0x82F22AD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22AD8: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F22B40 size=164
    let mut pc: u32 = 0x82F22B40;
    'dispatch: loop {
        match pc {
            0x82F22B40 => {
    //   block [0x82F22B40..0x82F22BE4)
	// 82F22B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22B44: 48285625  bl 0x831a8168
	ctx.lr = 0x82F22B48;
	sub_831A8130(ctx, base);
	// 82F22B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22B4C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F22B50: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F22B54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F22B58: 3BBC0044  addi r29, r28, 0x44
	ctx.r[29].s64 = ctx.r[28].s64 + 68;
	// 82F22B5C: 817C004C  lwz r11, 0x4c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F22B60: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F22B64: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F22B68: 40980024  bge cr6, 0x82f22b8c
	if !ctx.cr[6].lt {
	pc = 0x82F22B8C; continue 'dispatch;
	}
	// 82F22B6C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F22B70: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F22B74: 41980008  blt cr6, 0x82f22b7c
	if ctx.cr[6].lt {
	pc = 0x82F22B7C; continue 'dispatch;
	}
	// 82F22B78: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F22B7C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F22B80: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F22B84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F22B88: 4BF83C71  bl 0x82ea67f8
	ctx.lr = 0x82F22B8C;
	sub_82EA67F8(ctx, base);
	// 82F22B8C: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82F22B90: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F22B94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22B98: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F22B9C: D01C0010  stfs f0, 0x10(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F22BA0: 4099003C  ble cr6, 0x82f22bdc
	if !ctx.cr[6].gt {
	pc = 0x82F22BDC; continue 'dispatch;
	}
	// 82F22BA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F22BA8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22BAC: 7D2BF02E  lwzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F22BB0: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 82F22BB4: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F22BB8: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F22BBC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82F22BC0: 419A0010  beq cr6, 0x82f22bd0
	if ctx.cr[6].eq {
	pc = 0x82F22BD0; continue 'dispatch;
	}
	// 82F22BC4: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F22BC8: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 82F22BCC: B10A0006  sth r8, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F22BD0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F22BD4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82F22BD8: 4082FFD0  bne 0x82f22ba8
	if !ctx.cr[0].eq {
	pc = 0x82F22BA8; continue 'dispatch;
	}
	// 82F22BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F22BE0: 482855D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F22BE8 size=252
    let mut pc: u32 = 0x82F22BE8;
    'dispatch: loop {
        match pc {
            0x82F22BE8 => {
    //   block [0x82F22BE8..0x82F22CE4)
	// 82F22BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22BEC: 48285575  bl 0x831a8160
	ctx.lr = 0x82F22BF0;
	sub_831A8130(ctx, base);
	// 82F22BF0: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82F22BF4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22BF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22BFC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F22C00: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F22C04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F22C08: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F22C0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F22C10: 419A002C  beq cr6, 0x82f22c3c
	if ctx.cr[6].eq {
	pc = 0x82F22C3C; continue 'dispatch;
	}
	// 82F22C14: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F22C18: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F22C1C: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F22C20: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82F22C24: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 82F22C28: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F22C2C: 4BFF71AD  bl 0x82f19dd8
	ctx.lr = 0x82F22C30;
	sub_82F19DD8(ctx, base);
	// 82F22C30: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F22C34: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82F22C38: 48285578  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 82F22C3C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22C40: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F22C44: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F22C48: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F22C4C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22C50: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22C54: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F22C58: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22C5C: 4E800421  bctrl
	ctx.lr = 0x82F22C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22C60: 811F0048  lwz r8, 0x48(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22C64: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82F22C68: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82F22C6C: 4099006C  ble cr6, 0x82f22cd8
	if !ctx.cr[6].gt {
	pc = 0x82F22CD8; continue 'dispatch;
	}
	// 82F22C70: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 82F22C74: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82F22C78: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22C7C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F22C80: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F22C84: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F22C88: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F22C8C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22C90: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F22C94: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22C98: 4E800421  bctrl
	ctx.lr = 0x82F22C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22C9C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82F22CA0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22CE8 size=232
    let mut pc: u32 = 0x82F22CE8;
    'dispatch: loop {
        match pc {
            0x82F22CE8 => {
    //   block [0x82F22CE8..0x82F22DD0)
	// 82F22CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22CEC: 48285479  bl 0x831a8164
	ctx.lr = 0x82F22CF0;
	sub_831A8130(ctx, base);
	// 82F22CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22CF4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F22CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22CFC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F22D00: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F22D04: 388AED08  addi r4, r10, -0x12f8
	ctx.r[4].s64 = ctx.r[10].s64 + -4856;
	// 82F22D08: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22D0C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F22D10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F22D14: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F22D18: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22D1C: 4E800421  bctrl
	ctx.lr = 0x82F22D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22D20: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82F22D24: 55680000  rlwinm r8, r11, 0, 0, 0
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F22D28: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F22D2C: 409A0034  bne cr6, 0x82f22d60
	if !ctx.cr[6].eq {
	pc = 0x82F22D60; continue 'dispatch;
	}
	// 82F22D30: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22D34: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F22D38: 80FF0048  lwz r7, 0x48(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22D3C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F22D40: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22D44: 3889ECFC  addi r4, r9, -0x1304
	ctx.r[4].s64 = ctx.r[9].s64 + -4868;
	// 82F22D48: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F22D4C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F22D50: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F22D54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F22D58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F22D5C: 4E800421  bctrl
	ctx.lr = 0x82F22D60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22D60: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22D64: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F22D68: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F22D6C: 40990048  ble cr6, 0x82f22db4
	if !ctx.cr[6].gt {
	pc = 0x82F22DB4; continue 'dispatch;
	}
	// 82F22D70: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F22D74: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F22D78: 3B6BECC4  addi r27, r11, -0x133c
	ctx.r[27].s64 = ctx.r[11].s64 + -4924;
	// 82F22D7C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22D80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F22D84: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82F22D88: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F22D8C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F22D90: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F22D94: 7CCAF02E  lwzx r6, r10, r30
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F22D98: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22D9C: 4E800421  bctrl
	ctx.lr = 0x82F22DA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22DA0: 811F0048  lwz r8, 0x48(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82F22DA4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F22DA8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F22DAC: 7F1C4000  cmpw cr6, r28, r8
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F22DB0: 4198FFCC  blt cr6, 0x82f22d7c
	if ctx.cr[6].lt {
	pc = 0x82F22D7C; continue 'dispatch;
	}
	// 82F22DB4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22DB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F22DBC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F22DC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22DC4: 4E800421  bctrl
	ctx.lr = 0x82F22DC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F22DCC: 482853E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22DD0 size=364
    let mut pc: u32 = 0x82F22DD0;
    'dispatch: loop {
        match pc {
            0x82F22DD0 => {
    //   block [0x82F22DD0..0x82F22F3C)
	// 82F22DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22DD4: 48285381  bl 0x831a8154
	ctx.lr = 0x82F22DD8;
	sub_831A8130(ctx, base);
	// 82F22DD8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22DDC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22DE0: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F22DE4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82F22DE8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F22DEC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F22DF0: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F22DF4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F22DF8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F22DFC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F22E00: 40980020  bge cr6, 0x82f22e20
	if !ctx.cr[6].lt {
	pc = 0x82F22E20; continue 'dispatch;
	}
	// 82F22E04: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F22E08: 3909ED18  addi r8, r9, -0x12e8
	ctx.r[8].s64 = ctx.r[9].s64 + -4840;
	// 82F22E0C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F22E10: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F22E14: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F22E18: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F22E1C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F22E20: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F22E24: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82F22E28: 3BE40014  addi r31, r4, 0x14
	ctx.r[31].s64 = ctx.r[4].s64 + 20;
	// 82F22E2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F22E30: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82F22E34: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F22E38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F22E3C: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F22E40: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F22E44: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F22E48: 4E800421  bctrl
	ctx.lr = 0x82F22E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22E4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F22E50: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F22E54: 419A0078  beq cr6, 0x82f22ecc
	if ctx.cr[6].eq {
	pc = 0x82F22ECC; continue 'dispatch;
	}
	// 82F22E58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22E5C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F22E60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F22E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F22E68: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F22E6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22E70: 4E800421  bctrl
	ctx.lr = 0x82F22E74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22E74: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82F22E78: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F22E7C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F22E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F22E84: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 82F22E88: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22E8C: 80E80020  lwz r7, 0x20(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F22E90: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F22E94: 4E800421  bctrl
	ctx.lr = 0x82F22E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22E98: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F22E9C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F22EA0: 419A0008  beq cr6, 0x82f22ea8
	if ctx.cr[6].eq {
	pc = 0x82F22EA8; continue 'dispatch;
	}
	// 82F22EA4: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82F22EA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22EAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F22EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F22EB4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F22EB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22EBC: 4E800421  bctrl
	ctx.lr = 0x82F22EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22EC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F22EC4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F22EC8: 409AFF90  bne cr6, 0x82f22e58
	if !ctx.cr[6].eq {
	pc = 0x82F22E58; continue 'dispatch;
	}
	// 82F22ECC: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F22ED0: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 82F22ED4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F22ED8: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F22EDC: 419A0010  beq cr6, 0x82f22eec
	if ctx.cr[6].eq {
	pc = 0x82F22EEC; continue 'dispatch;
	}
	// 82F22EE0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F22EE4: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F22EE8: 7F8AE92E  stwx r28, r10, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	// 82F22EEC: 7D7CD050  subf r11, r28, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 82F22EF0: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F22EF4: 7D690034  cntlzw r9, r11
	ctx.r[9].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82F22EF8: 5528DFFE  rlwinm r8, r9, 0x1b, 0x1f, 0x1f
	ctx.r[8].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82F22EFC: 80EA000C  lwz r7, 0xc(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F22F00: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F22F04: 69060001  xori r6, r8, 1
	ctx.r[6].u64 = ctx.r[8].u64 ^ 1;
	// 82F22F08: 98D70000  stb r6, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[6].u8 ) };
	// 82F22F0C: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F22F10: 40980020  bge cr6, 0x82f22f30
	if !ctx.cr[6].lt {
	pc = 0x82F22F30; continue 'dispatch;
	}
	// 82F22F14: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F22F18: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F22F1C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F22F20: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F22F24: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F22F28: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F22F2C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F22F30: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F22F34: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82F22F38: 4828526C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F22F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F22F40 size=284
    let mut pc: u32 = 0x82F22F40;
    'dispatch: loop {
        match pc {
            0x82F22F40 => {
    //   block [0x82F22F40..0x82F2305C)
	// 82F22F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F22F44: 48285219  bl 0x831a815c
	ctx.lr = 0x82F22F48;
	sub_831A8130(ctx, base);
	// 82F22F48: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F22F4C: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22F50: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 82F22F54: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F22F58: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F22F5C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F22F60: 7D79D02E  lwzx r11, r25, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F22F64: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F22F68: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F22F6C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F22F70: 40980020  bge cr6, 0x82f22f90
	if !ctx.cr[6].lt {
	pc = 0x82F22F90; continue 'dispatch;
	}
	// 82F22F74: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F22F78: 3909ED24  addi r8, r9, -0x12dc
	ctx.r[8].s64 = ctx.r[9].s64 + -4828;
	// 82F22F7C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F22F80: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F22F84: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F22F88: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F22F8C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F22F90: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F22F94: 3BC30014  addi r30, r3, 0x14
	ctx.r[30].s64 = ctx.r[3].s64 + 20;
	// 82F22F98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F22F9C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F22FA0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22FA4: 4E800421  bctrl
	ctx.lr = 0x82F22FA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F22FAC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82F22FB0: 419A0074  beq cr6, 0x82f23024
	if ctx.cr[6].eq {
	pc = 0x82F23024; continue 'dispatch;
	}
	// 82F22FB4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22FB8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F22FBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F22FC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F22FC4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F22FC8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F22FCC: 4E800421  bctrl
	ctx.lr = 0x82F22FD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F22FD0: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F22FD4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F22FD8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F22FDC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82F22FE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F22FE4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F22FE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F22FEC: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82F22FF0: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F22FF4: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F22FF8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F22FFC: 4E800421  bctrl
	ctx.lr = 0x82F23000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23000: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23004: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F23008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F2300C: 80A6000C  lwz r5, 0xc(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F23010: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82F23014: 4E800421  bctrl
	ctx.lr = 0x82F23018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F2301C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82F23020: 409AFF94  bne cr6, 0x82f22fb4
	if !ctx.cr[6].eq {
	pc = 0x82F22FB4; continue 'dispatch;
	}
	// 82F23024: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F23028: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F2302C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23030: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F23034: 40980020  bge cr6, 0x82f23054
	if !ctx.cr[6].lt {
	pc = 0x82F23054; continue 'dispatch;
	}
	// 82F23038: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F2303C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F23040: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F23044: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F23048: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F2304C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F23050: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F23054: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 82F23058: 48285154  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F23060 size=164
    let mut pc: u32 = 0x82F23060;
    'dispatch: loop {
        match pc {
            0x82F23060 => {
    //   block [0x82F23060..0x82F23104)
	// 82F23060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F2306C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23070: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F23074: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F23078: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F2307C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F23080: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F23084: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F23088: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F2308C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23090: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F23094: 38600015  li r3, 0x15
	ctx.r[3].s64 = 21;
	// 82F23098: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F2309C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82F230A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F230A4: 3929BCD8  addi r9, r9, -0x4328
	ctx.r[9].s64 = ctx.r[9].s64 + -17192;
	// 82F230A8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82F230AC: 3908E584  addi r8, r8, -0x1a7c
	ctx.r[8].s64 = ctx.r[8].s64 + -6780;
	// 82F230B0: 38E7E55C  addi r7, r7, -0x1aa4
	ctx.r[7].s64 = ctx.r[7].s64 + -6820;
	// 82F230B4: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82F230B8: 3CC08000  lis r6, -0x8000
	ctx.r[6].s64 = -2147483648;
	// 82F230BC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F230C0: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82F230C4: C00A08A8  lfs f0, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F230C8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82F230CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F230D0: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82F230D4: 90DF004C  stw r6, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[6].u32 ) };
	// 82F230D8: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F230DC: 4BFFFA65  bl 0x82f22b40
	ctx.lr = 0x82F230E0;
	sub_82F22B40(ctx, base);
	// 82F230E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F230E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F230E8: 4BFFF731  bl 0x82f22818
	ctx.lr = 0x82F230EC;
	sub_82F22818(ctx, base);
	// 82F230EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F230F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F230F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F230F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F230FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F23100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23108 size=28
    let mut pc: u32 = 0x82F23108;
    'dispatch: loop {
        match pc {
            0x82F23108 => {
    //   block [0x82F23108..0x82F23124)
	// 82F23108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F2310C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F23110: B1630020  sth r11, 0x20(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u16 ) };
	// 82F23114: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82F23118: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82F2311C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F23120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23128 size=120
    let mut pc: u32 = 0x82F23128;
    'dispatch: loop {
        match pc {
            0x82F23128 => {
    //   block [0x82F23128..0x82F231A0)
	// 82F23128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23130: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F23134: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F2313C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F23144: 419A0048  beq cr6, 0x82f2318c
	if ctx.cr[6].eq {
	pc = 0x82F2318C; continue 'dispatch;
	}
	// 82F23148: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F2314C: 2F0A000A  cmpwi cr6, r10, 0xa
	ctx.cr[6].compare_i32(ctx.r[10].s32, 10, &mut ctx.xer);
	// 82F23150: 409A0018  bne cr6, 0x82f23168
	if !ctx.cr[6].eq {
	pc = 0x82F23168; continue 'dispatch;
	}
	// 82F23154: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F23158: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F2315C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23160: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F23164: 4E800421  bctrl
	ctx.lr = 0x82F23168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23168: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2316C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F23170: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 82F23174: 409A0018  bne cr6, 0x82f2318c
	if !ctx.cr[6].eq {
	pc = 0x82F2318C; continue 'dispatch;
	}
	// 82F23178: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F2317C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F23180: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23184: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F23188: 4E800421  bctrl
	ctx.lr = 0x82F2318C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F2318C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F23190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F23194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F23198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F2319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F231A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F231A0 size=140
    let mut pc: u32 = 0x82F231A0;
    'dispatch: loop {
        match pc {
            0x82F231A0 => {
    //   block [0x82F231A0..0x82F2322C)
	// 82F231A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F231A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F231A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F231AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F231B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F231B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F231B8: 3BC5FFD0  addi r30, r5, -0x30
	ctx.r[30].s64 = ctx.r[5].s64 + -48;
	// 82F231BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F231C0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F231C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F231C8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F231CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F231D0: 4E800421  bctrl
	ctx.lr = 0x82F231D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F231D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F231D8: 4098000C  bge cr6, 0x82f231e4
	if !ctx.cr[6].lt {
	pc = 0x82F231E4; continue 'dispatch;
	}
	// 82F231DC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F231E0: 48000034  b 0x82f23214
	pc = 0x82F23214; continue 'dispatch;
	// 82F231E4: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F231E8: 4199FFF4  bgt cr6, 0x82f231dc
	if ctx.cr[6].gt {
	pc = 0x82F231DC; continue 'dispatch;
	}
	// 82F231EC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F231F0: 395F0030  addi r10, r31, 0x30
	ctx.r[10].s64 = ctx.r[31].s64 + 48;
	// 82F231F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F231F8: 409A0014  bne cr6, 0x82f2320c
	if !ctx.cr[6].eq {
	pc = 0x82F2320C; continue 'dispatch;
	}
	// 82F231FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23200: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F23204: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82F23208: 4800000C  b 0x82f23214
	pc = 0x82F23214; continue 'dispatch;
	// 82F2320C: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82F23210: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82F23214: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F23218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F2321C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F23220: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F23224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F23228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23230 size=20
    let mut pc: u32 = 0x82F23230;
    'dispatch: loop {
        match pc {
            0x82F23230 => {
    //   block [0x82F23230..0x82F23244)
	// 82F23230: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F23234: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23238: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F2323C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F23240: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23248 size=144
    let mut pc: u32 = 0x82F23248;
    'dispatch: loop {
        match pc {
            0x82F23248 => {
    //   block [0x82F23248..0x82F232D8)
	// 82F23248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2324C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F23254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F23258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2325C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F23260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F23264: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F23268: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F2326C: 388AED34  addi r4, r10, -0x12cc
	ctx.r[4].s64 = ctx.r[10].s64 + -4812;
	// 82F23270: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23274: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F23278: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F2327C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23280: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F23284: 4E800421  bctrl
	ctx.lr = 0x82F23288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23288: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2328C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F23290: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F23294: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F23298: 3888ECC4  addi r4, r8, -0x133c
	ctx.r[4].s64 = ctx.r[8].s64 + -4924;
	// 82F2329C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F232A0: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F232A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F232A8: 4E800421  bctrl
	ctx.lr = 0x82F232AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F232AC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F232B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F232B4: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F232B8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F232BC: 4E800421  bctrl
	ctx.lr = 0x82F232C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F232C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F232C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F232C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F232CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F232D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F232D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F232D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F232D8 size=8
    let mut pc: u32 = 0x82F232D8;
    'dispatch: loop {
        match pc {
            0x82F232D8 => {
    //   block [0x82F232D8..0x82F232E0)
	// 82F232D8: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82F232DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F232E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F232E0 size=104
    let mut pc: u32 = 0x82F232E0;
    'dispatch: loop {
        match pc {
            0x82F232E0 => {
    //   block [0x82F232E0..0x82F23348)
	// 82F232E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F232E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F232E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F232EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F232F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F232F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F232F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F232FC: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F23300: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23304: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F23308: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F2330C: 4E800421  bctrl
	ctx.lr = 0x82F23310;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23310: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23348 size=180
    let mut pc: u32 = 0x82F23348;
    'dispatch: loop {
        match pc {
            0x82F23348 => {
    //   block [0x82F23348..0x82F233FC)
	// 82F23348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2334C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F23354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F23358: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2335C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82F23360: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F23364: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82F23368: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F2336C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F23370: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F23374: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F23378: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F2337C: 4200FFF0  bdnz 0x82f2336c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F2336C; continue 'dispatch;
	}
	// 82F23380: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23400 size=92
    let mut pc: u32 = 0x82F23400;
    'dispatch: loop {
        match pc {
            0x82F23400 => {
    //   block [0x82F23400..0x82F2345C)
	// 82F23400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F2340C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F23410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F23418: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F2341C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F23420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23424: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F23428: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F2342C: 4E800421  bctrl
	ctx.lr = 0x82F23430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23430: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23460 size=100
    let mut pc: u32 = 0x82F23460;
    'dispatch: loop {
        match pc {
            0x82F23460 => {
    //   block [0x82F23460..0x82F234C4)
	// 82F23460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23464: 48284D09  bl 0x831a816c
	ctx.lr = 0x82F23468;
	sub_831A8130(ctx, base);
	// 82F23468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2346C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F23470: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F23474: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F23478: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F2347C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23480: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F23484: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F23488: 4E800421  bctrl
	ctx.lr = 0x82F2348C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F2348C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F23490: 4099002C  ble cr6, 0x82f234bc
	if !ctx.cr[6].gt {
	pc = 0x82F234BC; continue 'dispatch;
	}
	// 82F23494: 393E0020  addi r9, r30, 0x20
	ctx.r[9].s64 = ctx.r[30].s64 + 32;
	// 82F23498: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82F2349C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F234C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F234C8 size=92
    let mut pc: u32 = 0x82F234C8;
    'dispatch: loop {
        match pc {
            0x82F234C8 => {
    //   block [0x82F234C8..0x82F23524)
	// 82F234C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F234CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F234D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F234D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F234D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F234DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F234E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F234E4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F234E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F234EC: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82F234F0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F234F4: 4E800421  bctrl
	ctx.lr = 0x82F234F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F234F8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23528 size=92
    let mut pc: u32 = 0x82F23528;
    'dispatch: loop {
        match pc {
            0x82F23528 => {
    //   block [0x82F23528..0x82F23584)
	// 82F23528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2352C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F23534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F23538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2353C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F23540: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F23544: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F23548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2354C: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F23550: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F23554: 4E800421  bctrl
	ctx.lr = 0x82F23558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23558: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23588 size=132
    let mut pc: u32 = 0x82F23588;
    'dispatch: loop {
        match pc {
            0x82F23588 => {
    //   block [0x82F23588..0x82F2360C)
	// 82F23588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2358C: 48284BDD  bl 0x831a8168
	ctx.lr = 0x82F23590;
	sub_831A8130(ctx, base);
	// 82F23590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23594: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F23598: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F2359C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F235A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F235A4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F235A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F235AC: 4E800421  bctrl
	ctx.lr = 0x82F235B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F235B0: 813D0018  lwz r9, 0x18(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F235B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F235B8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82F235BC: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82F235C0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F235C4: 80E80024  lwz r7, 0x24(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F235C8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F235CC: 4E800421  bctrl
	ctx.lr = 0x82F235D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F235D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F235D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F235D8: 40990028  ble cr6, 0x82f23600
	if !ctx.cr[6].gt {
	pc = 0x82F23600; continue 'dispatch;
	}
	// 82F235DC: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F23610 size=116
    let mut pc: u32 = 0x82F23610;
    'dispatch: loop {
        match pc {
            0x82F23610 => {
    //   block [0x82F23610..0x82F23684)
	// 82F23610: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F23614: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F23618: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F2361C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F23620: 390AE2CC  addi r8, r10, -0x1d34
	ctx.r[8].s64 = ctx.r[10].s64 + -7476;
	// 82F23624: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23628: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F2362C: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F23630: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82F23634: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F23638: 3949BD10  addi r10, r9, -0x42f0
	ctx.r[10].s64 = ctx.r[9].s64 + -17136;
	// 82F2363C: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82F23640: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F23644: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82F23648: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82F2364C: A1240004  lhz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23650: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F23654: 419A0010  beq cr6, 0x82f23664
	if ctx.cr[6].eq {
	pc = 0x82F23664; continue 'dispatch;
	}
	// 82F23658: A1440006  lhz r10, 6(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82F2365C: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82F23660: B1240006  sth r9, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F23664: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23688 size=132
    let mut pc: u32 = 0x82F23688;
    'dispatch: loop {
        match pc {
            0x82F23688 => {
    //   block [0x82F23688..0x82F2370C)
	// 82F23688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2368C: 48284AE1  bl 0x831a816c
	ctx.lr = 0x82F23690;
	sub_831A8130(ctx, base);
	// 82F23690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F23698: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F2369C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F236A0: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F236A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F236A8: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F236AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F236B0: 4E800421  bctrl
	ctx.lr = 0x82F236B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F236B4: 395E0020  addi r10, r30, 0x20
	ctx.r[10].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23710 size=300
    let mut pc: u32 = 0x82F23710;
    'dispatch: loop {
        match pc {
            0x82F23710 => {
    //   block [0x82F23710..0x82F2383C)
	// 82F23710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23714: 48284A55  bl 0x831a8168
	ctx.lr = 0x82F23718;
	sub_831A8130(ctx, base);
	// 82F23718: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2371C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23720: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F23724: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F23728: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F2372C: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F23730: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F23734: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23738: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F2373C: 40980020  bge cr6, 0x82f2375c
	if !ctx.cr[6].lt {
	pc = 0x82F2375C; continue 'dispatch;
	}
	// 82F23740: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F23744: 3909ECDC  addi r8, r9, -0x1324
	ctx.r[8].s64 = ctx.r[9].s64 + -4900;
	// 82F23748: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F2374C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F23750: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 82F23754: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F23758: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F2375C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F23760: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F23764: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82F23768: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F2376C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F23770: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F23774: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F23778: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F2377C: 4200FFF0  bdnz 0x82f2376c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F2376C; continue 'dispatch;
	}
	// 82F23780: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F23784: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F23788: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23840 size=8
    let mut pc: u32 = 0x82F23840;
    'dispatch: loop {
        match pc {
            0x82F23840 => {
    //   block [0x82F23840..0x82F23848)
	// 82F23840: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 82F23844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23848 size=232
    let mut pc: u32 = 0x82F23848;
    'dispatch: loop {
        match pc {
            0x82F23848 => {
    //   block [0x82F23848..0x82F23930)
	// 82F23848: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F2384C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F23850: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F23854: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F23858: 38CB41F8  addi r6, r11, 0x41f8
	ctx.r[6].s64 = ctx.r[11].s64 + 16888;
	// 82F2385C: 38AAED58  addi r5, r10, -0x12a8
	ctx.r[5].s64 = ctx.r[10].s64 + -4776;
	// 82F23860: B0E30106  sth r7, 0x106(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(262 as u32), ctx.r[7].u16 ) };
	// 82F23864: 3888ED48  addi r4, r8, -0x12b8
	ctx.r[4].s64 = ctx.r[8].s64 + -4792;
	// 82F23868: 90C30108  stw r6, 0x108(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), ctx.r[6].u32 ) };
	// 82F2386C: 39230100  addi r9, r3, 0x100
	ctx.r[9].s64 = ctx.r[3].s64 + 256;
	// 82F23870: 90A30100  stw r5, 0x100(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[5].u32 ) };
	// 82F23874: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 82F23878: 90830108  stw r4, 0x108(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(264 as u32), ctx.r[4].u32 ) };
	// 82F2387C: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82F23880: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F23884: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F23888: 409A0008  bne cr6, 0x82f23890
	if !ctx.cr[6].eq {
	pc = 0x82F23890; continue 'dispatch;
	}
	// 82F2388C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23890: 916AFFF8  stw r11, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82F23894: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F23898: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F2389C: 409A0008  bne cr6, 0x82f238a4
	if !ctx.cr[6].eq {
	pc = 0x82F238A4; continue 'dispatch;
	}
	// 82F238A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F238A4: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82F238A8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F238AC: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F238B0: 409A0008  bne cr6, 0x82f238b8
	if !ctx.cr[6].eq {
	pc = 0x82F238B8; continue 'dispatch;
	}
	// 82F238B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F238B8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82F238BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F238C0: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F238C4: 409A0008  bne cr6, 0x82f238cc
	if !ctx.cr[6].eq {
	pc = 0x82F238CC; continue 'dispatch;
	}
	// 82F238C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F238CC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F238D0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F238D4: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F238D8: 409A0008  bne cr6, 0x82f238e0
	if !ctx.cr[6].eq {
	pc = 0x82F238E0; continue 'dispatch;
	}
	// 82F238DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F238E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F238E4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F238E8: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F238EC: 409A0008  bne cr6, 0x82f238f4
	if !ctx.cr[6].eq {
	pc = 0x82F238F4; continue 'dispatch;
	}
	// 82F238F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F238F4: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82F238F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F238FC: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F23900: 409A0008  bne cr6, 0x82f23908
	if !ctx.cr[6].eq {
	pc = 0x82F23908; continue 'dispatch;
	}
	// 82F23904: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23908: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F2390C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F23910: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 82F23914: 409A0008  bne cr6, 0x82f2391c
	if !ctx.cr[6].eq {
	pc = 0x82F2391C; continue 'dispatch;
	}
	// 82F23918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F2391C: 916A0014  stw r11, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F23920: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F23924: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82F23928: 4082FF58  bne 0x82f23880
	if !ctx.cr[0].eq {
	pc = 0x82F23880; continue 'dispatch;
	}
	// 82F2392C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23930 size=44
    let mut pc: u32 = 0x82F23930;
    'dispatch: loop {
        match pc {
            0x82F23930 => {
    //   block [0x82F23930..0x82F2395C)
	// 82F23930: 35430100  addic. r10, r3, 0x100
	ctx.xer.ca = (ctx.r[3].u32 > (!(256 as u32)));
	ctx.r[10].s64 = ctx.r[3].s64 + 256;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F23934: 396A0008  addi r11, r10, 8
	ctx.r[11].s64 = ctx.r[10].s64 + 8;
	// 82F23938: 40820008  bne 0x82f23940
	if !ctx.cr[0].eq {
	pc = 0x82F23940; continue 'dispatch;
	}
	// 82F2393C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23940: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82F23944: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F23948: 38E941F8  addi r7, r9, 0x41f8
	ctx.r[7].s64 = ctx.r[9].s64 + 16888;
	// 82F2394C: 38C89EAC  addi r6, r8, -0x6154
	ctx.r[6].s64 = ctx.r[8].s64 + -24916;
	// 82F23950: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F23954: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F23958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23960 size=180
    let mut pc: u32 = 0x82F23960;
    'dispatch: loop {
        match pc {
            0x82F23960 => {
    //   block [0x82F23960..0x82F23A14)
	// 82F23960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23964: 48284805  bl 0x831a8168
	ctx.lr = 0x82F23968;
	sub_831A8130(ctx, base);
	// 82F23968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2396C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F23970: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F23974: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F23978: 37A5FFFF  addic. r29, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F2397C: 41800090  blt 0x82f23a0c
	if ctx.cr[0].lt {
	pc = 0x82F23A0C; continue 'dispatch;
	}
	// 82F23980: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23984: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F23988: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F2398C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F23990: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23994: 88EB0005  lbz r7, 5(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(5 as u32) ) } as u64;
	// 82F23998: 88CA0005  lbz r6, 5(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(5 as u32) ) } as u64;
	// 82F2399C: 7CE90774  extsb r9, r7
	ctx.r[9].s64 = ctx.r[7].s8 as i64;
	// 82F239A0: 81080004  lwz r8, 4(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F239A4: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F239A8: 7CCB0774  extsb r11, r6
	ctx.r[11].s64 = ctx.r[6].s8 as i64;
	// 82F239AC: 7CCB5214  add r6, r11, r10
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F239B0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F239B4: 4E800421  bctrl
	ctx.lr = 0x82F239B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F239B8: 88E30000  lbz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F239BC: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F239C0: 419A0040  beq cr6, 0x82f23a00
	if ctx.cr[6].eq {
	pc = 0x82F23A00; continue 'dispatch;
	}
	// 82F239C4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F239C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F239CC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F239D0: 892B0004  lbz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F239D4: 890A0004  lbz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F239D8: 7D270774  extsb r7, r9
	ctx.r[7].s64 = ctx.r[9].s8 as i64;
	// 82F239DC: 7D0B0774  extsb r11, r8
	ctx.r[11].s64 = ctx.r[8].s8 as i64;
	// 82F239E0: 54EA1838  slwi r10, r7, 3
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F239E4: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F239E8: 54C5103A  slwi r5, r6, 2
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F239EC: 7C65E02E  lwzx r3, r5, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F239F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F239F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F239F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F239FC: 4E800421  bctrl
	ctx.lr = 0x82F23A00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23A00: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F23A04: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F23A08: 4080FF78  bge 0x82f23980
	if !ctx.cr[0].lt {
	pc = 0x82F23980; continue 'dispatch;
	}
	// 82F23A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F23A10: 482847A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23A18 size=108
    let mut pc: u32 = 0x82F23A18;
    'dispatch: loop {
        match pc {
            0x82F23A18 => {
    //   block [0x82F23A18..0x82F23A84)
	// 82F23A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23A1C: 48284751  bl 0x831a816c
	ctx.lr = 0x82F23A20;
	sub_831A8130(ctx, base);
	// 82F23A20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23A24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F23A28: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F23A2C: 37C5FFFF  addic. r30, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F23A30: 4180004C  blt 0x82f23a7c
	if ctx.cr[0].lt {
	pc = 0x82F23A7C; continue 'dispatch;
	}
	// 82F23A34: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23A38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F23A3C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23A40: 892B0004  lbz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23A44: 890A0004  lbz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23A48: 7D270774  extsb r7, r9
	ctx.r[7].s64 = ctx.r[9].s8 as i64;
	// 82F23A4C: 7D0B0774  extsb r11, r8
	ctx.r[11].s64 = ctx.r[8].s8 as i64;
	// 82F23A50: 54EA1838  slwi r10, r7, 3
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F23A54: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F23A58: 54C5103A  slwi r5, r6, 2
	ctx.r[5].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F23A5C: 7C65E82E  lwzx r3, r5, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F23A60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23A64: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F23A68: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F23A6C: 4E800421  bctrl
	ctx.lr = 0x82F23A70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23A70: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F23A74: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F23A78: 4080FFBC  bge 0x82f23a34
	if !ctx.cr[0].lt {
	pc = 0x82F23A34; continue 'dispatch;
	}
	// 82F23A7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F23A80: 4828473C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23A88 size=1168
    let mut pc: u32 = 0x82F23A88;
    'dispatch: loop {
        match pc {
            0x82F23A88 => {
    //   block [0x82F23A88..0x82F23F18)
	// 82F23A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23A8C: 482846C1  bl 0x831a814c
	ctx.lr = 0x82F23A90;
	sub_831A8130(ctx, base);
	// 82F23A90: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23A94: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F23A98: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82F23A9C: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23AA0: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23AA4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F23AA8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23AAC: 41980008  blt cr6, 0x82f23ab4
	if ctx.cr[6].lt {
	pc = 0x82F23AB4; continue 'dispatch;
	}
	// 82F23AB0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F23AB4: 2F0A0020  cmpwi cr6, r10, 0x20
	ctx.cr[6].compare_i32(ctx.r[10].s32, 32, &mut ctx.xer);
	// 82F23AB8: 40980150  bge cr6, 0x82f23c08
	if !ctx.cr[6].lt {
	pc = 0x82F23C08; continue 'dispatch;
	}
	// 82F23ABC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82F23AC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F23AC4: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82F23AC8: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 82F23ACC: 40990100  ble cr6, 0x82f23bcc
	if !ctx.cr[6].gt {
	pc = 0x82F23BCC; continue 'dispatch;
	}
	// 82F23AD0: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 82F23AD4: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82F23AD8: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23ADC: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82F23AE0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82F23AE4: 4099005C  ble cr6, 0x82f23b40
	if !ctx.cr[6].gt {
	pc = 0x82F23B40; continue 'dispatch;
	}
	// 82F23AE8: 81380000  lwz r9, 0(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23AEC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23AF0: 7D054A14  add r8, r5, r9
	ctx.r[8].u64 = ctx.r[5].u64 + ctx.r[9].u64;
	// 82F23AF4: 7CE5482E  lwzx r7, r5, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F23AF8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23AFC: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F23B00: 409A0014  bne cr6, 0x82f23b14
	if !ctx.cr[6].eq {
	pc = 0x82F23B14; continue 'dispatch;
	}
	// 82F23B04: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23B08: 83A80004  lwz r29, 4(r8)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23B0C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82F23B10: 419A0030  beq cr6, 0x82f23b40
	if ctx.cr[6].eq {
	pc = 0x82F23B40; continue 'dispatch;
	}
	// 82F23B14: 83CB0004  lwz r30, 4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23B18: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F23B1C: 409A0010  bne cr6, 0x82f23b2c
	if !ctx.cr[6].eq {
	pc = 0x82F23B2C; continue 'dispatch;
	}
	// 82F23B20: 83C80004  lwz r30, 4(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23B24: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82F23B28: 419A0018  beq cr6, 0x82f23b40
	if ctx.cr[6].eq {
	pc = 0x82F23B40; continue 'dispatch;
	}
	// 82F23B2C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23B30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F23B34: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F23B38: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F23B3C: 4198FFBC  blt cr6, 0x82f23af8
	if ctx.cr[6].lt {
	pc = 0x82F23AF8; continue 'dispatch;
	}
	// 82F23B40: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F23B44: 409A0034  bne cr6, 0x82f23b78
	if !ctx.cr[6].eq {
	pc = 0x82F23B78; continue 'dispatch;
	}
	// 82F23B48: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82F23B4C: 419A0020  beq cr6, 0x82f23b6c
	if ctx.cr[6].eq {
	pc = 0x82F23B6C; continue 'dispatch;
	}
	// 82F23B50: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23B54: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82F23B58: 7D245A14  add r9, r4, r11
	ctx.r[9].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82F23B5C: 7D05582E  lwzx r8, r5, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F23B60: 7D04592E  stwx r8, r4, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[4].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82F23B64: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23B68: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F23B6C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F23B70: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 82F23B74: 48000044  b 0x82f23bb8
	pc = 0x82F23BB8; continue 'dispatch;
	// 82F23B78: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82F23B7C: 917A0004  stw r11, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F23B80: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23B84: 40980034  bge cr6, 0x82f23bb8
	if !ctx.cr[6].lt {
	pc = 0x82F23BB8; continue 'dispatch;
	}
	// 82F23B88: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F23B8C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23B90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F23B94: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F23B98: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82F23B9C: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F23BA0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F23BA4: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F23BA8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F23BAC: 80DA0004  lwz r6, 4(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23BB0: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82F23BB4: 4198FFD8  blt cr6, 0x82f23b8c
	if ctx.cr[6].lt {
	pc = 0x82F23B8C; continue 'dispatch;
	}
	// 82F23BB8: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23BBC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82F23BC0: 38A50008  addi r5, r5, 8
	ctx.r[5].s64 = ctx.r[5].s64 + 8;
	// 82F23BC4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23BC8: 4198FF10  blt cr6, 0x82f23ad8
	if ctx.cr[6].lt {
	pc = 0x82F23AD8; continue 'dispatch;
	}
	// 82F23BCC: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F23BD0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F23BD4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F23BD8: 40980024  bge cr6, 0x82f23bfc
	if !ctx.cr[6].lt {
	pc = 0x82F23BFC; continue 'dispatch;
	}
	// 82F23BDC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F23BE0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23BE4: 41980008  blt cr6, 0x82f23bec
	if ctx.cr[6].lt {
	pc = 0x82F23BEC; continue 'dispatch;
	}
	// 82F23BE8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F23BEC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F23BF0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F23BF4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82F23BF8: 4BF82C01  bl 0x82ea67f8
	ctx.lr = 0x82F23BFC;
	sub_82EA67F8(ctx, base);
	// 82F23BFC: 93F80004  stw r31, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82F23C00: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F23C04: 48284598  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 82F23C08: 4BF87D11  bl 0x82eab918
	ctx.lr = 0x82F23C0C;
	sub_82EAB918(ctx, base);
	// 82F23C0C: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23C10: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 82F23C14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F23C18: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82F23C1C: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F23C20: 55640036  rlwinm r4, r11, 0, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F23C24: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F23C28: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F23C2C: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F23C30: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F23C34: 41990010  bgt cr6, 0x82f23c44
	if ctx.cr[6].gt {
	pc = 0x82F23C44; continue 'dispatch;
	}
	// 82F23C38: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82F23C3C: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 82F23C40: 48000018  b 0x82f23c58
	pc = 0x82F23C58; continue 'dispatch;
	// 82F23C44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23C48: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F23C4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F23C50: 4E800421  bctrl
	ctx.lr = 0x82F23C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23C54: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F23C58: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F23C5C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82F23C60: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F23C64: 4BF87C75  bl 0x82eab8d8
	ctx.lr = 0x82F23C68;
	sub_82EAB8D8(ctx, base);
	// 82F23C68: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23C6C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 82F23C70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F23C74: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 82F23C78: 409900B0  ble cr6, 0x82f23d28
	if !ctx.cr[6].gt {
	pc = 0x82F23D28; continue 'dispatch;
	}
	// 82F23C7C: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82F23C80: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23C84: 7FDF582A  ldx r30, r31, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) };
	// 82F23C88: FBC10050  std r30, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u64 ) };
	// 82F23C8C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F23C90: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F23C94: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F23C98: 40990010  ble cr6, 0x82f23ca8
	if !ctx.cr[6].gt {
	pc = 0x82F23CA8; continue 'dispatch;
	}
	// 82F23C9C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F23CA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F23CA4: EBC10050  ld r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82F23CA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F23CAC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F23CB0: 4BF88349  bl 0x82eabff8
	ctx.lr = 0x82F23CB4;
	sub_82EABFF8(ctx, base);
	// 82F23CB4: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F23CB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F23CBC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23CC0: 40990008  ble cr6, 0x82f23cc8
	if !ctx.cr[6].gt {
	pc = 0x82F23CC8; continue 'dispatch;
	}
	// 82F23CC4: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82F23CC8: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82F23CCC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F23CD0: 419A002C  beq cr6, 0x82f23cfc
	if ctx.cr[6].eq {
	pc = 0x82F23CFC; continue 'dispatch;
	}
	// 82F23CD4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82F23CD8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F23CDC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F23CE0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F23CE4: 7D2B502A  ldx r9, r11, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) };
	// 82F23CE8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82F23CEC: 7D2B512A  stdx r9, r11, r10
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u64) };
	// 82F23CF0: 811A0000  lwz r8, 0(r26)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23CF4: 7EBF412E  stwx r21, r31, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32), ctx.r[21].u32) };
	// 82F23CF8: 4800001C  b 0x82f23d14
	pc = 0x82F23D14; continue 'dispatch;
	// 82F23CFC: 57AB402E  slwi r11, r29, 8
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F23D00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F23D04: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 82F23D08: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F23D0C: 61450001  ori r5, r10, 1
	ctx.r[5].u64 = ctx.r[10].u64 | 1;
	// 82F23D10: 4BF88209  bl 0x82eabf18
	ctx.lr = 0x82F23D14;
	sub_82EABF18(ctx, base);
	// 82F23D14: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23D18: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F23D1C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F23D20: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23D24: 4198FF5C  blt cr6, 0x82f23c80
	if ctx.cr[6].lt {
	pc = 0x82F23C80; continue 'dispatch;
	}
	// 82F23D28: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23D2C: 7EBCAB78  mr r28, r21
	ctx.r[28].u64 = ctx.r[21].u64;
	// 82F23D30: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 82F23D34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F23D38: 409900E0  ble cr6, 0x82f23e18
	if !ctx.cr[6].gt {
	pc = 0x82F23E18; continue 'dispatch;
	}
	// 82F23D3C: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 82F23D40: 7EBDAB78  mr r29, r21
	ctx.r[29].u64 = ctx.r[21].u64;
	// 82F23D44: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23D48: 7D5E582A  ldx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) };
	// 82F23D4C: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82F23D50: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F23D54: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F23D58: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F23D5C: 4099000C  ble cr6, 0x82f23d68
	if !ctx.cr[6].gt {
	pc = 0x82F23D68; continue 'dispatch;
	}
	// 82F23D60: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F23D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82F23D68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F23D6C: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82F23D70: 4BF88289  bl 0x82eabff8
	ctx.lr = 0x82F23D74;
	sub_82EABFF8(ctx, base);
	// 82F23D74: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F23D78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F23D7C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F23D80: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F23D84: 40990008  ble cr6, 0x82f23d8c
	if !ctx.cr[6].gt {
	pc = 0x82F23D8C; continue 'dispatch;
	}
	// 82F23D88: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 82F23D8C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82F23D90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F23D94: 419A004C  beq cr6, 0x82f23de0
	if ctx.cr[6].eq {
	pc = 0x82F23DE0; continue 'dispatch;
	}
	// 82F23D98: 7D6A2214  add r11, r10, r4
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82F23D9C: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F23DA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F23DA4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F23DA8: 7D6A482A  ldx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) };
	// 82F23DAC: 5568063E  clrlwi r8, r11, 0x18
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82F23DB0: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82F23DB4: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 82F23DB8: 40990010  ble cr6, 0x82f23dc8
	if !ctx.cr[6].gt {
	pc = 0x82F23DC8; continue 'dispatch;
	}
	// 82F23DBC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F23DC0: 7D6A492A  stdx r11, r10, r9
	unsafe { crate::rt::store_u64(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u64) };
	// 82F23DC4: 48000040  b 0x82f23e04
	pc = 0x82F23E04; continue 'dispatch;
	// 82F23DC8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F23DCC: 4BF8828D  bl 0x82eac058
	ctx.lr = 0x82F23DD0;
	sub_82EAC058(ctx, base);
	// 82F23DD0: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23DD4: 57EBD978  rlwinm r11, r31, 0x1b, 5, 0x1c
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 82F23DD8: 7EAB512E  stwx r21, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[21].u32) };
	// 82F23DDC: 48000028  b 0x82f23e04
	pc = 0x82F23E04; continue 'dispatch;
	// 82F23DE0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23DE4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F23DE8: 7D5D5A14  add r10, r29, r11
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[11].u64;
	// 82F23DEC: 7D3E5A14  add r9, r30, r11
	ctx.r[9].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82F23DF0: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 82F23DF4: 7D1E582E  lwzx r8, r30, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F23DF8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F23DFC: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23E00: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F23E04: 81780004  lwz r11, 4(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23E08: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82F23E0C: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82F23E10: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23E14: 4198FF30  blt cr6, 0x82f23d44
	if ctx.cr[6].lt {
	pc = 0x82F23D44; continue 'dispatch;
	}
	// 82F23E18: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F23E1C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F23E20: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82F23E24: 40980024  bge cr6, 0x82f23e48
	if !ctx.cr[6].lt {
	pc = 0x82F23E48; continue 'dispatch;
	}
	// 82F23E28: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F23E2C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23E30: 41980008  blt cr6, 0x82f23e38
	if ctx.cr[6].lt {
	pc = 0x82F23E38; continue 'dispatch;
	}
	// 82F23E34: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82F23E38: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F23E3C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F23E40: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82F23E44: 4BF829B5  bl 0x82ea67f8
	ctx.lr = 0x82F23E48;
	sub_82EA67F8(ctx, base);
	// 82F23E48: 93980004  stw r28, 4(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82F23E4C: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 82F23E50: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23E54: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82F23E58: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F23E5C: 40990050  ble cr6, 0x82f23eac
	if !ctx.cr[6].gt {
	pc = 0x82F23EAC; continue 'dispatch;
	}
	// 82F23E60: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 82F23E64: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 82F23E68: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23E6C: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F23E70: 7CC9502E  lwzx r6, r9, r10
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F23E74: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F23E78: 419A0020  beq cr6, 0x82f23e98
	if ctx.cr[6].eq {
	pc = 0x82F23E98; continue 'dispatch;
	}
	// 82F23E7C: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23E80: 7CA85214  add r5, r8, r10
	ctx.r[5].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82F23E84: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F23E88: 39080008  addi r8, r8, 8
	ctx.r[8].s64 = ctx.r[8].s64 + 8;
	// 82F23E8C: 90C50000  stw r6, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F23E90: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23E94: 90850004  stw r4, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F23E98: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23E9C: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82F23EA0: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82F23EA4: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23EA8: 4198FFC0  blt cr6, 0x82f23e68
	if ctx.cr[6].lt {
	pc = 0x82F23E68; continue 'dispatch;
	}
	// 82F23EAC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F23EB0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F23EB4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82F23EB8: 40980024  bge cr6, 0x82f23edc
	if !ctx.cr[6].lt {
	pc = 0x82F23EDC; continue 'dispatch;
	}
	// 82F23EBC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F23EC0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F23EC4: 41980008  blt cr6, 0x82f23ecc
	if ctx.cr[6].lt {
	pc = 0x82F23ECC; continue 'dispatch;
	}
	// 82F23EC8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82F23ECC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F23ED0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F23ED4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F23ED8: 4BF82921  bl 0x82ea67f8
	ctx.lr = 0x82F23EDC;
	sub_82EA67F8(ctx, base);
	// 82F23EDC: 93FA0004  stw r31, 4(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82F23EE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82F23EE4: 4BF87FFD  bl 0x82eabee0
	ctx.lr = 0x82F23EE8;
	sub_82EABEE0(ctx, base);
	// 82F23EE8: 7C77B02E  lwzx r3, r23, r22
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F23EEC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F23EF0: 93230020  stw r25, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 82F23EF4: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82F23EF8: 409A0018  bne cr6, 0x82f23f10
	if !ctx.cr[6].eq {
	pc = 0x82F23F10; continue 'dispatch;
	}
	// 82F23EFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23F00: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82F23F04: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F23F08: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F23F0C: 4E800421  bctrl
	ctx.lr = 0x82F23F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F23F10: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F23F14: 48284288  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23F18 size=8
    let mut pc: u32 = 0x82F23F18;
    'dispatch: loop {
        match pc {
            0x82F23F18 => {
    //   block [0x82F23F18..0x82F23F20)
	// 82F23F18: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 82F23F1C: 48000004  b 0x82f23f20
	sub_82F23F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F23F20 size=124
    let mut pc: u32 = 0x82F23F20;
    'dispatch: loop {
        match pc {
            0x82F23F20 => {
    //   block [0x82F23F20..0x82F23F9C)
	// 82F23F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F23F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F23F28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F23F2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F23F30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F23F34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F23F38: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82F23F3C: 409A0008  bne cr6, 0x82f23f44
	if !ctx.cr[6].eq {
	pc = 0x82F23F44; continue 'dispatch;
	}
	// 82F23F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F23F44: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82F23F48: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F23F4C: 390A41F8  addi r8, r10, 0x41f8
	ctx.r[8].s64 = ctx.r[10].s64 + 16888;
	// 82F23F50: 38E99EAC  addi r7, r9, -0x6154
	ctx.r[7].s64 = ctx.r[9].s64 + -24916;
	// 82F23F54: 548607FE  clrlwi r6, r4, 0x1f
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F23F58: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F23F5C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F23F60: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F23F64: 419A0020  beq cr6, 0x82f23f84
	if ctx.cr[6].eq {
	pc = 0x82F23F84; continue 'dispatch;
	}
	// 82F23F68: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23F6C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F23F70: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F23F74: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F23F78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F23F7C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F23F80: 4BF7C831  bl 0x82ea07b0
	ctx.lr = 0x82F23F84;
	sub_82EA07B0(ctx, base);
	// 82F23F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F23F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F23F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F23F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F23F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F23F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23FA0 size=64
    let mut pc: u32 = 0x82F23FA0;
    'dispatch: loop {
        match pc {
            0x82F23FA0 => {
    //   block [0x82F23FA0..0x82F23FE0)
	// 82F23FA0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F23FA4: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82F23FA8: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82F23FAC: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82F23FB0: 3CE08212  lis r7, -0x7dee
	ctx.r[7].s64 = -2112749568;
	// 82F23FB4: 38CB4294  addi r6, r11, 0x4294
	ctx.r[6].s64 = ctx.r[11].s64 + 17044;
	// 82F23FB8: 38AA4288  addi r5, r10, 0x4288
	ctx.r[5].s64 = ctx.r[10].s64 + 17032;
	// 82F23FBC: 38894268  addi r4, r9, 0x4268
	ctx.r[4].s64 = ctx.r[9].s64 + 17000;
	// 82F23FC0: 90C30000  stw r6, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F23FC4: 3968427C  addi r11, r8, 0x427c
	ctx.r[11].s64 = ctx.r[8].s64 + 17020;
	// 82F23FC8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F23FCC: 3947425C  addi r10, r7, 0x425c
	ctx.r[10].s64 = ctx.r[7].s64 + 16988;
	// 82F23FD0: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F23FD4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82F23FD8: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82F23FDC: 4B3C683C  b 0x822ea818
	sub_822EA818(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F23FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F23FE0 size=32
    let mut pc: u32 = 0x82F23FE0;
    'dispatch: loop {
        match pc {
            0x82F23FE0 => {
    //   block [0x82F23FE0..0x82F24000)
	// 82F23FE0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23FE4: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F23FE8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F23FEC: 392B0044  addi r9, r11, 0x44
	ctx.r[9].s64 = ctx.r[11].s64 + 68;
	// 82F23FF0: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F23FF4: 7CE8502E  lwzx r7, r8, r10
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F23FF8: 54E3BFFE  rlwinm r3, r7, 0x17, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[7].u32 as u64 & 0x000001FFu64;
	// 82F23FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F24000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F24000 size=392
    let mut pc: u32 = 0x82F24000;
    'dispatch: loop {
        match pc {
            0x82F24000 => {
    //   block [0x82F24000..0x82F24188)
	// 82F24000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F24004: 4828415D  bl 0x831a8160
	ctx.lr = 0x82F24008;
	sub_831A8130(ctx, base);
	// 82F24008: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2400C: 89640005  lbz r11, 5(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(5 as u32) ) } as u64;
	// 82F24010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F24014: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82F24018: 7F6B2214  add r27, r11, r4
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F2401C: 7F4B202E  lwzx r26, r11, r4
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82F24020: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82F24024: 419A0154  beq cr6, 0x82f24178
	if ctx.cr[6].eq {
	pc = 0x82F24178; continue 'dispatch;
	}
	// 82F24028: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F2402C: 54AB083C  slwi r11, r5, 1
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F24030: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F24034: 7D455A14  add r10, r5, r11
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[11].u64;
	// 82F24038: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F2403C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82F24040: 555D2036  slwi r29, r10, 4
	ctx.r[29].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82F24044: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F24048: 7CABEA14  add r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82F2404C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F24050: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F24054: 4E800421  bctrl
	ctx.lr = 0x82F24058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F24058: 88E30000  lbz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F2405C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F24060: 419A0118  beq cr6, 0x82f24178
	if ctx.cr[6].eq {
	pc = 0x82F24178; continue 'dispatch;
	}
	// 82F24064: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F24068: 3D208212  lis r9, -0x7dee
	ctx.r[9].s64 = -2112749568;
	// 82F2406C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F24070: 38BF0010  addi r5, r31, 0x10
	ctx.r[5].s64 = ctx.r[31].s64 + 16;
	// 82F24074: 395E0030  addi r10, r30, 0x30
	ctx.r[10].s64 = ctx.r[30].s64 + 48;
	// 82F24078: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F2407C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82F24080: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 82F24084: 3B9E0020  addi r28, r30, 0x20
	ctx.r[28].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F24188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F24188 size=296
    let mut pc: u32 = 0x82F24188;
    'dispatch: loop {
        match pc {
            0x82F24188 => {
    //   block [0x82F24188..0x82F242B0)
	// 82F24188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F2418C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F24190: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F24194: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F24198: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2419C: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F241A0: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F241A4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82F241A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F241AC: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F241B0: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F241B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F241B8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F241BC: 40980020  bge cr6, 0x82f241dc
	if !ctx.cr[6].lt {
	pc = 0x82F241DC; continue 'dispatch;
	}
	// 82F241C0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F241C4: 3888ED64  addi r4, r8, -0x129c
	ctx.r[4].s64 = ctx.r[8].s64 + -4764;
	// 82F241C8: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F241CC: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F241D0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F241D4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F241D8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F241DC: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F241E0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F241E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F241E8: 90E9000C  stw r7, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F241EC: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82F241F0: 409A0008  bne cr6, 0x82f241f8
	if !ctx.cr[6].eq {
	pc = 0x82F241F8; continue 'dispatch;
	}
	// 82F241F4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82F241F8: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F241FC: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F24200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F24204: 419A001C  beq cr6, 0x82f24220
	if ctx.cr[6].eq {
	pc = 0x82F24220; continue 'dispatch;
	}
	// 82F24208: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F2420C: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82F24210: 409A0008  bne cr6, 0x82f24218
	if !ctx.cr[6].eq {
	pc = 0x82F24218; continue 'dispatch;
	}
	// 82F24214: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82F24218: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F2421C: 48000008  b 0x82f24224
	pc = 0x82F24224; continue 'dispatch;
	// 82F24220: 91490034  stw r10, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82F24224: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F24228: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F242B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F242B0 size=260
    let mut pc: u32 = 0x82F242B0;
    'dispatch: loop {
        match pc {
            0x82F242B0 => {
    //   block [0x82F242B0..0x82F243B4)
	// 82F242B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F242B4: 48283EB9  bl 0x831a816c
	ctx.lr = 0x82F242B8;
	sub_831A8130(ctx, base);
	// 82F242B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F242BC: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F242C0: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F242C4: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82F242C8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F242CC: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F242D0: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F242D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F242D8: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82F242DC: 40980020  bge cr6, 0x82f242fc
	if !ctx.cr[6].lt {
	pc = 0x82F242FC; continue 'dispatch;
	}
	// 82F242E0: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F242E4: 3884ED74  addi r4, r4, -0x128c
	ctx.r[4].s64 = ctx.r[4].s64 + -4748;
	// 82F242E8: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F242EC: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F242F0: 3BAA000C  addi r29, r10, 0xc
	ctx.r[29].s64 = ctx.r[10].s64 + 12;
	// 82F242F4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F242F8: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82F242FC: 90A90004  stw r5, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F24300: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F24304: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82F24308: 39660014  addi r11, r6, 0x14
	ctx.r[11].s64 = ctx.r[6].s64 + 20;
	// 82F2430C: 409A0008  bne cr6, 0x82f24314
	if !ctx.cr[6].eq {
	pc = 0x82F24314; continue 'dispatch;
	}
	// 82F24310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F24314: 91690008  stw r11, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F24318: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F2431C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F24320: 419A0010  beq cr6, 0x82f24330
	if ctx.cr[6].eq {
	pc = 0x82F24330; continue 'dispatch;
	}
	// 82F24324: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F24328: 39660010  addi r11, r6, 0x10
	ctx.r[11].s64 = ctx.r[6].s64 + 16;
	// 82F2432C: 409A0008  bne cr6, 0x82f24334
	if !ctx.cr[6].eq {
	pc = 0x82F24334; continue 'dispatch;
	}
	// 82F24330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F24334: 91690034  stw r11, 0x34(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F24338: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F2433C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F243B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F243B8 size=452
    let mut pc: u32 = 0x82F243B8;
    'dispatch: loop {
        match pc {
            0x82F243B8 => {
    //   block [0x82F243B8..0x82F2457C)
	// 82F243B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F243BC: 48283D95  bl 0x831a8150
	ctx.lr = 0x82F243C0;
	sub_831A8130(ctx, base);
	// 82F243C0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F243C4: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F243C8: 3AC00018  li r22, 0x18
	ctx.r[22].s64 = 24;
	// 82F243CC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82F243D0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82F243D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F243D8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F243DC: 7D7AB02E  lwzx r11, r26, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F243E0: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82F243E4: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 82F243E8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F243EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F243F0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F243F4: 40980020  bge cr6, 0x82f24414
	if !ctx.cr[6].lt {
	pc = 0x82F24414; continue 'dispatch;
	}
	// 82F243F8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F243FC: 3909ED84  addi r8, r9, -0x127c
	ctx.r[8].s64 = ctx.r[9].s64 + -4732;
	// 82F24400: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F24404: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F24408: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F2440C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F24410: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F24414: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F24580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F24580 size=260
    let mut pc: u32 = 0x82F24580;
    'dispatch: loop {
        match pc {
            0x82F24580 => {
    //   block [0x82F24580..0x82F24684)
	// 82F24580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F24584: 48283BE5  bl 0x831a8168
	ctx.lr = 0x82F24588;
	sub_831A8130(ctx, base);
	// 82F24588: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F2458C: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F24590: 3BC00018  li r30, 0x18
	ctx.r[30].s64 = 24;
	// 82F24594: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F24598: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F2459C: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82F245A0: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F245A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F245A8: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82F245AC: 40980020  bge cr6, 0x82f245cc
	if !ctx.cr[6].lt {
	pc = 0x82F245CC; continue 'dispatch;
	}
	// 82F245B0: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F245B4: 3886ED94  addi r4, r6, -0x126c
	ctx.r[4].s64 = ctx.r[6].s64 + -4716;
	// 82F245B8: 908A0000  stw r4, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F245BC: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F245C0: 38CA000C  addi r6, r10, 0xc
	ctx.r[6].s64 = ctx.r[10].s64 + 12;
	// 82F245C4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F245C8: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F245CC: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F245D0: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F245D4: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82F245D8: 39670014  addi r11, r7, 0x14
	ctx.r[11].s64 = ctx.r[7].s64 + 20;
	// 82F245DC: 409A0008  bne cr6, 0x82f245e4
	if !ctx.cr[6].eq {
	pc = 0x82F245E4; continue 'dispatch;
	}
	// 82F245E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F245E4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82F245E8: 89650020  lbz r11, 0x20(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F245EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F245F0: 419A0010  beq cr6, 0x82f24600
	if ctx.cr[6].eq {
	pc = 0x82F24600; continue 'dispatch;
	}
	// 82F245F4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F245F8: 39670010  addi r11, r7, 0x10
	ctx.r[11].s64 = ctx.r[7].s64 + 16;
	// 82F245FC: 409A0008  bne cr6, 0x82f24604
	if !ctx.cr[6].eq {
	pc = 0x82F24604; continue 'dispatch;
	}
	// 82F24600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F24604: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82F24608: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F2460C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F24610: 39250010  addi r9, r5, 0x10
	ctx.r[9].s64 = ctx.r[5].s64 + 16;
	// 82F24614: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F24688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F24688 size=16
    let mut pc: u32 = 0x82F24688;
    'dispatch: loop {
        match pc {
            0x82F24688 => {
    //   block [0x82F24688..0x82F24698)
	// 82F24688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F2468C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F24690: 99640000  stb r11, 0(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F24694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F24698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F24698 size=4
    let mut pc: u32 = 0x82F24698;
    'dispatch: loop {
        match pc {
            0x82F24698 => {
    //   block [0x82F24698..0x82F2469C)
	// 82F24698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F246A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F246A0 size=8
    let mut pc: u32 = 0x82F246A0;
    'dispatch: loop {
        match pc {
            0x82F246A0 => {
    //   block [0x82F246A0..0x82F246A8)
	// 82F246A0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F246A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F246A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F246A8 size=360
    let mut pc: u32 = 0x82F246A8;
    'dispatch: loop {
        match pc {
            0x82F246A8 => {
    //   block [0x82F246A8..0x82F24810)
	// 82F246A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F246AC: 48283AC1  bl 0x831a816c
	ctx.lr = 0x82F246B0;
	sub_831A8130(ctx, base);
	// 82F246B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F246B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F246B8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F246BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F246C0: 395F0EB0  addi r10, r31, 0xeb0
	ctx.r[10].s64 = ctx.r[31].s64 + 3760;
	// 82F246C4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82F246C8: 90FF0190  stw r7, 0x190(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(400 as u32), ctx.r[7].u32 ) };
	// 82F246CC: 3BC00064  li r30, 0x64
	ctx.r[30].s64 = 100;
	// 82F246D0: 90FF0EA0  stw r7, 0xea0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3744 as u32), ctx.r[7].u32 ) };
	// 82F246D4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82F246D8: 813F1E30  lwz r9, 0x1e30(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7728 as u32) ) } as u64;
	// 82F246DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F246E0: 419A0030  beq cr6, 0x82f24710
	if ctx.cr[6].eq {
	pc = 0x82F24710; continue 'dispatch;
	}
	// 82F246E4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F246E8: 9BC90002  stb r30, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82F246EC: 813F1E34  lwz r9, 0x1e34(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7732 as u32) ) } as u64;
	// 82F246F0: 7CC95A14  add r6, r9, r11
	ctx.r[6].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F246F4: 9BC60002  stb r30, 2(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82F246F8: 813F1E38  lwz r9, 0x1e38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82F246FC: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F24700: 9BC50002  stb r30, 2(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82F24704: 813F1E3C  lwz r9, 0x1e3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82F24708: 7C895A14  add r4, r9, r11
	ctx.r[4].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82F2470C: 9BC40002  stb r30, 2(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82F24710: 9BAAF2F0  stb r29, -0xd10(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-3344 as u32), ctx.r[29].u8 ) };
	// 82F24714: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F24718: 9BAA0000  stb r29, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82F2471C: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82F24720: 9BAAF6F0  stb r29, -0x910(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(-2320 as u32), ctx.r[29].u8 ) };
	// 82F24724: 9BAA0400  stb r29, 0x400(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(1024 as u32), ctx.r[29].u8 ) };
	// 82F24728: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F2472C: 4082FFAC  bne 0x82f246d8
	if !ctx.cr[0].eq {
	pc = 0x82F246D8; continue 'dispatch;
	}
	// 82F24730: 2F0B0C00  cmpwi cr6, r11, 0xc00
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3072, &mut ctx.xer);
	// 82F24734: 4198FFA0  blt cr6, 0x82f246d4
	if ctx.cr[6].lt {
	pc = 0x82F246D4; continue 'dispatch;
	}
	// 82F24738: 3D6082F2  lis r11, -0x7d0e
	ctx.r[11].s64 = -2098069504;
	// 82F2473C: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 82F24740: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 82F24744: 3D0082F2  lis r8, -0x7d0e
	ctx.r[8].s64 = -2098069504;
	// 82F24748: 38CB61F0  addi r6, r11, 0x61f0
	ctx.r[6].s64 = ctx.r[11].s64 + 25072;
	// 82F2474C: 386861E8  addi r3, r8, 0x61e8
	ctx.r[3].s64 = ctx.r[8].s64 + 25064;
	// 82F24750: 38AA61E0  addi r5, r10, 0x61e0
	ctx.r[5].s64 = ctx.r[10].s64 + 25056;
	// 82F24754: 90DF09A0  stw r6, 0x9a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2464 as u32), ctx.r[6].u32 ) };
	// 82F24758: 388961D8  addi r4, r9, 0x61d8
	ctx.r[4].s64 = ctx.r[9].s64 + 25048;
	// 82F2475C: 907F09AC  stw r3, 0x9ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2476 as u32), ctx.r[3].u32 ) };
	// 82F24760: 90BF09A4  stw r5, 0x9a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2468 as u32), ctx.r[5].u32 ) };
	// 82F24764: 3D6082F2  lis r11, -0x7d0e
	ctx.r[11].s64 = -2098069504;
	// 82F24768: 909F09A8  stw r4, 0x9a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2472 as u32), ctx.r[4].u32 ) };
	// 82F2476C: 3D4082F2  lis r10, -0x7d0e
	ctx.r[10].s64 = -2098069504;
	// 82F24770: 3D2082F2  lis r9, -0x7d0e
	ctx.r[9].s64 = -2098069504;
	// 82F24774: 9BBF09B0  stb r29, 0x9b0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2480 as u32), ctx.r[29].u8 ) };
	// 82F24778: 98FF09B1  stb r7, 0x9b1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2481 as u32), ctx.r[7].u8 ) };
	// 82F2477C: 390B4688  addi r8, r11, 0x4688
	ctx.r[8].s64 = ctx.r[11].s64 + 18056;
	// 82F24780: 38EA4698  addi r7, r10, 0x4698
	ctx.r[7].s64 = ctx.r[10].s64 + 18072;
	// 82F24784: 93BF16B8  stw r29, 0x16b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5816 as u32), ctx.r[29].u32 ) };
	// 82F24788: 38C946A0  addi r6, r9, 0x46a0
	ctx.r[6].s64 = ctx.r[9].s64 + 18080;
	// 82F2478C: 911F16B0  stw r8, 0x16b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5808 as u32), ctx.r[8].u32 ) };
	// 82F24790: 90FF16B4  stw r7, 0x16b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5812 as u32), ctx.r[7].u32 ) };
	// 82F24794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F24798: 93BF16BC  stw r29, 0x16bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5820 as u32), ctx.r[29].u32 ) };
	// 82F2479C: 93BF16C0  stw r29, 0x16c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5824 as u32), ctx.r[29].u32 ) };
	// 82F247A0: 93BF16C4  stw r29, 0x16c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5828 as u32), ctx.r[29].u32 ) };
	// 82F247A4: 93BF16C8  stw r29, 0x16c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5832 as u32), ctx.r[29].u32 ) };
	// 82F247A8: 93BF16CC  stw r29, 0x16cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5836 as u32), ctx.r[29].u32 ) };
	// 82F247AC: 93BF16D8  stw r29, 0x16d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5848 as u32), ctx.r[29].u32 ) };
	// 82F247B0: 93BF16D0  stw r29, 0x16d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5840 as u32), ctx.r[29].u32 ) };
	// 82F247B4: 93BF16D4  stw r29, 0x16d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5844 as u32), ctx.r[29].u32 ) };
	// 82F247B8: 90DF16DC  stw r6, 0x16dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5852 as u32), ctx.r[6].u32 ) };
	// 82F247BC: 93BF16F0  stw r29, 0x16f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(5872 as u32), ctx.r[29].u32 ) };
	// 82F247C0: 480254C9  bl 0x82f49c88
	ctx.lr = 0x82F247C4;
	sub_82F49C88(ctx, base);
	// 82F247C4: 9BBF1E20  stb r29, 0x1e20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7712 as u32), ctx.r[29].u8 ) };
	// 82F247C8: 80BF1E38  lwz r5, 0x1e38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82F247CC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82F247D0: 419A0038  beq cr6, 0x82f24808
	if ctx.cr[6].eq {
	pc = 0x82F24808; continue 'dispatch;
	}
	// 82F247D4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82F247D8: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82F247DC: 813F1E38  lwz r9, 0x1e38(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7736 as u32) ) } as u64;
	// 82F247E0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F247E4: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F247E8: 9BC90002  stb r30, 2(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82F247EC: 813F1E3C  lwz r9, 0x1e3c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7740 as u32) ) } as u64;
	// 82F247F0: 7D0B4A14  add r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F247F4: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 82F247F8: 9BC80002  stb r30, 2(r8)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[30].u8 ) };
	// 82F247FC: 4082FFE0  bne 0x82f247dc
	if !ctx.cr[0].eq {
	pc = 0x82F247DC; continue 'dispatch;
	}
	// 82F24800: 2F0B0C00  cmpwi cr6, r11, 0xc00
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3072, &mut ctx.xer);
	// 82F24804: 4198FFD4  blt cr6, 0x82f247d8
	if ctx.cr[6].lt {
	pc = 0x82F247D8; continue 'dispatch;
	}
	// 82F24808: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F2480C: 482839B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


